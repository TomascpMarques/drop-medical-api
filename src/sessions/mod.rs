use sqlx::types::chrono;
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie, Cookies,
};
use tracing::{error, info, warn};

use crate::models::users::{User, UserSession};

pub mod auth;
/// All axum trait implementation, for a session struct, is located in this file.
mod axum_impls;
pub mod middleware;

pub const SESSION_COOKIE_NAME: &'static str = "SESSION";

pub type Result<T> = std::result::Result<T, SessionError>;

/// # About
/// Custom impl of a session manager using cookies to keep track of the session in the client-side,
/// and in de server side using a Postgre instance.
#[derive(Clone, Copy, Default)]
pub struct SessionManager;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Nao existe sessão para este Uuid")]
    NoSessionForGivenId,
    #[error("Falha ao criar sessão")]
    FailedToCreateSession,
    #[error("Ao tentar extender a sessão do utilizador")]
    FailedToExtendUserSession,
    #[error("Ao tentar excluir a sessão do utilizador")]
    FailedToDeleteUserSession,
    #[error("Could not get user's ID from user session")]
    FailedToRetrieveUserIdFromSession,
}

impl SessionManager {
    pub async fn create_new_session(
        pg_pool: &sqlx::postgres::PgPool,
        user: &User,
    ) -> Result<uuid::Uuid> {
        let expiration_time = chrono::Utc::now();

        let query_result = sqlx::query!(
            r#"INSERT INTO "user_session" (user_id, expires_in) VALUES ($1, $2) RETURNING id"#,
            user.id().unwrap(),
            expiration_time,
        )
        .fetch_one(pg_pool)
        .await;

        if query_result.is_ok() {
            return Ok(query_result.unwrap().id);
        }

        match query_result.unwrap_err() {
            sqlx::Error::Database(err) if err.is_unique_violation() => {
                // Need to create_new_session
                info!("Dropping old session, and creating anew");

                Self::drop_session(pg_pool, user.id().unwrap()).await?;

                let query_result = sqlx::query!(
                    r#"INSERT INTO "user_session" (user_id, expires_in) VALUES ($1, $2) RETURNING id"#,
                    user.id().unwrap(),
                    expiration_time,
                )
                .fetch_one(pg_pool)
                .await.expect("Should not fail to create this session");

                Ok(query_result.id)
            }
            e => {
                warn!("Possible db error: {e}");
                Err(SessionError::FailedToCreateSession)
            }
        }
    }

    pub async fn drop_session(pg_pool: &sqlx::postgres::PgPool, user_id: uuid::Uuid) -> Result<()> {
        let query_result =
            sqlx::query!(r#"DELETE FROM "user_session" where user_id = $1;"#, user_id,)
                .execute(pg_pool)
                .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Possible error: {e}");
                Err(SessionError::FailedToDeleteUserSession)
            }
        }
    }

    pub fn create_session_cookie<'a>(
        cookies: &Cookies,
        session_id: uuid::Uuid,
        expires_offset: Duration,
    ) {
        cookies.remove(Cookie::new(SESSION_COOKIE_NAME, ""));

        let cookie = Cookie::build((SESSION_COOKIE_NAME, session_id.to_string()))
            .domain("www.dropmedical.com")
            .expires(OffsetDateTime::now_utc().checked_add(expires_offset))
            .http_only(true)
            .max_age(Duration::days(1))
            .build();

        cookies.add(cookie);
    }

    async fn check_valid_session(
        pg_pool: &sqlx::postgres::PgPool,
        session_id: uuid::Uuid,
    ) -> Result<bool> {
        let user_sesh = sqlx::query_as!(
            UserSession,
            r#"SELECT * FROM user_session WHERE id = $1"#,
            session_id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(|err| {
            warn!("Possible DB error: {err}");
            SessionError::NoSessionForGivenId
        })?;

        if &chrono::Utc::now() > user_sesh.expires_in() {
            sqlx::query!(
                r#"DELETE FROM "user_session" where user_id = $1;"#,
                user_sesh.id(),
            )
            .execute(pg_pool)
            .await
            .map_err(|err| {
                warn!("Possible DB error: {err}");
                SessionError::FailedToDeleteUserSession
            })?;

            return Ok(false);
        }

        Ok(true)
    }

    pub async fn extend_session(
        pg_pool: &sqlx::postgres::PgPool,
        session_id: uuid::Uuid,
    ) -> Result<()> {
        let mut user_sesh = sqlx::query_as!(
            UserSession,
            r#"SELECT * FROM user_session WHERE id = $1"#,
            session_id
        )
        .fetch_one(pg_pool)
        .await
        .map_err(|err| {
            warn!("Possible DB error: {err}");
            SessionError::NoSessionForGivenId
        })?;

        // Defaults to 20 more minutes in current session
        user_sesh.extend_session(None);

        sqlx::query!(
            r#"UPDATE "user_session" set expires_in = $1 where user_id = $2;"#,
            user_sesh.expires_in(),
            user_sesh.id(),
        )
        .execute(pg_pool)
        .await
        .map_err(|err| {
            warn!("Possible DB error: {err}");
            SessionError::FailedToExtendUserSession
        })?;

        Ok(())
    }
}

/// UserSessionIdExtractor
/// # About
/// Extract the session id _([Uuid](uuid::Uuid) v4 value)_ from the session cookie.
/// To achieve this it implements the trait [FromRequestParts](axum::extract::FromRequestParts)
/// from the axum crate;
pub struct UserSessionIdExtractor(uuid::Uuid);

impl UserSessionIdExtractor {
    pub async fn get_user_id(&self, pg_pool: &sqlx::PgPool) -> Result<uuid::Uuid> {
        let query_res = sqlx::query!(
            r#"select user_id from "user_session" where id = $1"#,
            self.0
        )
        .fetch_one(pg_pool)
        .await
        .map_err(|err| {
            error!("Could not get user id from user's session id: {err}");
            SessionError::FailedToRetrieveUserIdFromSession
        })?;

        Ok(query_res.user_id)
    }
}

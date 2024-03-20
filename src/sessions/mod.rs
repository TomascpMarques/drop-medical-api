use crate::models::{User, UserSession};
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie, Cookies,
};

mod axum_impls;

pub const SESSION_COOKIE_NAME: &'static str = "SESSION";

#[derive(Clone, Copy, Default)]
pub struct SessionManager;

impl SessionManager {
    pub async fn create_new_session(
        pg_con: &sqlx::postgres::PgPool,
        user: &User,
    ) -> Result<uuid::Uuid, sqlx::Error> {
        let expiration_time = chrono::Utc::now();

        let query_result = sqlx::query!(
            r#"INSERT INTO "user_session" (user_id, expires_in) VALUES ($1, $2) RETURNING id"#,
            user.id().unwrap(),
            expiration_time,
        )
        .fetch_one(pg_con)
        .await?;

        Ok(query_result.id)
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

    pub async fn _check_session_exists(
        pg_pool: &sqlx::postgres::PgPool,
        session_id: uuid::Uuid,
    ) -> Result<bool, sqlx::Error> {
        _ = sqlx::query_as!(
            UserSession,
            r#"select * from user_session where id = $1"#,
            session_id
        )
        .fetch_one(pg_pool)
        .await?;

        Ok(true)
    }
}

struct UserSessionIdExtractor(uuid::Uuid);

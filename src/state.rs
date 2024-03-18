use crate::models::User;
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie, Cookies,
};

#[derive(Clone, getset::Getters)]
#[getset(get = "pub")]
pub struct AppStateManager {
    db_pool: sqlx::postgres::PgPool,
}

impl AppStateManager {
    pub fn new(db_con: sqlx::postgres::PgPool) -> Self {
        Self { db_pool: db_con }
    }
}

impl AppStateManager {
    pub fn create_session_cookie<'a>(
        cookies: &Cookies,
        session_id: uuid::Uuid,
        expires_offset: Duration,
    ) {
        let cookie = Cookie::build(("SESSION", session_id.to_string()))
            .domain("www.dropmedical.com")
            .expires(OffsetDateTime::now_utc().checked_add(expires_offset))
            .http_only(true)
            .max_age(Duration::days(1))
            .build();

        cookies.remove(Cookie::new("SESSION", ""));
        cookies.add(cookie);
    }

    pub async fn create_new_session(&self, user: &User) -> Result<uuid::Uuid, sqlx::Error> {
        let session_id = uuid::Uuid::new_v4();

        let expiration_time = chrono::Utc::now().to_rfc3339();

        sqlx::query!(
            r#"INSERT INTO "user_session" (user_id, expires_in) VALUES ($1, $2)"#,
            user.id(),
            expiration_time,
        )
        .execute(&self.db_pool)
        .await?;

        Ok(session_id)
    }
}

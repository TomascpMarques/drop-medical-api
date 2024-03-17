use crate::models::User;
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie, Cookies,
};
use uuid::NoContext;

#[derive(Clone, getset::Getters)]
#[getset(get = "pub")]
pub struct AppStateManager {
    db_con: libsql::Connection,
}

impl AppStateManager {
    pub fn new(db_con: libsql::Connection) -> Self {
        Self { db_con }
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

    pub async fn create_new_session(&self, user: &User) -> Result<uuid::Uuid, libsql::Error> {
        let ts = uuid::Timestamp::now(NoContext);
        let session_id = uuid::Uuid::new_v7(ts);

        let expiration_time = chrono::Utc::now().to_rfc3339();

        let mut preped_stmt = self
            .db_con
            .prepare(r#"INSERT INTO user_session (id, user_id, expires_in) values (?1, ?2, ?3)"#)
            .await?;

        preped_stmt
            .execute((
                session_id.to_bytes_le().to_vec(),
                user.id().to_bytes_le().to_vec(),
                expiration_time,
            ))
            .await?;

        Ok(session_id)
    }
}

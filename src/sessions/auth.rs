use tracing::warn;

use crate::models::users::User;

pub type Result<T> = std::result::Result<T, AuthenticationError>;

#[derive(Debug, serde::Serialize, thiserror::Error)]
pub enum AuthenticationError {
    #[error("Password invalida para o email fornecido")]
    InvalidPasswordForEmail { email: String },
}

// We are not worrying about sql injections now
// Or password hashing
pub async fn authenticate_user<'a>(
    pg_pool: &sqlx::PgPool,
    email: &'a str,
    password: &'a str,
) -> Result<User> {
    let usr = sqlx::query!(
        r#"select * from "user" where email = $1 and password = $2"#,
        email,
        password
    )
    .fetch_one(pg_pool)
    .await
    .map_err(|err| {
        warn!("Possible DB error: {err}");
        AuthenticationError::InvalidPasswordForEmail {
            email: email.into(),
        }
    })?;

    let usr = User {
        id: Some(usr.id),
        name: usr.name,
        email: usr.email,
        password: usr.password,
    };

    Ok(usr)
}

// TODO Log out user

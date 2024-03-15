use std::ops::Deref;

use uuid::{NoContext, Timestamp};

#[derive(Debug, serde::Deserialize, serde::Serialize, getset::Getters)]
#[getset(get = "pub")]
pub struct User {
    id: uuid::Uuid,
    name: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        let ts = Timestamp::now(NoContext);
        Self {
            id: uuid::Uuid::new_v7(ts),
            name,
            email,
            password,
        }
    }

    fn insert_sql(&self) -> &'static str {
        r#"INSERT INTO USER (id, name, email, password) VALUES (?1, ?2, ?3, ?4)"#
    }
    fn delete_sql(&self) -> &'static str {
        r#"DELETE FROM USER where id = ?1"#
    }
}

pub async fn create_user(con: &libsql::Connection, user: &User) -> anyhow::Result<()> {
    con.execute(
        user.insert_sql(),
        [
            libsql::Value::Blob(user.id().to_bytes_le().to_vec()),
            user.name().deref().into(),
            user.email().deref().into(),
            user.password().deref().into(),
        ],
    )
    .await?;
    Ok(())
}

#[derive(Debug, serde::Deserialize, serde::Serialize, getset::Getters, getset::MutGetters)]
#[getset(get = "pub")]
pub struct User {
    #[getset(get_mut = "pub")]
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: None,
            name,
            email,
            password,
        }
    }
}

pub fn serialize_utc_to_rfc_3339<'a, S>(
    time: &'a chrono::DateTime<chrono::Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let retn = time.to_rfc3339();
    serializer.serialize_str(retn.as_str())
}

#[derive(Debug, serde::Deserialize, serde::Serialize, getset::Getters, getset::Setters)]
#[getset(get = "pub", set)]
pub struct UserSession {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    #[serde(serialize_with = "serialize_utc_to_rfc_3339")]
    pub expires_in: chrono::DateTime<chrono::Utc>,
}

impl UserSession {
    pub fn new(
        id: uuid::Uuid,
        user_id: uuid::Uuid,
        expires_in: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            expires_in,
        }
    }

    pub fn extend_session(&mut self, duration: Option<std::time::Duration>) {
        let duration = duration.unwrap_or(std::time::Duration::from_secs(60 * 20));
        self.expires_in += duration;
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, getset::Getters, getset::MutGetters)]
#[getset(get = "pub")]
pub struct User {
    #[getset(get_mut = "pub")]
    id: Option<uuid::Uuid>,
    name: String,
    email: String,
    password: String,
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

#[derive(Debug, serde::Deserialize, serde::Serialize, getset::Getters)]
#[getset(get = "pub")]
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
}

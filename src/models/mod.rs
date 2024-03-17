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
        let id = uuid::Uuid::new_v7(ts);

        Self {
            id,
            name,
            email,
            password,
        }
    }
}

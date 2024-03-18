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
        let id = uuid::Uuid::new_v4();
        Self {
            id,
            name,
            email,
            password,
        }
    }
}

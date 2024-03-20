#[derive(Clone, getset::Getters)]
#[getset(get = "pub")]
pub struct AppStateManager {
    db_pool: sqlx::postgres::PgPool,
}

impl AppStateManager {
    pub fn new(db_pool: sqlx::postgres::PgPool) -> Self {
        Self { db_pool }
    }
}

use crate::configuration::{Environment, Settings};
use sqlx::postgres::PgPoolOptions;

pub async fn setup_db(config: &Settings) -> Result<sqlx::PgPool, sqlx::Error> {
    let db_uri = config.database().uri();

    let pg_pool_options = PgPoolOptions::new().max_connections(5).min_connections(1);
    match config.env() {
        Environment::Local => pg_pool_options.connect(db_uri).await,
        Environment::Production => pg_pool_options.connect_lazy(db_uri),
    }
}

pub fn connection_str_without_db(config: &Settings) -> String {
    let db_uri = config.database().uri();
    let x = db_uri.split_inclusive('/');
    let x = x.last().unwrap();
    let x = db_uri.replace(x, "");
    x.to_string()
}

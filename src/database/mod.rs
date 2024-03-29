use crate::configuration::{Environment, Settings};
use sqlx::{postgres::PgConnectOptions, PgPool};

pub async fn setup_db(config: &Settings) -> Result<sqlx::PgPool, sqlx::Error> {
    let db_uri = config.database().uri();

    let mut opts: PgConnectOptions = db_uri.parse().map_err(|err| {
        println!("Falha ao parssing do url da db");
        err
    })?;
    opts = opts.ssl_mode(sqlx::postgres::PgSslMode::Require);

    match config.env() {
        Environment::Local => PgPool::connect_with(opts.clone()).await,
        Environment::Production => PgPool::connect_with(opts).await,
    }
}

pub fn connection_str_without_db(config: &Settings) -> String {
    let db_uri = config.database().uri();
    let x = db_uri.split_inclusive('/');
    let x = x.last().unwrap();
    let x = db_uri.replace(x, "");
    x.to_string()
}

use std::env;

use libsql::Builder;
use tracing::info;

use crate::configuration::{Environment, Settings};

async fn get_db(config: &Settings) -> Result<libsql::Database, libsql::Error> {
    let url = config.database().uri();
    let token = env::var("TURSO_TOKEN").expect("TURSO_TOKEN must be set");

    match config.env() {
        Environment::Local => Builder::new_local(url.clone()).build().await,
        Environment::Production => Builder::new_remote(url.to_owned(), token).build().await,
    }
}

async fn build_db(connection: &libsql::Connection) -> Result<u64, libsql::Error> {
    info!("Populating database...");
    let sql = include_str!("../../database/schemas/main.sql").trim();
    connection.execute(sql, ()).await
}

pub async fn setup_turso(config: &Settings) -> Result<libsql::Database, libsql::Error> {
    let result = get_db(config).await?;

    let connection = result.connect()?;
    build_db(&connection).await?;

    Ok(result)
}

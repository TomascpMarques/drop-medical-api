use std::sync::Arc;

use libsql::Builder;
use tokio::sync::RwLock;

use crate::configuration::Settings;

use crate::configuration::Environment as Env;

pub type SyncDbConnection = Arc<RwLock<libsql::Connection>>;

pub async fn setup_db_connection<'a>(
    settings: &'a Settings,
) -> Result<libsql::Database, libsql::Error> {
    let url = settings.database().uri();
    let token = std::env::var("TURSO_TOKEN").expect("No turso auth token found");

    match settings.env() {
        Env::Local => {
            // Sets up the schema in local dev
            let temp_db = Builder::new_local(url).build().await?;
            temp_db
                .connect()?
                .execute(include_str!("../database/schemas/main.sql"), ())
                .await?;
            Builder::new_local(url).build().await
        }
        Env::Production => Builder::new_remote(url.to_owned(), token).build().await,
    }
}

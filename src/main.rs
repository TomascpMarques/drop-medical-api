use drop_medical_api::{configuration, database, setup_app_router};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Application access to env variables
    dotenvy::dotenv()?;

    // Server tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Server conf
    let config = configuration::get_config()?;

    // DB setup
    let db_pool = database::setup_db(&config)
        .await
        .expect("Failed to build DB");

    let app = setup_app_router(db_pool)?;

    let address = format!(
        "{}:{}",
        config.application().host(),
        config.application().port()
    );

    info!("Server listening on: http://{address}/");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app)
        .await
        .expect("Failed to run server");

    Ok(())
}

use anyhow::Context;
use drop_medical_api::{configuration, database, setup_app_router};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Application access to env variables
    // dotenvy::dotenv().map_err(|err| {
    //     println!("Err: {err:#?}");
    //     err
    // })?;

    // Server tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Server conf
    let config = configuration::get_config().context("Failed to get configuration")?;

    // DB setup
    let db_pool = database::setup_db(&config).await.map_err(|err| {
        println!("Err: {err}");
        err
    })?;

    let app = setup_app_router(&db_pool)?;

    let address = format!(
        "{}:{}",
        config.application().host(),
        config.application().port()
    );

    info!("Server listening on: http://{address}/");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .context("Failed to bind listener")?;

    axum::serve(listener, app)
        .await
        .expect("Failed to run server");

    Ok(())
}

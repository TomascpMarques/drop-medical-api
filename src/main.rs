use std::sync::Arc;

use axum::Router;
use tokio::sync::RwLock;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod api;
mod configuration;
mod model;
mod turso;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = configuration::get_config()?;

    let db = turso::setup_db_connection(&config).await?;
    let db_state: turso::SyncDbConnection = Arc::new(RwLock::from(db.connect()?));

    let app = Router::new()
        .nest("/api", api::merge_routes(db_state.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let address = format!(
        "{}:{}",
        config.application().host(),
        config.application().port()
    );

    println!("Server listening on: http://{address}/");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app)
        .await
        .expect("Failed to run server");

    Ok(())
}

use axum::{
    routing::{get, post},
    Router,
};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

mod api;
mod configuration;
mod models;
mod state;
mod turso;

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
    let data_base = turso::setup_turso(&config)
        .await
        .expect("Failed to build DB");

    let db_conn = data_base.connect().expect("Failed to connect to DB");
    let state = state::AppStateManager::new(db_conn.clone());

    let app = Router::new()
        .route("/api/user/register", post(api::register_user))
        .route("/aaaa", get(|| async { "AAA" }))
        // App State layer
        .with_state(state)
        // Add cookie support
        .layer(CookieManagerLayer::new())
        // Tracing layer
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

    info!("Server listening on: http://{address}/");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app)
        .await
        .expect("Failed to run server");

    Ok(())
}

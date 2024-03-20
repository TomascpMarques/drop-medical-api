use axum::{routing::post, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

mod api;
mod configuration;
mod database;
mod models;
mod sessions;
mod state;

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

    let state = state::AppStateManager::new(db_pool);

    let app = Router::new()
        // .route("/api/user/register", post(api::register_user))
        // .route("/api/user/login", post(api::login_user))
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

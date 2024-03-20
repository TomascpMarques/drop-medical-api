use axum::Router;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod api;
pub mod configuration;
pub mod database;
mod models;
mod sessions;
mod state;

pub fn setup_app_router(db_pool: sqlx::postgres::PgPool) -> anyhow::Result<Router> {
    let state = state::AppStateManager::new(db_pool);

    let app = Router::new()
        .nest("/api", api::routes::merge_routes(state.clone()))
        // Add cookie support
        .layer(CookieManagerLayer::new())
        // Tracing layer
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    Ok(app)
}

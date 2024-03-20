use crate::state::AppStateManager;
use axum::Router;

pub mod machines;
pub mod users;

pub fn merge_routes(app_state: AppStateManager) -> Router {
    Router::new()
        .nest("/users", users::merge_routes(app_state.clone()))
        .nest("/droppers", machines::merge_routes(app_state.clone()))
}

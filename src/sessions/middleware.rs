use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;
use tracing::{error, info, warn};

use crate::state::AppStateManager;

use super::{SessionManager, SESSION_COOKIE_NAME};

pub async fn authenticated(
    State(state): State<AppStateManager>,
    cookies: Cookies,
    request: Request,
    next: Next,
) -> Response {
    let session_cookie = if let Some(cookie) = cookies.get(SESSION_COOKIE_NAME) {
        cookie
    } else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid session"))
            .unwrap();
    };

    let session_id: uuid::Uuid = if let Ok(sesh_id) = session_cookie.value().parse() {
        info!("Checking session is authenticated: {sesh_id}");
        sesh_id
    } else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid session"))
            .unwrap();
    };

    let is_valid_session = SessionManager::check_valid_session(state.db_pool(), session_id).await;

    match is_valid_session {
        Ok(_) => {
            info!("Valid session!");
            let err = SessionManager::extend_session(state.db_pool(), session_id).await;
            if err.is_err() {
                warn!("Not able ti extend session!: {}", err.unwrap_err());
            }
        }
        Err(e) => {
            error!("{e}");
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Invalid session"))
                .unwrap();
        }
    };

    next.run(request).await
}

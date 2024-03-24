use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::post,
    Form, Json, Router,
};
use tracing::{error, instrument, warn};

use crate::{
    models::machines::Dropper,
    sessions::{self, UserSessionIdExtractor},
    state::AppStateManager,
};

pub fn merge_routes(app_state: AppStateManager) -> Router {
    Router::new()
        .route(
            "/register",
            post(register_dropper).route_layer(middleware::from_fn_with_state(
                app_state.clone(),
                sessions::middleware::authenticated,
            )),
        )
        .with_state(app_state)
}

#[derive(Debug, serde::Deserialize)]
struct RegisterDrooper {
    // #[serde(rename(deserialize = "owid"))]
    // owner_id: uuid::Uuid,
    #[serde(rename(deserialize = "n"))]
    name: String,
}

#[axum::debug_handler]
#[instrument(skip(state, user_sesh), name = "Registering a new machine")]
async fn register_dropper(
    user_sesh: UserSessionIdExtractor,
    State(state): State<AppStateManager>,
    Form(dropper): Form<RegisterDrooper>,
) -> Result<impl IntoResponse, DropperRouteResult> {
    let user_id = user_sesh
        .get_user_id(state.db_pool())
        .await
        .map_err(|_| DropperRouteResult::UserIdNotPresentInRequest)?;

    let new_dropper = Dropper::new(state.db_pool(), false, user_id, None, dropper.name)
        .await
        .map_err(|err| {
            warn!("Falha ao criad dropper: {err}");
            DropperRouteResult::FalhaAoRegistarDropper
        })?;

    Ok((
        StatusCode::OK,
        Json(serde_json::to_value(new_dropper).unwrap()),
    ))
}

#[derive(Debug, thiserror::Error)]
enum DropperRouteResult {
    #[error("Falha ao registar um novo dropper")]
    FalhaAoRegistarDropper,
    #[error("Tentativa de registar um dropper sem ID de utilizador")]
    UserIdNotPresentInRequest,
}

impl IntoResponse for DropperRouteResult {
    fn into_response(self) -> Response {
        match self {
            DropperRouteResult::FalhaAoRegistarDropper => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Dados invalidos para criar dropper"))
                .unwrap(),
            DropperRouteResult::UserIdNotPresentInRequest => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("No user associated to the request"))
                .unwrap(),
        }
    }
}

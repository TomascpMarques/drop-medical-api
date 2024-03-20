use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Form, Json, Router,
};
use tracing::{instrument, warn};

use crate::{models::machines::Dropper, state::AppStateManager};

pub fn merge_routes(app_state: AppStateManager) -> Router {
    Router::new()
        .route("/register", post(register_dropper))
        .with_state(app_state)
}

#[derive(Debug, serde::Deserialize)]
struct RegisterDrooper {
    owner_id: uuid::Uuid,
    name: String,
}

#[axum::debug_handler]
#[instrument(skip(state), name = "Registering a new machine")]
async fn register_dropper(
    State(state): State<AppStateManager>,
    Form(dropper): Form<RegisterDrooper>,
) -> Result<impl IntoResponse, DropperRouteResult> {
    /*
     * 1. Receber dados para registar
     * 2. Criar novo dropper, desativado e sem url de maquina
     * 3. Dropper sem medicamentos e schedules
     * */

    // Temos de criar o dropper, usar o metodo new da struct Dropper
    let new_dropper = Dropper::new(state.db_pool(), false, dropper.owner_id, None, dropper.name)
        .await
        .map_err(|err| DropperRouteResult::FalhaAoRegistarDropper(err))?;

    Ok((
        StatusCode::OK,
        Json(serde_json::to_value(new_dropper).unwrap()),
    ))
}

#[derive(Debug, thiserror::Error)]
enum DropperRouteResult {
    #[error("Falha ao registar um novo dropper")]
    FalhaAoRegistarDropper(sqlx::Error),
}

impl IntoResponse for DropperRouteResult {
    fn into_response(self) -> Response {
        match self {
            DropperRouteResult::FalhaAoRegistarDropper(e) => {
                warn!("Falha ao criad dropper: {e}");
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Dados invalidos para criar dropper"))
                    .unwrap()
            }
        }
    }
}

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde_json::json;

use crate::{
    model::{self, users::User},
    turso,
};

pub fn merge_routes(db_connection: turso::SyncDbConnection) -> axum::Router {
    Router::new()
        .route("/user", post(register_user).get(|| async { "AAA" }))
        .with_state(db_connection)
}

#[axum::debug_handler]
async fn register_user(
    State(state): State<turso::SyncDbConnection>,
) -> (StatusCode, Json<serde_json::Value>) {
    println!("REGISTER USER");

    let conn = state.clone();
    let conn = conn.read().await;

    let user = User::new("Supa".into(), "supa@email.com".into(), "password".into());

    let result = model::users::create_user(&conn, &user).await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            json!({
                "id" : user.id(),
            })
            .into(),
        ),
        Err(err) => {
            println!("Err: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({
                    "reason": err.to_string()
                })
                .into(),
            )
        }
    }
}

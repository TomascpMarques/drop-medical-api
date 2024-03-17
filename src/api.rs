use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};
use tracing::{error, info, instrument, warn};

use crate::{models::User, state::AppStateManager};

#[derive(Debug, serde::Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[axum::debug_handler]
#[instrument(skip(), name = "Logging in a new user")]
pub async fn login_user(// State(state): State<AppStateSend>,
    // Json(credentials): Json<LoginCredentials>,
) -> impl IntoResponse {
    Response::builder()
        .status(200)
        .header("Authed", "true")
        .body(Body::from("AAAA"))
        .unwrap()
}

#[derive(Debug, serde::Deserialize)]
pub struct RegisterCredentials {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[axum::debug_handler]
#[instrument(skip(state, credentials, cookies), name = "Creating a new user", fields(usr_name = %credentials.name))]
pub async fn register_user(
    cookies: Cookies,
    State(state): State<AppStateManager>,
    Json(credentials): Json<RegisterCredentials>,
) -> impl IntoResponse {
    let inner_state = state.clone();
    let db_con = inner_state.db_con();

    let user = User::new(credentials.name, credentials.email, credentials.password);
    let id_preped = libsql::Value::Blob(user.id().to_bytes_le().to_vec());

    let mut prep_stmt = db_con
        .prepare("insert into user (id, name, email, password) values (?1, ?2, ?3, ?4)")
        .await
        .expect("failed to prepare statement");

    let res = prep_stmt
        .execute((
            id_preped,
            user.name().to_owned(),
            user.email().to_owned(),
            user.password().to_owned(),
        ))
        .await;

    if res.is_err() {
        error!("Error creatting user: {}>", res.unwrap_err());

        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Could not register user"))
            .unwrap();
    }

    info!("New user created <{}>", user.name());

    let session_res = state.create_new_session(&user).await;
    if session_res.is_err() {
        warn!(
            "Could not create user session: {}",
            session_res.unwrap_err()
        );

        return Response::builder()
            .status(StatusCode::EXPECTATION_FAILED)
            .body(Body::empty())
            .unwrap();
    };
    let session_id = session_res.unwrap();

    let expires_offset = Duration::new(60 * 45, 0);
    AppStateManager::create_session_cookie(&cookies, session_id, expires_offset);

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

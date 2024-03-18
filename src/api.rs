use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use sqlx::Row;
use tower_cookies::{cookie::time::Duration, Cookies};
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
    let user = User::new(credentials.name, credentials.email, credentials.password);

    let prep_stmt = sqlx::query!(
        r#"insert into "user" (name, email, password) values ($1, $2, $3)"#,
        user.name(),
        user.email(),
        user.password()
    )
    .execute(state.db_pool())
    .await;

    match prep_stmt {
        Ok(_) => (),
        Err(e) => {
            error!("error creatting user: {}>", e);

            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("could not register user"))
                .unwrap();
        }
    };

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

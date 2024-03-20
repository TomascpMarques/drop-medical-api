use axum::{
    body::Body, extract::State, http::{Response, StatusCode}, response::IntoResponse, routing::post, Json, Router
};
use tower_cookies::{cookie::time::Duration, Cookies};
use tracing::{error, info, instrument, warn};

use crate::{models::users::User, sessions::SessionManager, state::AppStateManager};

pub fn merge_routes(app_state: AppStateManager) -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/register", post(register_user))
        .with_state(app_state)
}

#[derive(Debug, serde::Deserialize)]
struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[axum::debug_handler]
#[instrument(
    skip(_state, credentials), 
    name = "Logging in a user",
    fields(
        usr_name = %credentials.email
    )
)]
async fn login_user(
    State(_state): State<AppStateManager>,
    Json(credentials): Json<LoginCredentials>,
) -> impl IntoResponse {
    // TODO finish auth system
    Response::builder()
        .status(200)
        .body(Body::from("OK"))
        .unwrap()
}

#[derive(Debug, serde::Deserialize)]
struct RegisterCredentials {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[axum::debug_handler]
#[instrument(
    skip(state, credentials, cookies), 
    name = "Creating a new user", 
    fields(usr_name = %credentials.name)
)]
async fn register_user(
    cookies: Cookies,
    State(state): State<AppStateManager>,
    Json(credentials): Json<RegisterCredentials>,
) -> impl IntoResponse {
    let mut user = User::new(credentials.name, credentials.email, credentials.password);

    let prep_stmt = sqlx::query!(
        r#"insert into "user" ( name, email, password) values ($1, $2, $3) RETURNING id"#,
        user.name(),
        user.email(),
        user.password()
    )
    .fetch_one(state.db_pool())
    .await;

    match prep_stmt {
        Ok(r) => user.id_mut().replace(r.id),
        Err(e) => {
            error!("error creatting user: {}>", e);

            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("could not register user"))
                .unwrap();
        }
    };

    info!("New user created <{}>", user.name());

    let session_res = SessionManager::create_new_session(state.db_pool(), &user).await;
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
    SessionManager::create_session_cookie(&cookies, session_id, expires_offset);

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

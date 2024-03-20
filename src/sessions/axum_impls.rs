use tracing::{error, warn};

use axum::{
    async_trait,
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::Response,
};

use super::{UserSessionIdExtractor, SESSION_COOKIE_NAME};

#[async_trait]
impl<S> FromRequestParts<S> for UserSessionIdExtractor
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session_cookie = parts.headers.get_all(axum::http::header::COOKIE).iter();

        for header_cookie in session_cookie {
            let cookie_to_str = header_cookie.to_str().map_err(|_| {
                error!("Failed to parse header cookie to a str value!");
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body("Malformed cookies in header".into())
                    .unwrap()
            })?;

            let cookie = tower_cookies::Cookie::from(cookie_to_str);
            if cookie.name().ne(SESSION_COOKIE_NAME) {
                continue;
            }

            let session_id = cookie.value_trimmed().parse().map_err(|_| {
                error!("Failed to create Uuid from cookie value!");
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::empty())
                    .unwrap()
            })?;

            return Ok(UserSessionIdExtractor(session_id));
        }

        warn!("Failed to parse request cookies, session cookie not found!");
        Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Could not find session token".into())
            .unwrap())
    }
}

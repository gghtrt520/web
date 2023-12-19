use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    NotFound,
    Unauthorized,
    WrongAuthcation,
    NoCtx,
    NoAccess,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::WrongAuthcation => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::NoCtx => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::NoAccess => (StatusCode::UNAUTHORIZED, self.to_string()),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not Found"),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::WrongAuthcation => write!(f, "Wrong Authcation"),
            Error::NoCtx => write!(f, "No Ctx"),
            Error::NoAccess => write!(f, "No Access"),
        }
    }
}

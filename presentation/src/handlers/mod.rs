use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::{http::StatusCode, response::IntoResponse, Json};
use domain::error::DomainError;
use serde::Serialize;

pub mod user_handler;

type Result<T> = std::result::Result<T, ErrorResponse>;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}

impl ErrorResponse {
    pub fn new(code: u16, message: String) -> Self {
        ErrorResponse { code, message }
    }
}

impl From<DomainError> for ErrorResponse {
    fn from(value: DomainError) -> Self {
        let code = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
        match value {
            DomainError::Other(message) => ErrorResponse::new(code, message),
            DomainError::Validation(message) => ErrorResponse::new(code, message.to_string()),
            DomainError::User(message) => ErrorResponse::new(code, message.to_string()),
        }
    }
}

impl From<JsonRejection> for ErrorResponse {
    fn from(value: JsonRejection) -> Self {
        ErrorResponse::new(value.status().as_u16(), value.body_text())
    }
}

impl From<QueryRejection> for ErrorResponse {
    fn from(value: QueryRejection) -> Self {
        ErrorResponse::new(value.status().as_u16(), value.body_text())
    }
}

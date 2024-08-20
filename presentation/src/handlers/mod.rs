use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::{http::StatusCode, response::IntoResponse, Json};
use domain::error::{DomainError, ErrorTrait};
use serde::Serialize;

pub mod user_handler;

const BASE_CODE: u32 = 999_000;

type Result<T> = std::result::Result<T, ErrorResponse>;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u32,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let default_code = StatusCode::INTERNAL_SERVER_ERROR;
        let status_code = if self.code > BASE_CODE {
            let code = self.code - BASE_CODE;
            let code = code.try_into().unwrap_or(default_code.as_u16());
            StatusCode::from_u16(code).unwrap_or(default_code)
        } else {
            default_code
        };
        (status_code, Json(self)).into_response()
    }
}

impl ErrorResponse {
    pub fn new(code: u32, message: String) -> Self {
        log::error!("({}) {}", code, &message);
        ErrorResponse { code, message }
    }

    pub fn from_status(status: StatusCode, message: String) -> Self {
        let status_code = status.as_u16() as u32;
        let code = BASE_CODE + status_code;
        ErrorResponse { code, message }
    }
}

impl From<DomainError> for ErrorResponse {
    fn from(value: DomainError) -> Self {
        ErrorResponse::new(value.code(), value.message())
    }
}

impl From<JsonRejection> for ErrorResponse {
    fn from(value: JsonRejection) -> Self {
        ErrorResponse::from_status(value.status(), value.body_text())
    }
}

impl From<QueryRejection> for ErrorResponse {
    fn from(value: QueryRejection) -> Self {
        ErrorResponse::from_status(value.status(), value.body_text())
    }
}

use std::sync::Arc;

use crate::{handlers::user_handler, AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(user_handler::register))
        .route("/", get(user_handler::get_one))
}

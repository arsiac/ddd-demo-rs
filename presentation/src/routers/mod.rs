use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod user_route;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().nest("/user", user_route::routes())
}

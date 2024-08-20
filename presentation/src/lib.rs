use std::sync::Arc;

use axum::Router;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

mod handlers;
mod routers;

pub async fn run(connection: DatabaseConnection, port: u32) {
    let state = AppState {
        connection: Arc::new(connection),
    };

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await;
    if let Err(e) = listener {
        panic!("Failed to bind to address '{}': {}", addr, e);
    }

    let app = Router::new()
        .nest("/api/v1", routers::routes())
        .with_state(Arc::new(state));

    log::info!("Server started at {}", addr);
    if let Err(e) = axum::serve(listener.unwrap(), app).await {
        panic!("Failed to start server: {}", e);
    }
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub connection: Arc<DatabaseConnection>,
}

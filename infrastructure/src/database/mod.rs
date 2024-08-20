use core::time;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn enstablish(database_url: &str, sql_log: bool) -> DatabaseConnection {
    let mut option = ConnectOptions::new(database_url);
    option
        .min_connections(1)
        .max_connections(20)
        .acquire_timeout(time::Duration::from_secs(5))
        .sqlx_logging(sql_log)
        .sqlx_logging_level(log::LevelFilter::Debug);
    match Database::connect(option).await {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect database '{}': {}", &database_url, e),
    }
}

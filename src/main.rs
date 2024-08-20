use clap::Parser;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
mod arg;

#[tokio::main]
async fn main() {
    let args = arg::Args::parse();
    init_logger(&args.level);
    let connection = enstablish_database(args.sql_log).await;
    presentation::run(connection, args.port).await;
}

fn init_logger(level_text: &str) {
    let level = level_text.parse();
    if let Err(e) = level {
        panic!("Invalid log level '{}': {}", level_text, e)
    }
    if let Err(e) = simple_logger::SimpleLogger::new()
        .with_level(level.unwrap())
        .init()
    {
        panic!("Failed to initialize logger: {}", e)
    }
}

async fn enstablish_database(sql_log: bool) -> DatabaseConnection {
    let exe = std::env::current_exe();
    if let Err(e) = exe {
        panic!("Failed to get program file path: {}", e);
    }
    let exe = exe.unwrap();
    let exe_folder = exe.parent();
    if exe_folder.is_none() {
        panic!("Failed to get program folder path");
    }
    let exe_folder = exe_folder.unwrap();
    let database_file = exe_folder.to_path_buf().join("data.sqlite");
    let database_url = format!("sqlite://{}?mode=rwc", database_file.display());
    log::debug!("Connect to database '{}'", database_url);
    let connection = infrastructure::database::enstablish(&database_url, sql_log).await;
    if let Err(e) = migration::Migrator::up(&connection, None).await {
        panic!("Failed to migrate database: {}", e);
    }
    connection
}

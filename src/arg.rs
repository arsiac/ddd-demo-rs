use clap::Parser;

#[cfg(debug_assertions)]
const DEFAULT_LOG_LEVEL: &str = "debug";
#[cfg(not(debug_assertions))]
const DEFAULT_LOG_LEVEL: &str = "info";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(long, default_value_t = 8080, help = "Server port")]
    pub port: u32,

    #[cfg(debug_assertions)]
    #[arg(long, default_value = DEFAULT_LOG_LEVEL, help = "Log level")]
    pub level: String,

    #[arg(long, default_value_t = false, help = "Enable SQL log")]
    pub sql_log: bool,
}

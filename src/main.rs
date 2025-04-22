use dctools::{cli::Cli, config::Config, error, exit_error, info, warn};

use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = match Config::new("./config.toml").await {
        Ok(c) => {
            info!("Config loaded successfully\n{}", c);
            c
        }
        Err(e) => {
            exit_error!("{}", e);
        }
    };

    if let Err(e) = cli.exec(&config).await {
        match e.kind() {
            dctools::ErrorKind::Warn => {
                warn!("{e}");
            }
            _ => {
                if e.exit() {
                    exit_error!("{e}");
                } else {
                    error!("{e}");
                }
            }
        }
    };
}

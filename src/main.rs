use dctools::{cli::Cli, config::Config, exit_error, info};

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

    let _ = cli.exec(&config).await;
}

use dctools::{channel::Message, cli::Args, config::Config, error, info, success, warn};

use clap::Parser;
use reqwest::StatusCode;
use std::{process, thread, time::Duration};

#[tokio::main]
async fn main() {
    info!("Loading config...");
    let config = match Config::load_config("./config.toml") {
        Ok(c) => {
            info!("Config loaded successfully\n{}", c);
            c
        }
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };

    let args = Args::parse();
    let mut message = match Message::new(args.id, args.content) {
        Ok(m) => m,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };

    let client = reqwest::Client::new();

    loop {
        let res = match message.send(&client, &config.token).await {
            Ok(r) => r,
            Err(e) => {
                error!("{}", e);
                process::exit(1);
            }
        };

        match res.status() {
            StatusCode::OK => {
                if message.count > 1 {
                    print!("\x1b[1A\x1b[2K");
                }
                success!("Message sent x{}", message.count);
            }
            StatusCode::UNAUTHORIZED => {
                error!("Invalid token");
                process::exit(1);
            }
            StatusCode::FORBIDDEN => {
                error!("You have been blocked");
                process::exit(1);
            }
            StatusCode::TOO_MANY_REQUESTS => {
                error!(
                    "Too many requests, it is recommended to increase 'delay' in the config file"
                );
                process::exit(1);
            }
            status => {
                dbg!(status);
                dbg!(&res);
                warn!("Unknown response status code");
                process::exit(1);
            }
        };

        thread::sleep(Duration::from_millis(config.delay));
    }
}

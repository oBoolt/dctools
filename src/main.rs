use discord_spammer::{channel::Message, cli::Args, config::Config};

use clap::Parser;
use reqwest::StatusCode;
use std::{process, thread, time::Duration};

#[tokio::main]
async fn main() {
    let config = match Config::load_config("./config.toml") {
        Ok(c) => c,
        Err(e) => {
            println!("[\x1b[31mError\x1b[0m] {}", e);
            process::exit(1);
        }
    };

    let args = Args::parse();
    let mut message = match Message::new(args.id, args.content) {
        Ok(m) => m,
        Err(e) => {
            println!("[\x1b[31mError\x1b[0m] {}", e);
            process::exit(1);
        }
    };

    let client = reqwest::Client::new();

    loop {
        let res = match message.send(&client, config.token.clone()).await {
            Ok(r) => r,
            Err(e) => {
                println!("[\x1b[31mError\x1b[0m] {}", e);
                process::exit(1);
            }
        };

        match res.status() {
            StatusCode::OK => {
                println!("[\x1b[36mSuccess\x1b[0m] Message sent successfully");
            }
            StatusCode::UNAUTHORIZED => {
                println!("[\x1b[31mError\x1b[0m] Invalid token");
                process::exit(1);
            }
            StatusCode::FORBIDDEN => {
                println!("[\x1b[31mError\x1b[0m] You have been blocked");
                process::exit(1);
            }
            StatusCode::TOO_MANY_REQUESTS => {
                println!("[\x1b[31mError\x1b[0m] Too many requests, it is recommended to increase 'delay' in the config file");
                process::exit(1);
            }
            status => {
                dbg!(status);
                dbg!(&res);
                println!("[\x1b[33mUnkown\x1b[0m] Unknown response status code");
                process::exit(1);
            }
        };

        thread::sleep(Duration::from_millis(config.delay));
    }
}

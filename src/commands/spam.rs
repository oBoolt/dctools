use reqwest::StatusCode;
use std::{process, thread, time::Duration};

use crate::channel::Message;
use crate::config::Config;
use crate::utils;
use crate::{exit_error, success, warn};

pub async fn exec<S: Into<String>>(
    config: &Config,
    channel_id: S,
    content: S,
) -> anyhow::Result<()> {
    let mut message = match Message::new(channel_id, content) {
        Ok(m) => m,
        Err(e) => {
            exit_error!("{}", e);
        }
    };

    let client = reqwest::Client::new();

    loop {
        let res = match message.send(&client, &config.content.token).await {
            Ok(r) => r,
            Err(e) => {
                exit_error!("{}", e);
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
                exit_error!("Invalid token");
            }
            StatusCode::FORBIDDEN => {
                exit_error!("You have been blocked");
            }
            StatusCode::TOO_MANY_REQUESTS => {
                exit_error!(
                    "Too many requests, it is recommended to increase 'delay' in the config file"
                );
            }
            status => {
                dbg!(status);
                dbg!(&res);
                warn!("Unknown response status code");
                utils::pause();
                process::exit(1);
            }
        };

        thread::sleep(Duration::from_millis(config.content.delay));
    }
}

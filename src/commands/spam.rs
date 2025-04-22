use reqwest::StatusCode;
use std::{thread, time::Duration};

use crate::channel::Message;
use crate::config::Config;
use crate::success;
use crate::{Error, ErrorKind, Result};

pub async fn exec<S: Into<String>>(config: &Config, channel_id: S, content: S) -> Result<()> {
    let mut message = Message::new(channel_id, content);

    let client = reqwest::Client::new();

    loop {
        let res = message.send(&client, &config.content.token).await?;

        match res.status() {
            StatusCode::OK => {
                if message.count > 1 {
                    print!("\x1b[1A\x1b[2K");
                }
                success!("Message sent x{}", message.count);
            }
            StatusCode::UNAUTHORIZED => {
                return Err(Error::new(ErrorKind::SpamCommand, "invalid token"));
            }
            StatusCode::FORBIDDEN => {
                return Err(Error::new(ErrorKind::SpamCommand, "you have been blocked"));
            }
            StatusCode::TOO_MANY_REQUESTS => {
                return Err(Error::new(
                    ErrorKind::SpamCommand,
                    "too many requests, it is recommended to increase 'delay' in the config file",
                ));
            }
            status => {
                dbg!(&res);
                return Err(Error::warn(format!(
                    "unknown response status code: {}",
                    status
                )));
            }
        };

        thread::sleep(Duration::from_millis(config.content.delay));
    }
}

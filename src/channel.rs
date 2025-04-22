use rand::Rng;
use reqwest::{header, Client, Response};
use serde::Serialize;

use crate::{exit_error, Error, ErrorKind, Result};

const API_URL: &'static str = "https://discord.com/api/v9/channels";

#[derive(Debug)]
pub struct Message {
    channel_id: String,
    message_payload: MessagePayload,
    pub count: u16,
}

impl Message {
    pub fn new<S: Into<String>>(channel_id: S, content: S) -> Self {
        let payload = MessagePayload::new(content);

        Self {
            channel_id: channel_id.into(),
            message_payload: payload,
            count: 0,
        }
    }

    pub async fn send<S: Into<String>>(&mut self, client: &Client, token: S) -> Result<Response> {
        let url = format!("{}/{}/messages", API_URL, self.channel_id);
        let nonce = match MessagePayload::get_nonce()
            .map_err(|_| Error::new(ErrorKind::MessagePayload, "failed to create nonce"))
        {
            Ok(n) => n,
            Err(e) => {
                exit_error!("{}", e);
            }
        };
        self.message_payload.nonce = nonce;
        self.count = self.count + 1;

        let res = client
            .post(url)
            .header(header::AUTHORIZATION, token.into())
            .json(&self.message_payload)
            .send()
            .await
            .map_err(|e| Error::from(e).set_message("failed to send message"))?;

        Ok(res)
    }
}

#[derive(Serialize, Debug)]
pub struct MessagePayload {
    pub content: String,
    pub flags: u8,
    pub mobile_network_type: String,
    pub nonce: String,
    pub tts: bool,
}

impl MessagePayload {
    fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: content.into(),
            flags: 0,
            mobile_network_type: String::from("unknown"),
            nonce: String::from("0"),
            tts: false,
        }
    }

    fn get_nonce() -> Result<String> {
        let mut result = String::new();
        let mut rng = rand::thread_rng();
        let numbers = "1234567890";

        for _ in 0..19 {
            let char = numbers
                .chars()
                .nth(rng.gen_range(0..numbers.len()))
                .ok_or(0)
                .map_err(|_| {
                    Error::new(ErrorKind::MessagePayload, "failed to create random number")
                })?;
            result.push(char);
        }

        Ok(result)
    }
}

use std::process;

use anyhow::Context;
use rand::Rng;
use reqwest::{header, Client, Response};
use serde::Serialize;

use crate::exit_error;

const API_URL: &'static str = "https://discord.com/api/v9/channels";

#[derive(Debug)]
pub struct Message {
    channel_id: String,
    message_payload: MessagePayload,
    pub count: u16,
}

impl Message {
    pub fn new(channel_id: String, content: String) -> anyhow::Result<Self> {
        let payload = MessagePayload::new(content).context("Failed to create message payload")?;

        Ok(Self {
            channel_id,
            message_payload: payload,
            count: 0,
        })
    }

    pub async fn send(
        &mut self,
        client: &Client,
        token: impl Into<String>,
    ) -> reqwest::Result<Response> {
        let url = format!("{}/{}/messages", API_URL, self.channel_id);
        let nonce = match MessagePayload::get_nonce().context("Failed to create nonce") {
            Ok(n) => n,
            Err(e) => {
                exit_error!("{}", e);
            }
        };
        self.message_payload.nonce = nonce;
        self.count = self.count + 1;

        client
            .post(url)
            .header(header::AUTHORIZATION, token.into())
            .json(&self.message_payload)
            .send()
            .await
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
    fn new(content: String) -> anyhow::Result<Self> {
        Ok(Self {
            content,
            flags: 0,
            mobile_network_type: String::from("unknown"),
            nonce: String::from("0"),
            tts: false,
        })
    }

    fn get_nonce() -> anyhow::Result<String> {
        let mut result = String::new();
        let mut rng = rand::thread_rng();
        let numbers = "1234567890";

        for _ in 0..19 {
            let char = numbers
                .chars()
                .nth(rng.gen_range(0..numbers.len()))
                .context("Failed to create random number")?;
            result.push(char);
        }

        Ok(result)
    }
}

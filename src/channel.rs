use anyhow::Context;
use rand::Rng;
use serde::Serialize;

use crate::cli::Args;

#[derive(Serialize, Debug)]
pub struct MessagePayload {
    pub content: String,
    pub flags: u8,
    pub mobile_network_type: String,
    pub nonce: String,
    pub tts: bool,
}

impl MessagePayload {
    pub fn new(args: &Args) -> anyhow::Result<Self> {
        let nonce = Self::get_nonce().context("Failed to generate nonce")?;

        Ok(Self {
            content: args.content.clone(),
            flags: 0,
            mobile_network_type: String::from("unknown"),
            nonce,
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

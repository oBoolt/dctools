use anyhow::Context;
use serde::Deserialize;
use std::{
    fmt, fs,
    io::{self, Read},
    path,
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub delay: u64,
}

impl Config {
    pub fn load_config<P: AsRef<path::Path>>(path: P) -> anyhow::Result<Self> {
        let file = fs::File::open(path).context("Failed to open config file")?;
        let mut reader = io::BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .context("Failed to read config file content")?;

        let config: Self = toml::from_str(&content)
            .context("Failed to deserialize config file content to config struct")?;

        Ok(Self {
            token: config.token,
            delay: config.delay,
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\x1b[36mInfo\x1b[0m] delay = {}ms", self.delay)
    }
}

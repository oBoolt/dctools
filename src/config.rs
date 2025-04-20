use anyhow::Context;
use serde::Deserialize;
use std::{
    fmt, fs,
    io::{self, Read, Write},
    path, process,
};

use crate::{exit_error, info, success, utils};

const CONFIG_FILE_URL: &'static str =
    "https://raw.githubusercontent.com/oBoolt/dctools/refs/heads/main/config.template.toml";

#[derive(Debug)]
pub struct Config {
    /// The path to the config file
    path: path::PathBuf,
    /// The content of the config file deserialized
    pub content: ConfigContent,
}

impl Config {
    pub async fn new<P: AsRef<path::Path>>(path: P) -> anyhow::Result<Self> {
        info!("Loading config...");
        let config_content = ConfigContent::load_config(&path)
            .await
            .context("Failed to load config content")?;

        Ok(Self {
            path: path.as_ref().to_path_buf(),
            content: config_content,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct ConfigContent {
    /// The discord token used to make authentication
    pub token: String,
    /// The delay between messages
    pub delay: u64,
}

impl ConfigContent {
    async fn load_config<P: AsRef<path::Path>>(path: &P) -> anyhow::Result<Self> {
        let file_exists = match fs::exists(path) {
            Ok(b) => b,
            Err(_) => exit_error!("Can't check existence of config file"),
        };

        if !file_exists {
            match Self::download_config_file(path).await {
                Ok(_) => {
                    info!("Remember to change the token in the config file to your discord token");
                    utils::pause();
                    process::exit(0);
                }
                Err(e) => exit_error!("{}", e),
            }
        }

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

    async fn download_config_file<P: AsRef<path::Path>>(path: &P) -> anyhow::Result<()> {
        info!("Downloading config file template...");
        let content = reqwest::get(CONFIG_FILE_URL)
            .await
            .context("Failed to make request to get config file")?
            .bytes()
            .await
            .context("Failed to convert response to bytes")?;
        let mut dest = fs::File::create(path).context("Failed to create config file")?;
        dest.write_all(&content)
            .context("Failed to write content to config file")?;
        success!("Config file downloaded");
        Ok(())
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\x1b[36mInfo\x1b[0m] config path = '{}'\n[\x1b[36mInfo\x1b[0m] delay = {}ms",
            self.path.display(),
            self.content.delay
        )
    }
}

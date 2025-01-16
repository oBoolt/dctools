use clap::Parser;
use discord_spammer::{cli::Args, config::Config};
use std::process;

fn main() {
    let _config = match Config::load_config("./config.toml") {
        Ok(x) => x,
        Err(e) => {
            println!("[\x1b[31mError\x1b[0m] {}", e);
            process::exit(1);
        }
    };

    let args = Args::parse();
    dbg!(args);
}

use std::process;

fn main() {
    let config = match discord_spammer::Config::load_config("./config.toml") {
        Ok(x) => x,
        Err(e) => {
            println!("[\x1b[31mError\x1b[0m] {}", e);
            process::exit(1);
        }
    };
}

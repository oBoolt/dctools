use crate::{commands::spam, config::Config};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "dctools", version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn exec(self, config: &Config) -> anyhow::Result<()> {
        match self.command {
            Command::Spam(args) => spam::exec(config, args.id, args.content).await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Spam(SpamArgs),
}

#[derive(Debug, Args)]
struct SpamArgs {
    /// The channel id you want to spam
    #[arg(long)]
    id: String,

    /// The content of the message
    #[arg(short, long)]
    content: String,
}

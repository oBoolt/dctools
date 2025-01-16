use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "dc-spammer")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The channel id you want to spam
    #[arg(long)]
    pub id: String,

    /// The content of the message
    #[arg(short, long)]
    pub content: String,
}

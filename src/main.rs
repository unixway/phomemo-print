use anyhow::Result;
use clap::{arg, Parser, Subcommand};

mod scan;
mod print;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Scan {
        #[arg(short, long, default_value = "3")]
        timeout: u64,
    },
    PrintImage {
        #[arg(short, long)]
        mac: String,
        #[arg(short, long)]
        image: String,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan { timeout } => {
            scan::scan_devices(timeout).await
        }
        Command::PrintImage { mac, image } => {
            print::print_image(mac.as_str(), image.as_str()).await
        }
    }
}




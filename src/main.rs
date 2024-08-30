use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    Hello {
        /// Name to greet
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let args = Args::parse();

    dbg!(req().await?);

    Ok(())
}

async fn req() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    dbg!(body);
    Ok(())
}

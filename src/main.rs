use binance_client::BinanceWebSocket;
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[command(name = "Binance WebSocket CLI")]
#[command(author = "AnXi0usCat <mikhail.fjodorov@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Interacts with Binance WebSocket API", long_about = None)]
struct Cli {
    #[arg(long, num_args = 1..)]
    subscribe: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "wss://stream.binance.com:9443/ws";
    let mut binance_ws = BinanceWebSocket::connect(url).await?;

    let cli = Cli::parse();

    for stream in cli.subscribe {
        binance_ws.subscribe(stream.as_ref()).await?;
    }
    binance_ws.run().await?;

    Ok(())
}

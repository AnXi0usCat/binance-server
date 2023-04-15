use client::BinanceWebSocket;
use cli;
use std::error::Error;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let cmd = cli::create_cli();
    let matches = cmd.get_matches();

    let mut binance_ws = BinanceWebSocket::connect("wss://stream.binance.com:9443/ws").await?;
    binance_ws.subscribe(matches
                         .get_one::<String>("subscribe")
                         .unwrap().as_str()).await?;
    binance_ws.run().await?;

    Ok(())
}

extern crate binance_client as client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let url = "wss://stream.binance.com:9443/ws";
    let mut binance_ws = client::BinanceWebSocket::connect(url).await?;

    binance_ws.combined_stream(true).await?;
    binance_ws.subscribe("btcusdt@trade").await?;
    binance_ws.subscribe("btcusdt@bookTicker").await?;

    binance_ws.run().await?;

    Ok(())
}

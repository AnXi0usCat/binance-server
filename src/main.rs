use binance_client::BinanceWebSocket;
use cli;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cmd = cli::create_cli();
    let matches = cmd.get_matches();

    let mut binance_ws = BinanceWebSocket::connect("wss://stream.binance.com:9443/ws").await?;
    binance_ws.subscribe(matches
                         .get_one::<String>("subscribe")
                         .unwrap().as_str()).await?;
    binance_ws.run().await?;

    Ok(())
}

extern crate binance_client as client;
use clap::{App, Arg, SubCommand};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "wss://stream.binance.com:9443/ws";
    let mut binance_ws = client::BinanceWebSocket::connect(url).await?;
    let ws_handle = tokio::spawn(async move { binance_ws.run().await });

    let cli = App::new("Binance WebSocket CLI")
        .version("1.0")
        .author("AnXi0usCat <mikhail.fjodorov@gmail.com>")
        .about("Interacts with Binance WebSocket API")
        .subcommand(
            SubCommand::with_name("subscribe")
                .about("Subscribes to a stream")
                .arg(
                    Arg::with_name("STREAM")
                        .help("The stream to subscribe to")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("unsubscribe")
                .about("Unsubscribes from a stream")
                .arg(
                    Arg::with_name("STREAM")
                        .help("The stream to unsubscribe from")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("combined")
                .about("Creates combined stream")
                .arg(
                    Arg::with_name("STREAM")
                        .help("Use combined stream for subscrptions")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("exit").about("exit application").arg(
                Arg::with_name("STREAM")
                    .help("Use combined stream for subscrptions")
                    .required(true)
                    .index(1),
            ),
        );

    let mut reader = BufReader::new(io::stdin()).lines();

    while let Some(line) = reader.next_line().await? {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if let Ok(matches) = cli.clone().try_get_matches_from(parts) {
            match matches.subcommand() {
                ("subscribe", sub_m) => {
                    let stream = sub_m.value_of("STREAM").unwrap();
                    binance_ws.subscribe(stream).await?;
                    println!("Subscribed to stream: {}", stream);
                }
                ("unsubscribe", Some(sub_m)) => {
                    let stream = sub_m.value_of("STREAM").unwrap();
                    binance_ws.unsubscribe(stream).await?;
                    println!("Unsubscribed from stream: {}", stream);
                }
                ("combined", Some(sub_m)) => {
                    let stream = sub_m.value_of("STREAM").unwrap();
                    binance_ws.combined_stream(stream).await?;
                    println!("Use combined stream: {}", stream);
                }

                ("exit", Some(sub_m)) => {
                    let stream = sub_m.value_of("STREAM").unwrap();
                    println!("shitting down application: {}", stream);
                    break;
                }
                _ => {}
            }
        } else {
            println!("Invalid command or arguments. Supported commands are:");
            println!("  subscribe <stream>");
            println!("  unsubscribe <stream>");
            println!("  exit");
        }
    }

    ws_handle.await??;
    Ok(())
}

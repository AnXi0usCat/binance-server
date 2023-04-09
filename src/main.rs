use binance_client::{BinanceWebSocket, Command};
use clap::{App, Arg, SubCommand};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt, BufReader};

fn parse_command(input: &str, cli: &App) -> Option<Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let matches = cli.clone().try_get_matches_from(parts).ok()?;

    if let Some(sub_m) = matches.subcommand_matches("subscribe") {
        let stream = sub_m.value_of("STREAM").unwrap().to_string();
        Some(Command::Subscribe(stream))
    } else if let Some(sub_m) = matches.subcommand_matches("unsubscribe") {
        let stream = sub_m.value_of("STREAM").unwrap().to_string();
        Some(Command::Unsubscribe(stream))
    } else if let Some(sub_m) = matches.subcommand_matches("combined") {
        let stream = sub_m.value_of("STREAM").unwrap().parse().unwrap();
        Some(Command::CombinedStream(stream))
    } else if matches.subcommand_matches("exit").is_some() {
        Some(Command::Exit)
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = "wss://stream.binance.com:9443/ws";
    let mut binance_ws = BinanceWebSocket::connect(url).await?;

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
        match parse_command(&line, &cli) {
            Some(Command::Subscribe(stream)) => {
                binance_ws.subscribe(&stream).await?;
                println!("Subscribed to stream: {}", stream);
            }
            Some(Command::Unsubscribe(stream)) => {
                binance_ws.unsubscribe(&stream).await?;
                println!("Unsubscribed from stream: {}", stream);
            }
            Some(Command::CombinedStream(stream)) => {
                binance_ws.combined_stream(stream).await?;
                println!("Creating a combined stream: {}", stream);
            }
            Some(Command::Exit) => {
                break;
            }
            None => {
                println!("Invalid command or arguments. Supported commands are:");
                println!("  subscribe <stream>");
                println!("  unsubscribe <stream>");
                println!("  exit");
            }
        }
    }

    let ws_handle = tokio::spawn(async move { binance_ws.run().await });
    ws_handle.await??;
    Ok(())
}

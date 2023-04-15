use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream, tungstenite::protocol::Message, MaybeTlsStream};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use futures_util::{StreamExt, SinkExt};
use std::error::Error;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    Subscribe,
    Unsubscribe,
    SetProperty
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    method: Command,
    params: Vec<String>,
    id: u8,
}

pub struct BinanceWebSocket {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl BinanceWebSocket {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(url).await?;
        Ok(Self { ws_stream })
    }

    pub async fn combined_stream(&mut self, combined: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.ws_stream.send(Message::Text(BinanceWebSocket::create_payload(
                    Command::SetProperty,
                    vec![String::from("combined"), combined.to_string()])
                ?)
            ).await?;
        Ok(())
    }
    pub async fn subscribe(&mut self, stream: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.ws_stream.send(Message::Text(BinanceWebSocket::create_payload(
                    Command::Subscribe,
                    vec![String::from(stream)])
                ?)
            ).await?;
        Ok(())
    }

    pub async fn unsubscribe(&mut self, stream: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.ws_stream.send(Message::Text(BinanceWebSocket::create_payload(
                    Command::Unsubscribe,
                    vec![String::from(stream)])
                ?)
            ).await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        while let Some(msg) = self.ws_stream.next().await {
            let msg = msg?;

            match msg {
                Message::Text(text) => {
                    let value: Value = serde_json::from_str(&text)?;
                    println!("{}", value);
                }
                Message::Ping(ping) => {
                    println!("Received Ping: {:?}", ping);
                    self.ws_stream.send(Message::Pong(ping)).await?;
                    println!("Sent Pong");
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn create_payload(command: Command, message: Vec<String>) -> Result<String, serde_json::Error> {
        let payload = Subscribe { 
            method: command, 
            params: message, 
            id: 1 
        };
        serde_json::to_string(&payload)
    }
}


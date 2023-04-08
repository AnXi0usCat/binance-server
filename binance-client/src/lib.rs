use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream, tungstenite::protocol::Message, MaybeTlsStream};
use serde_json::Value;
use futures_util::{StreamExt, SinkExt};
use std::error::Error;

pub struct BinanceWebSocket {
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl BinanceWebSocket {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let (ws_stream, _) = connect_async(url).await?;
        Ok(Self { ws_stream })
    }

    pub async fn combined_stream(&mut self, combined: bool) -> Result<(), Box<dyn Error + Send + Sync>> {
        let property_msg = format!(r#"{{"method": "SET_PROPERTY", "params": ["combined", {}], "id": 1}}"#, combined);
        self.ws_stream.send(Message::Text(property_msg)).await?;
        Ok(())
    }
    pub async fn subscribe(&mut self, stream: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let subscribe_msg = format!(r#"{{"method": "SUBSCRIBE", "params": ["{}"], "id": 1}}"#, stream);
        self.ws_stream.send(Message::Text(subscribe_msg)).await?;
        Ok(())
    }

    pub async fn unsubscribe(&mut self, stream: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let unsubscribe_msg = format!(r#"{{"method": "UNSUBSCRIBE", "params": ["{}"], "id": 1}}"#, stream);
        self.ws_stream.send(Message::Text(unsubscribe_msg)).await?;
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
}


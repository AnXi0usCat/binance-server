use reqwest::{ Client, Response };
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct LimitOrder {
    symbol: String,
    side: String,
    r#type: String,
    time_in_force: String,
    quantity: String,
    price: String,
    timestamp: String,
    signature: String
}

pub struct BinanceRest {
    client: Client,
    api_key: String,
    secret_key: String,
    base_url: String,
}

impl BinanceRest {

    pub fn new(api_key: String, secret_key: String) -> Self {
        BinanceRest { 
            client: Client::new(), 
            api_key, 
            secret_key, 
            base_url: String::from("https://api.binance.com/") 
        }
    }

    fn get_timestamp(&self) -> String {
        let start = std::time::SystemTime::now();
        let since_epoch = start
            .duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
        since_epoch.as_millis().to_string()
    }

    fn create_signature(&self, timestamp: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        let query = format!("timestamp={}", timestamp);
        mac.update(query.as_bytes());

        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());

        signature
    }

    async fn post<T: Serialize + ?Sized>(&self, json_body: &T, query: &str) -> Result<Response, reqwest::Error> {
        let url = format!("{}/{}", &self.base_url, query);
        let response = self.client.post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .json(json_body)
            .send()
            .await;

        response
    }

    async fn get(&self, query: &str) -> Result<Response, reqwest::Error> {
        let url = format!("{}/{}",  &self.base_url, query);
        let response  = self.client.get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await;

        response
    }

    async fn delete(&self, query: &str) -> Result<Response, reqwest::Error> {
        let url = format!("{}/{}",  &self.base_url, query);
        let response = self.client.delete(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await;

        response
    }

    pub async fn test_limit_order(&self, symbol: String, side: String, quantity: String, price: String) {
        let url = "api/v3/order/test";
        let timestamp = self.get_timestamp();

        let limit_order = LimitOrder { 
            symbol,
            side,
            r#type: String::from("LIMIT"),
            time_in_force: String::from("GTC"),
            quantity,
            price,
            timestamp: timestamp.clone(),
            signature: self.create_signature(&timestamp)
        };

        let response = self.post(&limit_order, url).await;
        print!("received response {:?}", response);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_limit_order() {

    }
}

use reqwest::Client;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::Serialize;

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
            base_url: String::from("https://api.binance.com/api") 
        }
    }

    fn get_timestamp() -> String {
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

    async fn send_post_request<T: Serialize + ?Sized >(&self, json_body: &T) -> Result<NewOrderResponse, reqwest::Error> {
        let resp_json = self.client.post(&self.base_url)
            .header("X-MBX-APIKEY", &self.api_key)
            .json(json_body)
            .send()
            .await;

        response.json::<NewOrderResponse>().await
    }
}

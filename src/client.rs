use crate::{constants::PROTOCOL_VERSION, types::ApiUrl};
use std::time::Duration;

use crate::{
    constants::API_BASE_URL,
    types::{order::OrdersResponse, OrdersQuery, UniswapXApiError},
};
use reqwest::header::HeaderMap;
use reqwest::{header, Client, ClientBuilder};

#[derive(Debug, Clone)]
pub struct UniswapXApiClient {
    client: Client,
    url: ApiUrl,
}

#[derive(Debug, Clone)]
pub struct UniswapXApiConfig {
    pub timeout: u64,
}

impl Default for UniswapXApiConfig {
    fn default() -> Self {
        Self { timeout: 30 }
    }
}

impl UniswapXApiClient {
    /// Create a new client with the given configuration.
    pub fn new(cfg: UniswapXApiConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", header::HeaderValue::from_str("api_key").unwrap());

        let client = ClientBuilder::new().timeout(Duration::from_secs(cfg.timeout)).build().unwrap();
        let base_url = format!("{}/{}", API_BASE_URL, PROTOCOL_VERSION);

        Self { client, url: ApiUrl { base: base_url } }
    }

    pub async fn orders(&self, orders_query: &OrdersQuery) -> Result<OrdersResponse, UniswapXApiError> {
        let response = self.client.get(self.url.orders(orders_query)).send().await?;

        if !response.status().is_success() {
            let status_code = response.status();
            if status_code.as_u16() == 429 {
                return Err(UniswapXApiError::TooManyRequests);
            }
            let body = response.text().await?;
            return Err(UniswapXApiError::StatusNotOk(status_code, body));
        }
        let body = response.text().await?;
        let order_response = match serde_json::from_str(&body) {
            Ok(order_response) => order_response,
            Err(e) => {
                return Err(UniswapXApiError::SerdeJson(e, body));
            }
        };
        Ok(order_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::FixedBytes;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn can_deserialize_order_response() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/response_orders.json");

        let res = std::fs::read_to_string(d).unwrap();
        let res: OrdersResponse = serde_json::from_str(&res).unwrap();
        let expected: FixedBytes<32> = FixedBytes::from_str("0x8191cd348cc1f78e283d40aaef41e8e532b64e790bf60b9291b4917d9c0877f0").unwrap();
        assert_eq!(res.orders.first().unwrap().order_hash, expected);
    }
}

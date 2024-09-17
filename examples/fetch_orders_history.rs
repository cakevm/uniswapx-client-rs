use std::process::exit;
use uniswapx_client_rs::client::{UniswapXApiClient, UniswapXApiConfig};
use uniswapx_client_rs::types::SortKey::CreatedAt;
use uniswapx_client_rs::types::{OrdersQueryBuilder, UniswapXApiError};

#[tokio::main]
async fn main() {
    let client = UniswapXApiClient::new(UniswapXApiConfig::default());
    let mut query = OrdersQueryBuilder::default().limit(50u32).chain_id(1).sort_key(CreatedAt).desc(true).build().unwrap();

    let mut cursor = None;
    let mut first = true;
    while cursor.is_some() || first {
        first = false;

        if cursor.is_some() {
            query.cursor = cursor.clone();
        }

        let order_response = match client.orders(&query).await {
            Ok(order_response) => order_response,
            Err(e) => match e {
                UniswapXApiError::TooManyRequests => {
                    println!("Too many requests, sleeping for 5 seconds");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    continue;
                }
                _ => {
                    println!("Error: {:?}", e);
                    exit(1)
                }
            },
        };
        cursor = order_response.cursor;

        let created_at = match order_response.orders.last() {
            Some(order) => order.created_at,
            None => 0,
        };
        println!("Cursor: {:?}, len: {}, time: {}", &cursor, order_response.orders.len(), created_at);

        for mut order in order_response.orders {
            // normalize timestamps. Not all timestamps are in seconds
            if order.created_at > 9999999999u64 {
                order.created_at = order.created_at / 1000;
            }
            // Implement your own logic here
        }
    }
}

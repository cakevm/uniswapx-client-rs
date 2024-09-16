use uniswapx_client_rs::client::{UniswapXApiClient, UniswapXApiConfig};
use uniswapx_client_rs::types::OrdersQueryBuilder;
use uniswapx_client_rs::types::SortKey::CreatedAt;

#[tokio::main]
async fn main() {
    let client = UniswapXApiClient::new(UniswapXApiConfig::default());
    let query = OrdersQueryBuilder::default().limit(10u32).chain_id(1).sort_key(CreatedAt).desc(true).build().unwrap();

    let result = match client.orders(&query).await {
        Ok(order_response) => order_response,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    if let Some(order) = result.orders.get(0) {
        println!("{:#?}", order);
    } else {
        println!("No orders found");
    }
}

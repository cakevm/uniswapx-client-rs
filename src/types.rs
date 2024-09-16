pub mod order;

use crate::types::order::OrderStatus;
use derive_builder::Builder;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UniswapXApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to parse response {0} from '{1}'")]
    SerdeJson(serde_json::Error, String),
    #[error("Request failed with status code {0} and body '{1}'")]
    StatusNotOk(reqwest::StatusCode, String),
    #[error("Too many requests")]
    TooManyRequests,
}

/// API endpoints
#[derive(Debug, Clone)]
pub struct ApiUrl {
    pub base: String,
}

impl ApiUrl {
    pub fn orders(&self, orders_query: &OrdersQuery) -> String {
        let query_parameters = serde_url_params::to_string(orders_query).unwrap();
        if query_parameters.is_empty() {
            return format!("{}/orders", self.base);
        }
        format!("{}/orders?{}", self.base, query_parameters)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SortKey {
    CreatedAt,
}

#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(setter(into), default)]
#[serde(rename_all = "camelCase")]
pub struct OrdersQuery {
    /// Default value : 1
    #[builder(setter(into, strip_option), default)]
    pub limit: Option<u32>,
    /// Filter by order status.
    #[builder(setter(into, strip_option), default)]
    pub order_status: Option<OrderStatus>,
    /// Filter by order hash.
    #[builder(setter(into, strip_option), default)]
    pub order_hash: Option<String>,
    /// Filter by comma separated order hashes.
    pub order_hashes: Option<Vec<String>>,
    /// Filter by swapper address.
    pub swapper: Option<String>,
    /// Order the query results by the sort key.
    pub sort_key: Option<SortKey>,
    /// Sort query. For example: sort=gt(UNIX_TIMESTAMP), sort=between(1675872827, 1675872930), or lt(1675872930).
    pub sort: Option<String>,
    /// Boolean to sort query results by descending sort key.
    pub desc: Option<bool>,
    /// Cursor param to page through results. This will be returned in the previous query if the results have been paginated.
    pub cursor: Option<String>,
    /// Available values : 1, 137
    pub chain_id: Option<u8>,
}

#[cfg(test)]
mod test {
    use crate::types::{ApiUrl, OrdersQuery};

    #[test]
    fn order_url_empty_query_parameters_ok() {
        let api_url = ApiUrl { base: "https://test".to_string() };
        let order_url = api_url.orders(&OrdersQuery::default());
        assert_eq!(order_url, "https://test/orders".to_string());
    }

    #[test]
    fn order_url_with_limit_query_ok() {
        let api_url = ApiUrl { base: "https://test".to_string() };
        let order_url = api_url.orders(&OrdersQuery { limit: Some(5), ..OrdersQuery::default() });
        assert_eq!(order_url, "https://test/orders?limit=5".to_string());
    }
}

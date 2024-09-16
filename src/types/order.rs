use alloy_primitives::{Address, FixedBytes, TxHash, U256};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Dutch,
    #[serde(rename = "Dutch_V2")]
    DutchV2,
    Limit,
    // deprecated see: https://github.com/Uniswap/uniswapx-service/blob/62a0b995764dbec228f42bcaf9cc7f0cdf44152c/lib/util/field-validator.ts#L55
    DutchLimit,
    // TODO: Support Relay and Priority orders
    //Relay,
    //Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OrderStatus {
    Open,
    Expired,
    Error,
    Cancelled,
    Filled,
    InsufficientFunds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInput {
    pub token: Address,
    #[serde(serialize_with = "serialize_u256")]
    pub start_amount: U256,
    #[serde(serialize_with = "serialize_u256")]
    pub end_amount: U256,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderOutput {
    pub token: Address,
    #[serde(serialize_with = "serialize_u256")]
    pub start_amount: U256,
    #[serde(serialize_with = "serialize_u256")]
    pub end_amount: U256,
    pub recipient: Address,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettledAmount {
    pub token_out: Address,
    #[serde(serialize_with = "serialize_u256")]
    pub amount_out: U256,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_in: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_u256_option")]
    pub amount_in: Option<U256>,
}

// https://github.com/Uniswap/uniswapx-service/blob/main/lib/entities/Order.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub outputs: Vec<OrderOutput>,
    #[serde(with = "vec_u8_hex")]
    pub encoded_order: Vec<u8>,
    pub signature: FixedBytes<65>,
    #[serde(serialize_with = "serialize_u256")]
    pub nonce: U256,
    pub order_hash: FixedBytes<32>,
    pub order_status: OrderStatus,
    pub chain_id: u8,
    pub input: OrderInput,

    pub created_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,

    // settled order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settled_amounts: Option<Vec<SettledAmount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<TxHash>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdersResponse {
    pub orders: Vec<Order>,
    pub cursor: Option<String>,
}

pub mod vec_u8_hex {
    use alloy_primitives::hex;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode_prefixed(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

pub fn serialize_u256<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format!("{}", value).as_str())
}

pub fn serialize_u256_option<S>(value: &Option<U256>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(format!("{}", value.unwrap_or_default()).as_str())
}

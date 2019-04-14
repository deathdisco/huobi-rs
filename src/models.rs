#![allow(dead_code)]
#![allow(unused_variables)]

use std::str::FromStr;
use std::fmt::Display;
//use serde::de::{self, Deserialize, Deserializer};
use serde::{de, Deserialize, Serialize, Deserializer};

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse<R> {
    pub status: String,
    pub data: R,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerTime {
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub phase: String,
    pub revision: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balances {
    pub total: i32,
    #[serde(rename = "datas")]
    pub balances: Vec<Balance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
//    #[serde(rename = "coinType")]
    pub symbol: String,                 // Currency ID "BTC", "ETH"
    pub balance: f64,                   // Total amount of balance
//    #[serde(rename = "freezeBalance")]
    pub locked: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
    #[serde(rename = "base-currency")]
    pub base_currency: String,       // "eth", "btc"

    #[serde(rename = "quote-currency")]
    pub quote_currency: String,       // "eth", "btc"

    #[serde(rename = "price-precision")]
    pub price_precision: u32,

    #[serde(rename = "amount-precision")]
    pub amount_precision: u32,

    #[serde(rename = "symbol-partition")]
    pub symbol_partition: String,

    #[serde(rename = "symbol")]         // "edubtc", "linkusdt"
    pub symbol: String,
}

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AccountTransaction {
    amount: i64,
    currency: String,
    from: String,
}

#[derive(Deserialize, Serialize)]
pub struct CardTransaction {
    amount: i64,
    currency: String,
    merchant: String,
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "_id")]
    id: String,
    index: u32,
    guid: uuid::Uuid,
    is_active: bool,
    balance: String,
    transactions: Vec<AccountTransaction>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    #[serde(rename = "_id")]
    id: String,
    is_frozen: bool,
    card_number: String,
    transactions: Vec<CardTransaction>,
}

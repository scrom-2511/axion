use crate::transaction::Transaction;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: String,
    pub transaction: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u32, previous_hash: String) -> Self {
        let current_time = Utc::now().timestamp_millis();
        let nonce = 0;
        let difficulty = String::from("000");
        let transaction: Vec<Transaction> = vec![];

        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(current_time.to_string());
        hasher.update(&previous_hash);
        hasher.update(nonce.to_string());
        hasher.update(&difficulty.to_string());
        hasher.update(serde_json::to_string(&transaction).unwrap());

        let hash = format!("{:x}", hasher.finalize());

        Self {
            index: index,
            timestamp: current_time as u128,
            previous_hash: previous_hash,
            hash: hash,
            nonce: nonce,
            difficulty: difficulty,
            transaction: transaction,
        }
    }
}
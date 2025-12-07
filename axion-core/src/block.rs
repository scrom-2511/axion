use crate::{services::hasher::Hasher, transaction::Transaction};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

        let mut block_objects = Block {
            index,
            timestamp: current_time as u128,
            previous_hash,
            hash: String::from("temp"),
            nonce,
            difficulty,
            transaction,
        };

        let (nonce, hash) = Hasher::calculate_hash_with_prefix(&block_objects);
        block_objects.hash = hash;
        block_objects.nonce = nonce;

        block_objects
    }
}

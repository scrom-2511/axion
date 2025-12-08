use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::{
    block::Block,
    transaction::{self, Transaction, TxInput, TxOutput},
};

pub struct Hasher {}

impl Hasher {
    pub fn calculate_hash_with_prefix(block_objects: &Block) -> (u64, String) {
        let mut nonce: u64 = 0;

        loop {
            let mut hasher = Sha256::new();
            hasher.update(block_objects.index.to_string());
            hasher.update(block_objects.timestamp.to_string());
            hasher.update(&block_objects.difficulty);
            hasher.update(&block_objects.previous_hash);
            hasher.update(nonce.to_string());
            hasher.update(serde_json::to_string(&block_objects.transaction).unwrap());

            let hash = bs58::encode(hasher.finalize()).into_string();
            if hash.starts_with(&block_objects.difficulty) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }

    pub fn calculate_txid(transaction: Transaction) -> String {
        let mut clean_inputs = vec![];

        for input in &transaction.inputs {
            clean_inputs.push((&input.prev_tx_id, input.output_index, &input.pub_key));
        }

        let mut clean_outputs = vec![];

        for output in &transaction.outputs {
            clean_outputs.push((&output.recepient_pubkey, output.amount));
        }

        let current_time = Utc::now().to_string();

        let data = serde_json::to_string(&(clean_inputs, clean_outputs)).unwrap();

        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(current_time);
        let hash = bs58::encode(hasher.finalize()).into_string();

        hash
    }
}

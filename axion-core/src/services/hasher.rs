use sha2::{Digest, Sha256};

use crate::block::Block;

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

            let hash = format!("{:x}", hasher.finalize());
            if hash.starts_with(&block_objects.difficulty) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

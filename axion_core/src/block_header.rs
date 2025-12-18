use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_num: u128,
    pub previous_hash: String,
    pub timestamp: u128,
    pub nonce: u16,
    pub difficulty: u8,
    pub merkle_root: String,
    pub hash: String
}
pub struct BlockHeader {
    block_num: u128,
    previous_hash: String,
    timestamp: u128,
    nonce: u16,
    difficulty: u8,
    merkle_root: String,
    hash: String
}
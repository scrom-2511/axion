use ed25519_dalek::Signature;

pub struct TxInput {
    prev_txid: String,
    output_index: u8,
    signature: Signature,
    pub_key: String
}

pub struct TxOutput {
    pub_key: String,
    amount:u64
}
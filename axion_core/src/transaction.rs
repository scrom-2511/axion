pub struct TxInput {
    prev_txid: String,
    output_index: u8,
    signature: Signature,
    pub_key: String
}
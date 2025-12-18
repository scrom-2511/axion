use ed25519_dalek::Signature;

pub struct TxInput {
    pub prev_txid: String,
    pub output_index: u8,
    pub signature: Signature,
    pub pub_key: String
}

pub struct TxOutput {
    pub pub_key: String,
    pub amount:u64
}

pub struct Transaction {
    pub txid: String,
    pub tx_inputs: Vec<TxInput>,
    pub tx_outputs: Vec<TxOutput>
}
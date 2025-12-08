use ed25519_dalek::{SigningKey, ed25519::signature::SignerMut};
use serde::{Deserialize, Serialize};

use crate::services::hasher::Hasher;

#[derive(Serialize, Deserialize, Clone)]
pub struct TxInput {
    pub prev_tx_id: String,
    pub output_index: u32,
    pub signature: String,
    pub pub_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxOutput {
    pub recepient_pubkey: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn sign_transaction(transaction: &mut Transaction, private_key: String) {
        let key_bytes: &[u8; 32] = match private_key.as_bytes().try_into() {
            Ok(a) => a,
            Err(_) => {
                eprintln!("Private key must be 32 bytes!");
                return;
            }
        };

        let mut signing_key = SigningKey::from_bytes(key_bytes);

        let tx_id = Hasher::calculate_txid(transaction.clone());

        for input in transaction.inputs.iter_mut() {
            let signature = signing_key.sign(tx_id.as_bytes());
            input.signature = bs58::encode(signature.to_bytes()).into_string();
            input.pub_key = bs58::encode(signing_key.verifying_key().to_bytes()).into_string();
        }
    }
}

use ed25519_dalek::{
    Signature, SigningKey, VerifyingKey,
    ed25519::{signature::SignerMut},
};
use serde::{Deserialize, Serialize};

use crate::{errors::AxionError, services::hasher::Hasher};

#[derive(Serialize, Deserialize, Clone)]
pub struct TxInput {
    pub prev_tx_id: String,
    pub output_index: u32,
    pub pub_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxOutput {
    pub recepient_pubkey: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub txid: String,
    pub signature: Signature,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn string_to_u8_32_converter(string_data: String) -> Result<[u8; 32], AxionError> {
        let decoded_string_data = bs58::decode(string_data).into_vec().unwrap();
        let key_bytes: [u8; 32] = decoded_string_data.try_into().unwrap();

        Ok(key_bytes)
    }

    pub fn sign_transaction(
        transaction: &mut Transaction,
        private_key: String,
    ) -> Result<(), AxionError> {
        let key_bytes = Self::string_to_u8_32_converter(private_key)?;

        let mut signing_key = SigningKey::from_bytes(&key_bytes);

        let verifying_key = bs58::encode(signing_key.verifying_key()).into_string();

        for input in transaction.inputs.iter_mut() {
            if verifying_key != input.pub_key {
                return Err(AxionError::UnauthorizedSigner);
            }
        }

        let txid = Hasher::calculate_txid(transaction.clone());
        let signature = signing_key.sign(txid.as_bytes());
        transaction.signature = signature;

        Ok(())
    }

    pub fn verify_transaction(
        transaction: &mut Transaction,
        public_key: String,
    ) -> Result<bool, AxionError> {
        let key_bytes = Self::string_to_u8_32_converter(public_key)?;

        let verifying_key = VerifyingKey::from_bytes(&key_bytes).unwrap();
        match verifying_key.verify_strict(transaction.txid.as_bytes(), &transaction.signature) {
            Ok(()) => {}
            Err(_) => return Err(AxionError::VerificationFailed),
        }

        Ok(true)
    }
}

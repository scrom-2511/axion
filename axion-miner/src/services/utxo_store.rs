use std::collections::HashMap;
use axion_core::transaction::{TxInput, TxOutput};

pub struct UtxoStore {
    pub store: HashMap<String, TxOutput>,
}

impl UtxoStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn get_key(tx_input: &TxInput) -> String {
        format!("{}:{}", tx_input.prev_tx_id, tx_input.output_index)
    }

    pub fn add_to_store(&mut self, tx_output: TxOutput, tx_input: TxInput) {
        let key = Self::get_key(&tx_input);
        self.store.insert(key, tx_output);
    }

    pub fn spend_and_remove_from_store(&mut self, tx_input: TxInput) -> Option<TxOutput> {
        let key = Self::get_key(&tx_input);
        self.store.remove(&key)
    }
}
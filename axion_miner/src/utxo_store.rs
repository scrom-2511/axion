use std::collections::HashMap;

use axion_core::transaction::TxOutput;

pub struct UtxoStore {
    pub store: HashMap<String, TxOutput>
}
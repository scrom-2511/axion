use std::collections::HashMap;

use axion_core::transaction::TxOutput;

pub struct UtxoStore {
    store: HashMap<String, TxOutput>
}
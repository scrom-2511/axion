use std::collections::BinaryHeap;

use axion_core::transaction::{Transaction, TxOutput};

pub struct Mempool {
    pub mempool: BinaryHeap<Transaction>
}
use std::collections::BinaryHeap;

use axion_core::transaction::Transaction;

pub struct Mempool {
    mempool: BinaryHeap<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            mempool: BinaryHeap::new(),
        }
    }

    pub fn add_tx_to_pool(&mut self, tx: Transaction) {
        self.mempool.push(tx);
    }

    pub fn get_tx_from_pool(&mut self) -> Option<Transaction> {
        self.mempool.pop()
    }
}
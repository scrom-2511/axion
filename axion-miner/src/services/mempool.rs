use std::collections::VecDeque;

use axion_core::transaction::Transaction;

pub struct Mempool {
    mempool: VecDeque<Transaction>
}
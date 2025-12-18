use crate::{block_header::BlockHeader, transaction::Transaction};

pub struct Block {
    transactions: Vec<Transaction>,
    block_header: BlockHeader
}
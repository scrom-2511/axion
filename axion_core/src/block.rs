use crate::{block_header::BlockHeader, transaction::Transaction};

pub struct Block {
    pub transactions: Vec<Transaction>,
    pub block_header: BlockHeader
}
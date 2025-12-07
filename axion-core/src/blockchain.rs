use crate::{block::Block, services::hasher::Hasher};

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::from("0"));
        let blocks = vec![genesis_block];
        Self { blocks }
    }

    pub fn get_previous_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn validate_block(&self, new_block: &Block) -> bool {
        let last_block = match self.get_previous_block() {
            Some(block) => block,
            None => return false,
        };

        if new_block.timestamp < last_block.timestamp {
            return false;
        }

        if new_block.index != last_block.index + 1 {
            return false;
        };

        if new_block.previous_hash != last_block.hash {
            return false;
        };

        if !new_block.hash.starts_with("000") {
            return false;
        }

        let (nonce, hash) = Hasher::calculate_hash_with_prefix(&new_block);
        if hash != new_block.hash || nonce != new_block.nonce {
            return false;
        };

        true
    }

    pub fn add_block(&mut self, new_block: Block) {
        if self.validate_block(&new_block) {
            self.blocks.push(new_block);
        }
    }
}

use crate::{blockchain::Blockchain, mempool::Mempool, utxo_store::UtxoStore};

pub struct Miner {
    id: String,
    mempool: Mempool,
    utxo_store: UtxoStore,
    block_height: u128,
    reward_address: String,
    blockchain: Blockchain
}

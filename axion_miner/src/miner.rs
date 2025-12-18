use crate::{blockchain::Blockchain, mempool::Mempool, utxo_store::UtxoStore};

pub struct LatestBlockDetails {
    pub block_height: u128,
    pub block_hash: String,
}
pub struct Miner {
    pub id: String,
    pub mempool: Mempool,
    pub utxo_store: UtxoStore,
    pub reward_address: String,
    pub blockchain: Blockchain,
    pub latest_block_details: LatestBlockDetails
}

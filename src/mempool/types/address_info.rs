use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressInfo {
    pub address: String,
    pub chain_stats: Stats,
    pub mempool_stats: Stats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub funded_txo_count: u64,
    pub funded_txo_sum: u64,
    pub spent_txo_count: u64,
    pub spent_txo_sum: u64,
    pub tx_count: u64,
}

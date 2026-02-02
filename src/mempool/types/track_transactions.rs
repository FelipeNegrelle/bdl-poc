use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TxRequest {
    #[serde(rename = "track-txs")]
    pub track_txs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxResponse {
    #[serde(rename = "tracked-txs")]
    pub tracked_txs: HashMap<String, TxInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInfo {
    pub position: Option<Position>,
    pub cpfp: Option<Cpfp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub block: u64,
    pub vsize: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpfpEntry {
    pub txid: String,
    pub fee: i64,
    pub weight: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cpfp {
    pub ancestors: Vec<CpfpEntry>,
    #[serde(rename = "bestDescendant")]
    pub best_descendant: Option<CpfpEntry>,
    pub descendants: Vec<CpfpEntry>,
    #[serde(rename = "effectiveFeePerVsize")]
    pub effective_fee_per_vsize: f64,
    pub sigops: u64,
    #[serde(rename = "adjustedVsize")]
    pub adjusted_vsize: u64,
}

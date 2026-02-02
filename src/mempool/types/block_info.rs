use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockInfo {
    pub id: String,
    pub height: i32,
    pub version: i32,
    pub timestamp: i32,
    pub tx_count: i32,
    pub size: i32,
    pub weight: i32,
    pub merkle_root: String,
    pub previousblockhash: String,
    pub mediantime: i32,
    pub nonce: i32,
    pub bits: i32,
    pub difficulty: f32,
}

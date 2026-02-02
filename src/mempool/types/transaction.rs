use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub txid: String,
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<TransactionInput>,
    pub vout: Vec<TransactionOutput>,
    pub size: u32,
    pub weight: u32,
    pub sigops: Option<u32>,
    pub fee: u64,
    pub status: TransactionStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionInput {
    pub txid: String,
    pub vout: u32,
    pub prevout: Option<Prevout>,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    #[serde(default)]
    pub witness: Vec<String>,
    pub is_coinbase: bool,
    pub sequence: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_redeemscript_asm: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_witnessscript_asm: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prevout {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionOutput {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptpubkey_address: Option<String>,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionStatus {
    pub confirmed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<u64>,
}

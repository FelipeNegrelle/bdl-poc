use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddrRequest {
    #[serde(rename = "track-address")]
    pub track_address: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddrResponse {
    BlockTransactions {
        #[serde(rename = "block-transactions")]
        items: Vec<MempoolTransaction>,
    },
    AddressTransactions {
        #[serde(rename = "address-transactions")]
        items: Vec<MempoolTransaction>,
    },
    AddressRemovedTransactions {
        #[serde(rename = "address-removed-transactions")]
        items: Vec<MempoolTransaction>,
    },
}

impl AddrResponse {
    pub fn kind(&self) -> &'static str {
        match self {
            AddrResponse::BlockTransactions { .. } => "block-transactions",
            AddrResponse::AddressTransactions { .. } => "address-transactions",
            AddrResponse::AddressRemovedTransactions { .. } => "address-removed-transactions",
        }
    }

    pub fn transactions(&self) -> &[MempoolTransaction] {
        match self {
            AddrResponse::BlockTransactions { items }
            | AddrResponse::AddressTransactions { items }
            | AddrResponse::AddressRemovedTransactions { items } => items,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolTransaction {
    pub txid: String,
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<TransactionInput>,
    pub vout: Vec<TransactionOutput>,
    pub size: u64,
    pub weight: u64,
    pub sigops: u64,
    pub fee: u64,
    pub status: TransactionStatus,
    #[serde(default)]
    pub order: Option<u64>,
    #[serde(default)]
    pub vsize: Option<u64>,
    #[serde(rename = "adjustedVsize")]
    #[serde(default)]
    pub adjusted_vsize: Option<u64>,
    #[serde(rename = "feePerVsize")]
    #[serde(default)]
    pub fee_per_vsize: Option<f64>,
    #[serde(rename = "adjustedFeePerVsize")]
    #[serde(default)]
    pub adjusted_fee_per_vsize: Option<f64>,
    #[serde(rename = "effectiveFeePerVsize")]
    #[serde(default)]
    pub effective_fee_per_vsize: Option<f64>,
    #[serde(rename = "firstSeen")]
    #[serde(default)]
    pub first_seen: Option<u64>,
    #[serde(default)]
    pub inputs: Vec<serde_json::Value>,
    #[serde(rename = "cpfpDirty")]
    #[serde(default)]
    pub cpfp_dirty: Option<bool>,
    #[serde(default)]
    pub ancestors: Vec<serde_json::Value>,
    #[serde(default)]
    pub descendants: Vec<serde_json::Value>,
    #[serde(rename = "bestDescendant")]
    #[serde(default)]
    pub best_descendant: Option<serde_json::Value>,
    #[serde(default)]
    pub position: Option<TransactionPosition>,
    #[serde(default)]
    pub flags: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub txid: String,
    pub vout: u32,
    pub prevout: PrevOut,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub is_coinbase: bool,
    pub sequence: u64,
    #[serde(default)]
    pub inner_redeemscript_asm: Option<String>,
    #[serde(default)]
    pub witness: Option<Vec<String>>,
    #[serde(default)]
    pub inner_witnessscript_asm: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrevOut {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: Option<String>,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub confirmed: bool,
    #[serde(default)]
    pub block_height: Option<u32>,
    #[serde(default)]
    pub block_hash: Option<String>,
    #[serde(default)]
    pub block_time: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPosition {
    pub block: u32,
    pub vsize: u64,
}

//! [`api`](self).

use serde::{Deserialize, Serialize};

/// Recommended fees.
#[derive(Debug, Deserialize)]
pub struct RecommendedFees {
    /// Fastest fee.
    #[serde(alias = "fastestFee")]
    pub fastest_fee: u64,
    /// Half-hour fee.
    #[serde(alias = "halfHourFee")]
    pub half_hour_fee: u64,
    /// Hour fee.
    #[serde(alias = "hourFee")]
    pub hour_fee: u64,
    /// Economy fee.
    #[serde(alias = "economyFee")]
    pub economy_fee: u64,
    /// Minimum fee.
    #[serde(alias = "minimumFee")]
    pub minimum_fee: u64,
}

/// Element in the response to Get Address Transactions.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressTx {
    /// Transaction ID (txid) as a hex string.
    pub txid: String,
    /// Transaction version number.
    pub version: u32,
    /// Transaction locktime.
    pub locktime: u32,
    /// List of transaction inputs (vin).
    pub vin: Vec<Vin>,
    /// List of transaction outputs (vout).
    pub vout: Vec<Vout>,
    /// Transaction size in bytes.
    pub size: u32,
    /// Transaction weight (for segwit).
    pub weight: u32,
    /// Number of signature operations (sigops).
    pub sigops: u32,
    /// Transaction fee in satoshis.
    pub fee: u64,
    /// Confirmation status and block info.
    pub status: Status,
}

/// Represents a transaction input (vin).
#[derive(Debug, Serialize, Deserialize)]
pub struct Vin {
    /// Previous transaction ID referenced by this input.
    pub txid: String,
    /// Output index in the previous transaction.
    pub vout: u32,
    /// Previous output details.
    pub prevout: Vout,
    /// Script signature as a hex string.
    pub scriptsig: String,
    /// Script signature in ASM format.
    pub scriptsig_asm: String,
    /// True if this is a coinbase input.
    pub is_coinbase: bool,
    /// Sequence number for this input.
    pub sequence: u64,
}

/// Represents a transaction output (vout).
#[derive(Debug, Serialize, Deserialize)]
pub struct Vout {
    /// ScriptPubKey as a hex string.
    pub scriptpubkey: String,
    /// ScriptPubKey in ASM format.
    pub scriptpubkey_asm: String,
    /// Type of the scriptPubKey (e.g., p2pkh).
    pub scriptpubkey_type: String,
    /// Address associated with the scriptPubKey.
    #[serde(default)]
    pub scriptpubkey_address: String,
    /// Value of the output in satoshis.
    pub value: u64,
}

/// Represents the confirmation status and block information for a transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    /// True if the transaction is confirmed.
    pub confirmed: bool,
    /// Block height if confirmed.
    pub block_height: Option<u32>,
    /// Block hash if confirmed.
    pub block_hash: Option<String>,
    /// Block time (UNIX timestamp) if confirmed.
    pub block_time: Option<u64>,
}

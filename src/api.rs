//! [`api`](self).

use bitcoin::{BlockHash, ScriptBuf, TxMerkleNode, Txid};
use serde::{Deserialize, Serialize};

/// Represents response to GET Recommended Fees.
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
    pub txid: Txid,
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
    pub txid: Txid,
    /// Output index in the previous transaction.
    pub vout: u32,
    /// Previous output details.
    pub prevout: Vout,
    /// Script signature as a hex string.
    pub scriptsig: ScriptBuf,
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
    pub scriptpubkey: ScriptBuf,
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
    pub block_hash: Option<BlockHash>,
    /// Block time (UNIX timestamp) if confirmed.
    pub block_time: Option<u64>,
}

/// Represents response to Get Mempool.
#[derive(Debug, Serialize, Deserialize)]
pub struct MempoolStats {
    /// Number of transactions in the mempool.
    pub count: u64,
    /// Total virtual size of all mempool transactions.
    pub vsize: u64,
    /// Total fees in the mempool (sats).
    pub total_fee: u64,
    /// Fee histogram (array of (fee_rate, vsize) pairs).
    pub fee_histogram: Vec<(f64, u64)>,
}

/// Represents a Bitcoin transaction from Get Transaction.
// Note: This is the same as AddressTx.
#[derive(Debug, Serialize, Deserialize)]
pub struct TxInfo {
    /// Transaction ID (hex).
    pub txid: Txid,
    /// Transaction version.
    pub version: u32,
    /// Transaction locktime.
    pub locktime: u32,
    /// List of transaction inputs.
    pub vin: Vec<Vin>,
    /// List of transaction outputs.
    pub vout: Vec<Vout>,
    /// Transaction size in bytes.
    pub size: u32,
    /// Transaction weight (for segwit).
    pub weight: u32,
    /// Number of signature operations.
    pub sigops: u64,
    /// Transaction fee in satoshis.
    pub fee: u64,
    /// Confirmation status and block info.
    pub status: Status,
}

/// Represents a Bitcoin block from Get Block.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSummary {
    /// Block hash.
    pub id: BlockHash,
    /// Block height.
    pub height: u32,
    /// Block version.
    pub version: u32,
    /// Block timestamp (UNIX).
    pub timestamp: u64,
    /// Number of transactions in the block.
    pub tx_count: u32,
    /// Block size in bytes.
    pub size: u32,
    /// Block weight (WU).
    pub weight: u32,
    /// Merkle root.
    pub merkle_root: TxMerkleNode,
    /// Previous block hash (hex).
    pub previousblockhash: BlockHash,
    /// Median time past.
    pub mediantime: u64,
    /// Block nonce.
    pub nonce: u64,
    /// Block bits (difficulty target).
    pub bits: u32,
    /// Block difficulty.
    pub difficulty: f64,
}

/// Represents address details from Get Address.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressInfo {
    /// The address string.
    pub address: String,
    /// On-chain stats.
    pub chain_stats: AddressStats,
    /// Mempool stats.
    pub mempool_stats: AddressStats,
}

/// Represents address statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressStats {
    /// Number of funded outputs.
    pub funded_txo_count: u64,
    /// Sum of funded outputs (sats).
    pub funded_txo_sum: u64,
    /// Number of spent outputs.
    pub spent_txo_count: u64,
    /// Sum of spent outputs (sats).
    pub spent_txo_sum: u64,
    /// Number of transactions.
    pub tx_count: u64,
}

/// Represents a response to Get Transaction Merkle Proof.
#[derive(Debug, Deserialize)]
pub struct MerkleProof {
    /// Block height.
    pub block_height: u32,
    /// Merkle proof.
    pub merkle: Vec<Txid>,
    /// Index of the corresponding transaction in block.
    pub pos: usize,
}

/// Represents an element in the response to Get Address UTXO.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressUtxo {
    /// Transaction ID (txid) as a hex string.
    pub txid: Txid,
    /// Output index in the transaction.
    pub vout: u32,
    /// Value of the output in satoshis.
    pub value: u64,
    /// Status of the UTXO (confirmation info).
    pub status: Status,
}

/// Represents response to Get Transaction Outspend.
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputStatus {
    /// True if the output has been spent.
    pub spent: bool,
    /// Transaction ID of the spending transaction (if spent).
    pub txid: Option<Txid>,
    /// Input index in the spending transaction (if spent).
    pub vin: Option<u32>,
    /// Status of the spending transaction (if spent).
    pub status: Option<Status>,
}

/// Represents response to Get Block Status.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockStatus {
    /// True if the block is in the best chain.
    pub in_best_chain: bool,
    /// Height of the block.
    pub height: Option<u32>,
    /// Block hash of the next block in the best chain.
    pub next_best: Option<BlockHash>,
}

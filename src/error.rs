//! [`Error`].

use bitcoin::{consensus, hex};

/// Errors that can occur in this library.
#[derive(thiserror::Error, Debug)]
pub enum Error<E> {
    /// API error.
    #[error("API error: {0}")]
    Api(String),
    /// `bitcoin::consensus` encoding error.
    #[error("encoding error: {0}")]
    Decode(consensus::encode::Error),
    /// `bitcoin::consensus` encoding error (from hex).
    #[error("encoding error: {0}")]
    DecodeHex(consensus::encode::FromHexError),
    /// Transport error.
    #[error("transport error: {0:?}")]
    Transport(E),
    /// Converting from hex
    #[error("hex to array error: {0}")]
    HexToArray(hex::HexToArrayError),
}

//! [`Error`].

use bitcoin::{consensus, hex};

/// Errors that can occur in this library.
#[derive(Debug)]
pub enum Error<E> {
    /// `bitcoin::consensus` encoding error.
    Decode(consensus::encode::Error),
    /// `bitcoin::consensus` encoding error (from hex).
    DecodeHex(consensus::encode::FromHexError),
    /// Converting from hex to array
    HexToArray(hex::HexToArrayError),
    /// `serde_json` error.
    Json(serde_json::Error),
    /// Transport error.
    Transport(E),
}

impl<E: core::fmt::Display> core::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Decode(e) => write!(f, "{e}"),
            Self::DecodeHex(e) => write!(f, "{e}"),
            Self::HexToArray(e) => write!(f, "{e}"),
            Self::Json(e) => write!(f, "{e}"),
            Self::Transport(e) => write!(f, "{e}"),
        }
    }
}

impl<E> std::error::Error for Error<E> where E: core::fmt::Debug + core::fmt::Display {}

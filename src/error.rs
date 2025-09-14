//! [`Error`].

use crate::Transport;

/// Errors that can occur in this library.
#[derive(thiserror::Error, Debug)]
pub enum Error<T: Transport> {
    /// API error.
    #[error("API error: {0}")]
    Api(String),
    /// Transport error.
    #[error("transport error: {0}")]
    Transport(<T as Transport>::Err),
    /// Converting from hex
    #[error("hex to array error: {0}")]
    HexToArray(bitcoin::hex::HexToArrayError),
}

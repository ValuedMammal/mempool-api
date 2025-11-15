//! `mempool_space_api`
#![doc = include_str!("../README.md")]

pub mod api;
#[cfg(feature = "bitreq")]
mod bitreq_client;
mod client;
mod error;
mod http;

#[cfg(feature = "bitreq")]
pub use bitreq_client::*;
pub use client::*;
pub use error::*;
pub use http::*;

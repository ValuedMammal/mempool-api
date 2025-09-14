//! [`AsyncClient`].

use bitcoin::BlockHash;

use crate::Error;
use crate::Transport;
use crate::api::RecommendedFees;

/// Async client, generic over the [`Transport`].
pub struct AsyncClient<T> {
    /// Base url
    pub url: String,
    /// Transport.
    tx: T,
}

impl<T: Transport> AsyncClient<T> {
    /// New.
    pub fn new(url: &str, tx: T) -> Self {
        Self {
            url: url.to_string(),
            tx,
        }
    }

    /// GET `/blocks/tip/hash`.
    pub async fn get_tip_hash(&self) -> Result<BlockHash, Error<T>> {
        let path = format!("{}/blocks/tip/hash", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;
        let s = self
            .tx
            .parse_response_text(resp)
            .await
            .map_err(Error::Transport)?;

        s.parse().map_err(Error::HexToArray)
    }

    /// GET `/fees/recommended`.
    pub async fn get_recommended_fees(&self) -> Result<RecommendedFees, Error<T>> {
        let path = format!("{}/v1/fees/recommended", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx
            .parse_response_json(resp)
            .await
            .map_err(Error::Transport)
    }

    /// POST `/tx`.
    pub async fn broadcast(&self, tx: &bitcoin::Transaction) -> Result<String, Error<T>> {
        let path = format!("{}/tx", self.url);
        let hex = bitcoin::consensus::encode::serialize_hex(tx);
        let resp = self.tx.post(&path, hex).await.map_err(Error::Transport)?;

        self.tx
            .parse_response_text(resp)
            .await
            .map_err(Error::Transport)
    }
}

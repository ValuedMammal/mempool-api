//! [`AsyncClient`].

use core::fmt::{self, Debug};

use bitcoin::{
    Address, Block, BlockHash, Script, Transaction, Txid,
    block::Header,
    consensus,
    hashes::{Hash, sha256},
};

use crate::Error;
use crate::Transport;
use crate::api::{AddressInfo, AddressTx, BlockSummary, MempoolStats, RecommendedFees, TxInfo};

/// Async client, generic over the [`Transport`].
pub struct AsyncClient<T> {
    /// Base url
    pub url: String,
    /// Transport.
    tx: T,
}

impl<T: Debug> Debug for AsyncClient<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncClient")
            .field("url", &self.url)
            .field("tx", &self.tx)
            .finish()
    }
}

impl<T: Transport> AsyncClient<T> {
    /// New.
    pub fn new(url: &str, tx: T) -> Self {
        Self {
            url: url.to_string(),
            tx,
        }
    }

    /// GET `/tx/:txid/hex`.
    pub async fn get_tx(&self, txid: &Txid) -> Result<Transaction, Error<T::Err>> {
        let path = format!("{}/tx/{txid}/hex", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;
        let hex = self.tx.handle_response_text(resp).await.map_err(Error::Transport)?;

        consensus::encode::deserialize_hex(&hex).map_err(Error::DecodeHex)
    }

    /// GET `/tx/:txid`.
    pub async fn get_tx_info(&self, txid: &Txid) -> Result<TxInfo, Error<T::Err>> {
        let path = format!("{}/tx/{txid}", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx.handle_response_json(resp).await.map_err(Error::Transport)
    }

    /// GET `/blocks/tip/hash`.
    pub async fn get_tip_hash(&self) -> Result<BlockHash, Error<T::Err>> {
        let path = format!("{}/blocks/tip/hash", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;
        let s = self.tx.handle_response_text(resp).await.map_err(Error::Transport)?;

        s.parse().map_err(Error::HexToArray)
    }

    /// GET `/blocks/tip/height`.
    pub async fn get_tip_height(&self) -> Result<u32, Error<T::Err>> {
        let path = format!("{}/blocks/tip/height", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        Ok(self
            .tx
            .handle_response_text(resp)
            .await
            .map_err(Error::Transport)?
            .parse::<u32>()
            .unwrap())
    }

    /// GET `/scripthash/:hex/txs`.
    pub async fn get_scripthash_txs(
        &self,
        script: &Script,
        after_txid: Option<Txid>,
    ) -> Result<Vec<AddressTx>, Error<T::Err>> {
        let script_hash = sha256::Hash::hash(script.as_bytes());
        let path = match after_txid {
            Some(txid) => format!("{}/scripthash/{script_hash:x}/txs/chain/{txid}", self.url),
            None => format!("{}/scripthash/{script_hash:x}/txs", self.url),
        };
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx.handle_response_json(resp).await.map_err(Error::Transport)
    }

    /// GET `/address/:address/txs`.
    pub async fn get_address_txs(
        &self,
        address: &Address,
        after_txid: Option<Txid>,
    ) -> Result<Vec<AddressTx>, Error<T::Err>> {
        let path = match after_txid {
            Some(txid) => format!("{}/address/{address}/txs?after_txid={txid}", self.url),
            None => format!("{}/address/{address}/txs", self.url),
        };
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx.handle_response_json(resp).await.map_err(Error::Transport)
    }

    /// GET `/address/:address`.
    pub async fn get_address_info(&self, address: &Address) -> Result<AddressInfo, Error<T::Err>> {
        let path = format!("{}/address/{address}", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx.handle_response_json(resp).await.map_err(Error::Transport)
    }

    /// GET `/fees/recommended`.
    pub async fn get_recommended_fees(&self) -> Result<RecommendedFees, Error<T::Err>> {
        let path = format!("{}/v1/fees/recommended", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx.handle_response_json(resp).await.map_err(Error::Transport)
    }

    /// GET `/mempool`.
    pub async fn get_mempool_info(&self) -> Result<MempoolStats, Error<T::Err>> {
        let path = format!("{}/mempool", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx.handle_response_json(resp).await.map_err(Error::Transport)
    }

    /// GET `/mempool/txids`.
    pub async fn get_mempool_txids(&self) -> Result<Vec<Txid>, Error<T::Err>> {
        let path = format!("{}/mempool/txids", self.url);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;
        let txids: Vec<String> =
            self.tx.handle_response_json(resp).await.map_err(Error::Transport)?;
        txids
            .into_iter()
            .map(|s| s.parse().map_err(Error::HexToArray))
            .collect()
    }

    /// GET `/block/:hash/header`.
    pub async fn get_block_header(&self, hash: BlockHash) -> Result<Header, Error<T::Err>> {
        let path = format!("{}/block/{}/header", self.url, hash);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;
        let hex = self.tx.handle_response_text(resp).await.map_err(Error::Transport)?;

        consensus::encode::deserialize_hex(&hex).map_err(Error::DecodeHex)
    }

    /// GET `/block/:hash/raw`.
    pub async fn get_block(&self, hash: BlockHash) -> Result<Block, Error<T::Err>> {
        let path = format!("{}/block/{}/raw", self.url, hash);
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;
        let bytes = self.tx.handle_response_raw(resp).await.map_err(Error::Transport)?;

        consensus::encode::deserialize(&bytes).map_err(Error::Decode)
    }

    /// GET `/blocks/[:height]`.
    pub async fn get_blocks(
        &self,
        height: Option<u32>,
    ) -> Result<Vec<BlockSummary>, Error<T::Err>> {
        let path = match height {
            Some(height) => format!("{}/blocks/{height}", self.url),
            None => format!("{}/blocks", self.url),
        };
        let resp = self.tx.get(&path).await.map_err(Error::Transport)?;

        self.tx.handle_response_json(resp).await.map_err(Error::Transport)
    }

    /// POST `/tx`.
    pub async fn broadcast(&self, tx: &bitcoin::Transaction) -> Result<String, Error<T::Err>> {
        let path = format!("{}/tx", self.url);
        let hex = consensus::encode::serialize_hex(tx);
        let resp = self.tx.post(&path, hex).await.map_err(Error::Transport)?;

        self.tx.handle_response_text(resp).await.map_err(Error::Transport)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const URL: &str = "https://mempool.space/api";

    #[tokio::test]
    async fn test_get_tip() -> anyhow::Result<()> {
        let reqwest_client = crate::ReqwestClient::new();
        let client = AsyncClient::new(URL, reqwest_client);

        let _ = client.get_tip_height().await?;
        let _ = client.get_tip_hash().await?;

        Ok(())
    }
}

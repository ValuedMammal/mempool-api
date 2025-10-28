//! [`AsyncClient`].

use core::fmt::{self, Debug};

use bitcoin::{
    Address, Block, BlockHash, MerkleBlock, Script, Transaction, Txid,
    block::Header,
    consensus,
    hashes::{Hash, sha256},
};

use crate::Error;
use crate::api::{
    AddressInfo, AddressTx, AddressUtxo, BlockStatus, BlockSummary, MempoolStats, MerkleProof,
    OutputStatus, RecommendedFees, Status, TxInfo,
};
use crate::http::{Http, HttpMethod as Method};

/// Async client that is generic over the [`Http`] implementation.
pub struct AsyncClient<T> {
    /// Base url
    pub url: String,
    /// inner HTTP client.
    inner: T,
}

impl<T: Debug> Debug for AsyncClient<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncClient")
            .field("url", &self.url)
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: Http> AsyncClient<T> {
    /// New.
    pub fn new(url: &str, inner: T) -> Self {
        Self {
            url: url.to_string(),
            inner,
        }
    }

    /// Sends a GET request to the given `path` with an empty body.
    async fn get(&self, path: &str) -> Result<T::Body, T::Err> {
        self.inner.send(Method::GET, path, vec![]).await
    }

    /// GET `/blocks/tip/hash`.
    pub async fn get_tip_hash(&self) -> Result<BlockHash, Error<T::Err>> {
        let path = format!("{}/blocks/tip/hash", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let s = String::from_utf8_lossy(body.as_ref());

        s.parse().map_err(Error::HexToArray)
    }

    /// GET `/blocks/tip/height`.
    pub async fn get_tip_height(&self) -> Result<u32, Error<T::Err>> {
        let path = format!("{}/blocks/tip/height", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let s = String::from_utf8_lossy(body.as_ref());

        s.parse::<u32>().map_err(Error::ParseInt)
    }

    /// GET `/block-height/:height`.
    pub async fn get_block_hash(&self, height: u32) -> Result<BlockHash, Error<T::Err>> {
        let path = format!("{}/block-height/{height}", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let s = String::from_utf8_lossy(body.as_ref());

        s.parse().map_err(Error::HexToArray)
    }

    /// GET `/tx/:txid/hex`.
    pub async fn get_tx(&self, txid: &Txid) -> Result<Transaction, Error<T::Err>> {
        let path = format!("{}/tx/{txid}/hex", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let s = String::from_utf8_lossy(body.as_ref());

        consensus::encode::deserialize_hex(&s).map_err(Error::DecodeHex)
    }

    /// GET `/tx/:txid`.
    pub async fn get_tx_info(&self, txid: &Txid) -> Result<TxInfo, Error<T::Err>> {
        let path = format!("{}/tx/{txid}", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/tx/:txid/status`.
    pub async fn get_tx_status(&self, txid: &Txid) -> Result<Status, Error<T::Err>> {
        let path = format!("{}/tx/{txid}/status", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/tx/:txid/outspend/:vout`.
    pub async fn get_output_status(
        &self,
        txid: &Txid,
        vout: u32,
    ) -> Result<Option<OutputStatus>, Error<T::Err>> {
        // Note: `mempool/electrs` currently returns a default output status
        // for non-existent outputs, so here we fetch the correct outspends
        // and then lookup the specified vout.
        Ok(self.get_outspends(txid).await?.get(vout as usize).cloned())
    }

    /// GET `/tx/:txid/outspends`.
    pub async fn get_outspends(&self, txid: &Txid) -> Result<Vec<OutputStatus>, Error<T::Err>> {
        let path = format!("{}/tx/{txid}/outspends", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
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
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
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
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// Get `address/:address/utxo`
    pub async fn get_address_utxos(
        &self,
        address: &Address,
    ) -> Result<Vec<AddressUtxo>, Error<T::Err>> {
        let path = format!("{}/address/{address}/utxo", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/address/:address`.
    pub async fn get_address_info(&self, address: &Address) -> Result<AddressInfo, Error<T::Err>> {
        let path = format!("{}/address/{address}", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/fees/recommended`.
    pub async fn get_recommended_fees(&self) -> Result<RecommendedFees, Error<T::Err>> {
        let path = format!("{}/v1/fees/recommended", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/mempool`.
    pub async fn get_mempool_info(&self) -> Result<MempoolStats, Error<T::Err>> {
        let path = format!("{}/mempool", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/mempool/txids`.
    pub async fn get_mempool_txids(&self) -> Result<Vec<Txid>, Error<T::Err>> {
        let path = format!("{}/mempool/txids", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let txids: Vec<String> = serde_json::from_slice(body.as_ref()).map_err(Error::Json)?;

        txids
            .into_iter()
            .map(|s| s.parse().map_err(Error::HexToArray))
            .collect()
    }

    /// GET `/block/:hash/header`.
    pub async fn get_block_header(&self, hash: &BlockHash) -> Result<Header, Error<T::Err>> {
        let path = format!("{}/block/{hash}/header", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let s = String::from_utf8_lossy(body.as_ref());

        consensus::encode::deserialize_hex(&s).map_err(Error::DecodeHex)
    }

    /// GET `/block/:hash/raw`.
    pub async fn get_block(&self, hash: &BlockHash) -> Result<Block, Error<T::Err>> {
        let path = format!("{}/block/{hash}/raw", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        consensus::encode::deserialize(body.as_ref()).map_err(Error::Decode)
    }

    /// GET `/block/:hash/status`.
    pub async fn get_block_status(&self, hash: &BlockHash) -> Result<BlockStatus, Error<T::Err>> {
        let path = format!("{}/block/{hash}/status", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/blocks/:height`.
    pub async fn get_blocks(
        &self,
        height: Option<u32>,
    ) -> Result<Vec<BlockSummary>, Error<T::Err>> {
        let path = match height {
            Some(height) => format!("{}/blocks/{height}", self.url),
            None => format!("{}/blocks", self.url),
        };
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// POST `/tx`.
    pub async fn broadcast(&self, tx: &bitcoin::Transaction) -> Result<Txid, Error<T::Err>> {
        let path = format!("{}/tx", self.url);
        let hex = consensus::encode::serialize_hex(tx);
        let body = self
            .inner
            .send(Method::POST, &path, hex.as_bytes().to_vec())
            .await
            .map_err(Error::Http)?;

        let s = String::from_utf8_lossy(body.as_ref());

        s.parse().map_err(Error::HexToArray)
    }

    /// GET `/tx/:txid/merkle-proof`.
    pub async fn get_merkle_proof(&self, txid: &Txid) -> Result<MerkleProof, Error<T::Err>> {
        let path = format!("{}/tx/{txid}/merkle-proof", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;

        serde_json::from_slice(body.as_ref()).map_err(Error::Json)
    }

    /// GET `/block/:hash/txid/:index`.
    pub async fn get_tx_at_index(
        &self,
        hash: &BlockHash,
        index: usize,
    ) -> Result<Txid, Error<T::Err>> {
        let path = format!("{}/block/{hash}/txid/{index}", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let s = String::from_utf8_lossy(body.as_ref());

        s.parse().map_err(Error::HexToArray)
    }

    /// GET `/tx/:txid/merkleblock-proof`.
    pub async fn get_merkle_block(&self, txid: &Txid) -> Result<MerkleBlock, Error<T::Err>> {
        let path = format!("{}/tx/{txid}/merkleblock-proof", self.url);
        let body = self.get(&path).await.map_err(Error::Http)?;
        let s = String::from_utf8_lossy(body.as_ref());

        consensus::encode::deserialize_hex(&s).map_err(Error::DecodeHex)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const URL: &str = "https://mempool.space/api";

    #[tokio::test]
    async fn test_get_tip() -> anyhow::Result<()> {
        let bitreq_client = crate::BitreqClient::new();
        let client = AsyncClient::new(URL, bitreq_client);

        let _ = client.get_tip_height().await?;
        let _ = client.get_tip_hash().await?;

        Ok(())
    }
}

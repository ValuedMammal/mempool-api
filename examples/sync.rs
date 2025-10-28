use bitcoin::{Address, Network, ScriptBuf, secp256k1};
use futures::{TryStreamExt, stream::FuturesOrdered};
use mempool_space_api::{AsyncClient, api::AddressTx};
use miniscript::descriptor::Descriptor;
use std::sync::Arc;

/// Server url.
const URL: &str = "https://mempool.space/signet/api";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();

    let bitreq_client = mempool_space_api::BitreqClient::default();
    let client = AsyncClient::new(URL, &bitreq_client);

    let desc_str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/0/*)";
    let secp = secp256k1::Secp256k1::new();
    let desc = Descriptor::parse_descriptor(&secp, desc_str)?.0;

    let mut spks = (0..1000).map(|i| {
        let spk = desc.at_derivation_index(i).unwrap().script_pubkey();
        (i, spk)
    });

    let mut unused_ct = 0;
    let mut last_active = None;

    // Sync
    let client = Arc::new(client);
    loop {
        let futures = spks
            .by_ref()
            .take(5)
            .map(|(i, spk)| {
                let client = client.clone();
                let mut res_txs = vec![];
                let mut after_txid = None;
                async move {
                    loop {
                        let txs = client.get_scripthash_txs(&spk, after_txid).await?;
                        let tx_ct = txs.len();
                        after_txid = txs.last().map(|tx| tx.txid);
                        res_txs.extend(txs);
                        if tx_ct < 25 {
                            break;
                        }
                    }
                    Ok::<(u32, ScriptBuf, Vec<AddressTx>), anyhow::Error>((i, spk, res_txs))
                }
            })
            .collect::<FuturesOrdered<_>>();

        for (index, script, txs) in futures.try_collect::<Vec<_>>().await? {
            if txs.is_empty() {
                unused_ct += 1;
            } else {
                last_active = Some(index);
            }
            let addr = Address::from_script(&script, Network::Signet)?;
            log::info!("Txs of index {index} address {addr}");
            for tx in txs {
                log::info!("{}", tx.txid);
            }
        }

        // Gap limit reached
        if unused_ct > 20 {
            log::info!("Last active index {:?}", last_active);
            break;
        }
    }

    Ok(())
}

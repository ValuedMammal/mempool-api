#![allow(unused)]

use bitcoin::{Address, ScriptBuf};
use futures::{TryStreamExt, stream::FuturesOrdered};
use mempool_space_api::{AsyncClient, api::AddressTx};
use miniscript::descriptor::Descriptor;
use std::sync::Arc;

/// Server url.
const URL: &str = "https://mempool.space/signet/api";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init_timed();

    let reqwest_client = mempool_space_api::ReqwestClient::default();
    let client = AsyncClient::new(URL, &reqwest_client);

    let s = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/0/*)";
    let secp = bitcoin::secp256k1::Secp256k1::new();
    let desc = Descriptor::parse_descriptor(&secp, s)?.0;

    let mut addrs = (0..1000).map(|i| {
        let spk = desc.at_derivation_index(i).unwrap().script_pubkey();
        let addr = bitcoin::Address::from_script(&spk, bitcoin::Network::Signet).unwrap();
        (i, addr)
    });

    let mut unused_ct = 0;
    let mut last_active = None;

    // Sync
    let client = Arc::new(client);
    loop {
        let futures = addrs
            .by_ref()
            .take(5)
            .map(|(i, addr)| {
                let client = client.clone();
                let mut res_txs = vec![];
                let mut after_txid = None;
                async move {
                    loop {
                        let txs = client.get_address_txs(&addr, after_txid).await?;
                        let tx_ct = txs.len();
                        after_txid = txs.last().map(|tx| tx.txid);
                        res_txs.extend(txs);
                        if tx_ct < 25 {
                            break;
                        }
                    }
                    Ok::<(u32, Address, Vec<AddressTx>), anyhow::Error>((i, addr, res_txs))
                }
            })
            .collect::<FuturesOrdered<_>>();

        for (i, addr, txs) in futures.try_collect::<Vec<_>>().await? {
            if txs.is_empty() {
                unused_ct += 1;
            } else {
                last_active = Some(i);
            }
            for tx in txs {
                dbg!(i, &addr, tx.txid);
            }
        }

        // Gap limit reached
        if unused_ct > 20 {
            dbg!(last_active);
            break;
        }
    }

    Ok(())
}

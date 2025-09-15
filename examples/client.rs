#![allow(unused)]

use mempool_space_api::AsyncClient;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://mempool.space/api";
    let reqwest_client = mempool_space_api::ReqwestClient::new();
    let client = AsyncClient::new(url, &reqwest_client);

    // GET /blocks/tip/height.
    let res = client.get_tip_height().await?;
    println!("{res:#?}");

    // GET /blocks/tip/hash.
    let res = client.get_tip_hash().await?;
    println!("{res:#?}");

    Ok(())
}

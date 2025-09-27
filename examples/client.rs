#![allow(unused)]

use mempool_space_api::AsyncClient;

/// Server url.
const URL: &str = "https://mempool.space/api";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let reqwest_client = mempool_space_api::ReqwestClient::default();
    let client = AsyncClient::new(URL, &reqwest_client);

    // GET /blocks/tip/height.
    let res = client.get_tip_height().await?;
    println!("{res:#?}");

    // GET /blocks/tip/hash.
    let res = client.get_tip_hash().await?;
    println!("{res:#?}");

    Ok(())
}

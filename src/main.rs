use cdk::mint_url::MintUrl;
use cdk_common::MintInfo;

use std::str::FromStr;
use std::sync::Arc;

use cdk::nuts::CurrencyUnit;
use cdk::wallet::{HttpClient, MintConnector, Wallet};
use cdk_sqlite::wallet::memory;

let mint_url = "http://127.0.0.1:8085/";

// Mint Info
async fn mint_info() -> anyhow::Result<MintInfo> {
    println!("Trying to fetch the Mint Info");

    println!("Mint url: {}", mint_url);

    let client = HttpClient::new(MintUrl::from_str(mint_url)?, None);

    let mint_info = client.get_mint_info().await?;

    Ok(mint_info)
}

// Mint Quote
async fn mint_quote() -> anyhow::Result<()> {
    todo!("Implement the Mint Quote request")
}

// Mint
async fn mint_tokens() -> anyhow::Result<()> {
    todo!("Implement the Mint request")
}

// Melt Quote
async fn melt_quote() -> anyhow::Result<()> {
    todo!("Implement the melt quote request")
}

// Melt
async fn melt_tokens() -> anyhow::Result<()> {
    todo!("Implement the melt request")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("First CDK Wallet");

    let mint_info = mint_info().await?;
    println!("Mint info: {}", serde_json::to_string_pretty(&mint_info)?);

    let seed: [u8; 64] = rand::random();

    let mint_url = "http://127.0.0.1:8085/";
    println!("Mint url: {}", mint_url);

    let unit = CurrencyUnit::Sat;
    println!("Currency: {}", unit);

    let localstore = Arc::new(memory::empty().await?);

    let wallet = Wallet::new(mint_url, unit, localstore, seed, None)?;

    let balance = wallet.total_balance().await?;
    println!("Total wallet balance: {}", balance);

    Ok(())
}

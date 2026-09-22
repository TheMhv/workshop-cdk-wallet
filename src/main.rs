use std::{collections::HashMap, env, str::FromStr, sync::Arc};

use anyhow::Ok;
use cdk::wallet::{
    HttpClient, MeltQuote, MintConnector, MintQuote, ReceiveOptions, SendOptions, Wallet,
};
use cdk_common::{
    Amount, CurrencyUnit, FinalizedMelt, MintInfo, Proofs, Token, amount::SplitTarget,
    mint_url::MintUrl,
};
use cdk_sqlite::WalletSqliteDatabase;

mod cli;

async fn mint_info(mint_url: &MintUrl) -> anyhow::Result<MintInfo> {
    let client = HttpClient::new(mint_url.clone(), None);
    let mint_info = client.get_mint_info().await?;

    Ok(mint_info)
}

async fn create_wallet(
    mint_url: &MintUrl,
    unit: CurrencyUnit,
    database: WalletSqliteDatabase,
    seed: [u8; 64],
) -> anyhow::Result<Wallet> {
    let wallet = Wallet::new(&mint_url.to_string(), unit, Arc::new(database), seed, None)?;

    Ok(wallet)
}

async fn get_balance(wallet: &Wallet) -> anyhow::Result<Amount> {
    let balance = wallet.total_balance().await?;

    Ok(balance)
}

async fn mint_quote(
    mint_info: &MintInfo,
    wallet: &Wallet,
    amount: Amount,
) -> anyhow::Result<MintQuote> {
    let payment_method = mint_info.nuts.nut04.methods.first().unwrap().method.clone();

    let quote = wallet
        .mint_quote(payment_method, Some(amount), None, None)
        .await?;

    Ok(quote)
}

async fn get_mint_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MintQuote> {
    let quote = wallet.check_mint_quote(quote_id).await?;

    Ok(quote)
}

async fn mint_tokens(wallet: &Wallet, quote_id: &str) -> anyhow::Result<Proofs> {
    let proofs = wallet.mint(quote_id, SplitTarget::None, None).await?;

    Ok(proofs)
}

async fn create_token(wallet: &Wallet, amount: Amount) -> anyhow::Result<Token> {
    let token = wallet
        .prepare_send(amount, SendOptions::default())
        .await?
        .confirm(None)
        .await?;

    Ok(token)
}

async fn receive_token(wallet: &Wallet, token: &Token) -> anyhow::Result<Amount> {
    let amount = wallet
        .receive(&token.to_string(), ReceiveOptions::default())
        .await?;

    Ok(amount)
}

async fn melt_quote(
    wallet: &Wallet,
    mint_info: MintInfo,
    receive_invoice: &str,
) -> anyhow::Result<MeltQuote> {
    let method = mint_info.nuts.nut05.methods.first().unwrap().method.clone();
    let quote = wallet
        .melt_quote(method, receive_invoice, None, None)
        .await?;

    Ok(quote)
}

async fn get_melt_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MeltQuote> {
    let quote = wallet.check_melt_quote_status(quote_id).await?;

    Ok(quote)
}

async fn melt(wallet: &Wallet, quote_id: &str) -> anyhow::Result<FinalizedMelt> {
    let melt = wallet
        .prepare_melt(quote_id, HashMap::new())
        .await?
        .confirm()
        .await?;

    Ok(melt)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mint_url: MintUrl =
        MintUrl::from_str(&env::var("MINT_URL").expect("Unable to set MINT URL"))?;

    cli::cli(mint_url).await
}

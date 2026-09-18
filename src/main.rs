use std::{env, str::FromStr, sync::Arc, time::Duration};

use anyhow::Ok;
use cdk::wallet::{HttpClient, MeltQuote, MintConnector, MintQuote, SendOptions, Wallet};
use cdk_common::{
    Amount, CurrencyUnit, MintInfo, MintQuoteState, Proofs, Token, amount::SplitTarget,
    mint_url::MintUrl,
};
use cdk_sqlite::{WalletSqliteDatabase, wallet};
use lnd_grpc_rust::lnrpc::Payment;
use tokio::time::sleep;

use crate::lnd::LND;

mod cli;
mod lnd;

async fn mint_info(mint_url: &MintUrl) -> anyhow::Result<MintInfo> {
    let client = HttpClient::new(mint_url.clone(), None);
    let info = client.get_mint_info().await?;

    Ok(info)
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
    let mint_quote = wallet
        .mint_quote(payment_method, Some(amount), None, None)
        .await?;

    Ok(mint_quote)
}

async fn get_mint_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MintQuote> {
    let quote = wallet.check_mint_quote(&quote_id).await?;

    Ok(quote)
}

async fn mint_tokens(wallet: &Wallet, quote_id: &str) -> anyhow::Result<Proofs> {
    let proofs = wallet.mint(&quote_id, SplitTarget::None, None).await?;

    Ok(proofs)
}

async fn create_token(wallet: &Wallet, amount: Amount) -> anyhow::Result<Token> {
    let prepare_send = wallet.prepare_send(amount, SendOptions::default()).await?;
    let token = prepare_send.confirm(None).await?;

    Ok(token)
}

async fn receive_token(wallet: &Wallet, token: &Token) -> anyhow::Result<Amount> {
    let amount = wallet.receive(&token.to_string(), ReceiveOptions::default());

    Ok(amout)
}

async fn melt_quote(
    wallet: &Wallet,
    mint_info: MintInfo,
    receive_invoice: &str,
) -> anyhow::Result<MeltQuote> {
    let payment_method = mint_info.nuts.nut05.methods.first().unwrap().method.clone();
    let melt_quote = wallet
        .melt_quote(payment_method, receive_invoice, None, None)
        .await?;

    Ok(melt_quote)
}

async fn get_melt_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MeltQuote> {
    let quote = wallet.check_melt_quote_status(&quote_id).await?;

    Ok(quote)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mint_url: MintUrl =
        MintUrl::from_str(&env::var("MINT_URL").expect("Unable to set MINT URL"))?;

    let macaroon_path = env::var("LND_MACAROON_PATH").expect("Unable to find LND macaroon file");

    let cert_path = env::var("LND_CERT_PATH").expect("Unable to find LND cert file");

    let lnd_url = env::var("LND_URL").expect("Unable to find LND URL server");

    let lnd_node = LND::new(macaroon_path, cert_path, lnd_url).await?;

    cli::cli(mint_url, lnd_node).await
}

use std::{env, str::FromStr};

use cdk::wallet::{MeltQuote, MintQuote, Wallet};
use cdk_common::{Amount, CurrencyUnit, FinalizedMelt, MintInfo, Proofs, Token, mint_url::MintUrl};
use cdk_sqlite::WalletSqliteDatabase;

use crate::lnd::LND;

mod cli;
mod lnd;

async fn mint_info(mint_url: &MintUrl) -> anyhow::Result<MintInfo> {
    todo!("Implement a function to get the mint info");
}

async fn create_wallet(
    mint_url: &MintUrl,
    unit: CurrencyUnit,
    database: WalletSqliteDatabase,
    seed: [u8; 64],
) -> anyhow::Result<Wallet> {
    todo!(
        "Implement a function to create a new Wallet specifying the mint, the unit, the database that will be used and a seed"
    );
}

async fn get_balance(wallet: &Wallet) -> anyhow::Result<Amount> {
    todo!("Implement a function to get the current balance of the Wallet");
}

async fn mint_quote(
    mint_info: &MintInfo,
    wallet: &Wallet,
    amount: Amount,
) -> anyhow::Result<MintQuote> {
    todo!("Implement a function to create a Mint Quote from Wallet specifying the amount");
}

async fn get_mint_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MintQuote> {
    todo!("Implement a function to get a Mint Quote from Wallet from quote id");
}

async fn mint_tokens(wallet: &Wallet, quote_id: &str) -> anyhow::Result<Proofs> {
    todo!("Implement a function to mint tokens from Mint Quote by quote id");
}

async fn create_token(wallet: &Wallet, amount: Amount) -> anyhow::Result<Token> {
    todo!("Implement a function to create a token from Wallet");
}

async fn receive_token(wallet: &Wallet, token: &Token) -> anyhow::Result<Amount> {
    todo!("Implement a function to redeem a token for Wallet");
}

async fn melt_quote(
    wallet: &Wallet,
    mint_info: MintInfo,
    receive_invoice: &str,
) -> anyhow::Result<MeltQuote> {
    todo!(
        "Implement a function to create a Melt Quote from Wallet using a payment request that Mint accepts"
    );
}

async fn get_melt_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MeltQuote> {
    todo!("Implement a function to get a Melt Quote from Wallet with quote id");
}

async fn melt(wallet: &Wallet, quote_id: &str) -> anyhow::Result<FinalizedMelt> {
    todo!("Implement a function to melt the Melt Quote from Wallet with quote id");
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

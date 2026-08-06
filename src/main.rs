use std::{str::FromStr, sync::Arc, time::Duration};

use anyhow::Ok;
use cdk::wallet::{HttpClient, MeltQuote, MintConnector, MintQuote, SendOptions, Wallet};
use cdk_common::{
    Amount, CurrencyUnit, MintInfo, MintQuoteState, PaymentMethod, Proofs, Token, amount::SplitTarget, mint_url::MintUrl, nut00::KnownMethod::Bolt11
};
use cdk_sqlite::{WalletSqliteDatabase, wallet::memory};
use rand::{RngExt, random, rng};
use tokio::time::sleep;

async fn mint_info(mint_url: &MintUrl) -> anyhow::Result<MintInfo> {
    let client = HttpClient::new(mint_url.clone(), None);
    Ok(client.get_mint_info().await?)
}

async fn create_wallet(
    mint_url: &MintUrl,
    unit: CurrencyUnit,
    database: WalletSqliteDatabase,
    seed: [u8; 64],
) -> anyhow::Result<Wallet> {
    Ok(Wallet::new(
        &mint_url.to_string(),
        unit,
        Arc::new(database),
        seed,
        None,
    )?)
}

async fn mint_quote(mint_info: &MintInfo, wallet: &Wallet) -> anyhow::Result<MintQuote> {
    let payment_method = mint_info.nuts.nut04.methods.first().unwrap().method.clone();

    Ok(wallet
        .mint_quote(payment_method, Some(Amount::from(21)), None, None)
        .await?)
}

async fn pay_mint_quote(wallet: &Wallet, quote: &MintQuote) -> anyhow::Result<()> {
    loop {
        let status = wallet.check_mint_quote_status(&quote.id).await?;
        match status.state {
            MintQuoteState::Paid => {
                println!("Invoice paid!");
                break;
            }
            _ => {
                println!("Pay Invoice: {}", quote.request)
                std::io::Write::flush(&mut std::io::stdout())?;
            }
        }

        sleep(Duration::from_secs(5));
    }

    Ok(())
}

async fn mint_tokens(wallet: &Wallet, quote: &MintQuote) -> anyhow::Result<Proofs> {
    Ok(wallet.mint(&quote.id, SplitTarget::None, None).await?)
}

async fn create_token(wallet: &Wallet, amount: u64) -> anyhow::Result<Token> {
    let prepare_send = wallet.prepare_send(Amount::from(amount), SendOptions::default()).await?;
    Ok(prepare_send.confirm(None).await?)
}

async fn melt_quote(
    wallet: &Wallet,
    mint_info: MintInfo,
    receive_invoice: &str,
) -> anyhow::Result<MeltQuote> {
    let payment_method = mint_info.nuts.nut05.methods.first().unwrap().method.clone();
    Ok(wallet.melt_quote(payment_method, receive_invoice, None, None).await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mint_url: MintUrl = MintUrl::from_str("http://server:8085")?;

    // Get mint info
    let mint_info: MintInfo = mint_info(&mint_url).await?;

    // Setup the wallet
    let walletdb: WalletSqliteDatabase = memory::empty().await?;
    let seed: [u8; 64] = random();
    let wallet: Wallet = create_wallet(&mint_url, CurrencyUnit::Sat, walletdb, seed).await?;

    // Minting tokens
    let quote: MintQuote = mint_quote(&mint_info, &wallet).await?;
    pay_mint_quote(&wallet, &quote).await.ok();
    let proofs: Proofs = mint_tokens(&wallet, &quote).await?;
    println!("Minted proofs: {:?}", proofs);

    // Creating tokens
    let balance: Amount = wallet.total_balance().await?;
    let amount = rng().random_range(..(balance.to_u64() / 4));
    let token: Token = create_token(&wallet, amount).await?;
    println!("Minted tokens: {:?}", token);
    // send token to another user

    // Melting tokens
    let melt_quote: MeltQuote = melt_quote(&wallet, mint_info, "TODO: IMPLEMENT").await?;
    todo!("Tokens to melt and wait until payment");
}

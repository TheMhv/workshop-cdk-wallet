use std::{collections::HashMap, env, str::FromStr, sync::Arc, time::Duration};

use anyhow::Ok;
use cdk::wallet::{HttpClient, MeltQuote, MintConnector, MintQuote, SendOptions, Wallet};
use cdk_common::{
    Amount, CurrencyUnit, MintInfo, MintQuoteState, Proofs, Token, amount::SplitTarget,
    mint_url::MintUrl,
};
use cdk_sqlite::{WalletSqliteDatabase, wallet::memory};
use lnd_grpc_rust::lnrpc::{Invoice, Payment, invoice};
use rand::{RngExt, random, rng};
use tokio::time::sleep;

use crate::lnd::LND;

mod lnd;

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

async fn pay_mint_quote(
    wallet: &Wallet,
    quote: &MintQuote,
    payment: Payment,
) -> anyhow::Result<()> {
    loop {
        let status = wallet.check_mint_quote_status(&quote.id).await?;
        match status.state {
            MintQuoteState::Paid => {
                println!("Invoice paid!");
                break;
            }
            _ => {
                println!("Payment Status: {:?}", payment.status());
                std::io::Write::flush(&mut std::io::stdout())?;
            }
        }

        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

async fn mint_tokens(wallet: &Wallet, quote: &MintQuote) -> anyhow::Result<Proofs> {
    Ok(wallet.mint(&quote.id, SplitTarget::None, None).await?)
}

async fn create_token(wallet: &Wallet, amount: u64) -> anyhow::Result<Token> {
    let prepare_send = wallet
        .prepare_send(Amount::from(amount), SendOptions::default())
        .await?;
    Ok(prepare_send.confirm(None).await?)
}

async fn melt_quote(
    wallet: &Wallet,
    mint_info: MintInfo,
    receive_invoice: &str,
) -> anyhow::Result<MeltQuote> {
    let payment_method = mint_info.nuts.nut05.methods.first().unwrap().method.clone();
    Ok(wallet
        .melt_quote(payment_method, receive_invoice, None, None)
        .await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mut lnd_node = LND::new().await?;
    println!("LND node info: {:?}", lnd_node.get_info().await?);

    let mint_url: MintUrl =
        MintUrl::from_str(&env::var("MINT_URL").unwrap_or("http://localhost:8085".to_string()))?;

    // Get mint info
    let mint_info: MintInfo = mint_info(&mint_url).await?;

    // Setup the wallet
    let walletdb: WalletSqliteDatabase = memory::empty().await?;
    let seed: [u8; 64] = random();
    let wallet: Wallet = create_wallet(&mint_url, CurrencyUnit::Sat, walletdb, seed).await?;

    // Minting tokens
    let quote: MintQuote = mint_quote(&mint_info, &wallet).await?;
    let payment = lnd_node.pay_invoice(quote.clone().request).await?;
    pay_mint_quote(&wallet, &quote, payment).await.ok();
    let proofs: Proofs = mint_tokens(&wallet, &quote).await?;
    println!("Minted proofs: {:?}", proofs);

    // Creating tokens
    let balance: Amount = wallet.total_balance().await?;
    let amount = rng().random_range(..(balance.to_u64() / 4));
    let token: Token = create_token(&wallet, amount).await?;
    println!("Minted tokens: {:?}", token);
    // send token to another user

    // Melting tokens
    let receive_invoice = lnd_node
        .create_invoice(Invoice {
            memo: "Paid by Mint".to_string(),
            value: token.value()?.to_i64().unwrap(),
            ..Default::default()
        })
        .await?;
    let melt_quote: MeltQuote =
        melt_quote(&wallet, mint_info, &receive_invoice.payment_request).await?;
    println!("Melt Quote: {:?}", melt_quote);

    let prepare = wallet.prepare_melt(&melt_quote.id, HashMap::new()).await?;
    let melted = prepare.confirm().await?;
    println!("Melted: {:?}", melted);

    let melt_payment = lnd_node.get_transaction(receive_invoice.r_hash).await?;
    println!("Melt payment: {:?}", melt_payment);

    assert_eq!(melt_payment.state(), invoice::InvoiceState::Settled);

    Ok(())
}

use std::{env, str::FromStr, sync::Arc, time::Duration};

use anyhow::Ok;
use cdk::wallet::{HttpClient, MeltQuote, MintConnector, MintQuote, SendOptions, Wallet};
use cdk_common::{
    Amount, CurrencyUnit, MintInfo, MintQuoteState, Proofs, Token, amount::SplitTarget,
    mint_url::MintUrl,
};
use cdk_sqlite::WalletSqliteDatabase;
use lnd_grpc_rust::lnrpc::Payment;
use tokio::time::sleep;

use crate::lnd::LND;

mod cli;
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

async fn mint_quote(
    mint_info: &MintInfo,
    wallet: &Wallet,
    amount: Amount,
) -> anyhow::Result<MintQuote> {
    let payment_method = mint_info.nuts.nut04.methods.first().unwrap().method.clone();

    Ok(wallet
        .mint_quote(payment_method, Some(amount), None, None)
        .await?)
}

async fn pay_mint_quote(wallet: &Wallet, quote_id: &str, payment: Payment) -> anyhow::Result<()> {
    loop {
        let status = wallet.check_mint_quote_status(&quote_id).await?;
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

async fn mint_tokens(wallet: &Wallet, quote_id: &str) -> anyhow::Result<Proofs> {
    Ok(wallet.mint(&quote_id, SplitTarget::None, None).await?)
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

    let mint_url: MintUrl =
        MintUrl::from_str(&env::var("MINT_URL").expect("Unable to set MINT URL"))?;

    let macaroon_path = env::var("LND_MACAROON_PATH").expect("Unable to find LND macaroon file");

    let cert_path = env::var("LND_CERT_PATH").expect("Unable to find LND cert file");

    let lnd_url = env::var("LND_URL").expect("Unable to find LND URL server");

    let lnd_node = LND::new(macaroon_path, cert_path, lnd_url).await?;

    cli::cli(mint_url, lnd_node).await
}

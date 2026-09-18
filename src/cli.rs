use std::collections::HashMap;
use std::str::FromStr;

use cdk::wallet::Wallet;
use cdk_common::Token;
use cdk_common::bitcoin::hashes::{Hash, sha256};
use cdk_common::util::hex;
use cdk_common::{Amount, CurrencyUnit, mint_url::MintUrl};
use cdk_sqlite::WalletSqliteDatabase;
use clap::{Parser, Subcommand};
use lnd_grpc_rust::lnrpc::Invoice;
use rand::random;
use serde_json::json;
use tokio::fs;

use crate::lnd::LND;
use crate::{
    create_token, create_wallet, get_balance, get_melt_quote, get_mint_quote, melt_quote,
    mint_info, mint_quote, mint_tokens, pay_mint_quote, receive_token,
};

const DB_PATH: &str = "wallet.sqlite";
const SEED_PATH: &str = "wallet.seed";

async fn seed() -> anyhow::Result<[u8; 64]> {
    if let Ok(bytes) = fs::read(SEED_PATH).await {
        if bytes.len() == 64 {
            let mut seed = [0u8; 64];
            seed.copy_from_slice(&bytes);
            return Ok(seed);
        }
    }

    let seed: [u8; 64] = random();
    fs::write(SEED_PATH, seed).await?;
    Ok(seed)
}

async fn wallet(mint_url: &MintUrl) -> anyhow::Result<Wallet> {
    let db = WalletSqliteDatabase::new(DB_PATH).await?;
    create_wallet(&mint_url, CurrencyUnit::Sat, db, seed().await?).await
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show lnd info
    GetLndInfo,
    /// Show mint info
    GetMintInfo,
    /// Create (or open) the wallet
    CreateWallet,
    /// Request a mint quote for AMOUNT sats
    MintQuote { amount: u64 },
    /// Look up a mint quote by quote id
    GetMintQuote { quote_id: String },
    /// Pay a mint quote's invoice via LND
    PayInvoice { invoice: String },
    /// Mint tokens for a paid quote
    MintTokens { quote_id: String },
    /// Show wallet balance
    GetBalance,
    /// Create a sendable token for AMOUNT sats
    CreateToken { amount: u64 },
    /// Request a melt quote with invoice
    MeltQuote { invoice: String },
    /// Look up a melt quote by quote id
    GetMeltQuote { quote_id: String },
    /// Redeem a token into the wallet
    ReceiveToken { token: String },
    /// Pay a melt quote using the wallet's balance
    Melt { quote_id: String },
    /// Create a lightining invoice via LND for AMOUNT sats
    CreateInvoice { amount: i64, memo: Option<String> },
    /// Look up a lightning transaction by its payment hash (hex-encoded)
    GetInvoice { preimage: String },
}

pub async fn cli(mint_url: MintUrl, mut lnd: LND) -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::GetLndInfo => {
            let info = lnd.get_info().await?;
            println!("{}", serde_json::to_string_pretty(&info)?);
        }

        Commands::GetMintInfo => {
            let info = mint_info(&mint_url).await?;
            println!("{}", serde_json::to_string_pretty(&info)?);
        }

        Commands::CreateWallet => {
            let wallet = wallet(&mint_url).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "mint_url": wallet.mint_url,
                    "unit": wallet.unit,
                }))?
            );
        }

        Commands::MintQuote { amount } => {
            let info = mint_info(&mint_url).await?;
            let wallet = wallet(&mint_url).await?;
            let amount = Amount::from(amount);
            let quote = mint_quote(&info, &wallet, amount).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "quote_id": quote.id,
                    "invoice": quote.request,
                }))?
            );
        }

        Commands::GetMintQuote { quote_id } => {
            let wallet = wallet(&mint_url).await?;
            let quote = get_mint_quote(&wallet, &quote_id).await?;

            println!("{}", serde_json::to_string_pretty(&quote)?);
        }

        Commands::PayInvoice { invoice } => {
            let wallet = wallet(&mint_url).await?;
            let payment = lnd.pay_invoice(invoice).await?;

            println!("{}", serde_json::to_string_pretty(&payment)?);
        }

        Commands::MintTokens { quote_id } => {
            let wallet = wallet(&mint_url).await?;
            let proofs = mint_tokens(&wallet, &quote_id).await?;

            println!("{}", serde_json::to_string_pretty(&proofs)?);
        }

        Commands::GetBalance => {
            let wallet = wallet(&mint_url).await?;
            let balance = get_balance(&wallet).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "balance": balance,
                }))?
            );
        }

        Commands::CreateToken { amount } => {
            let wallet = wallet(&mint_url).await?;
            let amount = Amount::from(amount);
            let token = create_token(&wallet, amount).await?;

            println!("{}", serde_json::to_string_pretty(&token.to_string())?);
        }

        Commands::CreateInvoice { amount, memo } => {
            let invoice = lnd
                .create_invoice(Invoice {
                    memo: memo.unwrap_or_default(),
                    value: amount,
                    ..Default::default()
                })
                .await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "payment_request": &invoice.payment_request,
                    "r_hash": &hex::encode(invoice.r_hash),
                }))?
            );
        }

        Commands::GetInvoice { preimage } => {
            let r_hash = sha256::Hash::hash(&hex::decode(&preimage)?)
                .to_byte_array()
                .to_vec();
            let invoice = lnd.get_transaction(r_hash).await?;

            println!("{}", serde_json::to_string_pretty(&invoice)?);
        }

        Commands::MeltQuote { invoice } => {
            let info = mint_info(&mint_url).await?;
            let wallet = wallet(&mint_url).await?;
            let quote = melt_quote(&wallet, info, &invoice).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "quote_id": quote.id,
                    "amount": quote.amount,
                    "fee_reserve": quote.fee_reserve,
                    "total_needed": quote.amount + quote.fee_reserve,
                }))?
            );
        }

        Commands::GetMeltQuote { quote_id } => {
            let wallet = wallet(&mint_url).await?;
            let quote = get_melt_quote(&wallet, &quote_id).await?;

            println!("{}", serde_json::to_string_pretty(&quote)?)
        }

        Commands::ReceiveToken { token } => {
            let wallet = wallet(&mint_url).await?;
            let token = Token::from_str(token)?;
            let amount = receive_token(&wallet, &token).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({ "received": amount }))?
            );
        }

        Commands::Melt { quote_id } => {
            let wallet = wallet(&mint_url).await?;
            let melted = wallet
                .prepare_melt(&quote_id, HashMap::new())
                .await?
                .confirm()
                .await?;

            println!("{}", serde_json::to_string_pretty(&melted)?);
        }
    }

    anyhow::Ok(())
}

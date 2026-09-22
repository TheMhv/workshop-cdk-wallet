use std::str::FromStr;

use cdk::wallet::Wallet;
use cdk_common::Token;
use cdk_common::{Amount, CurrencyUnit, mint_url::MintUrl};
use cdk_sqlite::WalletSqliteDatabase;
use clap::{Parser, Subcommand};
use rand::random;
use serde_json::json;
use tokio::fs;

use crate::{
    create_token, create_wallet, get_balance, get_melt_quote, get_mint_quote, melt, melt_quote,
    mint_info, mint_quote, mint_tokens, receive_token,
};

const DB_PATH: &str = "wallet.sqlite";
const SEED_PATH: &str = "wallet.seed";

async fn seed(name: Option<String>) -> anyhow::Result<[u8; 64]> {
    let path: &str = match name {
        Some(n) => &format!("{}.seed", &n),
        None => SEED_PATH,
    };

    if let Ok(bytes) = fs::read(path).await {
        if bytes.len() == 64 {
            let mut seed = [0u8; 64];
            seed.copy_from_slice(&bytes);
            return Ok(seed);
        }
    }

    let seed: [u8; 64] = random();
    fs::write(path, seed).await?;
    Ok(seed)
}

async fn wallet(mint_url: &MintUrl, name: Option<String>) -> anyhow::Result<Wallet> {
    let path: &str = match name {
        Some(ref n) => &format!("{}.sqlite", &n),
        None => DB_PATH,
    };

    let db = WalletSqliteDatabase::new(path).await?;
    create_wallet(&mint_url, CurrencyUnit::Sat, db, seed(name).await?).await
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(long)]
    wallet: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show mint info
    GetMintInfo,
    /// Create (or open) the wallet
    CreateWallet { name: Option<String> },
    /// Request a mint quote for AMOUNT sats
    MintQuote { amount: u64 },
    /// Look up a mint quote by quote id
    GetMintQuote { quote_id: String },
    /// Mint tokens for a paid quote
    Mint { quote_id: String },
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
}

pub async fn cli(mint_url: MintUrl) -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::GetMintInfo => {
            let info = mint_info(&mint_url).await?;
            println!("{}", serde_json::to_string_pretty(&info)?);
        }

        Commands::CreateWallet { name } => {
            let wallet = wallet(&mint_url, name.or(args.wallet)).await?;

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
            let wallet = wallet(&mint_url, args.wallet).await?;
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
            let wallet = wallet(&mint_url, args.wallet).await?;
            let quote = get_mint_quote(&wallet, &quote_id).await?;

            println!("{}", serde_json::to_string_pretty(&quote)?);
        }

        Commands::Mint { quote_id } => {
            let wallet = wallet(&mint_url, args.wallet).await?;
            let proofs = mint_tokens(&wallet, &quote_id).await?;

            println!("{}", serde_json::to_string_pretty(&proofs)?);
        }

        Commands::GetBalance => {
            let wallet = wallet(&mint_url, args.wallet).await?;
            let balance = get_balance(&wallet).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "balance": balance,
                }))?
            );
        }

        Commands::CreateToken { amount } => {
            let wallet = wallet(&mint_url, args.wallet).await?;
            let amount = Amount::from(amount);
            let token = create_token(&wallet, amount).await?;

            println!("{}", serde_json::to_string_pretty(&token.to_string())?);
        }

        Commands::MeltQuote { invoice } => {
            let info = mint_info(&mint_url).await?;
            let wallet = wallet(&mint_url, args.wallet).await?;
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
            let wallet = wallet(&mint_url, args.wallet).await?;
            let quote = get_melt_quote(&wallet, &quote_id).await?;

            println!("{}", serde_json::to_string_pretty(&quote)?)
        }

        Commands::ReceiveToken { token } => {
            let wallet = wallet(&mint_url, args.wallet).await?;
            let token = Token::from_str(&token)?;
            let amount = receive_token(&wallet, &token).await?;

            println!(
                "{}",
                serde_json::to_string_pretty(&json!({ "received": amount }))?
            );
        }

        Commands::Melt { quote_id } => {
            let wallet = wallet(&mint_url, args.wallet).await?;
            let melted = melt(&wallet, &quote_id).await?;

            println!("{}", serde_json::to_string_pretty(&melted)?);
        }
    }

    anyhow::Ok(())
}

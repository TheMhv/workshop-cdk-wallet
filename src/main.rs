use cdk::wallet::{MeltQuote, MintQuote, SendOptions, Wallet};
use cdk_common::{Amount, CurrencyUnit, MintInfo, Proofs, Token, amount::SplitTarget};
use cdk_sqlite::{WalletSqliteDatabase, wallet::memory};
use rand::{RngExt, random, rng};

async fn mint_info(mint_url: &str) -> anyhow::Result<MintInfo> {
    todo!("Get mint info");
}

async fn create_wallet(
    mint_url: &str,
    unit: CurrencyUnit,
    database: WalletSqliteDatabase,
    seed: [u8; 64],
) -> anyhow::Result<Wallet> {
    todo!("create the wallet");
}

async fn mint_quote(mint_info: &MintInfo, wallet: &Wallet) -> anyhow::Result<MintQuote> {
    todo!("Create and get mint quote");
}

async fn pay_mint_quote(wallet: &Wallet, quote: &MintQuote) -> anyhow::Result<()> {
    todo!("Pay mint quote request");
}

async fn mint_tokens(wallet: &Wallet, quote: &MintQuote) -> anyhow::Result<Proofs> {
    todo!("Mint tokens")
}

async fn create_token(wallet: &Wallet, amount: u64) -> anyhow::Result<Token> {
    todo!("Create token to send to another user")
}

async fn melt_quote(
    wallet: &Wallet,
    mint_info: MintInfo,
    receive_invoice: &str,
) -> anyhow::Result<MeltQuote> {
    todo!("Create and get melt quote");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mint_url = "http://server:8085";

    // Get mint info
    let mint_info = mint_info(mint_url).await?;

    // Setup the wallet
    let walletdb = memory::empty().await?;
    let seed: [u8; 64] = random();
    let wallet = create_wallet(mint_url, CurrencyUnit::Sat, walletdb, seed).await?;

    // Minting tokens
    let quote = mint_quote(&mint_info, &wallet).await?;
    pay_mint_quote(&wallet, &quote).await.ok();
    let proofs = mint_tokens(&wallet, &quote);

    // Creating tokens
    let balance = wallet.total_balance().await?;
    let amount = rng().random_range(..(balance.to_u64() / 4));
    let token = create_token(&wallet, amount).await?;
    // send token to another user

    // Melting tokens
    let melt_quote = melt_quote(&wallet, mint_info, "TODO: IMPLEMENT");
    todo!("Tokens to melt and wait until payment");
}

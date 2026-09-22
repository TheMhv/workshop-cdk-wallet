use std::{env, str::FromStr};

use cdk::wallet::{MeltQuote, MintQuote, Wallet};
use cdk_common::{Amount, CurrencyUnit, FinalizedMelt, MintInfo, Proofs, Token, mint_url::MintUrl};
use cdk_sqlite::WalletSqliteDatabase;

mod cli;

async fn mint_info(mint_url: &MintUrl) -> anyhow::Result<MintInfo> {
    // 1. Build an HTTP client pointing at the mint URL.
    // 2. Request the mint's info endpoint (GET /v1/info).
    // 3. Return the parsed MintInfo (name, supported NUTs, limits, etc.).
    todo!("Implement a function to get the mint info");
}

async fn create_wallet(
    mint_url: &MintUrl,
    unit: CurrencyUnit,
    database: WalletSqliteDatabase,
    seed: [u8; 64],
) -> anyhow::Result<Wallet> {
    // 1. Wrap the database in an Arc so the wallet can share it.
    // 2. Build the Wallet from the mint URL, unit, database and seed.
    // 3. Handle any construction error and return the Wallet.
    todo!(
        "Implement a function to create a new Wallet specifying the mint, the unit, the database that will be used and a seed"
    );
}

async fn get_balance(wallet: &Wallet) -> anyhow::Result<Amount> {
    // 1. Ask the wallet for the total of its unspent proofs.
    // 2. Return the resulting Amount.
    todo!("Implement a function to get the current balance of the Wallet");
}

async fn mint_quote(
    mint_info: &MintInfo,
    wallet: &Wallet,
    amount: Amount,
) -> anyhow::Result<MintQuote> {
    // 1. Check in mint_info that minting is enabled for the wallet's unit and payment method.
    // 2. Check the amount is within the mint's min/max limits.
    // 3. Request a mint quote from the wallet for the given amount.
    // 4. Return the quote.
    todo!("Implement a function to create a Mint Quote from Wallet specifying the amount");
}

async fn get_mint_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MintQuote> {
    // 1. Look up the quote by its id.
    // 2. Return an error if the quote does not exist.
    // 3. Return the up-to-date MintQuote.
    todo!("Implement a function to get a Mint Quote from Wallet from quote id");
}

async fn mint_tokens(wallet: &Wallet, quote_id: &str) -> anyhow::Result<Proofs> {
    // 1. Fetch the quote and verify its invoice has been paid.
    // 2. Ask the wallet to mint the proofs for that quote.
    // 3. Return the newly minted Proofs.
    todo!("Implement a function to mint tokens from Mint Quote by quote id");
}

async fn create_token(wallet: &Wallet, amount: Amount) -> anyhow::Result<Token> {
    // 1. Check the wallet balance is enough for the requested amount.
    // 2. Prepare a send: select proofs.
    // 3. Confirm the send to obtain the Token.
    // 4. Return the Token.
    todo!("Implement a function to create a token from Wallet");
}

async fn receive_token(wallet: &Wallet, token: String) -> anyhow::Result<Amount> {
    // 1. Decode the token string into a mint URL + encoded proofs.
    // 2. Refuse tokens for a different mint or unit.
    // 3. Resolve the encoded proofs into full Proof objects.
    // 4. Ask the mint whether these proofs are still spendable.
    // 5. Perform a swap operation
    todo!("Implement a function to redeem a token for Wallet");
}

async fn melt_quote(
    wallet: &Wallet,
    mint_info: MintInfo,
    receive_invoice: &str,
) -> anyhow::Result<MeltQuote> {
    // 1. Parse and validate the payment request.
    // 2. Check in mint_info that melting is enabled for the wallet's unit and this payment method.
    // 3. Request a melt quote from the wallet for the invoice.
    // 4. Return the quote.
    todo!(
        "Implement a function to create a Melt Quote from Wallet using a payment request that Mint accepts"
    );
}

async fn get_melt_quote(wallet: &Wallet, quote_id: &str) -> anyhow::Result<MeltQuote> {
    // 1. Look up the quote by its id.
    // 2. Return an error if the quote does not exist.
    // 3. Return the up-to-date MeltQuote.
    todo!("Implement a function to get a Melt Quote from Wallet with quote id");
}

async fn melt(wallet: &Wallet, quote_id: &str) -> anyhow::Result<FinalizedMelt> {
    // 1. Fetch the quote and verify it is still unpaid.
    // 2. Check the wallet balance covers the amount plus the fee reserve.
    // 3. Ask the wallet to melt: it selects proofs and sends them to the mint.
    // 4. The mint pays the invoice and returns any change for unused fee reserve.
    // 5. Return the FinalizedMelt.
    todo!("Implement a function to melt the Melt Quote from Wallet with quote id");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mint_url: MintUrl =
        MintUrl::from_str(&env::var("MINT_URL").expect("Unable to set MINT URL"))?;

    cli::cli(mint_url).await
}

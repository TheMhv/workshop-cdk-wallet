# Workshop - CDK Wallet

The purpose of this workshop is to prepare participants to implement a simple Cashu wallet using the [Cashu Dev Kit (CDK)](https://docs.rs/cdk/0.16.0/cdk/).

By the end you will have a small wallet app that can **mint** ecash tokens from a Lightning payment, **send and receive** tokens between participants, and **melt** tokens to pay a real Lightning invoice.

## How it works

The *HOST* runs:

- a Cashu **Mint** that accepts mint and melt requests;
- an **LND node** that can create and pay Lightning invoices.

Each participant builds a wallet on top of CDK's official `Wallet` struct and talks to the host's Mint.

## Prerequisites

- If you running nix, just `nix develop`
- Rust toolchain (stable) and `cargo`
- The Mint URL provided by the host

## Setup

1. Copy the `.env.example` file to `.env` in the project root:

   ```sh
   cp .env.example .env
   ```

2. Build the project:

   ```sh
   cargo build
   ```

3. Run it.

   ```sh
   cargo run -- --help
   ```

## Your task

Open `src/main.rs`. Every function contains numbered comments describing the steps. Replace each `todo!()` with a working implementation. The checklist below follows the order in which you should work, since later steps depend on earlier ones.

### Overview

| Function | What it does |
|---------|--------------|
| `mint_info` | Fetch the Mint's info |
| `create_wallet` | Build the CDK `Wallet` |
| `get_balance` | Read the wallet balance |
| `mint_quote` / `get_mint_quote` | Ask the Mint for an invoice to fund the wallet |
| `mint_tokens` | Turn a paid quote into proofs |
| `create_token` | Export an amount as an encoded token |
| `receive_token` | Redeem a token from someone else |
| `melt_quote` / `get_melt_quote` | Ask the Mint to pay an external invoice |
| `melt` | Spend your tokens to pay that invoice |

### Steps

- [ ] **1. Get the Mint information** — `mint_info`

  The Mint Info describes the Mint: its name, supported NUTs, payment methods, fees and limits. Your wallet will use it to validate requests before sending them.

  Build an HTTP client for the mint URL, call the info endpoint (`GET /v1/info`) and return the parsed `MintInfo`.

  - [NUT-06: Mint information](https://github.com/cashubtc/nuts/blob/main/06.md)
  - [cdk::wallet::HttpClient](https://docs.rs/cdk/0.16.0/cdk/wallet/type.HttpClient.html)
  - [cashu::nuts::nut06::MintInfo](https://docs.rs/cashu/0.16.0/cashu/nuts/nut06/struct.MintInfo.html)

  <!-- cdk::wallet::HttpClient.get_mint_info -->

- [ ] **2. Create the Wallet** — `create_wallet`

  CDK provides an official `Wallet` struct, and we build on it. A wallet is defined by the mint URL, the currency unit, a database and a 64-byte seed. The database (`WalletSqliteDatabase`) must be wrapped in an `Arc` so the wallet can share it. Handle any construction error and return the `Wallet`.

  - [cdk::wallet](https://docs.rs/cdk/0.16.0/cdk/wallet/index.html)

- [ ] **3. Check the balance** — `get_balance`

  Ask the wallet for the total of its unspent proofs and return it as an `Amount`. You will use this to verify each of the following steps.

- [ ] **4. Create a Mint Quote** — `mint_quote`, `get_mint_quote`

  Minting starts with a *Mint Quote*: a request telling the Mint that your wallet wants new tokens. Before requesting one, use `MintInfo` to check that:

  1. minting is enabled for the wallet's unit and payment method (in this workshop, Lightning);
  2. the amount is within the Mint's min/max limits.

  Then request the quote from the wallet and return it. The quote holds two things you need to keep: the `quote_id` and the `payment_request` (a Lightning invoice).

  `get_mint_quote` looks a quote up by its id, returns an error if it does not exist, and returns the up-to-date state (for example, whether the invoice has been paid).

  - [NUT-04: Mint tokens](https://github.com/cashubtc/nuts/blob/main/04.md)
  - [cdk::wallet::Wallet.mint_quote](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.mint_quote)

  <!-- cashu::nuts::nut06::MintInfo.nuts.nut04.methods[0].method -->

- [ ] **5. Pay the Mint Quote invoice**

  Since we use Lightning payments, the quote's `payment_request` is a Lightning invoice. You can pay it with the *HOST* lightning invoice interface.

- [ ] **6. Mint the tokens** — `mint_tokens`

  Once the invoice is paid, fetch the quote, verify that it has been paid, and ask the wallet to mint. Behind the scenes your wallet sends blinded messages to the Mint, which signs them; each (`secret`, `signature`) pair is a token (a *proof*). Return the new `Proofs`, then check that `get_balance` went up.

  - [NUT-04: Mint tokens](https://github.com/cashubtc/nuts/blob/main/04.md)
  - [cdk::wallet::Wallet.mint](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.mint)

- [ ] **7. Send tokens to another participant** — `create_token`

  Encode part of your balance as a token string that you can hand to someone else:

  1. check that the balance covers the requested amount;
  2. prepare a send, which selects the proofs;
  3. confirm the send to obtain the `Token`;
  4. return the `Token`.

  The proofs leave your wallet, so your balance decreases. Share the encoded token with another participant.

  - [V4 tokens](https://github.com/cashubtc/nuts/blob/main/00.md#v4-tokens)
  - [cdk::wallet::Wallet.prepare_send](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.prepare_send)

- [ ] **8. Receive tokens from another participant** — `receive_token`

  Redeeming a token means swapping someone else's proofs for fresh ones that only you control. Your function should:

  1. decode the token string into a mint URL and encoded proofs;
  2. refuse tokens issued by a different mint or in a different unit;
  3. resolve the encoded proofs into full `Proof` objects;
  4. ask the Mint whether the proofs are still spendable;
  5. perform a swap, and return the received `Amount`.

  - [cdk::wallet::Wallet.receive](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.receive)

- [ ] **9. Create an invoice to receive real money**

  Let's go back to the REAL MONEY. To exercise melting, you need a Lightning invoice created by an external node. You can use the *HOST* lightning invoice interface to create your payment.

- [ ] **10. Create a Melt Quote** — `melt_quote`, `get_melt_quote`

  A *Melt Quote* asks the Mint how much it will cost to pay an external invoice. Your function should:

  1. parse and validate the payment request;
  2. check in `MintInfo` that melting is enabled for the wallet's unit and this payment method;
  3. request the quote from the wallet and return it.

  The quote includes the invoice amount plus a *fee reserve*. `get_melt_quote` works like `get_mint_quote`: look up by id, error if missing, return the up-to-date state.

  - [NUT-05: Melting tokens](https://github.com/cashubtc/nuts/blob/main/05.md)
  - [cdk::wallet::Wallet.melt_quote](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.melt_quote)

  <!-- cashu::nuts::nut06::MintInfo.nuts.nut05.methods[0].method -->

- [ ] **11. Melt tokens to pay the invoice** — `melt`

  This consumes your tokens and pays the invoice:

  1. fetch the quote and verify that it is still unpaid;
  2. check that your balance covers the amount plus the fee reserve;
  3. ask the wallet to melt: it selects proofs and sends them to the Mint;
  4. the Mint pays the invoice and returns any change for the unused fee reserve;
  5. return the `FinalizedMelt`.

  - [NUT-05: Melting tokens](https://github.com/cashubtc/nuts/blob/main/05.md)
  - [cdk::wallet::Wallet.prepare_melt](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.prepare_melt)

## Suggested end-to-end test

1. Mint a small amount and confirm the balance.
2. Send part of it to a neighbor and confirm your balance dropped.
3. Receive a token from a neighbor and confirm your balance rose.
4. Create an invoice, get a melt quote, melt, and confirm the invoice was paid and change came back.

## Further reading

- [Cashu protocol specs (NUTs)](https://github.com/cashubtc/nuts)
- [CDK documentation](https://docs.rs/cdk/0.16.0/cdk/)

# Workshop - CDK Wallet

The propurse of this workshop is prepare the participants to implement a simple cashu wallet using Cashu Dev Kit.

## How will work

The *HOST* will run a cashu Mint that can receive Mint and Melt requests, and a LND node that can create and pay invoices.

The goal for participants is create a wallet-app using CDK that can mint tokens, transfer between then and melt those tokens.

## Resources
- [ ] Create the Wallet struct

  CDK has a official struct for a Cashu Wallet. We will use this struct to create our custom wallet.

  [cdk::wallet](https://docs.rs/cdk/0.16.0/cdk/wallet/index.html)

- [ ] Get Mint information

  First of all, we need to get Mint Info, the params from the Mint. This informations will be useful for you Wallet knows things like payment methods, fee amounts and other Mint things.
  
  [NUT-06: Mint information](https://github.com/cashubtc/nuts/blob/main/06.md)
  
  You can use the Wallet HTTP Client to create requests and get responses directly from the Mint:
  <!-- cdk::wallet::HttpClient.get_mint_info -->

  [cdk::wallet::HttpClient](https://docs.rs/cdk/0.16.0/cdk/wallet/type.HttpClient.html)

  [cashu::nuts::nut06::MintInfo](https://docs.rs/cashu/0.16.0/cashu/nuts/nut06/struct.MintInfo.html)
  
- [ ] Creates a Mint Quote with payment method specified by Mint
  
  The first part of minting tokens is creating a Mint Quote. The Mint Quote is a request that Wallet do to the Mint telling that your Wallet wants to Mint new tokens.

  In our case, the Wallet needs to store two things: the `quote_id` and the `payment_request`.
  
  [NUT-04: Mint tokens](https://github.com/cashubtc/nuts/blob/main/04.md)

  You should use the `MintInfo` that you getted before to filter the payments methods that the Mint accepts.
  <!-- cashu::nuts::nut06::MintInfo.nuts.nut04.methods[0].method -->

  [cdk::wallet::Wallet.mint_quote](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.mint_quote)

- [ ] Pay Mint Quote invoice
  
  TODO: CHANGE TO WEB INTERFACE
  You can use the CLI `pay-invoice` command to do this

  Since we use lightning payments, our mint quote `request_payment` is an lightning invoice.
  
- [ ] Mint the tokens with mint quote
  
  After we pay the `payment_request`, we can use the paid mint quote to mint new tokens. Your Wallet needs to send a request to the Mint asking then for sign the _blinded message_, the pair (`secret`, `signature`) is your token.

  [NUT-04: Mint tokens](https://github.com/cashubtc/nuts/blob/main/04.md)

  This will mint a fresh ones tokens for your wallet

  [cdk::wallet::Wallet.mint](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.mint)

TODO: MAYBE WE CAN PERFORM A SWAP INSTEAD JUST SPENT FROM WALLET?
- [ ] Exchange tokens between participants

  Now you can encode the token to a format that will simplify the exchangable things. Note that you will need to remove the old token from your wallet.

  [V4 tokens](https://github.com/cashubtc/nuts/blob/main/00.md#v4-tokens)

  Now, you can get a encoded Cashu token and exchange with some other participant. 
  
  TODO: SHOW THE INFORMATIONS BEFORE CONFIRM
  [cdk::wallet::Wallet.prepare_send](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.prepare_send)

  Just share your encoded token with someone

- [ ] Receive tokens from other participant

  You can receive new tokens from the others participants. The CDK Wallet library can help with that.

  [cdk::wallet::Wallet.receive](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.receive)

- [ ] Create a new invoice

  Okay, but let's go back to the REAL MONEY. You needs to create a invoice from your lightning node to creates a new Melt Quote request.

TODO: CHANGE TO A WEB INTERFACE
  You can use the CLI `create-invoice` command to do this

- [ ] Create a Melt Quote with the invoice

  [NUT-05: Melting tokens](https://github.com/cashubtc/nuts/blob/main/05.md)

  You should use the `MintInfo` getted before to create a Melt Quote with payment method accepted by Mint.
  <!-- cashu::nuts::nut06::MintInfo.nuts.nut05.methods[0].method -->

  [cdk::wallet::Wallet.melt_quote](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.melt_quote)

- [ ] Melt tokens to pay Melt Quote

  [NUT-05: Melting tokens](https://github.com/cashubtc/nuts/blob/main/05.md)

  This will consume your tokens and pay your invoice

  [cdk::wallet::Wallet.prepare_melt](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.prepare_melt)

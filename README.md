# Workshop - CDK Wallet

The propurse of this workshop is prepare the participants to implement a simple cashu wallet using Cashu Dev Kit.

## How will work

The *HOST* will run a cashu Mint that can receive Mint and Melt requests, and a LND node that can create and pay invoices.

The goal for participants is create a wallet-app using CDK that can mint tokens, transfer between then and melt those tokens.

## TODO
  - [ ] Get Mint information
  
    [NUT-06: Mint information](https://github.com/cashubtc/nuts/blob/main/06.md)
  
    [cashu::nuts::nut06::MintInfo](https://docs.rs/cashu/0.16.0/cashu/nuts/nut06/struct.MintInfo.html)
  
  - [ ] Creates a Mint Quote with payment method specified by Mint
  
    [NUT-04: Mint tokens](https://github.com/cashubtc/nuts/blob/main/04.md)
  
    [cdk::wallet::Wallet.mint_quote](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.mint_quote)
    
  - [ ] Pay Mint Quote invoice
  
    You can use the CLI `pay-invoice` command to do this
  
  - [ ] Mint the tokens with mint quote
  
    [NUT-04: Mint tokens](https://github.com/cashubtc/nuts/blob/main/04.md)
  
    [cdk::wallet::Wallet.mint](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.mint)
  
  - [ ] Exchange tokens between participants
  
    [V4 tokens](https://github.com/cashubtc/nuts/blob/main/00.md#v4-tokens)
  
    [cdk::wallet::Wallet.prepare_send](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.prepare_send)
  
    Just share your encoded token with someone
  
  - [ ] Create the invoice
  
    You can use the CLI `create-invoice` command to do this

  - [ ] Create a Melt Quote with the invoice
  
    [NUT-05: Melting tokens](https://github.com/cashubtc/nuts/blob/main/05.md)
  
    [cdk::wallet::Wallet.melt_quote](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.melt_quote)
  
  - [ ] Melt tokens to pay Melt Quote
  
    [NUT-05: Melting tokens](https://github.com/cashubtc/nuts/blob/main/05.md)
  
    [cdk::wallet::Wallet.prepare_melt](https://docs.rs/cdk/0.16.0/cdk/wallet/struct.Wallet.html#method.prepare_melt)

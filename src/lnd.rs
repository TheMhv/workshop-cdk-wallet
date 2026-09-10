use std::env;

use anyhow::{Ok, anyhow};
use cdk_common::util::hex;
use lnd_grpc_rust::{
    lnrpc::{Invoice, PaymentHash},
    routerrpc::SendPaymentRequest,
};
use tokio::fs;

pub struct LND {
    inner: lnd_grpc_rust::LndClient,
}

impl LND {
    pub async fn new() -> anyhow::Result<Self> {
        let macaroon_path =
            env::var("LND_MACAROON_PATH").expect("Unable to find LND macaroon file");
        let cert_path = env::var("LND_CERT_PATH").expect("Unable to find LND cert file");

        let macaroon = fs::read(macaroon_path.clone())
            .await
            .expect("LND macaroon file doesn't exists");
        let cert = fs::read(cert_path.clone())
            .await
            .expect("LND cert file doesn't exists");

        let lnd_url = env::var("LND_URL").unwrap_or("https://localhost:10009".to_string());

        Ok(Self {
            inner: lnd_grpc_rust::connect(hex::encode(cert), hex::encode(macaroon), lnd_url)
                .await
                .expect("Unable to connect with LND node"),
        })
    }

    pub async fn get_info(&mut self) -> anyhow::Result<lnd_grpc_rust::lnrpc::GetInfoResponse> {
        let info = self
            .inner
            .lightning()
            .get_info(lnd_grpc_rust::lnrpc::GetInfoRequest {})
            .await?;

        Ok(info.into_inner())
    }

    pub async fn pay_invoice(
        &mut self,
        request: String,
    ) -> anyhow::Result<lnd_grpc_rust::lnrpc::Payment> {
        let mut streaming = self
            .inner
            .router()
            .send_payment_v2(SendPaymentRequest {
                payment_request: request,
                timeout_seconds: 60,
                fee_limit_sat: 1000,
                ..Default::default()
            })
            .await?
            .into_inner();

        if let Some(payment) = streaming.message().await? {
            Ok(payment)
        } else {
            Err(anyhow!("Unable to send payment"))
        }
    }

    pub async fn create_invoice(
        &mut self,
        invoice: Invoice,
    ) -> anyhow::Result<lnd_grpc_rust::lnrpc::AddInvoiceResponse> {
        let response = self
            .inner
            .lightning()
            .add_invoice(invoice)
            .await?
            .into_inner();

        Ok(response)
    }

    pub async fn get_transaction(&mut self, payment_hash: Vec<u8>) -> anyhow::Result<Invoice> {
        let response = self
            .inner
            .lightning()
            .lookup_invoice(PaymentHash {
                r_hash: payment_hash,
                ..Default::default()
            })
            .await?
            .into_inner();

        Ok(response)
    }
}

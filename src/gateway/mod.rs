mod error;
mod solana;

pub use error::*;
pub use solana::*;

#[cfg(feature = "web")]
use solana_client_wasm::WasmClient;

use solana_sdk::{
    hash::Hash,
    pubkey::Pubkey,
    signature::Signature,
    transaction::VersionedTransaction,
};

#[cfg(feature = "web")]
pub const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

pub struct Gateway<R: Rpc> {
    pub rpc: R,
}

impl<R: Rpc> Gateway<R> {
    pub fn new(rpc_url: String) -> Gateway<R> {
        Gateway {
            rpc: R::new(rpc_url),
        }
    }
}

pub trait Rpc {
    fn new(rpc_url: String) -> Self;
    async fn get_balance(&self, pubkey: &Pubkey) -> GatewayResult<u64>;
    async fn get_latest_blockhash(&self) -> GatewayResult<Hash>;
    async fn get_signature_statuses(
        &self,
        signatures: &[Signature],
    ) -> GatewayResult<Vec<Option<TransactionConfirmationStatus>>>;
    async fn send_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<Signature>;
}

#[cfg(feature = "web")]
pub struct WebRpc(WasmClient);

#[cfg(feature = "web")]
impl Rpc for WebRpc {
    fn new(rpc_url: String) -> Self {
        WebRpc(WasmClient::new(rpc_url.as_str()))
    }
    async fn get_balance(&self, pubkey: &Pubkey) -> GatewayResult<u64> {
        self.0.get_balance(pubkey).await.map_err(From::from)
    }
    async fn get_latest_blockhash(&self) -> GatewayResult<Hash> {
        self.0.get_latest_blockhash().await.map_err(From::from)
    }
    async fn get_signature_statuses(
        &self,
        signatures: &[Signature],
    ) -> GatewayResult<Vec<Option<TransactionConfirmationStatus>>> {
        let vec = self.0.get_signature_statuses(signatures).await?;
        let vec = vec.into_iter().map(|opt| {
            if let Some(status) = opt {
                if let Some(status) = status.confirmation_status {
                    match status {
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Processed =>  Some(TransactionConfirmationStatus::Processed),
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Confirmed => Some(TransactionConfirmationStatus::Confirmed),
                        solana_extra_wasm::transaction_status::TransactionConfirmationStatus::Finalized => Some(TransactionConfirmationStatus::Finalized),
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();
        Ok(vec)
    }
    async fn send_transaction(
        &self,
        transaction: &VersionedTransaction,
    ) -> GatewayResult<Signature> {
        self.0
            .send_versioned_transaction(transaction)
            .await
            .map_err(From::from)
    }
}

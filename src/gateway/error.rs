use solana_sdk::pubkey::ParsePubkeyError;
use solana_client_wasm::WasmClientError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GatewayError {
    Unknown,
    WalletDisconnected,
    InvalidAddress,
    InsufficientBalance,
    TransactionTimeout,
    FailedDeserialization,
}

impl From<WasmClientError> for GatewayError {
    fn from(_: WasmClientError) -> Self {
        GatewayError::Unknown
    }
}

impl From<ParsePubkeyError> for GatewayError {
    fn from(_: ParsePubkeyError) -> Self {
        GatewayError::InvalidAddress
    }
}

pub type GatewayResult<T> = Result<T, GatewayError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionConfirmationStatus {
    Processed,
    Confirmed,
    Finalized,
}

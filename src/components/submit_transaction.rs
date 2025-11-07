use base64::Engine;
use dioxus::{document::eval, prelude::*};
use solana_sdk::{
    hash::Hash,
    message::VersionedMessage,
    transaction::VersionedTransaction,
    signature::Signature,
};

use crate::{
    gateway::{GatewayError, SolanaGateway},
    hooks::use_gateway,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionStatus {
    Waiting,
    Sending(usize),
    Done(Signature),
    Denied,
    Timeout,
    Error(GatewayError),
}

/// Signs and submits a transaction
pub fn submit_transaction(mut tx: VersionedTransaction) {
    let gateway = use_gateway();
    let gateway_clone = gateway.read().clone();

    spawn(async move {
        // Set blockhash
        if let Ok(hash) = gateway_clone.rpc.get_latest_blockhash().await {
            match &mut tx.message {
                VersionedMessage::V0(message) => {
                    message.recent_blockhash = hash;
                }
                VersionedMessage::Legacy(message) => {
                    message.recent_blockhash = hash;
                }
            }
        }

        // Build eval command for wallet signing
        let mut eval = eval(
            r#"
            let msg = await dioxus.recv();
            let signed = await window.SolanaTxSigner({b64: msg});
            dioxus.send(signed);
            "#,
        );

        // Serialize the transaction to send to wallet
        match bincode::serialize(&tx) {
            Ok(vec) => {
                let b64 = base64::engine::general_purpose::STANDARD.encode(vec);
                let res = eval.send(serde_json::Value::String(b64));
                match res {
                    Ok(()) => {
                        // Execute eval command
                        let res = eval.recv().await;

                        // Process eval result
                        match res {
                            // Process valid signing result
                            Ok(serde_json::Value::String(string)) => {
                                // Decode signed transaction
                                let decode_res = base64::engine::general_purpose::STANDARD
                                    .decode(string)
                                    .ok();
                                let decode_res = decode_res.and_then(|buffer| {
                                    bincode::deserialize::<VersionedTransaction>(&buffer).ok()
                                });

                                // Send transaction to rpc
                                let rpc_res = match decode_res {
                                    Some(tx) => gateway_clone.rpc.send_transaction(&tx).await.ok(),
                                    None => {
                                        log::info!("error decoding tx");
                                        None
                                    }
                                };

                                // Confirm transaction
                                match rpc_res {
                                    Some(sig) => {
                                        let confirmed = gateway_clone.rpc.confirm_signature(sig).await;
                                        if confirmed.is_ok() {
                                            log::info!("Transaction confirmed: {}", sig);
                                        } else {
                                            log::error!("Transaction timeout");
                                        }
                                    }
                                    None => {
                                        log::info!("error sending tx");
                                    }
                                }
                            }

                            // Process signing errors
                            Ok(serde_json::Value::Null) => {
                                log::info!("Transaction signing denied by user");
                            }
                            Err(err) => {
                                log::error!("error signing transaction: {}", err);
                            }
                            _ => {
                                log::error!("unrecognized signing response");
                            }
                        };
                    }

                    // Process eval errors
                    Err(err) => {
                        log::error!("error executing wallet signing script: {}", err);
                    }
                }
            }

            // Process serialization errors
            Err(err) => {
                log::error!("err serializing tx: {}", err);
            }
        };
    });
}

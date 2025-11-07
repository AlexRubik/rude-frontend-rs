use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Wallet {
    Connected(Pubkey),
    Disconnected,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet::Disconnected
    }
}

pub fn use_wallet_provider() {
    use_context_provider(|| Signal::new(Wallet::Disconnected));

    #[cfg(feature = "web")]
    {
        let mut wallet = use_context::<Signal<Wallet>>();
        
        use_effect(move || {
            spawn(async move {
                // Listen for wallet connection events from JavaScript
                let eval = dioxus::document::eval(
                    r#"
                    window.addEventListener('ore-pubkey', (event) => {
                        if (event.detail.pubkey) {
                            dioxus.send(JSON.stringify(event.detail.pubkey));
                        } else {
                            dioxus.send(null);
                        }
                    });
                    "#
                );
                
                loop {
                    if let Ok(result) = eval.recv::<String>().await {
                        if result == "null" {
                            wallet.set(Wallet::Disconnected);
                        } else {
                            if let Ok(pubkey_bytes) = serde_json::from_str::<Vec<u8>>(&result) {
                                if pubkey_bytes.len() == 32 {
                                    let mut array = [0u8; 32];
                                    array.copy_from_slice(&pubkey_bytes);
                                    let pubkey = Pubkey::from(array);
                                    wallet.set(Wallet::Connected(pubkey));
                                }
                            }
                        }
                    }
                }
            });
        });
    }
}

pub fn use_wallet() -> Signal<Wallet> {
    use_context::<Signal<Wallet>>()
}

#[cfg(feature = "web")]
pub async fn disconnect_wallet() {
    let eval = dioxus::document::eval(
        r#"
        if (window.OreWalletDisconnecter) {
            window.OreWalletDisconnecter();
        }
        "#
    );
    let _ = eval.await;
}

#[cfg(feature = "web")]
pub async fn sign_and_send_transaction(transaction_base64: String) -> Result<String, String> {
    use dioxus::document::eval;
    
    let script = format!(
        r#"
        try {{
            if (!window.OreTxSigner) {{
                return "ERROR: Wallet not connected";
            }}
            const signed = await window.OreTxSigner({{ b64: "{}" }});
            return signed;
        }} catch (err) {{
            return "ERROR: " + err.message;
        }}
        "#,
        transaction_base64
    );
    
    let eval = eval(&script);
    
    match eval.await {
        Ok(result) => {
            if let Ok(result_str) = result.downcast::<String>() {
                if result_str.starts_with("ERROR:") {
                    Err(result_str)
                } else {
                    Ok(result_str)
                }
            } else {
                Err("Failed to parse result".to_string())
            }
        }
        Err(e) => Err(format!("Eval error: {:?}", e)),
    }
}

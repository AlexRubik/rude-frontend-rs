use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Wallet {
    Connected(Pubkey),
    Disconnected,
}

pub fn use_wallet_provider() {
    let mut signal = use_context_provider(|| Signal::new(Wallet::Disconnected));
    let mut eval = eval(
        r#"
            window.addEventListener("solana-pubkey", (event) => {
                dioxus.send(event.detail.pubkey);
            });
        "#,
    );
    spawn(async move {
        while let Ok(json_val) = eval.recv().await {
            if let Some(array) = json_val.as_array() {
                let bytes: Result<Vec<u8>, _> = array.iter().map(|v| v.as_u64().map(|n| n as u8)).collect();
                if let Ok(bytes) = bytes {
                    if bytes.len() == 32 {
                        let mut pubkey_bytes = [0u8; 32];
                        pubkey_bytes.copy_from_slice(&bytes);
                        if let Ok(pubkey) = Pubkey::try_from(pubkey_bytes) {
                            signal.set(Wallet::Connected(pubkey));
                            continue;
                        }
                    }
                }
            }
            signal.set(Wallet::Disconnected);
        }
    });
}

pub fn use_wallet() -> Signal<Wallet> {
    use_context()
}

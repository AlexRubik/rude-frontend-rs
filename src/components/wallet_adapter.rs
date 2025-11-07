use dioxus::prelude::*;
use dioxus::document::eval;
use crate::hooks::{use_wallet, Wallet, disconnect_wallet};

#[component]
pub fn WalletAdapter() -> Element {
    let wallet = use_wallet();

    // Mount the wallet adapter on first render
    let mut wallet_mount = use_future(move || async move {
        if Wallet::Disconnected == wallet.cloned() {
            #[cfg(feature = "web")]
            {
                async_std::task::sleep(std::time::Duration::from_millis(500)).await;
                let eval = eval(
                    r#"
                        if (window.MountWalletAdapter) {
                            window.MountWalletAdapter();
                        }
                        return
                    "#,
                );
                let _ = eval.await;
            }
        }
    });

    match wallet.cloned() {
        Wallet::Connected(address) => {
            let len = address.to_string().len();
            let first_four = &address.to_string()[0..4];
            let last_four = &address.to_string()[len - 4..len];

            rsx! {
                div {
                    class: "flex items-center space-x-2",
                    div {
                        class: "bg-purple-600 text-white px-4 py-2 rounded-lg font-medium",
                        "{first_four}...{last_four}"
                    }
                    button {
                        class: "bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg font-medium transition-colors",
                        onclick: move |_| {
                            spawn(async move {
                                disconnect_wallet().await;
                            });
                        },
                        "Disconnect"
                    }
                }
            }
        }
        Wallet::Disconnected => {
            rsx! {
                div {
                    class: "rounded-lg transition-all duration-300 ease-in-out",
                    div {
                        id: "ore-wallet-adapter",
                        class: "wallet-adapter-button"
                    }
                }
            }
        }
    }
}

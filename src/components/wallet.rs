use dioxus::document::eval;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::hooks::{use_wallet, Wallet};

pub fn WalletAdapter() -> Element {
    let wallet = use_wallet();

    let mut wallet_mount = use_future(move || async move {
        if Wallet::Disconnected == wallet.cloned() {
            gloo_timers::future::TimeoutFuture::new(500).await;
            let eval = eval(
                r#"
                    window.MountWalletAdapter();
                    return
                "#,
            );
            let _ = eval.await;
        }
    });

    match wallet.cloned() {
        Wallet::Connected(address) => {
            let len = address.to_string().len();
            let first_four = &address.to_string()[0..4];
            let last_four = &address.to_string()[len - 4..len];

            rsx! {
                div {
                    class: "rounded-full transition my-auto h-12 text-black bg-white hover:cursor-pointer hover:scale-105 duration-300 ease-in-out bg-controls-primary px-5 flex items-center",
                    span {
                        class: "text-sm font-semibold",
                        "{first_four}...{last_four}"
                    }
                }
            }
        }
        Wallet::Disconnected => {
            rsx! {
                div {
                    class: "rounded-full transition my-auto h-12 text-black bg-white hover:cursor-pointer hover:scale-105 duration-300 ease-in-out bg-controls-primary",
                    nav {
                        id: "solana-wallet-adapter"
                    }
                }
            }
        }
    }
}

#![allow(non_snake_case)]

mod components;
mod hooks;
mod pages;
mod route;

use dioxus::prelude::*;
use tracing::Level;

use crate::{
    hooks::{use_wallet_provider},
    route::Route,
};

const CSS: &str = include_str!("../public/tailwind.css");

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::default());
    
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App)
}

pub fn App() -> Element {
    use_wallet_provider();

    rsx! {
        style { "{CSS}" }
        document::Link { rel: "icon", href: asset!("/public/favicon.png") }
        document::Script { src: asset!("/public/wallet.js") }
        Router::<Route> {}
    }
}

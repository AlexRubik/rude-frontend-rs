#![allow(non_snake_case)]
mod components;
mod gateway;
mod hooks;
mod pages;
mod route;

use dioxus::prelude::*;
use tracing::Level;

const CSS: &str = include_str!("../public/tailwind.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(App)
}

pub fn App() -> Element {
    use_wallet_provider();
    use_gateway_provider();

    rsx! {
        style { "{CSS}" }
        document::Link { rel: "icon", href: asset!("/public/favicon.png") }
        document::Script { src: asset!("/public/wallet.js") }
        Router::<route::Route> {}
    }
}

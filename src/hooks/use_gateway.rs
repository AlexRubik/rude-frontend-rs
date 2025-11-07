use dioxus::prelude::*;
use crate::gateway::{Gateway, WebRpc, RPC_URL};

pub fn use_gateway_provider() {
    use_context_provider(|| {
        Signal::new(Gateway::<WebRpc>::new(RPC_URL.to_string()))
    });
}

pub fn use_gateway() -> Signal<Gateway<WebRpc>> {
    use_context()
}

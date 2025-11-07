use dioxus::prelude::*;
use crate::components::Navbar;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900",
            Navbar {}
            div {
                class: "container mx-auto px-4 py-8",
                Outlet::<Route> {}
            }
        }
    }
}

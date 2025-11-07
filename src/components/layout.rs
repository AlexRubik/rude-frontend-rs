use dioxus::prelude::*;
use crate::components::WalletAdapter;
use crate::route::Route;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-base-bg",
            Navbar {}
            main {
                class: "container mx-auto px-4 py-8",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn Navbar() -> Element {
    rsx! {
        nav {
            class: "bg-surface-elevated border-b border-gray-800",
            div {
                class: "container mx-auto px-4 py-4 flex justify-between items-center",
                div {
                    class: "flex gap-6 items-center",
                    Link {
                        to: Route::Home {},
                        class: "text-elements-highEmphasis hover:text-elements-midEmphasis transition",
                        "Home"
                    }
                    Link {
                        to: Route::About {},
                        class: "text-elements-highEmphasis hover:text-elements-midEmphasis transition",
                        "About"
                    }
                }
                WalletAdapter {}
            }
        }
    }
}

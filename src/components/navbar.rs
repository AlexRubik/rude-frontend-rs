use dioxus::prelude::*;
use crate::route::Route;
use crate::components::WalletAdapter;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            class: "bg-gray-800/50 backdrop-blur-md border-b border-gray-700",
            div {
                class: "container mx-auto px-4",
                div {
                    class: "flex items-center justify-between h-16",
                    
                    // Logo and title
                    div {
                        class: "flex items-center space-x-4",
                        Link {
                            to: Route::Home {},
                            class: "text-2xl font-bold text-white hover:text-purple-400 transition-colors",
                            "Solana App"
                        }
                    }

                    // Navigation links
                    div {
                        class: "flex items-center space-x-6",
                        Link {
                            to: Route::Home {},
                            class: "text-gray-300 hover:text-white transition-colors font-medium",
                            "Home"
                        }
                        Link {
                            to: Route::About {},
                            class: "text-gray-300 hover:text-white transition-colors font-medium",
                            "About"
                        }
                        
                        // Wallet adapter button
                        WalletAdapter {}
                    }
                }
            }
        }
    }
}

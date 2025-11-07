use dioxus::prelude::*;
use crate::route::Route;

#[component]
pub fn NotFound(_route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900 flex items-center justify-center px-4",
            div {
                class: "text-center",
                h1 {
                    class: "text-9xl font-bold text-white mb-4",
                    "404"
                }
                p {
                    class: "text-2xl text-gray-300 mb-8",
                    "Page not found"
                }
                Link {
                    to: Route::Home {},
                    class: "bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg transition-colors inline-block",
                    "Go Home"
                }
            }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "max-w-4xl mx-auto",
            
            // Header
            div {
                class: "text-center mb-12",
                h1 {
                    class: "text-5xl font-bold text-white mb-4",
                    "About"
                }
            }

            // Content card
            div {
                class: "bg-gray-800/50 backdrop-blur-md rounded-2xl p-8 border border-gray-700 shadow-2xl",
                
                div {
                    class: "prose prose-invert max-w-none",
                    
                    p {
                        class: "text-gray-300 text-lg leading-relaxed mb-6",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
                    }

                    p {
                        class: "text-gray-300 text-lg leading-relaxed mb-6",
                        "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                    }

                    h2 {
                        class: "text-2xl font-bold text-white mt-8 mb-4",
                        "Features"
                    }

                    ul {
                        class: "text-gray-300 space-y-3 mb-6",
                        li {
                            class: "flex items-start",
                            span { class: "text-purple-400 mr-2", "•" }
                            span { "Built with Rust and Dioxus for optimal performance" }
                        }
                        li {
                            class: "flex items-start",
                            span { class: "text-purple-400 mr-2", "•" }
                            span { "Integrated Solana wallet adapter for seamless blockchain interactions" }
                        }
                        li {
                            class: "flex items-start",
                            span { class: "text-purple-400 mr-2", "•" }
                            span { "Simple and intuitive interface for SOL transfers" }
                        }
                        li {
                            class: "flex items-start",
                            span { class: "text-purple-400 mr-2", "•" }
                            span { "Secure transaction signing with wallet confirmation" }
                        }
                    }

                    h2 {
                        class: "text-2xl font-bold text-white mt-8 mb-4",
                        "Technology Stack"
                    }

                    p {
                        class: "text-gray-300 text-lg leading-relaxed mb-6",
                        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo."
                    }

                    p {
                        class: "text-gray-300 text-lg leading-relaxed",
                        "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit."
                    }
                }
            }

            // Additional info card
            div {
                class: "mt-8 bg-gradient-to-r from-purple-900/30 to-pink-900/30 rounded-xl p-6 border border-purple-500/50",
                p {
                    class: "text-center text-gray-300 italic",
                    "\"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\""
                }
            }
        }
    }
}

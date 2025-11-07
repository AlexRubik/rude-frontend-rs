use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            class: "max-w-4xl mx-auto",
            h1 {
                class: "text-4xl font-bold mb-6 text-elements-highEmphasis",
                "About"
            }
            div {
                class: "prose prose-invert max-w-none",
                p {
                    class: "text-elements-midEmphasis mb-4",
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
                }
                p {
                    class: "text-elements-midEmphasis mb-4",
                    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
                }
                p {
                    class: "text-elements-midEmphasis mb-4",
                    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo."
                }
            }
        }
    }
}

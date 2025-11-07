use dioxus::prelude::*;
use crate::components::*;
use crate::pages::*;

#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
    #[end_layout]
}

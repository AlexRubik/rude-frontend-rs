use dioxus::prelude::*;

use crate::components::*;
use crate::pages::*;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq, Eq)]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
    #[end_layout]

    #[route("/:.._route")]
    NotFound { _route: Vec<String> }
}

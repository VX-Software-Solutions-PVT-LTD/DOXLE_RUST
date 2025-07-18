use dioxus::prelude::*;
use crate::Components::nav::Nav;
use crate::Components::home::Home;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
}

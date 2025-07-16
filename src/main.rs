use dioxus::prelude::*;
mod Components;

use crate::Components::nav::Nav;
use crate::Components::home::Home;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx!(
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    )
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[layout(Nav)]
    #[route("/")]
    Home {},
}
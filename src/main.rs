mod Components;
mod router;

use dioxus::prelude::*;
use router::Route;

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

use dioxus::prelude::*;
use crate::Components::footer::Footer;
use crate::router::Route;

#[component]
pub fn Nav() -> Element {
    rsx!(
        div {
            class: "min-h-screen bg-gray-50",

            nav {
                class: "bg-white text-black p-4",
                style: "position: fixed; top: 0; left: 0; right: 0; z-index: 40; font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif; font-weight: 600; font-size: 18px; line-height: 100%; letter-spacing: 0%; background-color: white; text-shadow: none; padding-top: 20px;",
                div {
                    class: "max-w-md mx-auto",
                    "10 Riddle St, Bentleigh"
                }
            }

            main {
                class: "relative",
                Outlet::<Route>{}
            }

            Footer {}
        }
    )
}

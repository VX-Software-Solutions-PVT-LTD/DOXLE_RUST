use dioxus::prelude::*;

const FILES: Asset = asset!("/assets/files.svg");
const DRAWINGS: Asset = asset!("/assets/drawings.svg");
const PLUS: Asset = asset!("/assets/plus.svg");
const BUDGET: Asset = asset!("/assets/budget.svg");
const CHECKLIST: Asset = asset!("/assets/checklist.svg");

#[component]
pub fn Footer() -> Element {
    rsx!(
        footer {
            class: "bg-[#34363a] flex justify-around items-end py-4",
            style: "position: fixed; bottom: 0; left: 0; right: 0; z-index: 50;",
            div {
                class: "flex flex-row justify-around items-end w-full max-w-3xl mx-auto",
                div { class: "flex flex-col items-center",
                    img {
                        src: "{FILES}",
                        style: "width:21px;height:18px;opacity:1;"
                    }
                    span {
                        style: "font-family:'Helvetica Neue',Helvetica,Arial,sans-serif;font-weight:400;font-size:8px;line-height:100%;letter-spacing:0%;color:#fff;background:transparent;margin-top:4px;",
                        "Files"
                    }
                }
                div { class: "flex flex-col items-center",
                    img {
                        src: "{DRAWINGS}",
                        style: "width:21.47px;height:18px;opacity:1;"
                    }
                    span {
                        style: "font-family:'Helvetica Neue',Helvetica,Arial,sans-serif;font-weight:400;font-size:8px;line-height:100%;letter-spacing:0%;color:#fff;background:transparent;margin-top:4px;",
                        "Drawings"
                    }
                }
                // Plus
                div { class: "flex flex-col items-center",
                    button {
                        style: "width:56px;height:32px;background:#83ADFF;border-radius:16px;display:flex;align-items:center;justify-content:center;box-shadow:0 2px 8px rgba(0,0,0,0.10);padding:0;border:none;margin-bottom:4px;",
                        img { src: "{PLUS}", style: "width:16px;height:16px;" }
                    }
                }
                
                div { class: "flex flex-col items-center",
                    img {
                        src: "{BUDGET}",
                        style: "width:20.25px;height:18px;opacity:1;"
                    }
                    span {
                        style: "font-family:'Helvetica Neue',Helvetica,Arial,sans-serif;font-weight:400;font-size:8px;line-height:100%;letter-spacing:0%;color:#fff;background:transparent;margin-top:4px;",
                        "Budget"
                    }
                }
                div { class: "flex flex-col items-center",
                    img {
                        src: "{CHECKLIST}",
                        style: "width:17px;height:19px;opacity:1;"
                    }
                    span {
                        style: "font-family:'Helvetica Neue',Helvetica,Arial,sans-serif;font-weight:400;font-size:8px;line-height:100%;letter-spacing:0%;color:#fff;background:transparent;margin-top:4px;",
                        "Checklist"
                    }
                }
            }
        }
    )
}
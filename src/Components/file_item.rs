use dioxus::prelude::*;
use crate::Components::file_types::{FileItem, FileType};

#[component]
pub fn FileItemComponent(item: FileItem, on_delete: EventHandler<u32>, icon: Asset) -> Element {
    let text = match item.file_type {
        FileType::Photo => "Photo",
        FileType::File => "File",
        FileType::Folder => "Folder",
    };

    rsx! {
        div {
            class: "flex items-center justify-between p-4 bg-white rounded-lg mb-2",
            div {
                class: "flex items-center space-x-3",
                img { src: "{icon}", class: "w-10 h-10 object-contain" }
                span {
                    class: "text-lg text-gray-900 font-normal",
                    style: "font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;",
                    "{item.name}"
                }
            }
            button {
                class: "text-gray-400 hover:text-gray-600 p-2",
                onclick: move |_| on_delete.call(item.id),
                "⋯"
            }
        }
    }
}
use dioxus::prelude::*;
use crate::Components::file_types::{FileItem, FileType};

#[component]
pub fn FileItemComponent(
    item: FileItem,
    on_delete: EventHandler<u32>,
    on_folder_click: EventHandler<u32>,
    on_video_click: EventHandler<String>,
    on_pdf_click: EventHandler<String>,
    icon: Asset,
        on_image_click:EventHandler<String>,
) -> Element {
    let item_name = item.name.clone();
    let item_id = item.id;
    let item_type = item.file_type.clone();

    let handle_click = move |_| {
        match item_type {
            FileType::Folder => on_folder_click.call(item_id),
            FileType::Video => on_video_click.call(item_name.clone()),
            FileType::PDF => on_pdf_click.call(item_name.clone()),
             FileType::Photo => on_image_click.call(item_name.clone()),
            _ => {},
        }
    };

    rsx! {
        div {
            class: "flex items-center justify-between p-4 bg-white rounded-lg mb-2 shadow-sm",
            div {
                class: "flex items-center space-x-3 flex-1",
                onclick: handle_click,
                cursor: if matches!(item.file_type, FileType::Folder | FileType::Video | FileType::PDF) { "pointer" } else { "default" },
                div {
                    class: "w-10 h-10 rounded-lg flex items-center justify-center",
                    img { src: "{icon}", class: "w-10 h-10" }
                }
                div {
                    span {
                        class: "text-sm text-gray-600",
                        "{item.name}"
                    }
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

use dioxus::prelude::*;
use crate::Components::file_types::{FileItem, FileType};

#[component]
pub fn FileItemComponent(
    item: FileItem,
    on_delete: EventHandler<u32>,
    on_folder_click: EventHandler<u32>,
    on_video_click: EventHandler<String>,
    on_pdf_click: EventHandler<String>,
    on_canvas_click: EventHandler<String>,
    on_file_click: EventHandler<String>,
    icon: Asset,
    on_image_click: EventHandler<FileItem>,
) -> Element {
    let item_name = item.name.clone();
    let item_id = item.id;
    let item_type = item.file_type.clone();
    let item_clone = item.clone();

    let handle_click = move |_| {
        match item_type {
            FileType::Folder => on_folder_click.call(item_id),
            FileType::Video => on_video_click.call(item_name.clone()),
            FileType::PDF => on_pdf_click.call(item_name.clone()),
            FileType::Photo => on_image_click.call(item_clone.clone()),
            FileType::Canvas => on_canvas_click.call(item_name.clone()),
            FileType::File => on_file_click.call(item_name.clone()),
        }
    };

    rsx! {
        div {
            class: "flex items-center justify-between p-4 bg-white rounded-lg mb-2",
            div {
                class: "flex items-center space-x-3 flex-1",
                onclick: handle_click,
                cursor: if matches!(item.file_type, FileType::Folder | FileType::Video | FileType::PDF | FileType::Photo | FileType::Canvas | FileType::File) { "pointer" } else { "default" },
                div {
                    class: "w-10 h-10 rounded-lg flex items-center justify-center",
                    img { src: "{icon}", class: "w-10 h-10" }
                }
                div {
                    span {
                        class: "text-sm text-gray-800 font-semibold",
                        "{item.name}"
                    }
                }
            }
            button {
                class: "text-gray-800 hover:text-gray-900 p-2 font-black text-xl leading-none",
                style: "font-weight: 900; letter-spacing: 4px;",
                onclick: move |_| on_delete.call(item.id),
                "⋯"
            }
        }
    }
}

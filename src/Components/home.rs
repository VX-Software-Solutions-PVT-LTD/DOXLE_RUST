use dioxus::prelude::*;
use crate::Components::file_types::{FileItem, FileType};
use crate::Components::file_item::FileItemComponent;
use crate::Components::footer::Footer;

const PHOTO: Asset = asset!("/assets/Photo.svg");
const FILE: Asset = asset!("/assets/File.svg");
const FOLDER: Asset = asset!("/assets/Folder.svg");
const MAIN_FOLDER: Asset = asset!("/assets/Mainfolder.svg");
const SEARCH_ICON: Asset = asset!("/assets/Search.svg");

#[component]
pub fn Home() -> Element {
    let mut search_query = use_signal(|| String::new());
    

    let mut file_items = use_signal(|| vec![
        FileItem::new(1, "Archives".to_string(), FileType::Folder),
        FileItem::new(2, "Banks and Taxes".to_string(), FileType::Folder),
        FileItem::new(3, "Temp Folder".to_string(), FileType::Folder),
        FileItem::new(4, "New Folder".to_string(), FileType::Folder),
    ]);

    let add_photo = move |_| {
        let mut items = file_items.write();
        let new_id = items.len() as u32 + 1;
        items.push(FileItem::new(new_id, "Photo".to_string(), FileType::Photo));
    };

    let add_file = move |_| {
        let mut items = file_items.write();
        let new_id = items.len() as u32 + 1;
        items.push(FileItem::new(new_id, "File".to_string(), FileType::File));
    };

    let add_folder = move |_| {
        let mut items = file_items.write();
        let new_id = items.len() as u32 + 1;
        items.push(FileItem::new(new_id, "New Folder".to_string(), FileType::Folder));
    };

    let delete_item = move |id: u32| {
        let mut items = file_items.write();
        items.retain(|item| item.id != id);
    };


    let filtered_items = file_items.read()
        .iter()
        .filter(|item| {
            if search_query.read().is_empty() {
                true
            } else {
                item.name.to_lowercase().contains(&search_query.read().to_lowercase())
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            style: "padding-top: 72px;", 
            div {
                class: "max-w-md mx-auto bg-white min-h-screen",
                div {
                    class: "p-4 bg-white",
                    div {
                        class: "relative mb-4",
                        div {
                            class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                            img { src: "{SEARCH_ICON}", style: "width:38px;height:24px;" }
                        }
                        input {
                            class: "w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg bg-gray-50 ml-2 pl-2",
                            style: "font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;",
                            placeholder: "  Search for files",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value()),
                        }
                    }
                    div {
                        class: "flex space-x-4 mb-6",
                        button {
                            class: "flex flex-col items-center space-y-1 p-0",
                            style: "width:79px;height:36px;background:#F2F3FE;border:1px solid #EAECEF;border-radius:4px;display:flex;align-items:center;justify-content:center;opacity:1;",
                            onclick: add_photo,
                            img { src: "{PHOTO}", style: "width:38px;height:24px;" }
                        }
                        button {
                            class: "flex flex-col items-center space-y-1 p-0",
                            style: "width:79px;height:36px;background:#F2F3FE;border:1px solid #EAECEF;border-radius:4px;display:flex;align-items:center;justify-content:center;opacity:1;",
                            onclick: add_file,
                            img { src: "{FILE}", style: "width:34px;height:24px;" }
                        }
                        button {
                            class: "flex flex-col items-center space-y-1 p-0",
                            style: "width:79px;height:36px;background:#F2F3FE;border:1px solid #EAECEF;border-radius:4px;display:flex;align-items:center;justify-content:center;opacity:1;",
                            onclick: add_folder,
                            img { src: "{FOLDER}", style: "width:38px;height:24px;" }
                        }
                    }
                }
                
                div {
                    class: "px-4",
                    for item in filtered_items {
                        {
                            let icon = match item.file_type {
                                FileType::Photo => PHOTO,
                                FileType::File => FILE,
                                FileType::Folder => MAIN_FOLDER,
                            };
                            rsx!(
                                FileItemComponent {
                                    item: item.clone(),
                                    on_delete: delete_item,
                                    icon: icon,
                                }
                            )
                        }
                    }
                }
            }
            
         
        }
    }
}
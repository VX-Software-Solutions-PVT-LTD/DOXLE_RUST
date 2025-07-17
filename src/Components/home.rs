use dioxus::html::mo;
use dioxus::prelude::*;
use crate::Components::file_types::{FileItem, FileType};
use crate::Components::file_item::FileItemComponent;
use crate::Components::footer::Footer;

const PHOTO: Asset = asset!("/assets/Photo.svg");
const FILE: Asset = asset!("/assets/File.svg");
const FOLDER: Asset = asset!("/assets/Folder.svg");
const MAIN_FOLDER: Asset = asset!("/assets/Mainfolder.svg");
const SEARCH_ICON: Asset = asset!("/assets/Search.svg");
const VIDEO: Asset = asset!("/assets/Video.svg");
const PDF: Asset = asset!("/assets/PDF.svg");
const VIDEO_TO_PLAY: Asset = asset!("/assets/Video.mp4");
const PDF_TO_PLAY: Asset = asset!("/assets/PDF.pdf");

#[component]
pub fn Home() -> Element {
    let search_query = use_signal(|| String::new());
    let mut current_folder = use_signal(|| None::<u32>);
    let mut selected_video = use_signal(|| None::<String>);
    let mut selected_pdf = use_signal(|| None::<String>);

    let mut file_items = use_signal(|| vec![
        FileItem::new(1, "Archives".to_string(), FileType::Folder),
        FileItem::new(2, "Banks and Taxes".to_string(), FileType::Folder),
        FileItem::new(3, "Temp Folder".to_string(), FileType::Folder),
        FileItem::new(4, "New Folder".to_string(), FileType::Folder),
        FileItem::new(5, "Sample Photo".to_string(), FileType::Photo),
        FileItem::new(6, "Document.pdf".to_string(), FileType::PDF),
    ]);

   let get_next_id = move || {
        file_items.read().iter().map(|item| item.id).max().unwrap_or(0) + 1
    };

      let add_photo = move |_| {
        let mut items = file_items.write();
        let new_id = get_next_id();
        let item = if let Some(folder_id) = current_folder.read().as_ref() {
            FileItem::new_with_parent(new_id, "Photo".to_string(), FileType::Photo, *folder_id)
        } else {
            FileItem::new(new_id, "Photo".to_string(), FileType::Photo)
        };
        items.push(item);
        
        // Update parent folder's children
        if let Some(folder_id) = current_folder.read().as_ref() {
            if let Some(folder) = items.iter_mut().find(|f| f.id == *folder_id) {
                folder.children.push(new_id);
            }
        }
    };
  let add_file = move |_| {
        let mut items = file_items.write();
        let new_id = get_next_id();
        let item = if let Some(folder_id) = current_folder.read().as_ref() {
            FileItem::new_with_parent(new_id, "File".to_string(), FileType::File, *folder_id)
        } else {
            FileItem::new(new_id, "File".to_string(), FileType::File)
        };
        items.push(item);
        
        if let Some(folder_id) = current_folder.read().as_ref() {
            if let Some(folder) = items.iter_mut().find(|f| f.id == *folder_id) {
                folder.children.push(new_id);
            }
        }
    };

    let add_folder = move |_| {
        let mut items = file_items.write();
        let new_id = get_next_id();
        let item = if let Some(folder_id) = current_folder.read().as_ref() {
            FileItem::new_with_parent(new_id, "New Folder".to_string(), FileType::Folder, *folder_id)
        } else {
            FileItem::new(new_id, "New Folder".to_string(), FileType::Folder)
        };
        items.push(item);
        
        if let Some(folder_id) = current_folder.read().as_ref() {
            if let Some(folder) = items.iter_mut().find(|f| f.id == *folder_id) {
                folder.children.push(new_id);
            }
        }
    };

    let add_video = move |_: Event<MouseData>| {
        let mut items = file_items.write();
        let new_id = get_next_id();
        let item = if let Some(folder_id) = current_folder.read().as_ref() {
            FileItem::new_with_parent(new_id, "Video.mp4".to_string(), FileType::Video, *folder_id)
        } else {
            FileItem::new(new_id, "Video.mp4".to_string(), FileType::Video)
        };
        items.push(item);
        if let Some(folder_id) = current_folder.read().as_ref() {
            if let Some(folder) = items.iter_mut().find(|f| f.id == *folder_id) {
                folder.children.push(new_id);
            }
        }
    };
    let add_pdf = move |_: Event<MouseData>| {
        let mut items = file_items.write();
        let new_id = get_next_id();
        let item = if let Some(folder_id) = current_folder.read().as_ref() {
            FileItem::new_with_parent(new_id, "Document.pdf".to_string(), FileType::PDF, *folder_id)
        } else {
            FileItem::new(new_id, "Document.pdf".to_string(), FileType::PDF)
        };
        items.push(item);
        if let Some(folder_id) = current_folder.read().as_ref() {
            if let Some(folder) = items.iter_mut().find(|f| f.id == *folder_id) {
                folder.children.push(new_id);
            }
        }
    };
 let delete_item = move |id: u32| {
        let mut items = file_items.write();
        
        // Find the item to delete
        if let Some(item) = items.iter().find(|item| item.id == id).cloned() {
            // If it's a folder, collect all items to delete (recursive)
            let mut items_to_delete = vec![id];
            
            if item.file_type == FileType::Folder {
                let mut stack = item.children.clone();
                while let Some(child_id) = stack.pop() {
                    items_to_delete.push(child_id);
                    if let Some(child) = items.iter().find(|i| i.id == child_id) {
                        if child.file_type == FileType::Folder {
                            stack.extend(child.children.clone());
                        }
                    }
                }
            }
            
            // Remove all items
            items.retain(|item| !items_to_delete.contains(&item.id));
            
            // Update parent folder's children if item had a parent
            if let Some(parent_id) = item.parent_id {
                if let Some(parent) = items.iter_mut().find(|f| f.id == parent_id) {
                    parent.children.retain(|&child_id| child_id != id);
                }
            }
        }
    };

let open_folder = move |id: u32| current_folder.set(Some(id));
    let play_video = move |name: String| selected_video.set(Some(name));
    let open_pdf = move |name: String| selected_pdf.set(Some(name));
 
    let filtered_items = file_items.read()
        .iter()
        .filter(|item| {
            let in_current_folder = match current_folder.read().as_ref() {
                None => item.parent_id.is_none(),
                Some(folder_id) => item.parent_id == Some(*folder_id),
            };
            let matches_search = if search_query.read().is_empty() {
                true
            } else {
                item.name.to_lowercase().contains(&search_query.read().to_lowercase())
            };
            in_current_folder && matches_search
        })
        .cloned()
        .collect::<Vec<_>>();

    // Get current folder name for breadcrumb
    let current_folder_name = current_folder.read().as_ref().and_then(|folder_id| {
        file_items.read().iter().find(|item| item.id == *folder_id).map(|item| item.name.clone())
    });

 rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            style: "padding-top: 72px;",
            div {
                class: "max-w-md mx-auto bg-white min-h-screen",
                div {
                    class: "p-4 bg-white",
                    // Search and add buttons (unchanged except onclicks)
                    div {
                        class: "flex space-x-4 mb-6",
                        button { onclick: add_photo, img { src: "{PHOTO}", style: "width:38px;height:24px;" } }
                        button { onclick: add_file, img { src: "{FILE}", style: "width:34px;height:24px;" } }
                        button { onclick: add_folder, img { src: "{FOLDER}", style: "width:38px;height:24px;" } }
                        button { onclick: add_video, img { src: "{VIDEO}", style: "width:38px;height:24px;" } }
                        button { onclick: add_pdf, img { src: "{PDF}", style: "width:38px;height:24px;" } }
                    }
                }

                div {
                    class: "px-4",
                    for item in filtered_items {
                        if let FileType::Video = item.file_type {
                            div {
                                class: "mb-4",
                                h3 { class: "text-md font-semibold mb-2", "Playing: {item.name}" }
                                video {
                                    src: "{VIDEO_TO_PLAY}",
                                    controls: true,
                                    autoplay: true,
                                    class: "w-full rounded-lg",
                                    "Your browser does not support the video tag."
                                }
                            }
                        } else {
                            FileItemComponent {
                                item: item.clone(),
                                on_delete: delete_item,
                                on_folder_click: open_folder,
                                on_video_click: play_video,
                                on_pdf_click: open_pdf,
                                icon: match item.file_type {
                                    FileType::Photo => PHOTO,
                                    FileType::File => FILE,
                                    FileType::Folder => MAIN_FOLDER,
                                    FileType::Video => VIDEO,
                                    FileType::PDF => PDF,
                                },
                            }
                        }
                    }
                }
                }

                // VIDEO MODAL REMOVED: Now videos play inline

                // PDF MODAL
                if let Some(pdf_name) = selected_pdf.read().as_ref() {
                    div {
                        class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                        onclick: move |_| selected_pdf.set(None),
                        div {
                            class: "bg-white p-4 rounded-lg max-w-3xl w-full h-[90%] mx-4 overflow-hidden",
                            onclick: move |e| e.stop_propagation(),
                            h3 { class: "text-lg font-semibold mb-2", "Reading: {pdf_name}" }
                            iframe {
                                class: "w-full h-full",
                                src: "{PDF_TO_PLAY}",
                            }
                        }
                    }
                }
            }
        }
    
}
    
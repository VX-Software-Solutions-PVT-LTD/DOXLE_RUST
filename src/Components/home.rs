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
const IMAGE:Asset = asset!("/assets/random.jpg");

#[component]
pub fn Home() -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut current_folder = use_signal(|| None::<u32>);
    let mut selected_video = use_signal(|| None::<String>);
    let mut selected_pdf = use_signal(|| None::<String>);
    let mut selected_image = use_signal(|| None::<String>);
    let mut file_items = use_signal(|| vec![
        FileItem::new(1, "Archives".to_string(), FileType::Folder),
        FileItem::new(2, "Banks and Taxes".to_string(), FileType::Folder),
        FileItem::new(3, "Temp Folder".to_string(), FileType::Folder),
        FileItem::new(4, "New Folder".to_string(), FileType::Folder),

        FileItem::new(7, "Video.mp4".to_string(), FileType::Video), // Moved to root
        FileItem::new_with_parent(8, "Folder Video.mp4".to_string(), FileType::Video, 3), 
            FileItem::new(9, "Root PDF.pdf".to_string(), FileType::PDF),
    FileItem::new_with_parent(10, "Temp PDF.pdf".to_string(), FileType::PDF, 3),
                FileItem::new(11, "random.svg".to_string(), FileType::Photo),
    FileItem::new_with_parent(12, "random.svg".to_string(), FileType::Photo, 3),
    ]);

  let delete_item = move |id: u32| {
    let mut items = file_items.write();
    if let Some(item) = items.iter().find(|item| item.id == id).cloned() {
        let mut items_to_delete = vec![id];
        // Add all children recursively
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
            // Also delete root items with similar names (optional)
            let folder_name = item.name.clone();
            for root_item in items.iter() {
                if root_item.parent_id.is_none() &&
                   (root_item.name.contains("Video.mp4") || root_item.name.contains("PDF.pdf") ||  root_item.name.contains("random.svg")) {
                    items_to_delete.push(root_item.id);
                }
            }
        }
        items.retain(|item| !items_to_delete.contains(&item.id));
        if let Some(parent_id) = item.parent_id {
            if let Some(parent) = items.iter_mut().find(|f| f.id == parent_id) {
                parent.children.retain(|&child_id| child_id != id);
            }
        }
    }
};

    let open_folder = move |id: u32| {
        current_folder.set(Some(id));
        selected_video.set(None);
        selected_pdf.set(None);
    };

    let play_video = move |name: String| {
        // Always go back to root when playing video
        current_folder.set(None);
        selected_video.set(Some(name));
        selected_pdf.set(None);
    };

  let open_pdf = move |name: String| {
    let current = current_folder.read();
    // Only allow opening PDFs if in root folder
    if current.is_none() {
        selected_pdf.set(Some(name));
        selected_video.set(None);
    }
};

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

    let current_folder_name = current_folder.read().as_ref().and_then(|folder_id| {
        file_items.read().iter().find(|item| item.id == *folder_id).map(|item| item.name.clone())
    });
let add_photo = move |_:Event<MouseData>| {
    let mut items = file_items.write();
    let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
    items.push(FileItem::new(new_id, format!("New Photo {}", new_id), FileType::Photo));
};

let preview_image = move |name: String| {
    if current_folder.read().is_none() {
        // Build the path for the image using the file name
        selected_image.set(Some(format!("/assets/{}", name)));
        selected_video.set(None);
        selected_pdf.set(None);
    }
};

let add_file = move |_:Event<MouseData>| {
    let mut items = file_items.write();
    let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
    items.push(FileItem::new(new_id, format!("New File {}", new_id), FileType::File));
};

let add_folder = move |_:Event<MouseData>| {
    let mut items = file_items.write();
    let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
    items.push(FileItem::new(new_id, format!("New Folder {}", new_id), FileType::Folder));
};
    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            style: "padding-top: 72px;",
            div {
                class: "max-w-md mx-auto bg-white min-h-screen",
                     div {
        class: "flex items-center bg-gray-100 rounded-full px-4 py-2",
        img {
            src: "{SEARCH_ICON}",
            class: "w-5 h-5 mr-2",
        }
     input {
    r#type: "text",
    class: "bg-transparent outline-none flex-1 text-gray-700",
    placeholder: " Search for files",
    value: "{search_query.read()}",
    oninput: move |evt| search_query.set(evt.value()),
}
    }
                // Header
                div {
                    class: "p-4 bg-white",

                    if let Some(folder_name) = current_folder_name {
                        div {
                            class: "mb-4",
                            button {
                                class: "",
                                onclick: move |_| {
                                    current_folder.set(None);
                                    selected_video.set(None);
                                    selected_pdf.set(None);
                                },
                                "← Back to Root"
                            }
                            h2 { class: "text-lg font-semibold mt-2", "Folder: {folder_name}" }
                        }
                    }

             if current_folder.read().is_none() {
    div {
        class: "flex space-x-4 mb-6",
        button {
            style: "width:79px;height:36px;border-radius:4px;border-width:1px;background:#F2F3FE;border:1px solid #EAECEF;opacity:1;display:flex;align-items:center;justify-content:center;",
            onclick: add_photo,
            img {
                src: "{PHOTO}",
                style: "width:39px;height:17px;border-radius:1px;opacity:1;",
            }
        }
        button {
            style: "width:79px;height:36px;border-radius:4px;border-width:1px;background:#F2F3FE;border:1px solid #EAECEF;opacity:1;display:flex;align-items:center;justify-content:center;",
            onclick: add_file,
            img {
                src: "{FILE}",
                style: "width:39px;height:17px;border-radius:1px;opacity:1;",
            }
        }
        button {
            style: "width:79px;height:36px;border-radius:4px;border-width:1px;background:#F2F3FE;border:1px solid #EAECEF;opacity:1;display:flex;align-items:center;justify-content:center;",
              onclick: add_folder,
            img {
                src: "{FOLDER}",
                style: "width:39px;height:17px;border-radius:1px;opacity:1;",
            }
        }
    }
}
                  
                }

                div {
                    class: "px-4",

                    
                    if current_folder.read().is_none() {
                        if let Some(video_name) = selected_video.read().as_ref() {
                            div {
                                class: "mb-4 bg-gray-100 p-4 rounded-lg",
                                div {
                                    class: "flex justify-between items-center mb-2",
                                    h3 { class: "text-md font-semibold", "Playing: {video_name}" }
                                    button {
                                        class: "text-red-500 hover:text-red-700",
                                        onclick: move |_| selected_video.set(None),
                                        "✕ Close"
                                    }
                                }
                                video {
                                    src: "{VIDEO_TO_PLAY}",
                                    controls: true,
                                    autoplay: true,
                                    class: "w-full rounded-lg",
                                    "Your browser does not support the video tag."
                                }
                            }
                        }

                        if let Some(pdf_name) = selected_pdf.read().as_ref() {
                            div {
                                class: "mb-4 bg-gray-100 p-4 rounded-lg",
                                div {
                                    class: "flex justify-between items-center mb-2",
                                    h3 { class: "text-md font-semibold", "Opening: {pdf_name}" }
                                    button {
                                        class: "text-red-500 hover:text-red-700",
                                        onclick: move |_| selected_pdf.set(None),
                                        "✕ Close"
                                    }
                                }
                                iframe {
                                    src: "{PDF_TO_PLAY}",
                                    class: "w-full h-96 rounded-lg",
                                    "Your browser does not support PDF preview."
                                }
                            }
                        }
                    }
if let Some(image_path) = selected_image.read().as_ref() {
    div {
        class: "mb-4 bg-gray-100 p-4 rounded-lg",
        div {
            class: "flex justify-between items-center mb-2",
            h3 { class: "text-md font-semibold", "Previewing: {image_path}" }
            button {
                class: "text-red-500 hover:text-red-700",
                onclick: move |_| selected_image.set(None),
                "✕ Close"
            }
        }
        img {
            src: "{IMAGE}",
            class: "w-full rounded-lg",
            alt: "{IMAGE}",
        }
    }
}
                    for item in filtered_items {
                        FileItemComponent {
                            item: item.clone(),
                            on_delete: delete_item,
                            on_folder_click: open_folder,
                            on_video_click: play_video,
                            on_pdf_click: open_pdf,
                            on_image_click: preview_image,
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
    }
}
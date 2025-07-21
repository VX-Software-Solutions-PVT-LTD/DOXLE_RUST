use dioxus::prelude::*;
use crate::Components::file_types::{FileItem, FileType};
use crate::Components::file_item::FileItemComponent;

// #[cfg(not(target_arch = "wasm32"))]
// use skia_safe::{Canvas as SkiaCanvas, Paint, Path, Color, Surface, EncodedImageFormat};

// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

// #[cfg(target_arch = "wasm32")]
// use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

const PHOTO: Asset = asset!("/assets/Photo.svg");
const FILE: Asset = asset!("/assets/File.svg");
const FOLDER: Asset = asset!("/assets/Folder.svg");
const MAIN_FOLDER: Asset = asset!("/assets/Mainfolder.svg");
const SEARCH_ICON: Asset = asset!("/assets/Search.svg");
const VIDEO: Asset = asset!("/assets/Video.svg");
const PDF: Asset = asset!("/assets/PDF.svg");
const VIDEO_TO_PLAY: Asset = asset!("/assets/Video.mp4");
const PDF_TO_PLAY: Asset = asset!("/assets/PDF.pdf");
const IMAGE: Asset = asset!("/assets/random.jpg");
const CANVAS: Asset = asset!("/assets/File.svg");

#[derive(Clone, Debug)]
struct DrawingPath {
    points: Vec<(f32, f32)>,
    color: String,
    width: f32,
}

#[component]
pub fn Home() -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut current_folder = use_signal(|| None::<u32>);
    let mut selected_video = use_signal(|| None::<String>);
    let mut selected_pdf = use_signal(|| None::<String>);
    let mut selected_image = use_signal(|| None::<String>);
    let mut selected_canvas = use_signal(|| None::<String>);
    let mut selected_file = use_signal(|| None::<String>);
    
    
    let mut drawing_paths = use_signal(|| Vec::<DrawingPath>::new());
    // let mut is_drawing = use_signal(|| false);
    let mut current_path = use_signal(|| Vec::<(f32, f32)>::new());
    let mut canvas_key = use_signal(|| 0u32);

    let mut file_items = use_signal(|| vec![
        FileItem::new(1, "Archives".to_string(), FileType::Folder),
        FileItem::new(2, "Banks and Taxes".to_string(), FileType::Folder),
        FileItem::new(3, "Temp Folder".to_string(), FileType::Folder),
        FileItem::new(4, "New Folder".to_string(), FileType::Folder),
        FileItem::new(5, "Video.mp4".to_string(), FileType::Video),
        FileItem::new_with_parent(6, "Folder Video.mp4".to_string(), FileType::Video, 3), 
        FileItem::new(7, "Root PDF.pdf".to_string(), FileType::PDF),
        FileItem::new_with_parent(8, "Temp PDF.pdf".to_string(), FileType::PDF, 3),
        FileItem::new_with_parent(10, "random.svg".to_string(), FileType::Photo, 3),
    ]);


    // Tried these  Drawing event handlers but not working
// let start_drawing = move |evt: Event<MouseData>| {
//     evt.stop_propagation();
//     is_drawing.set(true);

//     let coords = evt.element_coordinates();
//     let x = coords.x as f32 * 350.0 / 100.0; // Scale to viewBox width
//     let y = coords.y as f32 * 280.0 / 100.0; // Scale to viewBox height
    
//     current_path.set(vec![(x, y)]);
// };

// let continue_drawing = move |evt: Event<MouseData>| {
//     if *is_drawing.read() {
//         evt.stop_propagation();
        
//         let coords = evt.element_coordinates();
//         let x = coords.x as f32 * 350.0 / 100.0;
//         let y = coords.y as f32 * 280.0 / 100.0;
        
//         let mut path = current_path.write();
//         path.push((x, y));
//     }
// };

// let stop_drawing = move |evt: Event<MouseData>| {
//     if *is_drawing.read() {
//         evt.stop_propagation();
//         is_drawing.set(false);
//         let path = current_path.read().clone();
//         if path.len() > 1 {
//             let drawing_path = DrawingPath {
//                 points: path,
//                 color: "#10B981".to_string(), // Green for completed paths
//                 width: 3.0,
//             };
//             let mut paths = drawing_paths.write();
//             paths.push(drawing_path);
//             current_path.set(Vec::new());
//         }
//     }
// };


    let delete_item = move |id: u32| {
        let mut items = file_items.write();
        if let Some(item) = items.iter().find(|item| item.id == id).cloned() {
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
            items.retain(|item| !items_to_delete.contains(&item.id));
        }
    };

    let open_folder = move |id: u32| {
        current_folder.set(Some(id));
        selected_video.set(None);
        selected_pdf.set(None);
        selected_canvas.set(None);
        selected_image.set(None);
        selected_file.set(None);
    };

    let play_video = move |name: String| {
        current_folder.set(None);
        selected_video.set(Some(name));
        selected_pdf.set(None);
        selected_canvas.set(None);
        selected_image.set(None);
        selected_file.set(None);
    };

    let open_pdf = move |name: String| {
        if current_folder.read().is_none() {
            selected_pdf.set(Some(name));
            selected_video.set(None);
            selected_canvas.set(None);
            selected_image.set(None);
            selected_file.set(None);
        }
    };

    let open_canvas = move |name: String| {
        if current_folder.read().is_none() {
            selected_canvas.set(Some(name));
            selected_video.set(None);
            selected_pdf.set(None);
            selected_image.set(None);
            selected_file.set(None);
            drawing_paths.set(Vec::new());
            current_path.set(Vec::new());
            canvas_key.set(canvas_key() + 1);
        }
    };

    let open_file = move |name: String| {
        if current_folder.read().is_none() {
            selected_file.set(Some(name));
            selected_video.set(None);
            selected_pdf.set(None);
            selected_canvas.set(None);
            selected_image.set(None);
        }
    };

    let clear_canvas = move |_| {
        drawing_paths.set(Vec::new());
        current_path.set(Vec::new());
        canvas_key.set(canvas_key() + 1);
    };

    let save_canvas = move |_| {
        
        selected_canvas.set(None);
        drawing_paths.set(Vec::new());
        current_path.set(Vec::new());
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

    let preview_image = move |item: FileItem| {
        if current_folder.read().is_none() {
            selected_image.set(Some(item.name.clone()));
            selected_video.set(None);
            selected_pdf.set(None);
            selected_canvas.set(None);
            drawing_paths.set(Vec::new()); // Clear drawing paths for new image
            current_path.set(Vec::new());
            canvas_key.set(canvas_key() + 1);
        }
    };

    let add_file = move |_:Event<MouseData>| {
        let mut items = file_items.write();
        let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        items.push(FileItem::new(new_id, format!("Document {}.txt", new_id), FileType::File));
    };

    let add_folder = move |_:Event<MouseData>| {
        let mut items = file_items.write();
        let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        items.push(FileItem::new(new_id, format!("New Folder {}", new_id), FileType::Folder));
    };

    let add_canvas = move |_:Event<MouseData>| {
        let mut items = file_items.write();
        let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        items.push(FileItem::new(new_id, format!("New Canvas {}", new_id), FileType::Canvas));
    };

    rsx! {
        div {
            class: "min-h-screen bg-white overflow-y-auto",
            style: "padding-top: 40px;",
            div {
                class: "max-w-md mx-auto bg-white",
                

                div {
                    class: "p-4 bg-white sticky top-20 z-10 border-b border-gray-100 ",
                    
                    div {
                        class: "flex items-center bg-gray-100 rounded-full px-4 py-2 mb-4",
                        img {
                            src: "{SEARCH_ICON}",
                            class: "w-4 h-4 mr-3",
                        }
                        input {
                            r#type: "text",
                            class: "bg-transparent outline-none flex-1 text-gray-600 text-sm",
                            placeholder: "   Search for files",
                            value: "{search_query.read()}",
                            oninput: move |evt| search_query.set(evt.value()),
                        }
                    }
                    
                  
    div {
    style: "display: flex; gap: 12px; margin-top: 12px; margin-bottom: 20px;",

    // Photo Button
    button {
        onclick: add_photo,
        style: "width: 95px; height: 36px; border-radius: 4px; border: 1px solid #EAECEF; background: rgba(236, 239, 253, 0.6); display: flex; align-items: center; justify-content: center;",
        img {
            src: "{PHOTO}",
            style: "width: 60;
height: 17;
top: 141px;
left: 54px;
angle: 0 deg;
opacity: 1;
",
        }
        
    }

    // File Button
    button {
        onclick: add_file,
        style: "width: 79px; height: 36px; border-radius: 4px; border: 1px solid #EAECEF; background: #F2F3FE; display: flex; align-items: center; justify-content: center;",
        img {
            src: "{FILE}",
            style: "width: 39;
height: 17;
top: 141px;
left: 157px;
border-radius: 1px;
angle: 0 deg;
opacity: 1;
",
        }
        
    }

    // Folder Button
    button {
        onclick: add_folder,
        style: "width: 95px; height: 36px; border-radius: 4px; border: 1px solid #EAECEF; background: #F2F3FE; display: flex; align-items: center; justify-content: center;",
        img {
            src: "{FOLDER}",
            style: "width: 62;
height: 17;
top: 141px;
left: 235px;
angle: 0 deg;
opacity: 1;
",
        }
        
    }
}
                    if let Some(folder_name) = current_folder_name {
                        div {
                            class: "mt-4",
                            button {
                                class: "flex items-center text-blue-600 hover:text-blue-800 mb-2",
                                onclick: move |_| {
                                    current_folder.set(None);
                                    selected_video.set(None);
                                    selected_pdf.set(None);
                                    selected_canvas.set(None);
                                    selected_image.set(None);
                                    selected_file.set(None);
                                },
                                "← Back to Root"
                            }
                            h2 { class: "text-lg font-semibold", "Folder: {folder_name}" }
                        }
                    }
                }

                div {
                    class: "px-4 pb-20",
                    style: "max-height: calc(100vh - 200px); overflow-y: auto;",


if let Some(canvas_name) = selected_canvas.read().as_ref() {
    div {
        class: "mb-4 bg-gray-100 p-4 rounded-lg",
        div {
            class: "flex justify-between items-center mb-2",
            h3 { class: "text-md font-semibold", "Drawing: {canvas_name}" }
            div {
                class: "flex space-x-2",
                button {
                    class: "bg-blue-500 text-white px-3 py-1 rounded text-sm hover:bg-blue-600",
                    onclick: clear_canvas,
                    "Clear"
                }
                button {
                    class: "bg-green-500 text-white px-3 py-1 rounded text-sm hover:bg-green-600",
                    onclick: save_canvas,
                    "Save"
                }
                button {
                    class: "text-red-500 hover:text-red-700 px-2",
                    onclick: move |_| selected_canvas.set(None),
                    "✕ Close"
                }
            }
        }
        div {
            class: "bg-white rounded-lg border-2 border-gray-300 p-4",
            style: "width: 100%; height: 300px;",
            
            
            div {
                style: "width: 100%; height: 100%; background-image: url('{IMAGE}'); background-size: cover; background-position: center; background-repeat: no-repeat; cursor: crosshair; border-radius: 8px; position: relative;",
                onclick: move |evt: Event<MouseData>| {
                    
                    let coords = evt.element_coordinates();
                    let drawing_path = DrawingPath {
                        points: vec![(coords.x as f32, coords.y as f32), (coords.x as f32 + 1.0, coords.y as f32 + 1.0)],
                        color: "#ff0000".to_string(), 
                        width: 10.0,
                    };
                    let mut paths = drawing_paths.write();
                    paths.push(drawing_path);
                },
                
                
                for (i, path) in drawing_paths.read().iter().enumerate() {
                    if let Some(point) = path.points.first() {
                        div {
                            key: "{i}",
                            style: "position: absolute; left: {point.0}px; top: {point.1}px; width: 10px; height: 10px; background: {path.color}; border-radius: 50%; transform: translate(-50%, -50%); border: 2px solid white; box-shadow: 0 2px 4px rgba(0,0,0,0.3);",
                        }
                    }
                }
            }
        }
        div {
            class: "mt-2 text-xs text-gray-500",
            "Click anywhere to draw dots on the image. Use Clear to erase."
        }
    }
}
                    
                    
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

                    

                    if let Some(image_name) = selected_image.read().as_ref() {
                        div {
                            class: "mb-4 bg-gray-100 p-4 rounded-lg",
                            div {
                                class: "flex justify-between items-center mb-2",
                                h3 { class: "text-md font-semibold", "Drawing on: {image_name}" }
                                div {
                                    class: "flex space-x-2",
                                    button {
                                        class: "bg-blue-500 text-white px-3 py-1 rounded text-sm hover:bg-blue-600",
                                        onclick: clear_canvas,
                                        "Clear"
                                    }
                                    button {
                                        class: "bg-green-500 text-white px-3 py-1 rounded text-sm hover:bg-green-600",
                                        onclick: move |_| {
                                            selected_image.set(None);
                                            drawing_paths.set(Vec::new());
                                            current_path.set(Vec::new());
                                        },
                                        "Save"
                                    }
                                    button {
                                        class: "text-red-500 hover:text-red-700 px-2",
                                        onclick: move |_| {
                                            selected_image.set(None);
                                            drawing_paths.set(Vec::new());
                                            current_path.set(Vec::new());
                                        },
                                        "✕ Close"
                                    }
                                }
                            }
                            div {
                                class: "bg-white rounded-lg border-2 border-gray-300 p-4",
                                style: "width: 100%; height: 300px;",
                                
                                div {
                                    style: "width: 100%; height: 100%; background-image: url('{IMAGE}'); background-size: cover; background-position: center; background-repeat: no-repeat; cursor: crosshair; border-radius: 8px; position: relative;",
                                    onclick: move |evt: Event<MouseData>| {
                                        let coords = evt.element_coordinates();
                                        let drawing_path = DrawingPath {
                                            points: vec![(coords.x as f32, coords.y as f32), (coords.x as f32 + 1.0, coords.y as f32 + 1.0)],
                                            color: "#ff0000".to_string(), 
                                            width: 10.0,
                                        };
                                        let mut paths = drawing_paths.write();
                                        paths.push(drawing_path);
                                    },
                                    
                                    for (i, path) in drawing_paths.read().iter().enumerate() {
                                        if let Some(point) = path.points.first() {
                                            div {
                                                key: "{i}",
                                                style: "position: absolute; left: {point.0}px; top: {point.1}px; width: 10px; height: 10px; background: {path.color}; border-radius: 50%; transform: translate(-50%, -50%); border: 2px solid white; box-shadow: 0 2px 4px rgba(0,0,0,0.3);",
                                            }
                                        }
                                    }
                                }
                            }
                            div {
                                class: "mt-2 text-xs text-gray-500",
                                "Click anywhere to draw red dots on the image. Use Clear to erase all dots."
                            }
                        }
                    }

                    
                    if let Some(file_name) = selected_file.read().as_ref() {
                        div {
                            class: "mb-4 bg-gray-100 p-4 rounded-lg",
                            div {
                                class: "flex justify-between items-center mb-2",
                                h3 { class: "text-md font-semibold", "Editing: {file_name}" }
                                button {
                                    class: "text-red-500 hover:text-red-700 px-2",
                                    onclick: move |_| selected_file.set(None),
                                    "✕ Close"
                                }
                            }
                            div {
                                class: "bg-white rounded-lg border-2 border-gray-300 p-4",
                                style: "width: 100%; height: 300px;",
                                textarea {
                                    class: "w-full h-full resize-none outline-none text-sm",
                                    placeholder: "Type your content here...",
                                    "This is a dummy text file. You can edit this content."
                                }
                            }
                            div {
                                class: "mt-2 text-xs text-gray-500",
                                "Edit your document here. Changes are saved automatically."
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
                            on_canvas_click: open_canvas,
                            on_file_click: open_file,
                            icon: match item.file_type {
                                FileType::Photo => PHOTO,
                                FileType::File => FILE,
                                FileType::Folder => MAIN_FOLDER,
                                FileType::Video => VIDEO,
                                FileType::PDF => PDF,
                                FileType::Canvas => CANVAS,
                            },
                        }
                    }
                }
            }
        }
    }
}
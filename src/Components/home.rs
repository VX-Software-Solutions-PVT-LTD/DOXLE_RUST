use dioxus::prelude::*;
use crate::Components::file_types::{FileItem, FileType};
use crate::Components::file_item::FileItemComponent;
use crate::Components::footer::Footer;

// Conditional imports based on target
#[cfg(not(target_arch = "wasm32"))]
use skia_safe::{Canvas as SkiaCanvas, Paint, Path, Color, Surface, EncodedImageFormat};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

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
    
    // Canvas drawing state
    let mut drawing_paths = use_signal(|| Vec::<DrawingPath>::new());
    let mut is_drawing = use_signal(|| false);
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
        FileItem::new(9, "random.svg".to_string(), FileType::Photo),
        FileItem::new_with_parent(10, "random.svg".to_string(), FileType::Photo, 3),
        FileItem::new(11, "My Drawing.canvas".to_string(), FileType::Canvas),
    ]);

    // Skia-based drawing functions (for desktop)
    #[cfg(not(target_arch = "wasm32"))]
    let render_with_skia = move |paths: &Vec<DrawingPath>| -> Option<String> {
        let mut surface = Surface::new_raster_n32_premul((350, 280))?;
        let canvas = surface.canvas();
        
        // Clear background
        canvas.clear(Color::WHITE);
        
        // Draw paths
        for drawing_path in paths {
            let mut paint = Paint::default();
            paint.set_color(Color::from_argb(255, 16, 185, 129)); // Green color
            paint.set_stroke_width(drawing_path.width);
            paint.set_style(skia_safe::PaintStyle::Stroke);
            paint.set_stroke_cap(skia_safe::PaintCap::Round);
            paint.set_stroke_join(skia_safe::PaintJoin::Round);
            
            if drawing_path.points.len() > 1 {
                let mut path = Path::new();
                let first_point = drawing_path.points[0];
                path.move_to((first_point.0, first_point.1));
                
                for point in &drawing_path.points[1..] {
                    path.line_to((point.0, point.1));
                }
                
                canvas.draw_path(&path, &paint);
            }
        }
        
        // Convert to base64 for web display
        surface.image_snapshot()
            .encode(None, EncodedImageFormat::PNG, None)
            .map(|data| format!("data:image/png;base64,{}", base64::encode(data.as_bytes())))
    };

    // Drawing event handlers - Fixed coordinate handling
let start_drawing = move |evt: Event<MouseData>| {
    evt.stop_propagation();
    is_drawing.set(true);
    
    // Get coordinates relative to the SVG viewBox
    let coords = evt.element_coordinates();
    let x = coords.x as f32 * 350.0 / 100.0; // Scale to viewBox width
    let y = coords.y as f32 * 280.0 / 100.0; // Scale to viewBox height
    
    current_path.set(vec![(x, y)]);
};

let continue_drawing = move |evt: Event<MouseData>| {
    if *is_drawing.read() {
        evt.stop_propagation();
        
        // Get coordinates relative to the SVG viewBox
        let coords = evt.element_coordinates();
        let x = coords.x as f32 * 350.0 / 100.0;
        let y = coords.y as f32 * 280.0 / 100.0;
        
        let mut path = current_path.write();
        path.push((x, y));
    }
};

let stop_drawing = move |evt: Event<MouseData>| {
    if *is_drawing.read() {
        evt.stop_propagation();
        is_drawing.set(false);
        let path = current_path.read().clone();
        if path.len() > 1 {
            let drawing_path = DrawingPath {
                points: path,
                color: "#10B981".to_string(), // Green for completed paths
                width: 3.0,
            };
            let mut paths = drawing_paths.write();
            paths.push(drawing_path);
            current_path.set(Vec::new());
        }
    }
};

    // Rest of your existing functions (delete_item, open_folder, etc.)
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
    };

    let play_video = move |name: String| {
        current_folder.set(None);
        selected_video.set(Some(name));
        selected_pdf.set(None);
        selected_canvas.set(None);
    };

    let open_pdf = move |name: String| {
        if current_folder.read().is_none() {
            selected_pdf.set(Some(name));
            selected_video.set(None);
            selected_canvas.set(None);
        }
    };

    let open_canvas = move |name: String| {
        if current_folder.read().is_none() {
            selected_canvas.set(Some(name));
            selected_video.set(None);
            selected_pdf.set(None);
            selected_image.set(None);
            // Reset drawing state
            drawing_paths.set(Vec::new());
            current_path.set(Vec::new());
            canvas_key.set(canvas_key() + 1);
        }
    };

    let clear_canvas = move |_| {
        drawing_paths.set(Vec::new());
        current_path.set(Vec::new());
        canvas_key.set(canvas_key() + 1);
    };

    let save_canvas = move |_| {
        // In a real app, you'd save the drawing_paths data here
        selected_canvas.set(None);
        drawing_paths.set(Vec::new());
        current_path.set(Vec::new());
    };

    // Filter items and other logic (same as before)
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
            selected_image.set(Some(format!("/assets/{}", name)));
            selected_video.set(None);
            selected_pdf.set(None);
            selected_canvas.set(None);
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

    let add_canvas = move |_:Event<MouseData>| {
        let mut items = file_items.write();
        let new_id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
        items.push(FileItem::new(new_id, format!("New Canvas {}", new_id), FileType::Canvas));
    };

    rsx! {
        div {
            class: "min-h-screen bg-gray-50",
            style: "padding-top: 72px;",
            div {
                class: "max-w-md mx-auto bg-white min-h-screen",
                
                // Search bar
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
                
                // Header with buttons
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
                                    selected_canvas.set(None);
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
                            button {
                                style: "width:79px;height:36px;border-radius:4px;border-width:1px;background:#F2F3FE;border:1px solid #EAECEF;opacity:1;display:flex;align-items:center;justify-content:center;",
                                onclick: add_canvas,
                                img {
                                    src: "{CANVAS}",
                                    style: "width:39px;height:17px;border-radius:1px;opacity:1;",
                                }
                            }
                        }
                    }
                }

                div {
                    class: "px-4",

// Simple working canvas - replace the entire canvas interface
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
            
            // Simple drawing area with click feedback
            div {
                style: "width: 100%; height: 100%; background: linear-gradient(45deg, #f0f0f0 25%, transparent 25%), linear-gradient(-45deg, #f0f0f0 25%, transparent 25%), linear-gradient(45deg, transparent 75%, #f0f0f0 75%), linear-gradient(-45deg, transparent 75%, #f0f0f0 75%); background-size: 20px 20px; background-position: 0 0, 0 10px, 10px -10px, -10px 0px; cursor: crosshair; border-radius: 8px; position: relative;",
                onclick: move |evt: Event<MouseData>| {
                    // Simple click drawing - add a dot where clicked
                    let coords = evt.element_coordinates();
                    let drawing_path = DrawingPath {
                        points: vec![(coords.x as f32, coords.y as f32), (coords.x as f32 + 1.0, coords.y as f32 + 1.0)],
                        color: "#ef4444".to_string(),
                        width: 8.0,
                    };
                    let mut paths = drawing_paths.write();
                    paths.push(drawing_path);
                },
                
                // Show dots where user has clicked
                for (i, path) in drawing_paths.read().iter().enumerate() {
                    if let Some(point) = path.points.first() {
                        div {
                            key: "{i}",
                            style: "position: absolute; left: {point.0}px; top: {point.1}px; width: 8px; height: 8px; background: {path.color}; border-radius: 50%; transform: translate(-50%, -50%);",
                        }
                    }
                }
            }
        }
        div {
            class: "mt-2 text-xs text-gray-500",
            "Click anywhere to draw dots. Use Clear to erase."
        }
    }
}
                    
                    // Video, PDF, and Image viewers (same as before)
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

                    // File list
                    for item in filtered_items {
                        FileItemComponent {
                            item: item.clone(),
                            on_delete: delete_item,
                            on_folder_click: open_folder,
                            on_video_click: play_video,
                            on_pdf_click: open_pdf,
                            on_image_click: preview_image,
                            on_canvas_click: open_canvas,
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
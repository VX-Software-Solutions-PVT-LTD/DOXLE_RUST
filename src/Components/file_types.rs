#[derive(Clone, PartialEq, Debug)]
pub enum FileType {
    Photo,
    File,
    Folder,
    Video,
    PDF,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FileItem {
    pub id: u32,
    pub name: String,
    pub file_type: FileType,
    pub parent_id: Option<u32>,
     pub children: Vec<u32>,
}

impl FileItem {
    pub fn new(id: u32, name: String, file_type: FileType) -> Self {
        Self { id, name, file_type, parent_id: None, children: Vec::new() }
    }
    
pub fn new_with_parent(id: u32, name: String, file_type: FileType, parent_id: u32) -> Self {
        Self { 
            id, 
            name, 
            file_type,
            parent_id: Some(parent_id),
            children: Vec::new(),
        }
    }
}
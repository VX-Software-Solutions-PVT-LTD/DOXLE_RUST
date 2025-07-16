#[derive(Clone, PartialEq, Debug)]
pub enum FileType {
    Photo,
    File,
    Folder,
}

#[derive(Clone, PartialEq, Debug)]
pub struct FileItem {
    pub id: u32,
    pub name: String,
    pub file_type: FileType,
}

impl FileItem {
    pub fn new(id: u32, name: String, file_type: FileType) -> Self {
        Self { id, name, file_type }
    }
}
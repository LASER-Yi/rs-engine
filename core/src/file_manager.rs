use std::{error::Error, fs};

use self::blob::FileBlob;

pub mod blob;

pub struct FileManager {}

impl FileManager {
    pub fn read_file(file_path: &str) -> Result<FileBlob, Box<dyn Error>> {
        let contents = fs::read(file_path)?;

        Ok(FileBlob::new(contents))
    }
}

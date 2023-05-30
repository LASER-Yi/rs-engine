use std::usize;

pub struct FileBlob {
    blob: Vec<u8>,
}

impl FileBlob {
    pub fn new(in_blob: Vec<u8>) -> Self {
        FileBlob { blob: in_blob }
    }

    pub fn get_size(&self) -> usize {
        self.blob.len()
    }

    pub fn get_data(&self) -> &[u8] {
        &self.blob
    }
}

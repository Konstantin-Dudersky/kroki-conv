use std::path::PathBuf;

use bytes::Bytes;

pub struct DiagramOutput {
    pub path: PathBuf,
    pub content: Bytes,
}

impl DiagramOutput {
    pub fn new(path: PathBuf, content: Bytes) -> Self {
        Self { path, content }
    }
}

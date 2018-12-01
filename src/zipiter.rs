use std::fs::File;
use std::io::Read;
use std::io::Seek;
use zip::read::{ZipArchive, ZipFile};

pub struct ZipIter<T: Read + Seek> {
    zip: ZipArchive<T>,
    index: usize,
}

impl ZipIter<File> {
    pub fn new(zip: ZipArchive<File>) -> Self {
        Self { zip, index: 0 }
    }
}

impl Iterator for ZipIter<File> {
    type Item = Box<(String, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.zip.len() {
            let mut f = self.zip.by_index(self.index).unwrap();
            self.index += 1;
            let mut buf: Vec<u8> = Vec::with_capacity(f.size() as usize);
            f.read_to_end(&mut buf).unwrap();
            Some(Box::new((f.name().to_string(), buf)))
        } else {
            None
        }
    }
}

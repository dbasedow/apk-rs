use std::io::{self, Read, Seek};
use std::fs::File;
use std::io::SeekFrom;
use std::rc::Rc;

#[derive(Debug,Clone)]
pub enum ReaderWrapper {
    FileReader(FileReader),
}

impl Open for ReaderWrapper {
    fn do_open(&mut self) -> io::Result<()> {
        match self {
            ReaderWrapper::FileReader(r) => r.do_open(),
        }
    }
}

impl Read for ReaderWrapper {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            ReaderWrapper::FileReader(r) => r.read(buf),
        }
    }
}

impl Seek for ReaderWrapper {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match self {
            ReaderWrapper::FileReader(r) => r.seek(pos),
        }
    }
}


pub trait Open {
    fn do_open(&mut self) -> io::Result<()>;
}

#[derive(Debug)]
pub struct FileReader {
    path: Rc<String>,
    file: Option<File>,
}

impl FileReader {
    pub fn open(path: &str) -> io::Result<FileReader> {
        let file = File::open(path)?;

        Ok(FileReader {
            path: Rc::new(path.into()),
            file: Some(file),
        })
    }
}

impl Open for FileReader {
    fn do_open(&mut self) -> io::Result<()> {
        match self.file {
            None => self.file = Some(File::open(self.path.as_ref())?),
            _ => {}
        }
        Ok(())
    }
}

impl Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.file.is_none() {
            self.do_open()?;
        }
        self.file.as_ref().unwrap().read(buf)
    }
}

impl Seek for FileReader {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        if self.file.is_none() {
            self.do_open()?;
        }
        self.file.as_ref().unwrap().seek(pos)
    }
}

impl Clone for FileReader {
    fn clone(&self) -> FileReader {
        FileReader {
            path: self.path.clone(),
            file: None,
        }
    }
}

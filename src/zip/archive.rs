use std::io::{self, Read, Seek};
use std::fs::File;
use std::io::SeekFrom;
use std::rc::Rc;
use crate::zip::io::{Open, FileReader};
use crate::zip::parser;
use nom::*;
use crate::zip::parser::CentralDirectoryFileHeader;
use std::borrow::Cow;
use flate2::read::DeflateDecoder;
use crate::zip::io::ReaderWrapper;

pub enum Compression {
    Store,
    Deflate,
    Bzip2,
    LZMA,
}

impl From<u16> for Compression {
    fn from(c: u16) -> Compression {
        match c {
            0 => Compression::Store,
            8 => Compression::Deflate,
            12 => Compression::Bzip2,
            14 => Compression::LZMA,
            _ => unimplemented!(),
        }
    }
}

fn get_range_of_central_directory<R: Read + Seek>(data: &mut R) -> io::Result<(usize, usize)> {
    let seek_offset = data.seek(SeekFrom::End(-1024))? as usize;
    let mut buf = vec![0; 1024];
    data.read_exact(&mut buf)?;
    for (offset, _) in buf.windows(4).enumerate().filter(|(_, w)| w == &[0x50, 0x4b, 0x05, 0x06]).rev() {
        let comment_len_pos = offset + 20;
        if comment_len_pos > buf.len() {
            continue;
        }
        let comment_len = buf[comment_len_pos] as usize + (buf[comment_len_pos + 1] as usize) << 8;
        if offset + 22 + comment_len != 1024 {
            continue;
        }

        let offset_offset = offset + 16;
        let mut cd_offset = buf[offset_offset] as usize;
        cd_offset += (buf[offset_offset + 1] as usize) << 8;
        cd_offset += (buf[offset_offset + 2] as usize) << 16;
        cd_offset += (buf[offset_offset + 3] as usize) << 24;

        let size_offset = offset + 12;
        let mut cd_size = buf[size_offset] as usize;
        cd_size += (buf[size_offset + 1] as usize) << 8;
        cd_size += (buf[size_offset + 2] as usize) << 16;
        cd_size += (buf[size_offset + 3] as usize) << 24;

        return Ok((cd_offset, cd_size));
    }

    Err(io::Error::new(io::ErrorKind::Other, "end of central directory signature not found"))
}

pub struct ZipArchive {
    reader: ReaderWrapper,
    entries: Rc<Vec<CentralDirectoryFileHeader>>,
}

#[derive(Debug)]
pub struct ZipEntry {
    reader: ReaderWrapper,
    pub header: CentralDirectoryFileHeader,
}

impl ZipEntry {
    pub fn file_name(&self) -> String {
        self.header.file_name()
    }

    pub fn compression(&self) -> Compression {
        self.header.compression_method.into()
    }

    pub fn len(&self) -> usize {
        self.header.uncompressed_size as usize
    }

    pub fn content(&self) -> io::Result<Box<Read>> {
        let mut r = self.reader.clone();
        r.seek(SeekFrom::Start(self.header.relative_offset_of_local_header as u64))?;
        let mut header_buf = vec![0; 30];
        r.read_exact(&mut header_buf)?;
        if let IResult::Done(_, (file_name_len, extra_field_len)) = parser::parse_local_file_header(&header_buf) {
            r.seek(SeekFrom::Current(file_name_len + extra_field_len))?;
        }
        if self.header.compression_method == 8 {
            return Ok(Box::new(DeflateDecoder::new(r.take(self.header.compressed_size as u64))));
        }
        Ok(Box::new(r.take(self.header.compressed_size as u64)))
    }
}

impl ZipArchive {
    pub fn open(path: &str) -> io::Result<ZipArchive> {
        let mut reader = ReaderWrapper::FileReader(FileReader::open(path)?);
        let (offset, size) = get_range_of_central_directory(&mut reader)?;

        reader.seek(SeekFrom::Start(offset as u64))?;

        let mut buf = vec![0; size];
        reader.read_exact(&mut buf)?;
        let entries: Vec<CentralDirectoryFileHeader>;
        if let IResult::Done(foo, res) = parser::parse_central_directory(&buf) {
            entries = res;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "error parsing central directory"));
        }

        Ok(ZipArchive {
            reader,
            entries: Rc::new(entries),
        })
    }

    pub fn by_name(&self, name: &str) -> io::Result<Option<ZipEntry>> {
        for entry in self.entries.iter() {
            if entry.file_name() == name {
                return Ok(Some(ZipEntry {
                    reader: self.reader.clone(),
                    header: entry.clone(),
                }));
            }
        }
        Ok(None)
    }

    pub fn files(&self) -> ZipIter {
        ZipIter {
            reader: self.reader.clone(),
            entries: self.entries.clone(),
            index: 0,
        }
    }
}

pub struct ZipIter {
    reader: ReaderWrapper,
    entries: Rc<Vec<CentralDirectoryFileHeader>>,
    index: usize,
}

impl Iterator for ZipIter {
    type Item = ZipEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entries.len() {
            let res = Some(ZipEntry {
                reader: self.reader.clone(),
                header: self.entries[self.index].clone(),
            });
            self.index += 1;
            return res;
        }
        None
    }
}
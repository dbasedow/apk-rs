use zip;
use nom::IResult;
use crate::resources::resources::Resources;
use std::io::{self, Read};
use std::fs::File;
use crate::resources::resources::parse_resource_table;
use zip::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipResult;
use std::io::Seek;

pub struct ApkFileIter<'a> {
    files: &'a Vec<ApkFile>,
    index: usize,
}

impl<'a> ApkFileIter<'a> {
    pub fn new<'b>(files: &'b Vec<ApkFile>) -> ApkFileIter<'b> {
        ApkFileIter { files, index: 0 }
    }
}

impl<'a> Iterator for ApkFileIter<'a> {
    type Item = ApkFile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.files.len() {
            let f = self.files[self.index].clone();
            self.index += 1;
            Some(f)
        } else {
            None
        }
    }
}


pub struct Apk {
    zip_archive: zip::ZipArchive<File>,
    resources: Option<Resources>,
    files: Vec<ApkFile>,
}

#[derive(Clone, Debug)]
pub struct ApkFile {
    pub name: String,
    pub size: usize,
    pub compressed_size: usize,
    index: usize,
}

impl ApkFile {
    pub fn from_zip_file(z: ZipFile, index: usize) -> ApkFile {
        ApkFile {
            name: z.name().into(),
            size: z.size() as usize,
            compressed_size: z.compressed_size() as usize,
            index,
        }
    }
}

impl Apk {
    pub fn open(path: &str) -> io::Result<Apk> {
        let file = File::open(path)?;
        let mut zip_archive = zip::ZipArchive::new(file)?;
        let mut resources = None;
        {
            let mut res_file = zip_archive.by_name("resources.arsc")?;
            let mut buf = Vec::with_capacity(res_file.size() as usize);
            res_file.read_to_end(&mut buf)?;
            let r = parse_resource_table(&buf);
            if let IResult::Done(_, r) = r {
                resources = r;
            }
        }
        let mut files = Vec::with_capacity(zip_archive.len());
        for i in 0..zip_archive.len() {
            let f = zip_archive.by_index(i).unwrap();
            let apkfile = ApkFile::from_zip_file(f, i);
            files.push(apkfile);
        }

        Ok(Apk {
            zip_archive,
            resources,
            files,
        })
    }

    pub fn files(&self) -> ApkFileIter {
        ApkFileIter::new(&self.files)
    }
}
use nom::IResult;
use crate::resources::resources::Resources;
use std::io::{self, Read};
use std::fs::File;
use crate::resources::resources::parse_resource_table;
use std::io::Seek;
use crate::zip::archive::ZipEntry;
use crate::zip::archive::ZipArchive;

pub struct Apk {
    zip_archive: ZipArchive,
    resources: Option<Resources>,
}

#[derive(Debug)]
pub struct ApkFile(ZipEntry);

impl ApkFile {
    pub fn from_zip_file(z: ZipEntry) -> ApkFile {
        ApkFile(z)
    }
}

impl Apk {
    pub fn open(path: &str) -> io::Result<Apk> {
        let zip_archive = ZipArchive::open(path)?;
        let mut resources = None;
        {
            if let Some(res_file) = zip_archive.by_name("resources.arsc")? {
                let mut buf = Vec::with_capacity(res_file.len());
                let mut reader = res_file.content()?;
                reader.read_to_end(&mut buf)?;
                let r = parse_resource_table(&buf);
                if let IResult::Done(_, r) = r {
                    resources = r;
                }
            }
        }

        Ok(Apk {
            zip_archive,
            resources,
        })
    }

}
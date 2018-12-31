use nom::IResult;
use crate::resources::resources::Resources;
use std::io::{self, Read};
use std::fs::File;
use crate::resources::resources::parse_resource_table;
use std::io::Seek;
use crate::zip::archive::ZipEntry;
use crate::zip::archive::ZipArchive;
use std::iter::Map;
use crate::zip::archive::ZipIter;
use crate::signature::get_key_fingerprint_sha256;

pub struct Apk {
    zip_archive: ZipArchive,
    resources: Option<Resources>,
}

pub struct ApkFile(ZipEntry);

impl ApkFile {
    pub fn name(&self) -> String {
        self.0.header.file_name()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn compressed_len(&self) -> usize {
        self.0.header.compressed_size as usize
    }

    pub fn content(&self) -> io::Result<Box<Read>> {
        self.0.content()
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

    pub fn files(&self) -> ApkIter {
        ApkIter(self.zip_archive.files())
    }

    pub fn file_by_name(&self, name: &str) -> io::Result<Option<ApkFile>> {
        if let Some(f) = self.zip_archive.by_name(name)? {
            return Ok(Some(ApkFile(f)));
        }
        Ok(None)
    }

    pub fn get_resources(&self) -> Option<&Resources> {
        self.resources.as_ref()
    }

    pub fn get_certificate_fingerprint_sha256(&self) -> io::Result<Vec<u8>> {
        if let Some(cert_file) = self.file_by_name("META-INF/CERT.RSA")? {
            let mut reader = cert_file.content()?;
            let mut buf = Vec::with_capacity(cert_file.len());
            reader.read_to_end(&mut buf)?;

            let fingerprint = get_key_fingerprint_sha256(&buf);
            if fingerprint.is_err() {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "unable to parse certificate"));
            }
            return Ok(fingerprint.unwrap());
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "CERT.RSA not found"))
    }
}

pub struct ApkIter(ZipIter);

impl Iterator for ApkIter {
    type Item = ApkFile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.0.next() {
            return Some(ApkFile(f));
        }
        None
    }
}
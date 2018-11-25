extern crate zip;
#[macro_use]
extern crate nom;
use crate::parser::{handle_xml_file, is_binary_xml};
use std::env;
use std::fs::File;
use std::io::Read;
use zip::result::ZipError;

fn main() {
    let apk = env::args().last().unwrap();
    let file = File::open(apk).unwrap();

    let zip = zip::ZipArchive::new(file).unwrap();

    let ziter = zipiter::ZipIter::new(zip);

    for b in ziter
        .filter(|b| b.0.ends_with(".xml"))
        .filter(|b| is_binary_xml(&b.1[..]))
    {
        handle_xml_file(&b.1);
    }
}

fn extract_xml_by_name(apk: &str, name: &str) -> zip::result::ZipResult<Box<Vec<u8>>> {
    let file = File::open(apk)?;

    let mut zip = zip::ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        if file.name() == name {
            let mut buf: Vec<u8> = Vec::with_capacity(file.size() as usize);
            file.read_to_end(&mut buf)?;
            return Ok(Box::new(buf));
        }
    }

    Err(ZipError::FileNotFound)
}

mod parser;
mod stringpool;
mod zipiter;
mod chunk;
mod typedvalue;
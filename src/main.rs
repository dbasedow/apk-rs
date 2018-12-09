extern crate zip;
#[macro_use]
extern crate nom;

use crate::parser::{is_binary_xml, XmlElementStream, XmlEvent};
use crate::resources::resources::parse_resource_table;
use std::env;
use nom::IResult;
use std::fs::File;
use std::io::Read;
use zip::result::ZipError;

fn main() {
    let apk = env::args().last().unwrap();
    let file = File::open(apk).unwrap();

    let zip = zip::ZipArchive::new(file).unwrap();

    let ziter = zipiter::ZipIter::new(zip);

    for b in ziter.filter(|b| b.0.ends_with(".arsc")) {
        println!("{}", b.0);
        let resources = parse_resource_table(&b.1);
        if let IResult::Done(_, Some(resources)) = resources {
            let id = 0x7f070000;
            println!("{:?} {:?}", resources.get_key_name(id), resources.get_resource_type(id));
        }
        panic!("done {}");
    }

    /*
    for b in ziter
        .filter(|b| b.0.ends_with(".xml"))
        .filter(|b| is_binary_xml(&b.1[..]))
    {
        render_plain(&b.1);
    }
    */
}

fn render_plain(data: &[u8]) {
    if let Ok(it) = XmlElementStream::new(data) {
        let mut indent = 0;
        for e in it {
            match e {
                XmlEvent::ElementStart(e) => {
                    indent_ouput(indent);
                    print!("<{}", e.name);
                    if e.attribute_len() > 0 {
                        for a in e.attributes.unwrap() {
                            print!(" {}=\"{}\"", a.name, a.value.to_string());
                        }
                    }
                    println!(" />");
                    indent += 1;
                }
                XmlEvent::ElementEnd(e) => {
                    indent -= 1;
                    indent_ouput(indent);
                    println!("</{}>", e.name);
                }
                _ => {}
            }
        }
    }
}

fn indent_ouput(level: u32) {
    for _ in 0..level {
        print!("  ");
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

mod chunk;
mod parser;
mod stringpool;
mod typedvalue;
mod zipiter;
pub mod resources;

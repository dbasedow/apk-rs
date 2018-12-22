#[macro_use]
extern crate nom;

use crate::parser::{is_binary_xml, XmlElementStream, XmlEvent};
use crate::resources::resources::parse_resource_table;
use std::env;
use nom::IResult;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use crate::resources::resources::Resources;
use crate::typedvalue::TypedValue;
use crate::resources::resources::is_package_reference;

fn main() -> Result<(), Box<std::error::Error>> {
    let apk_path = env::args().last().unwrap();
    let apk = apk::Apk::open(&apk_path)?;
    for f in apk.files() {
        println!("{}: {}/{}", f.name(), f.len(), f.compressed_len());
        if f.name() == "AndroidManifest.xml" {
            let mut buf = Vec::with_capacity(f.len());
            let mut rdr = f.content()?;
            rdr.read_to_end(&mut buf)?;
            let res = apk.get_resources().unwrap();

            render_plain(&buf, res);
            return Ok(());
        }
    }

    Ok(())
}

fn render_plain(data: &[u8], resources: &Resources) {
    if let Ok(it) = XmlElementStream::new(data) {
        let mut indent = 0;
        for e in it {
            match e {
                XmlEvent::ElementStart(e) => {
                    indent_ouput(indent);
                    print!("<{}", e.name);
                    if e.attribute_len() > 0 {
                        for a in e.attributes.unwrap() {
                            let foo = if a.value.is_reference_type() {
                                match a.value {
                                    TypedValue::Reference(r) if is_package_reference(r) => resources.get_human_reference(r).unwrap(),
                                    TypedValue::Reference(_) => a.value.to_string(),
                                    _ => "".to_string(),
                                }
                            } else {
                                a.value.to_string()
                            };
                            print!(" {}=\"{}\"", a.name, foo);
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

mod apk;
mod chunk;
mod parser;
mod stringpool;
mod typedvalue;
mod zip;
pub mod resources;

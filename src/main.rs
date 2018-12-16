#[macro_use]
extern crate nom;

use crate::parser::{is_binary_xml, XmlElementStream, XmlEvent};
use crate::resources::resources::parse_resource_table;
use std::env;
use nom::IResult;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() -> Result<(), Box<std::error::Error>> {
    let apk_path = env::args().last().unwrap();
    let apk = apk::Apk::open(&apk_path)?;

    Ok(())
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

mod apk;
mod chunk;
mod parser;
mod stringpool;
mod typedvalue;
mod zip;
pub mod resources;

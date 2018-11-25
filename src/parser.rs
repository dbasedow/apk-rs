use crate::chunk::*;
use crate::stringpool::{parse_string_pool_chunk, StringPool};
use crate::typedvalue::TypedValue;
use nom::IResult;

pub fn is_binary_xml(data: &[u8]) -> bool {
    data[0] == 0x03 && data[1] == 0x00
}

//TODO: return iterator
//TODO: take Read
pub fn handle_xml_file(data: &Vec<u8>) {
    let r = parse_chunk(&data);
    let r = r.unwrap();
    if let IResult::Done(_, s) = r.1.get_sub_chunks() {
        let mut string_pool: Option<StringPool> = None;

        for chunk in s.iter() {
            if let IResult::Done(_, meta) = parse_xml_chunk_header(chunk.additional_header) {
                match chunk.typ {
                    0x100 => {
                        if let IResult::Done(_, ns) = parse_namespace_body(chunk.data) {
                            let st = Namespace::from(&ns, &meta, &string_pool.as_ref().unwrap());
                        }
                    }
                    0x101 => {
                        if let IResult::Done(_, ns) = parse_namespace_body(chunk.data) {
                            let st = Namespace::from(&ns, &meta, &string_pool.as_ref().unwrap());
                        }
                    }
                    0x102 => {
                        if let IResult::Done(_, tag) = parse_start_element_chunk(chunk.data) {
                            let st =
                                ElementStart::from(&tag, &meta, &string_pool.as_ref().unwrap());
                        }
                    }
                    0x103 => {
                        if let IResult::Done(_, tag) = parse_end_element_chunk(chunk.data) {
                            let st = ElementEnd::from(&tag, &meta, &string_pool.as_ref().unwrap());
                        }
                    }
                    0x104 => {
                        if let IResult::Done(_, tag) = parse_cdata_chunk(chunk.data) {
                            let st = CData::from(&tag, &meta, &string_pool.as_ref().unwrap());
                        }
                    }
                    0x001 => {
                        if let Ok(sp) = parse_string_pool_chunk(chunk) {
                            string_pool = Some(sp)
                        }
                    }
                    t => println!("0x{:X}", t),
                }
            }
        }
    }
}

enum ParseError {
    WrongChunkType,
}

enum XmlEvent {
    NamespaceStart(Namespace),
    NamespaceEnd(Namespace),
    ElementStart(ElementStart),
    ElementEnd(ElementEnd),
}

#[derive(Debug)]
struct Namespace {
    line_number: u32,
    comment: Option<String>,
    prefix: String,
    uri: String,
}

impl Namespace {
    fn from(chunk: &NamespaceChunk, meta: &XmlChunkHeader, sp: &StringPool) -> Self {
        Self {
            line_number: meta.line_number,
            comment: sp.get_optional(meta.comment),
            prefix: sp.get(chunk.prefix),
            uri: sp.get(chunk.uri),
        }
    }
}

#[derive(Debug)]
struct ElementStart {
    line_number: u32,
    comment: Option<String>,
    ns: Option<String>,
    name: String,
    attributes: Option<Vec<Attribute>>,
}

impl ElementStart {
    fn from(chunk: &XmlStartChunk, meta: &XmlChunkHeader, sp: &StringPool) -> Self {
        let ns = sp.get_optional(chunk.ns);
        let attributes = if chunk.attributes.len() > 0 {
            Some(
                (&chunk.attributes)
                    .into_iter()
                    .map(|raw| Attribute::from(&raw, sp))
                    .collect(),
            )
        } else {
            None
        };

        Self {
            line_number: meta.line_number,
            comment: sp.get_optional(meta.comment),
            ns,
            name: sp.get(chunk.name),
            attributes,
        }
    }

    fn attribute_len(&self) -> usize {
        if let Some(ref attrs) = self.attributes {
            attrs.len()
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct Attribute {
    ns: Option<String>,
    name: String,
    value: TypedValue,
}

impl Attribute {
    fn from(raw: &RawAttribute, strings: &StringPool) -> Self {
        Self {
            ns: strings.get_optional(raw.ns),
            name: strings.get(raw.name),
            value: TypedValue::from(raw.typed_value, strings),
        }
    }
}

#[derive(Debug)]
struct ElementEnd {
    line_number: u32,
    comment: Option<String>,
    ns: Option<String>,
    name: String,
}

impl ElementEnd {
    fn from(chunk: &XmlEndNode, meta: &XmlChunkHeader, strings: &StringPool) -> Self {
        Self {
            line_number: meta.line_number,
            comment: strings.get_optional(meta.comment),
            ns: strings.get_optional(chunk.ns),
            name: strings.get(chunk.name),
        }
    }
}

struct CData {
    line_number: u32,
    comment: Option<String>,
    data: String,
}

impl CData {
    fn from(chunk: &CdataChunk, meta: &XmlChunkHeader, strings: &StringPool) -> Self {
        Self {
            line_number: meta.line_number,
            comment: strings.get_optional(meta.comment),
            data: strings.get(chunk.data),
        }
    }
}

use crate::stringpool::{parse_string_pool_chunk, StringPool};
use nom::*;
use std::mem;

pub fn is_binary_xml(data: &[u8]) -> bool {
    data[0] == 0x03 && data[1] == 0x00
}

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
                        if let IResult::Done(_, tag) = parse_start_tag_body(chunk.data) {
                            let st =
                                ElementStart::from(&tag, &meta, &string_pool.as_ref().unwrap());
                        }
                    }
                    0x103 => {
                        if let IResult::Done(_, tag) = parse_end_tag_body(chunk.data) {
                            let st = ElementEnd::from(&tag, &meta, &string_pool.as_ref().unwrap());
                        }
                    }
                    //TODO: Add CDATA 0x104
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

#[derive(Debug)]
pub struct Chunk<'a> {
    pub typ: u16,
    pub additional_header: &'a [u8],
    pub data: &'a [u8],
}

impl<'a> Chunk<'a> {
    fn get_sub_chunks(&self) -> IResult<&[u8], Vec<Chunk>> {
        do_parse!(self.data, res: many0!(parse_chunk) >> (res))
    }
}

enum ParseError {
    WrongChunkType,
}

named!(parse_chunk<&[u8], Chunk>, do_parse!(
        typ: le_u16
            >> header_size: le_u16
            >> chunk_size: le_u32
            >> additional_header: take!((header_size - 8))
            >> data: take!((chunk_size - header_size as u32))
            >> (Chunk {
                typ,
                additional_header,
                data
            })

));

enum XmlEvent {
    NamespaceStart(Namespace),
    NamespaceEnd(Namespace),
    ElementStart(ElementStart),
    ElementEnd(ElementEnd),
}

#[derive(Debug)]
struct XmlChunkHeader {
    line_number: u32,
    comment: u32,
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
struct NamespaceChunk {
    prefix: u32,
    uri: u32,
}

named!(parse_xml_chunk_header<&[u8], XmlChunkHeader>, do_parse!(
    line_number: le_u32 >>
    comment: le_u32 >>
    (XmlChunkHeader {line_number, comment})
));

named!(parse_namespace_body<&[u8], NamespaceChunk>, do_parse!(
    prefix: le_u32 >>
    uri: le_u32 >>
    (NamespaceChunk {prefix, uri})
));

struct XmlStartChunk {
    ns: u32,
    name: u32,
    attributes: Vec<RawAttribute>,
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

named!(parse_start_tag_body<&[u8], XmlStartChunk>, do_parse!(
    ns: le_u32 >>
    name: le_u32 >>
    attribute_start: le_u16 >>
    attribute_size: le_u16 >>
    attribute_count: le_u16 >>
    id_index: le_u16 >>
    class_index: le_u16 >>
    style_index: le_u16 >>
    attributes: many0!(parse_attribute) >>
    (XmlStartChunk {ns, name, attributes})
));

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
struct XmlEndNode {
    ns: u32,
    name: u32,
}

named!(parse_end_tag_body<&[u8], XmlEndNode>, do_parse!(
    ns: le_u32 >>
    name: le_u32 >>
    (XmlEndNode {ns, name})
));

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

#[derive(Debug)]
struct RawAttribute {
    ns: u32,
    name: u32,
    raw_value: u32,
    typed_value: (u8, u32),
}

named!(parse_attribute<&[u8], RawAttribute>, do_parse!(
    ns: le_u32 >>
    name: le_u32 >>
    raw_value: le_u32 >>
    typed_value: parse_typed_value >>
    (RawAttribute {ns, name, raw_value, typed_value})
));

#[derive(Debug)]
enum TypedValue {
    Reference(u32),
    Attribute(u32),
    String(String),
    Float(f32),
    Dimension(u32),
    Fraction(u32),
    Boolean(bool),
    IntDecimal(i32),
    IntHex(i32),
    Argb8(u32),
    Rgb8(u32),
    Argb4(u32),
    Rgb4(u32),
}

impl TypedValue {
    fn from(typed_value: (u8, u32), strings: &StringPool) -> TypedValue {
        match typed_value.0 {
            0x01 => TypedValue::Reference(typed_value.1),
            0x02 => TypedValue::Attribute(typed_value.1),
            0x03 => TypedValue::String(strings.get(typed_value.1)),
            0x04 => unsafe {
                let f = mem::transmute::<u32, f32>(typed_value.1);
                TypedValue::Float(f)
            },
            0x05 => TypedValue::Dimension(typed_value.1),
            0x06 => TypedValue::Fraction(typed_value.1),
            0x10 => TypedValue::IntDecimal(typed_value.1 as i32),
            0x11 => TypedValue::IntHex(typed_value.1 as i32),
            0x12 => TypedValue::Boolean(typed_value.1 == 1),
            0x1c => TypedValue::Argb8(typed_value.1),
            0x1d => TypedValue::Rgb8(typed_value.1),
            0x1e => TypedValue::Argb4(typed_value.1),
            0x1f => TypedValue::Rgb4(typed_value.1),
            t => panic!("unknown value type {}", t),
        }
    }

    fn to_string(&self) -> String {
        match self {
            TypedValue::Reference(r) => format!("@ref/0x{:x}", r),
            TypedValue::Attribute(a) => format!("@attr/0x{:x}", a),
            TypedValue::String(s) => s.clone(),
            TypedValue::Float(f) => format!("{}", f),
            TypedValue::Dimension(d) => format!("dimension({})", d),
            TypedValue::Fraction(f) => format!("fraction({})", f),
            TypedValue::IntDecimal(d) => format!("{}", d),
            TypedValue::IntHex(d) => format!("0x{:x}", d),
            TypedValue::Boolean(b) if *b => "true".to_string(),
            TypedValue::Boolean(b) if !b => "false".to_string(),
            TypedValue::Argb8(c) => format!("argb8(0x{:x})", c),
            TypedValue::Rgb8(c) => format!("rgb8(0x{:x})", c),
            TypedValue::Argb4(c) => format!("argb4(0x{:x})", c),
            TypedValue::Rgb4(c) => format!("rgb4(0x{:x})", c),
            _ => unreachable!(),
        }
    }
}

named!(parse_typed_value<&[u8], (u8, u32)>, do_parse!(
    size: le_u16 >>
    take!(1) >>
    data_type: le_u8 >>
    data: le_u32 >>
    take!(size - 8) >>
    ((data_type, data))
));

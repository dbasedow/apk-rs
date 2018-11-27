use crate::chunk::*;
use crate::stringpool::{parse_string_pool_chunk, StringPool};
use crate::typedvalue::TypedValue;
use nom::IResult;

pub fn is_binary_xml(data: &[u8]) -> bool {
    data[0] == 0x03 && data[1] == 0x00
}

pub struct XmlElementStream<'a> {
    chunks: Vec<Chunk<'a>>,
    string_pool: StringPool,
    index: usize,
}

impl<'a> XmlElementStream<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, ParseError> {
        if let IResult::Done(_, r) = parse_chunk(&data) {
            if let IResult::Done(_, s) = r.get_sub_chunks() {
                if let Ok(string_pool) = parse_string_pool_chunk(&s[0]) {
                    //TODO: actually handle res chunks
                    let index = if s[1].typ == 0x180 { 2 } else { 1 };
                    return Ok(Self {
                        chunks: s.clone(),
                        string_pool,
                        index,
                    });
                }
            }
        }

        Err(ParseError::WrongChunkType)
    }
}

impl<'a> Iterator for XmlElementStream<'a> {
    type Item = XmlEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        if self.index < self.chunks.len() {
            let chunk = &self.chunks[self.index];
            self.index += 1;
            let meta = chunk.get_additional_header();

            match chunk.typ {
                0x100 => {
                    if let IResult::Done(_, ns) = parse_namespace_body(chunk.data) {
                        result = Some(XmlEvent::NamespaceStart(Namespace::from(
                            &ns,
                            &meta.unwrap(),
                            &self.string_pool,
                        )));
                    }
                }
                0x101 => {
                    if let IResult::Done(_, ns) = parse_namespace_body(chunk.data) {
                        result = Some(XmlEvent::NamespaceEnd(Namespace::from(
                            &ns,
                            &meta.unwrap(),
                            &self.string_pool,
                        )));
                    }
                }
                0x102 => {
                    if let IResult::Done(_, tag) = parse_start_element_chunk(chunk.data) {
                        result = Some(XmlEvent::ElementStart(ElementStart::from(
                            &tag,
                            &meta.unwrap(),
                            &self.string_pool,
                        )));
                    }
                }
                0x103 => {
                    if let IResult::Done(_, tag) = parse_end_element_chunk(chunk.data) {
                        result = Some(XmlEvent::ElementEnd(ElementEnd::from(
                            &tag,
                            &meta.unwrap(),
                            &self.string_pool,
                        )));
                    }
                }
                0x104 => {
                    if let IResult::Done(_, tag) = parse_cdata_chunk(chunk.data) {
                        result = Some(XmlEvent::CData(CData::from(
                            &tag,
                            &meta.unwrap(),
                            &self.string_pool,
                        )));
                    }
                }
                0x180 => {
                    println!("TODO: implement chunk type 0x180");
                }
                t => unreachable!("found chunk type 0x{:x}", t),
            }
        }
        result
    }
}

pub enum ParseError {
    WrongChunkType,
}

#[derive(Debug)]
pub enum XmlEvent {
    NamespaceStart(Namespace),
    NamespaceEnd(Namespace),
    ElementStart(ElementStart),
    ElementEnd(ElementEnd),
    CData(CData),
}

#[derive(Debug)]
pub struct Namespace {
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
pub struct ElementStart {
    pub line_number: u32,
    pub comment: Option<String>,
    pub ns: Option<String>,
    pub name: String,
    pub attributes: Option<Vec<Attribute>>,
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

    pub fn attribute_len(&self) -> usize {
        if let Some(ref attrs) = self.attributes {
            attrs.len()
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    pub ns: Option<String>,
    pub name: String,
    pub value: TypedValue,
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
pub struct ElementEnd {
    pub line_number: u32,
    pub comment: Option<String>,
    pub ns: Option<String>,
    pub name: String,
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
pub struct CData {
    pub line_number: u32,
    pub comment: Option<String>,
    pub data: String,
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

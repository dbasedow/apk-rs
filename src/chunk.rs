use crate::typedvalue::parse_typed_value;
use nom::*;

#[derive(Debug, Clone)]
pub struct Chunk<'a> {
    pub typ: u16,
    pub additional_header: &'a [u8],
    pub data: &'a [u8],
}

impl<'a> Chunk<'a> {
    pub fn get_sub_chunks(&self) -> IResult<&[u8], Vec<Chunk<'a>>> {
        do_parse!(self.data, res: many0!(parse_chunk) >> (res))
    }

    pub fn get_additional_header(&self) -> Option<XmlChunkHeader> {
        if self.additional_header.len() > 0 {
            if let IResult::Done(_, meta) = parse_xml_chunk_header(self.additional_header) {
                return Some(meta);
            }
        }

        None
    }
}

named!(pub parse_chunk<&[u8], Chunk>, do_parse!(
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

#[derive(Debug)]
pub struct NamespaceChunk {
    pub prefix: u32,
    pub uri: u32,
}

named!(pub parse_namespace_body<&[u8], NamespaceChunk>, do_parse!(
    prefix: le_u32 >>
    uri: le_u32 >>
    (NamespaceChunk {prefix, uri})
));

#[derive(Debug)]
pub struct XmlChunkHeader {
    pub line_number: u32,
    pub comment: u32,
}

named!(pub parse_xml_chunk_header<&[u8], XmlChunkHeader>, do_parse!(
    line_number: le_u32 >>
    comment: le_u32 >>
    (XmlChunkHeader {line_number, comment})
));

#[derive(Debug)]
pub struct RawAttribute {
    pub ns: u32,
    pub name: u32,
    pub raw_value: u32,
    pub typed_value: (u8, u32),
}

named!(parse_attribute<&[u8], RawAttribute>, do_parse!(
    ns: le_u32 >>
    name: le_u32 >>
    raw_value: le_u32 >>
    typed_value: parse_typed_value >>
    (RawAttribute {ns, name, raw_value, typed_value})
));

pub struct XmlStartChunk {
    pub ns: u32,
    pub name: u32,
    pub attributes: Vec<RawAttribute>,
}

named!(pub parse_start_element_chunk<&[u8], XmlStartChunk>, do_parse!(
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
pub struct XmlEndNode {
    pub ns: u32,
    pub name: u32,
}

named!(pub parse_end_element_chunk<&[u8], XmlEndNode>, do_parse!(
    ns: le_u32 >>
    name: le_u32 >>
    (XmlEndNode {ns, name})
));

#[derive(Debug)]
pub struct CdataChunk {
    pub data: u32,
}

named!(pub parse_cdata_chunk<&[u8], CdataChunk>, do_parse!(
    data: le_u32 >>
    (CdataChunk {data})
));

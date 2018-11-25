use nom::*;
use crate::parser::{Chunk};

pub struct StringPool {
    pool: Vec<String>,
}

impl StringPool {
    pub fn get_optional(&self, index: u32) -> Option<String> {
        if index != 0xff_ff_ff_ff {
            Some(self.pool[index as usize].clone())
        } else {
            None
        }
    }

    pub fn get(&self, index: u32) -> String {
        self.pool[index as usize].clone()
    }
}

#[derive(Debug)]
struct StringPoolHeader {
    string_count: u32,
    style_count: u32,
    flags: u32,
    string_start: u32,
    style_start: u32,
}

impl StringPoolHeader {
    fn is_utf8(&self) -> bool {
        self.flags & 0b1_0000_0000 == 0b1_0000_0000
    }
}

named!(parse_string_pool_additional_header<&[u8], StringPoolHeader>, do_parse!(
    string_count: le_u32 >>
    style_count: le_u32 >>
    flags: le_u32 >>
    string_start: le_u32 >>
    style_start: le_u32 >>
    (StringPoolHeader {string_count, style_count, flags, string_start, style_start})
));

pub enum ParseError {
    WrongChunkType,
}

named!(parse_utf8_string_len<&[u8], usize>, do_parse!(
    len_bytes: be_u16 >>
    cond!(len_bytes & 0x8000 == 0x8000, take!(2)) >>
    (if len_bytes & 0x8000 == 0x8000 { (len_bytes & 0x7fff) as usize } else {(len_bytes & 0xff) as usize})
));

named!(parse_string_pool_entry_utf8<&[u8], String>, do_parse!(
    string_length: parse_utf8_string_len >>
    data: map!(count!(le_u8, string_length as usize), |u| String::from_utf8(u).unwrap()) >>
    take!(1) >>
    (data)
));

named!(parse_string_pool_entry_utf16<&[u8], String>, do_parse!(
    string_length: le_u16 >>
    data: map!(count!(le_u16, string_length as usize), |u| String::from_utf16(&u).unwrap()) >>
    take!(2) >>
    (data)
));

pub fn parse_string_pool_chunk(chunk: &Chunk) -> Result<StringPool, ParseError> {
    if chunk.typ != 0x0001 {
        return Err(ParseError::WrongChunkType);
    }

    let sph = parse_string_pool_additional_header(chunk.additional_header)
        .unwrap()
        .1;

    if sph.style_count > 0 {
        panic!("stylecount: {}", sph.style_count);
    }

    let char_fn = if sph.is_utf8() {
        parse_string_pool_entry_utf8
    } else {
        parse_string_pool_entry_utf16
    };

    let u16_strings = do_parse!(
        chunk.data,
        offsets: count!(le_u32, sph.string_count as usize) >> (offsets)
    );

    if let IResult::Done(rest, offsets) = u16_strings {
        let mut strings: Vec<String> = Vec::with_capacity(offsets.len());
        for offset in offsets {
            let string = char_fn(&rest[offset as usize..]).unwrap().1;
            strings.push(string);
        }
        return Ok(StringPool { pool: strings });
    }
    Err(ParseError::WrongChunkType)
}

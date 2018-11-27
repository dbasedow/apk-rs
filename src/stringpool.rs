use crate::chunk::Chunk;
use nom::*;

pub struct StringPool {
    pool: Vec<String>,
    styles: Vec<Vec<StringStyling>>,
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

fn parse_utf8_len(input: &[u8]) -> IResult<&[u8], usize> {
    if input[0] & 0x80 == 0x80 {
        let result: usize = ((input[0] as usize & 0x7f) << 8) + input[1] as usize;
        return IResult::Done(&input[2..], result);
    } else {
        return IResult::Done(&input[1..], input[0] as usize);
    }
}

named!(parse_string_pool_entry_utf8<&[u8], String>, do_parse!(
    len_chars: parse_utf8_len >>
    len_bytes: parse_utf8_len >>
    data: map!(count!(le_u8, len_bytes as usize), |u| String::from_utf8(u).unwrap()) >>
    take!(1) >>
    (data)
));

named!(parse_string_pool_entry_utf16<&[u8], String>, do_parse!(
    string_length: le_u16 >>
    data: map!(count!(le_u16, string_length as usize), |u| String::from_utf16(&u).unwrap()) >>
    take!(2) >>
    (data)
));

named!(parse_string_style_entry<&[u8], StringStyling>, do_parse!(
    name: le_u32 >> 
    fc: le_u32 >> 
    lc: le_u32 >> 
    (StringStyling { name: name as usize, start_char: fc as usize, end_char: lc as usize}))
);

fn find_end_of_string_styles(input: &[u8]) -> IResult<&[u8], &[u8]> {
    do_parse!(input, n: tag!(&[0xff, 0xff, 0xff, 0xff]) >> (n))
}

named!(parse_string_style_entries<&[u8], Vec<StringStyling>>, do_parse!(
    styles: many_till!(parse_string_style_entry, find_end_of_string_styles) >>
    (styles.0)
));

#[derive(Debug)]
struct StringStyling {
    name: usize,
    start_char: usize,
    end_char: usize,
}

pub fn parse_string_pool_chunk(chunk: &Chunk) -> Result<StringPool, ParseError> {
    if chunk.typ != 0x0001 {
        return Err(ParseError::WrongChunkType);
    }

    let sph = parse_string_pool_additional_header(chunk.additional_header)
        .unwrap()
        .1;

    let char_fn = if sph.is_utf8() {
        parse_string_pool_entry_utf8
    } else {
        parse_string_pool_entry_utf16
    };

    let u16_strings = do_parse!(
        chunk.data,
        str_offsets: count!(le_u32, sph.string_count as usize)
            >> sty_offsets: count!(le_u32, sph.style_count as usize)
            >> ((str_offsets, sty_offsets))
    );

    if let IResult::Done(rest, (string_offsets, style_offsets)) = u16_strings {
        let mut strings: Vec<String> = Vec::with_capacity(string_offsets.len());
        for offset in string_offsets {
            let string = char_fn(&rest[offset as usize..]).unwrap().1;
            strings.push(string);
        }

        let mut styles: Vec<Vec<StringStyling>> = Vec::with_capacity(sph.style_count as usize);

        if sph.style_count > 0 {
            let style_rest = &rest[(sph.style_start - sph.string_start) as usize..];
            for offset in style_offsets {
                if let IResult::Done(_, styles_for_str) = parse_string_style_entries(&style_rest[offset as usize..]) {
                    styles.push(styles_for_str);
                }
            }
        }
        return Ok(StringPool { pool: strings, styles });
    }
    Err(ParseError::WrongChunkType)
}

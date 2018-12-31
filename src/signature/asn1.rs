extern crate nom;

use nom::*;
use std::mem::size_of;

enum ASN1Type {
    Sequence(Vec<ASN1Type>),
    ObjectIdentifier(DataElement),
    Any,
}

fn is_last_octet(octet: u8) -> bool {
    octet & 0x80 == 0x00
}

fn parse_octet_series(input: &[u8]) -> IResult<&[u8], &[u8]> {
    for i in 0..input.len() {
        if is_last_octet(input[i]) {
            return IResult::Done(&input[i + 1..], &input[0..i + 1]);
        }
    }
    return IResult::Error(ErrorKind::Custom(0));
}

#[test]
fn test_parse_octet_series() {
    let d = b"\x86\x48";
    let (rest, result) = parse_octet_series(d).unwrap();
    assert_eq!(rest.len(), 0);
    assert_eq!(result, b"\x86\x48");

    let d = b"\x2a";
    let (rest, result) = parse_octet_series(d).unwrap();
    assert_eq!(rest.len(), 0);
    assert_eq!(result, b"\x2a");
}

named!(parse_sub_identifiers<&[u8], Vec<&[u8]>>, do_parse!(
    sub_identifiers: many0!(parse_octet_series) >>
    (sub_identifiers)
));

fn decode_object_identifier(data: &[u8]) -> Vec<u32> {
    let (_, sub_ids) = parse_sub_identifiers(data).unwrap();
    let mut parts: Vec<u32> = Vec::with_capacity(sub_ids.len());
    for (i, sub_id) in sub_ids.iter().enumerate() {
        let mut part: u32 = 0;
        for (j, d) in sub_id.iter().rev().enumerate() {
            part |= (*d as u32 & 0x7f) << (j * 7);
        }
        if i == 0 {
            let y = part % 40;
            let x = part / 40;
            parts.push(x);
            parts.push(y);
        } else {
            parts.push(part);
        }
    }
    parts
}

#[test]
fn test_decode_object_identifier() {
    let d = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x07, 0x02];
    let r = decode_object_identifier(&d);
    assert_eq!(r, [1, 2, 840, 113549, 1, 7, 2]);
}

#[derive(Debug)]
pub struct DataElement {
    tag_class: TagClass,
    constructed: bool,
    id: u32,
    data: Vec<u8>,
}

impl DataElement {
    pub fn parse_data(&self) -> IResult<&[u8], Vec<DataElement>> {
        parse_data_elements(&self.data)
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

#[derive(Debug, PartialEq)]
enum TagClass {
    Universal,
    Application,
    Context,
    Private,
}

fn parse_identifier(input: &[u8]) -> IResult<&[u8], (TagClass, bool, u32)> {
    let tag_class = match (input[0] & 0xC0) >> 6 {
        0 => TagClass::Universal,
        1 => TagClass::Application,
        2 => TagClass::Context,
        3 => TagClass::Private,
        _ => unimplemented!(),
    };

    let constructed = input[0] & 0x20 == 0x20;

    if input[0] & 0x1f != 0x1f {
        let id = input[0] as u32 & 0x1f;
        return IResult::Done(&input[1..], (tag_class, constructed, id));
    } else {
        for i in 1..5 {
            if input[i] & 0x80 != 0x80 {
                let mut identifier: u32 = 0;
                for j in 1..=i {
                    let shift = (i - j) * 7;
                    identifier |= (input[j] as u32 & 0x7f) << shift;
                }
                return IResult::Done(&input[i..], (tag_class, constructed, identifier));
            }
        }
        return IResult::Error(ErrorKind::Custom(0));
    }
}

#[test]
fn test_parse_identifier() {
    let d = b"\x2a";
    let (_, r) = parse_identifier(d).unwrap();
    assert_eq!(r.0, TagClass::Universal);
    assert_eq!(r.1, true);
    assert_eq!(r.2, 0xa);

    let d = b"\xff\x2a";
    let (_, r) = parse_identifier(d).unwrap();
    assert_eq!(r.0, TagClass::Private);
    assert_eq!(r.1, true);
    assert_eq!(r.2, 0x2a);

    let d = b"\xff\x8a\x2a";
    let (_, r) = parse_identifier(d).unwrap();
    assert_eq!(r.0, TagClass::Private);
    assert_eq!(r.1, true);
    assert_eq!(r.2, 0x52a);
}

fn parse_length(input: &[u8]) -> IResult<&[u8], usize> {
    let mut length: usize = 0;
    let rest: &[u8];

    if input[0] & 0x80 != 0x80 {
        length = input[0] as usize & 0x7f;
        rest = &input[1..];
    } else {
        let length_length = input[0] as usize & 0x7f;
        if length_length > size_of::<usize>() {
            return IResult::Error(ErrorKind::Custom(0));
        }

        for i in 1..=length_length {
            length |= (input[i] as usize) << ((length_length - i) * 8);
        }
        rest = &input[length_length + 1..];
    }
    return IResult::Done(rest, length);
}

#[test]
fn test_parse_length() {
    let d = b"\x82\x05\x63";
    let (rest, r) = parse_length(d).unwrap();
    assert_eq!(rest.len(), 0);
    assert_eq!(r, 1379);

    let d = b"\x09";
    let (rest, r) = parse_length(d).unwrap();
    assert_eq!(rest.len(), 0);
    assert_eq!(r, 9);
}

named!(pub parse_data_element<&[u8], DataElement>, do_parse!(
    id: parse_identifier >>
    data: length_bytes!(parse_length) >>
    (DataElement {
        id: id.2,
        tag_class: id.0,
        constructed: id.1,
        data: data.to_vec(),
    })
));

named!(pub parse_data_elements<&[u8], Vec<DataElement>>, do_parse!(
    elements: many0!(parse_data_element) >>
    (elements)
));

use nom::*;
use std::borrow::Cow;

named!(pub parse_local_file_header<&[u8], (i64, i64)>, do_parse!(
    tag!([0x50, 0x4b, 0x03, 0x04]) >>
    version: le_u16 >>
    flags: le_u16 >>
    compression: le_u16 >>
    mod_time: le_u16 >>
    mod_date: le_u16 >>
    crc32: le_u32 >>
    compressed_size: le_u32 >>
    uncompressed_size: le_u32 >>
    file_name_len: le_u16 >>
    extra_field_len: le_u16 >>
    ((file_name_len as i64, extra_field_len as i64))
));

#[derive(Debug, Clone)]
pub struct CentralDirectoryFileHeader {
    //version_producer: u16,
    //min_version_extractor: u16,
    pub general_purpose_flags: u16,
    pub compression_method: u16,
    //last_mod_time: u16,
    //last_mod_date: u16,
    crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,

    //disk_number_start: u16,
    //internal_file_attributes: u16,
    //external_file_attributes: u32,
    pub relative_offset_of_local_header: u32,
    file_name: Vec<u8>,
    extra_field: Vec<u8>,
    //file_comment: Vec<u8>,
}

impl CentralDirectoryFileHeader {
    pub fn file_name(&self) -> String {
        String::from_utf8_lossy(&self.file_name).to_string()
    }

    pub fn is_utf8(&self) -> bool {
        self.general_purpose_flags & (0x800) == 0x800
    }
}

named!(pub parse_central_directory<&[u8], Vec<CentralDirectoryFileHeader>>, do_parse!(
    headers: many0!(parse_central_file_header) >>
    (headers)
));

named!(pub parse_central_file_header<&[u8], CentralDirectoryFileHeader>, do_parse!(
    tag!([0x50, 0x4b, 0x01, 0x02]) >>
    version_producer: le_u16 >>
    min_version_extractor: le_u16 >>
    general_purpose_flags: le_u16 >>
    compression_method: le_u16 >>
    last_mod_time: le_u16 >>
    last_mod_date: le_u16 >>
    crc32: le_u32 >>
    compressed_size: le_u32 >>
    uncompressed_size: le_u32 >>
    filename_length: le_u16 >>
    extra_field_length: le_u16 >>
    file_comment_length: le_u16 >>
    disk_number_start: le_u16 >>
    internal_file_attributes: le_u16 >>
    external_file_attributes: le_u32 >>
    relative_offset_of_local_header: le_u32 >>
    file_name: take!(filename_length) >>
    extra_field: take!(extra_field_length) >>
    file_comment: take!(file_comment_length) >>
    (CentralDirectoryFileHeader {
        //version_producer,
        //min_version_extractor,
        general_purpose_flags,
        compression_method,
        //last_mod_time,
        //last_mod_date,
        crc32,
        compressed_size,
        uncompressed_size,

        //disk_number_start,
        //internal_file_attributes,
        //external_file_attributes,
        relative_offset_of_local_header,
        file_name: file_name.into(),
        extra_field: extra_field.into(),
        //file_comment: file_comment.into(),
    })
));

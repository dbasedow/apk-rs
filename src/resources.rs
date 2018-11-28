use crate::chunk::{parse_chunk, parse_chunks};
use crate::stringpool::parse_string_pool_chunk;
use nom::*;

pub fn parse_resource_table(data: &[u8]) -> IResult<&[u8], ()> {
    let (_, main_chunk) = try_parse!(data, parse_chunk);
    let num_packages = parse_table_chunk_header(main_chunk.additional_header);
    let (_, chunks) = try_parse!(main_chunk.data, parse_chunks);
    let strings = parse_string_pool_chunk(&chunks[0]);
    for chunk in chunks {
        if chunk.typ == 0x200 {
            let (_, pch) = try_parse!(chunk.additional_header, parse_package_chunk_header);
            let (_, package_chunks) = try_parse!(chunk.data, parse_chunks);
            let type_strings = parse_string_pool_chunk(&package_chunks[0]).ok();
            let key_strings = parse_string_pool_chunk(&package_chunks[1]).ok();
            println!("{:?}", pch);
            for sub_chunk in package_chunks {
                if sub_chunk.typ == 0x201 {
                    let rtth = try_parse!(
                        sub_chunk.additional_header,
                        parse_resource_table_type_header
                    );
                    let bod = parse_resource_table_type_body(sub_chunk.data, rtth.1.entry_count);
                    println!("{:?}", bod);
                }
                println!("{:?}", sub_chunk.typ);
            }
        }
    }

    IResult::Done(&[], ())
}

named!(parse_table_chunk_header<&[u8], usize>, do_parse!(
    n: le_u32 >> (n as usize)
));

#[derive(Debug)]
pub struct PackageChunkHeader {
    pub id: u32,
    pub name: String,
    pub type_strings_offset: usize,
    pub last_public_type: u32,
    pub key_strings_offset: usize,
    pub last_public_key: u32,
}

fn convert_zero_terminated_u16(data: &[u16]) -> String {
    for (i, ch) in data.iter().enumerate() {
        if *ch == 0 {
            return String::from_utf16_lossy(&data[..i]);
        }
    }
    String::from_utf16_lossy(data)
}

fn convert_zero_terminated_u8(data: &[u8]) -> String {
    for (i, ch) in data.iter().enumerate() {
        if *ch == 0 {
            return String::from_utf8_lossy(&data[..i]).to_string();
        }
    }
    String::from_utf8_lossy(data).to_string()
}

named!(parse_package_chunk_header<&[u8], PackageChunkHeader>, do_parse!(
    id: le_u32 >>
    name_u16: count!(le_u16, 128) >>
    type_strings_offset: le_u32 >>
    last_public_type: le_u32 >>
    key_strings_offset: le_u32 >>
    last_public_key: le_u32 >>
    (PackageChunkHeader {
        id,
        name: convert_zero_terminated_u16(&name_u16[..]),
        type_strings_offset: type_strings_offset as usize,
        last_public_type,
        key_strings_offset: key_strings_offset as usize,
        last_public_key,
    })
));

#[derive(Debug)]
struct ResourceTableTypeHeader {
    id: u8,
    entry_count: usize,
    entries_start: usize,
    config: ResourceTableConfig,
}

named!(parse_resource_table_type_header<&[u8], ResourceTableTypeHeader>, do_parse!(
    id: le_u8 >>
    take!(3) >>
    entry_count: le_u32 >>
    entries_start: le_u32 >>
    config: parse_resource_table_config >>
    (ResourceTableTypeHeader {
        id,
        entry_count: entry_count as usize,
        entries_start: entries_start as usize,
        config,
    })
));

#[derive(Debug)]
struct ResourceTableConfig {
    imsi_mcc: u16,
    imsi_mnc: u16,
    //locale
    language: String,
    country: String,
    //screen
    orientation: u8,
    touchscreen: u8,
    density: u16,
    //input
    keyboard: u8,
    navigation: u8,
    input_flags: u8,
    input_pad_0: u8,

    screen_width: u16,
    screen_height: u16,
    //version
    sdk_version: u16,
    minor_version: u16,

    screen_layout: u8,
    ui_mode: u8,
    smallest_screen_width_dp: u16,

    screen_width_dp: u16,
    screen_height_dp: u16,

    locale_script: String,
    locale_variant: String,

    screen_layout2: u8,
    color_mode: u8,
    locale_script_was_computed: bool,

    locale_numbering_system: String,
}

named!(parse_resource_table_config<&[u8], ResourceTableConfig>, do_parse!(
    size: le_u32 >>
    imsi_mcc: le_u16 >>
    imsi_mnc: le_u16 >>
    language: take!(2) >>
    country: take!(2) >>
    orientation: le_u8 >>
    touchscreen: le_u8 >>
    density: le_u16 >>
    keyboard: le_u8 >>
    navigation: le_u8 >>
    input_flags: le_u8 >>
    input_pad_0: le_u8 >>
    screen_width: le_u16 >>
    screen_height: le_u16 >>
    sdk_version: le_u16 >>
    minor_version: le_u16 >>
    screen_layout: le_u8 >>
    ui_mode: le_u8 >>
    smallest_screen_width_dp: le_u16 >>

    screen_width_dp: le_u16 >>
    screen_height_dp: le_u16 >>

    locale_script: take!(4) >>
    locale_variant: take!(8) >>

    screen_layout2: le_u8 >>
    color_mode: le_u8 >>
    screen_config_pad: take!(2) >>
    locale_script_was_computed: le_u8 >>
    locale_numbering_system: take!(8) >>
    (ResourceTableConfig { 
        imsi_mcc,
        imsi_mnc,
        language: convert_zero_terminated_u8(language),
        country: convert_zero_terminated_u8(country),
        orientation,
        touchscreen,
        density,
        keyboard,
        navigation,
        input_flags,
        input_pad_0,
        screen_width,
        screen_height,
        sdk_version,
        minor_version,

        screen_layout,
        ui_mode,
        smallest_screen_width_dp,
        screen_width_dp,
        screen_height_dp,

        locale_script: convert_zero_terminated_u8(locale_script),
        locale_variant: convert_zero_terminated_u8(locale_variant),

        screen_layout2,
        color_mode,
        locale_script_was_computed: locale_script_was_computed == 1,
        locale_numbering_system: convert_zero_terminated_u8(locale_numbering_system),
    })
));

fn parse_resource_table_type_body(input: &[u8], entry_count: usize) -> IResult<&[u8], Vec<u32>> {
    do_parse!(input,
        offsets: count!(le_u32, entry_count) >>
    (offsets))
}
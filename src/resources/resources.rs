use crate::chunk::{parse_chunk, parse_chunks};
use crate::stringpool::parse_string_pool_chunk;
use crate::resources::config::{Configuration, parse_resource_table_config};
use crate::typedvalue::ResourceValue;
use crate::typedvalue::{parse_res_value, TypedValue};
use nom::*;
use std::collections::HashSet;
use crate::stringpool::StringPool;

pub fn parse_resource_table(data: &[u8]) -> IResult<&[u8], Option<Resources>> {
    let (_, main_chunk) = try_parse!(data, parse_chunk);
    let (_, num_packages) = try_parse!(main_chunk.additional_header, parse_table_chunk_header);
    if num_packages != 1 {
        panic!("num packages is {}", num_packages);
    }
    let (_, chunks) = try_parse!(main_chunk.data, parse_chunks);
    let strings = parse_string_pool_chunk(&chunks[0]).ok().unwrap();
    for chunk in chunks {
        if chunk.typ == 0x200 {
            let (_, pch) = try_parse!(chunk.additional_header, parse_package_chunk_header);
            let (_, package_chunks) = try_parse!(chunk.data, parse_chunks);
            let type_strings = parse_string_pool_chunk(&package_chunks[0]).ok().unwrap();
            let key_strings = parse_string_pool_chunk(&package_chunks[1]).ok().unwrap();

            let mut resources = Resources {
                device_config: None,
                resource_types: Vec::new(),
                values: strings,
                keys: key_strings,
                types: type_strings,
            };
            for sub_chunk in package_chunks {
                if sub_chunk.typ == 0x201 {
                    let (_, rtth) = try_parse!(
                        sub_chunk.additional_header,
                        parse_resource_table_type_header
                    );
                    if let IResult::Done(_, entries) = parse_resource_table_type_body(sub_chunk.data, rtth.entry_count) {
                        let rd = ResourceData {
                            config: rtth.config,
                            values: entries,
                        };
                        resources.add_resource_data(rtth.id, rd);
                        /*
                        for entry in entries {
                            if let Some(entry) = entry {
                                if let Some(EntryData::Simple(d)) = entry.data {
                                    if d.typ == 0x03 {
                                        print!("{}:", key_strings.get(entry.key));
                                        println!("{}", strings.get(d.value));
                                    }
                                }
                                if let Some(EntryData::Complex(ds)) = entry.data {
                                    println!("{}:", key_strings.get(entry.key));
                                    for d in ds.mappings {
                                        if d.value.typ == 0x03 {
                                            println!(
                                                "  {} {}",
                                                key_strings.get(d.name & 0xffff),
                                                strings.get(d.value.value)
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        */
                    }
                }
                /*
                if sub_chunk.typ == 0x202 {
                    let (_, head) = try_parse!(sub_chunk.additional_header, parse_resource_table_type_spec_head);
                    let (_, entries) = try_parse!(sub_chunk.data, parse_resource_table_type_spec_entries);
                    let mut configurations: HashSet<u32> = HashSet::new();
                    println!("{:?}", head);
                    for entry in entries {
                        configurations.insert(entry);
                    }
                    println!("configs: {}", configurations.len());
                }
                */
            }
            return IResult::Done(&[], Some(resources));
        }
    }

    IResult::Done(&[], None)
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

pub fn convert_zero_terminated_u16(data: &[u16]) -> String {
    for (i, ch) in data.iter().enumerate() {
        if *ch == 0 {
            return String::from_utf16_lossy(&data[..i]);
        }
    }
    String::from_utf16_lossy(data)
}

pub fn convert_zero_terminated_u8(data: &[u8]) -> String {
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
    config: Configuration,
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





fn parse_resource_table_type_body(
    input: &[u8],
    entry_count: usize,
) -> IResult<&[u8], Vec<Option<Entry>>> {
    let (rest, offsets) = try_parse!(
        input,
        do_parse!(offsets: count!(le_u32, entry_count) >> (offsets))
    );
    let mut entries: Vec<Option<Entry>> = Vec::with_capacity(entry_count);

    for offset in offsets {
        if offset != 0xff_ff_ff_ff {
            let (re, mut entry) = try_parse!(&rest[offset as usize..], parse_entry);
            if entry.is_complex() {
                let (_, map) = try_parse!(re, parse_resource_table_map_entry);
                entry.data = Some(EntryData::Complex(map));
                entries.push(Some(entry));
            } else {
                let (_, val) = try_parse!(re, parse_res_value);
                entry.data = Some(EntryData::Simple(val));
                entries.push(Some(entry));
            }
        } else {
            entries.push(None);
        }
    }

    IResult::Done(&[], entries)
}

#[derive(Debug)]
enum EntryData {
    Simple(ResourceValue),
    Complex(ResourceTableMapEntry),
}

#[derive(Debug)]
struct Entry {
    flags: u16,
    key: u32,
    data: Option<EntryData>,
}

impl Entry {
    fn is_complex(&self) -> bool {
        self.flags & 0x0001 == 1
    }
}

named!(parse_entry<&[u8], Entry>, do_parse!(
    size: le_u16 >>
    flags: le_u16 >>
    key: le_u32 >>
    (Entry { flags, key, data: None })
));

#[derive(Debug)]
struct ResourceTableMapping {
    name: u32,
    value: ResourceValue,
}

named!(parse_resource_table_mapping<&[u8], ResourceTableMapping>, do_parse!(
    name: le_u32 >>
    value: parse_res_value >>
    (ResourceTableMapping { name, value })
));

#[derive(Debug)]
struct ResourceTableMapEntry {
    parent: u32,
    count: u32,
    mappings: Vec<ResourceTableMapping>,
}

named!(parse_resource_table_map_entry<&[u8], ResourceTableMapEntry>, do_parse!(
    parent: le_u32 >>
    count: le_u32 >>
    mappings: count!(parse_resource_table_mapping, count as usize) >>
    (ResourceTableMapEntry {
        parent,
        count,
        mappings,
    })
));

#[derive(Debug)]
struct ResourceTableTypeSpec {
    id: u8,
    entry_count: usize,
}

named!(parse_resource_table_type_spec_head<&[u8],ResourceTableTypeSpec>, do_parse!(
    id: le_u8 >>
    take!(3) >>
    entry_count: le_u32 >>
    (ResourceTableTypeSpec {
        id,
        entry_count: entry_count as usize
    })
));

named!(parse_resource_table_type_spec_entries<&[u8],Vec<u32>>, do_parse!(
    entries: many0!(le_u32) >>
    (entries)
));


enum ConfigurationBits {
    MCC,
    MNC,
    Locale,
    Touchscreen,
    Keyboard,
    KeyboardHidden,
    Navigation,
    Orientation,
    Density,
    ScreenSize,
    Version,
    ScreenLayout,
    UiMode,
    SmallestScreenSize,
    LayoutDirection,
    ScreenRound,
    ColorMode,
}

impl From<u32> for ConfigurationBits {
    fn from(n: u32) -> ConfigurationBits {
        match n {
            0x0001 => ConfigurationBits::MCC,
            0x0002 => ConfigurationBits::MNC,
            0x0004 => ConfigurationBits::Locale,
            0x0008 => ConfigurationBits::Touchscreen,
            0x0010 => ConfigurationBits::Keyboard,
            0x0020 => ConfigurationBits::KeyboardHidden,
            0x0040 => ConfigurationBits::Navigation,
            0x0080 => ConfigurationBits::Orientation,
            0x0100 => ConfigurationBits::Density,
            0x0200 => ConfigurationBits::ScreenSize,
            0x0400 => ConfigurationBits::Version,
            0x0800 => ConfigurationBits::ScreenLayout,
            0x1000 => ConfigurationBits::UiMode,
            0x2000 => ConfigurationBits::SmallestScreenSize,
            0x4000 => ConfigurationBits::LayoutDirection,
            0x8000 => ConfigurationBits::ScreenRound,
            0x10000 => ConfigurationBits::ColorMode,
            n => unimplemented!("unknown configuration dimension: {}", n),
        }
    }
}

fn get_configuration_dimensions(flags: u32) -> Vec<ConfigurationBits> {
    let mut result: Vec<ConfigurationBits> = Vec::new();
    let mut mask = 1;
    for _ in 0..31 {
        let v = flags & mask;
        if v != 0 {
            result.push(v.into());
        }
        mask = mask << 1;
    }
    vec![ConfigurationBits::MCC]
}


//////////////////////

fn get_resource_type_from_id(id: u32) -> u8 {
    ((id & 0x00ff0000) >> 16) as u8
}

pub struct ResourceData {
    //Configuration
    config: Configuration,
    values: Vec<Option<Entry>>,
}

pub struct ResourceType {
    id: u8,
    data: Vec<ResourceData>,
}

pub struct Resources {
    //configuration to check against
    device_config: Option<Configuration>,
    resource_types: Vec<ResourceType>,

    //String tables
    values: StringPool,
    keys: StringPool,
    types: StringPool,
}

impl Resources {
    fn add_resource_data(&mut self, resource_type_id: u8, data: ResourceData) {
        for resource_type in &mut self.resource_types {
            if resource_type.id == resource_type_id {
                resource_type.data.push(data);
                return;
            }
        }

        let mut resource_type = ResourceType {
            id: resource_type_id,
            data: Vec::new(),
        };
        resource_type.data.push(data);
        self.resource_types.push(resource_type);
    }

    fn get_resource_type_by_id(&self, id: u32) -> Option<&ResourceType> {
        let res_type_id = get_resource_type_from_id(id);
        for res_type in &self.resource_types {
            if res_type.id == res_type_id {
                return Some(&res_type);
            }
        }
        None
    }

    pub fn get_resource_type(&self, id: u32) -> Option<String> {
        let type_id = get_resource_type_from_id(id);
        let type_index = type_id - 1;
        self.types.get_optional(type_index as u32)
    }

    pub fn get_key_name(&self, id: u32) -> Option<String> {
        let index = (id & 0x0000ffff) as usize;
        if let Some(res_type) = self.get_resource_type_by_id(id) {
            let first_existing = &res_type.data
                .iter()
                .map(|d| &d.values[index])
                .filter(|v| v.is_some())
                .next();
            if let Some(Some(entry)) = first_existing {
                return self.keys.get_optional(entry.key);
            }
        }
        None
    }

    fn get_entry_by_id_all_configs(&self, id: u32) -> Option<Vec<(&Configuration, &Entry)>> {
        let index = (id & 0x0000ffff) as usize;
        if let Some(res_type) = self.get_resource_type_by_id(id) {
            let entries: Vec<(&Configuration, &Entry)> = res_type.data
                .iter()
                .map(|d| (&d.config, &d.values[index]))
                .filter(|v| v.1.is_some())
                .map(|v| (v.0, v.1.as_ref().unwrap()))
                .collect();
            return Some(entries);
        }

        None
    }

    pub fn get_string_by_id_all_configs(&self, id: u32) -> Option<Vec<(&Configuration, String)>> {
        if let Some(entry) = self.get_entry_by_id_all_configs(id) {
            let mut result: Vec<(&Configuration, String)> = Vec::with_capacity(entry.len());
            for e in entry {
                if let Some(EntryData::Simple(s)) = e.1.data {
                    if s.typ == 0x03 {
                        result.push((e.0, self.values.get(s.value)));
                    } else {
                        panic!("expected string type, found {:X}", s.typ)
                    }
                }
            }
            return Some(result);
        }
        None
    }

    pub fn get_string_by_id(&self, id: u32) -> Option<String> {
        let index = id & 0x0000ffff;
        if let Some(res_type) = self.get_resource_type_by_id(id) {
            if let Some(entry) = &res_type.data[0].values[index as usize] {
                println!("{:?}", entry);
            }
        }
        None
    }
}
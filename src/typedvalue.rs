use crate::stringpool::StringPool;
use nom::*;
use std::mem;

#[derive(Debug)]
pub enum TypedValue {
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
    pub fn from(typed_value: ResourceValue, strings: &StringPool) -> TypedValue {
        match typed_value.typ {
            0x01 => TypedValue::Reference(typed_value.value),
            0x02 => TypedValue::Attribute(typed_value.value),
            0x03 => TypedValue::String(strings.get(typed_value.value)),
            0x04 => unsafe {
                let f = mem::transmute::<u32, f32>(typed_value.value);
                TypedValue::Float(f)
            },
            0x05 => TypedValue::Dimension(typed_value.value),
            0x06 => TypedValue::Fraction(typed_value.value),
            0x10 => TypedValue::IntDecimal(typed_value.value as i32),
            0x11 => TypedValue::IntHex(typed_value.value as i32),
            0x12 => TypedValue::Boolean(typed_value.value == 0xFF_FF_FF_FF),
            0x1c => TypedValue::Argb8(typed_value.value),
            0x1d => TypedValue::Rgb8(typed_value.value),
            0x1e => TypedValue::Argb4(typed_value.value),
            0x1f => TypedValue::Rgb4(typed_value.value),
            t => panic!("unknown value type {}", t),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TypedValue::Reference(r) => format!("@ref/0x{:x}", r),
            TypedValue::Attribute(a) => format!("@attr/0x{:x}", a),
            TypedValue::String(s) => s.clone(),
            TypedValue::Float(f) => format!("{}", f),
            TypedValue::Dimension(d) => format!("dimension({})", d), //TODO: the unit is encoded in the value
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

    /** Returns true if this value references a value in resources */
    pub fn is_reference_type(&self) -> bool {
        match self {
            TypedValue::Reference(_) => true,
            TypedValue::Attribute(_) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ResourceValue {
    pub typ: u8,
    pub value: u32,
}

named!(pub parse_res_value<&[u8], ResourceValue>, do_parse!(
    size: le_u16 >>
    take!(1) >>
    data_type: le_u8 >>
    data: le_u32 >>
    take!(size - 8) >>
    (ResourceValue {typ: data_type, value: data })
));

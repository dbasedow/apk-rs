use nom::*;
use crate::resources::resources::convert_zero_terminated_u8;

#[derive(Debug)]
pub enum MCC {
    Any,
    Some(u16),
}

impl MCC {
    pub fn to_string(&self) -> Option<String> {
        match self {
            MCC::Any => None,
            MCC::Some(mcc) => Some(format!("mcc{}", mcc)),
        }
    }
}

impl From<u16> for MCC {
    fn from(mcc: u16) -> MCC {
        if mcc != 0 {
            MCC::Some(mcc)
        } else {
            MCC::Any
        }
    }
}

#[derive(Debug)]
pub enum MNC {
    Any,
    Some(u16),
}

impl MNC {
    pub fn to_string(&self) -> Option<String> {
        match self {
            MNC::Any => None,
            MNC::Some(mnc) => Some(format!("mnc{}", mnc)),
        }
    }
}

impl From<u16> for MNC {
    fn from(mnc: u16) -> MNC {
        if mnc != 0 {
            MNC::Some(mnc)
        } else {
            MNC::Any
        }
    }
}

#[derive(Debug)]
pub enum Language {
    Any,
    Some(u16),
}

impl Language {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Language::Any => None,
            Language::Some(language) => language_or_region_to_string(*language),
        }
    }
}

impl From<u16> for Language {
    fn from(language: u16) -> Language {
        if language != 0 {
            Language::Some(language)
        } else {
            Language::Any
        }
    }
}

#[derive(Debug)]
pub enum Region {
    Any,
    Some(u16),
}

impl Region {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Region::Any => None,
            Region::Some(region) => if let Some(r) = language_or_region_to_string(*region) {
                Some(format!("r{}", r))
            } else {
                None
            },
        }
    }
}

impl From<u16> for Region {
    fn from(region: u16) -> Region {
        if region != 0 {
            Region::Some(region)
        } else {
            Region::Any
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Any,
    Portrait,
    Landscape,
    Square,
}

impl Orientation {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Orientation::Any => None,
            Orientation::Portrait => Some("port".to_string()),
            Orientation::Landscape => Some("land".to_string()),
            Orientation::Square => Some("square".to_string()),
        }
    }
}

impl From<u8> for Orientation {
    fn from(orientation: u8) -> Self {
        match orientation {
            0x00 => Orientation::Any,
            0x01 => Orientation::Portrait,
            0x02 => Orientation::Landscape,
            0x03 => Orientation::Square,
            n => unimplemented!("unknown orientation {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Touchscreen {
    Any,
    NoTouch,
    Stylus,
    Finger,
}

impl Touchscreen {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Touchscreen::Any => None,
            Touchscreen::NoTouch => Some("notouch".to_string()),
            Touchscreen::Stylus => Some("stylus".to_string()),
            Touchscreen::Finger => Some("finger".to_string()),
        }
    }
}

impl From<u8> for Touchscreen {
    fn from(touchscreen: u8) -> Touchscreen {
        match touchscreen {
            0x00 => Touchscreen::Any,
            0x01 => Touchscreen::NoTouch,
            0x02 => Touchscreen::Stylus,
            0x03 => Touchscreen::Finger,
            n => unimplemented!("unknown touchscreen {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Density {
    Default,
    Low,
    Medium,
    TV,
    High,
    XHigh,
    XXHigh,
    XXXHigh,
    Any,
    None,
}

impl Density {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Density::Low => Some("ldpi".to_string()),
            Density::Medium => Some("mdpi".to_string()),
            Density::High => Some("hdpi".to_string()),
            Density::XHigh => Some("xhdpi".to_string()),
            Density::XXHigh => Some("xxhdpi".to_string()),
            Density::XXXHigh => Some("xxxhdpi".to_string()),
            Density::TV => Some("tvdpi".to_string()),
            Density::None => Some("nodpi".to_string()),
            Density::Any => Some("anydpi".to_string()),
            _ => None,
        }
    }
}

impl From<u16> for Density {
    fn from(density: u16) -> Density {
        match density {
            0 => Density::Default,
            120 => Density::Low,
            160 => Density::Medium,
            213 => Density::TV,
            240 => Density::High,
            320 => Density::XHigh,
            480 => Density::XXHigh,
            640 => Density::XXXHigh,
            0xfffe => Density::Any,
            0xffff => Density::None,
            n => unimplemented!("unknown density {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyboard {
    Any,
    NoKeys,
    QWERTY,
    TwelveKey,
}

impl Keyboard {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Keyboard::Any => None,
            Keyboard::NoKeys => Some("nokeys".to_string()),
            Keyboard::QWERTY => Some("qwerty".to_string()),
            Keyboard::TwelveKey => Some("12key".to_string()),
        }
    }
}

impl From<u8> for Keyboard {
    fn from(keyboard: u8) -> Keyboard {
        match keyboard {
            0x00 => Keyboard::Any,
            0x01 => Keyboard::NoKeys,
            0x02 => Keyboard::QWERTY,
            0x03 => Keyboard::TwelveKey,
            n => unimplemented!("unknown keyboard {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Navigation {
    Any,
    NoNav,
    DPad,
    Trackball,
    Wheel,
}

impl Navigation {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Navigation::Any => None,
            Navigation::NoNav => Some("nonav".to_string()),
            Navigation::DPad => Some("dpad".to_string()),
            Navigation::Trackball => Some("trackball".to_string()),
            Navigation::Wheel => Some("wheel".to_string()),
        }
    }
}

impl From<u8> for Navigation {
    fn from(navigation: u8) -> Navigation {
        match navigation {
            0x00 => Navigation::Any,
            0x01 => Navigation::NoNav,
            0x02 => Navigation::DPad,
            0x03 => Navigation::Trackball,
            0x04 => Navigation::Wheel,
            n => unimplemented!("unknown navigation {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum KeysHidden {
    Any,
    No,
    Yes,
    Soft,
}

impl KeysHidden {
    pub fn to_string(&self) -> Option<String> {
        match self {
            KeysHidden::Any => None,
            KeysHidden::No => Some("keysexposed".to_string()),
            KeysHidden::Yes => Some("keyshidden".to_string()),
            KeysHidden::Soft => Some("keyssoft".to_string()),
        }
    }
}

impl From<u8> for KeysHidden {
    fn from(input_flags: u8) -> KeysHidden {
        match input_flags & 0x03 {
            0 => KeysHidden::Any,
            1 => KeysHidden::No,
            2 => KeysHidden::Yes,
            3 => KeysHidden::Soft,
            n => unimplemented!("unknown keys_hidden {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ScreenWidth {
    Any,
    Some(u16),
}

#[derive(Debug, PartialEq)]
pub enum ScreenHeight {
    Any,
    Some(u16),
}

#[derive(Debug, PartialEq)]
pub enum SdkVersion {
    Any,
    Some(u16),
}

impl SdkVersion {
    pub fn to_string(&self) -> Option<String> {
        match self {
            SdkVersion::Any => None,
            SdkVersion::Some(v) => Some(format!("v{}", v)),
        }
    }
}

impl From<u16> for SdkVersion {
    fn from(sdk_version: u16) -> SdkVersion {
        if sdk_version != 0 {
            SdkVersion::Some(sdk_version)
        } else {
            SdkVersion::Any
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MinorVersion {
    Any,
    Some(u16),
}

impl MinorVersion {
    fn to_string(&self) -> Option<String> {
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum ScreenSize {
    Any,
    Small,
    Normal,
    Large,
    XLarge,
}

impl ScreenSize {
    pub fn to_string(&self) -> Option<String> {
        match self {
            ScreenSize::Any => None,
            ScreenSize::Small => Some("small".to_string()),
            ScreenSize::Normal => Some("normal".to_string()),
            ScreenSize::Large => Some("large".to_string()),
            ScreenSize::XLarge => Some("xlarge".to_string()),
        }
    }
}

impl From<u8> for ScreenSize {
    fn from(screen_layout: u8) -> ScreenSize {
        match screen_layout & 0xf {
            0x00 => ScreenSize::Any,
            0x01 => ScreenSize::Small,
            0x02 => ScreenSize::Normal,
            0x03 => ScreenSize::Large,
            0x04 => ScreenSize::XLarge,
            n => unimplemented!("unknown screen size {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LayoutDirection {
    Any,
    LeftToRight,
    RightToLeft,
}

impl LayoutDirection {
    pub fn to_string(&self) -> Option<String> {
        match self {
            LayoutDirection::Any => None,
            LayoutDirection::LeftToRight => Some("ldltr".to_string()),
            LayoutDirection::RightToLeft => Some("ldlrtl".to_string()),
        }
    }
}

impl From<u8> for LayoutDirection {
    fn from(screen_layout: u8) -> Self {
        match (screen_layout & 0xC0) >> 6 {
            0x00 => LayoutDirection::Any,
            0x01 => LayoutDirection::LeftToRight,
            0x02 => LayoutDirection::RightToLeft,
            n => unimplemented!("unknown layout direction {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UiMode {
    Any,
    Normal,
    Desk,
    Car,
    Television,
    Appliance,
    Watch,
    VRHeadset,
}

impl UiMode {
    pub fn to_string(&self) -> Option<String> {
        match self {
            //UiMode::Any => None,
            //UiMode::Normal => Some("normal"),
            UiMode::Desk => Some("desk".to_string()),
            UiMode::Car => Some("car".to_string()),
            UiMode::Television => Some("television".to_string()),
            UiMode::Appliance => Some("appliance".to_string()),
            UiMode::Watch => Some("watch".to_string()),
            UiMode::VRHeadset => Some("vrheadset".to_string()),
            _ => None,
        }
    }
}

impl From<u8> for UiMode {
    fn from(mode: u8) -> UiMode {
        match mode & 0x0f {
            0x00 => UiMode::Any,
            0x01 => UiMode::Normal,
            0x02 => UiMode::Desk,
            0x03 => UiMode::Car,
            0x04 => UiMode::Television,
            0x05 => UiMode::Appliance,
            0x06 => UiMode::Watch,
            0x07 => UiMode::VRHeadset,
            n => unimplemented!("unknown ui mode {}", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ScreenWidthDp {
    Any,
    Some(u16),
}

impl ScreenWidthDp {
    pub fn to_string(&self) -> Option<String> {
        match self {
            ScreenWidthDp::Any => None,
            ScreenWidthDp::Some(w) => Some(format!("w{}dp", w)),
        }
    }
}

impl From<u16> for ScreenWidthDp {
    fn from(screen_width_dp: u16) -> ScreenWidthDp {
        if screen_width_dp != 0 {
            ScreenWidthDp::Some(screen_width_dp)
        } else {
            ScreenWidthDp::Any
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ScreenHeightDp {
    Any,
    Some(u16),
}

impl ScreenHeightDp {
    pub fn to_string(&self) -> Option<String> {
        match self {
            ScreenHeightDp::Any => None,
            ScreenHeightDp::Some(w) => Some(format!("h{}dp", w)),
        }
    }
}

impl From<u16> for ScreenHeightDp {
    fn from(screen_height_dp: u16) -> ScreenHeightDp {
        if screen_height_dp != 0 {
            ScreenHeightDp::Some(screen_height_dp)
        } else {
            ScreenHeightDp::Any
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SmallestWidthDp {
    Any,
    Some(u16),
}

impl SmallestWidthDp {
    pub fn to_string(&self) -> Option<String> {
        match self {
            SmallestWidthDp::Any => None,
            SmallestWidthDp::Some(w) => Some(format!("sw{}dp", w)),
        }
    }
}

impl From<u16> for SmallestWidthDp {
    fn from(screen_width_dp: u16) -> SmallestWidthDp {
        if screen_width_dp != 0 {
            SmallestWidthDp::Some(screen_width_dp)
        } else {
            SmallestWidthDp::Any
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum TripleState {
    Any,
    Yes,
    No,
}

#[derive(Debug)]
pub struct ScreenLong(TripleState);

impl ScreenLong {
    pub fn to_string(&self) -> Option<String> {
        match self.0 {
            TripleState::Any => None,
            TripleState::Yes => Some("long".into()),
            TripleState::No => Some("notlong".into()),
        }
    }
}

impl From<u8> for ScreenLong {
    fn from(screen_layout: u8) -> ScreenLong {
        match (screen_layout & 0x30) >> 4 {
            0x00 => ScreenLong(TripleState::Any),
            0x01 => ScreenLong(TripleState::No),
            0x02 => ScreenLong(TripleState::Yes),
            n => unimplemented!("unknown screen long {}", n),
        }
    }
}

#[derive(Debug)]
pub struct NightMode(TripleState);

impl NightMode {
    pub fn to_string(&self) -> Option<String> {
        match self.0 {
            TripleState::Any => None,
            TripleState::Yes => Some("night".into()),
            TripleState::No => Some("notnight".into()),
        }
    }
}

impl From<u8> for NightMode {
    fn from(mode: u8) -> Self {
        match (mode & 0x30) >> 4 {
            0x00 => Self(TripleState::Any),
            0x01 => Self(TripleState::No),
            0x02 => Self(TripleState::Yes),
            n => unimplemented!("unknown night mode {}", n),
        }
    }
}

#[derive(Debug)]
pub struct NavigationHidden(TripleState);

impl NavigationHidden {
    pub fn to_string(&self) -> Option<String> {
        match self.0 {
            TripleState::Any => None,
            TripleState::Yes => Some("navexposed".into()),
            TripleState::No => Some("navhidden".into()),
        }
    }
}

impl From<u8> for NavigationHidden {
    fn from(input_flags: u8) -> Self {
        match (input_flags & 0x0c) >> 2 {
            0x00 => Self(TripleState::Any),
            0x01 => Self(TripleState::No),
            0x02 => Self(TripleState::Yes),
            n => unimplemented!("unknown nav hidden {}", n),
        }
    }
}

#[derive(Debug)]
pub struct WideColorGamut(TripleState);

impl WideColorGamut {
    pub fn to_string(&self) -> Option<String> {
        match self.0 {
            TripleState::Any => None,
            TripleState::Yes => Some("widecg".into()),
            TripleState::No => Some("nowidecg".into()),
        }
    }
}

impl From<u8> for WideColorGamut {
    fn from(color_mode: u8) -> Self {
        match (color_mode & 0x03) {
            0x00 => Self(TripleState::Any),
            0x01 => Self(TripleState::No),
            0x02 => Self(TripleState::Yes),
            n => unimplemented!("unknown wide color gamut {}", n),
        }
    }
}

#[derive(Debug)]
pub struct HighDynamicRange(TripleState);

impl HighDynamicRange {
    pub fn to_string(&self) -> Option<String> {
        match self.0 {
            TripleState::Any => None,
            TripleState::Yes => Some("highdr".into()),
            TripleState::No => Some("lowdr".into()),
        }
    }
}

impl From<u8> for HighDynamicRange {
    fn from(color_mode: u8) -> Self {
        match (color_mode & 0x0c) >> 2 {
            0x00 => Self(TripleState::Any),
            0x01 => Self(TripleState::No),
            0x02 => Self(TripleState::Yes),
            n => unimplemented!("unknown HDR {}", n),
        }
    }
}

#[derive(Debug)]
pub struct ScreenRound(TripleState);

impl ScreenRound {
    pub fn to_string(&self) -> Option<String> {
        match self.0 {
            TripleState::Any => None,
            TripleState::Yes => Some("round".into()),
            TripleState::No => Some("notround".into()),
        }
    }
}

impl From<u8> for ScreenRound {
    fn from(screen_layout2: u8) -> Self {
        match screen_layout2 & 0x03 {
            0x00 => Self(TripleState::Any),
            0x01 => Self(TripleState::No),
            0x02 => Self(TripleState::Yes),
            n => unimplemented!("unknown screen round {}", n),
        }
    }
}

fn language_or_region_to_string(v: u16) -> Option<String> {
    if v == 0 {
        return None;
    }
    if v & 0x8080 == 0 {
        let hi = ((v & 0xff00) >> 8) as u8;
        let lo = (v & 0x00ff) as u8;
        let bs = vec![hi, lo];
        let s = String::from_utf8(bs).unwrap();
        return Some(s);
    }
    //TODO add support for three letter codes
    None
}

use nom::*;
use crate::resources::resources::convert_zero_terminated_u8;

#[derive(Debug, PartialEq)]
pub enum Orientation {
    Any,
    Portrait,
    Landscape,
    Square,
}

impl Orientation {
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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
    fn to_string(&self) -> Option<String> {
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


/**
For:
ScreenRound
WideColorGamut
HDR
*/
#[derive(Debug, PartialEq)]
pub enum TripleState {
    Any,
    Yes,
    No,
}

#[derive(Debug)]
struct ScreenLong(TripleState);

impl ScreenLong {
    fn to_string(&self) -> Option<String> {
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
struct NightMode(TripleState);

impl NightMode {
    fn to_string(&self) -> Option<String> {
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
struct NavigationHidden(TripleState);

impl NavigationHidden {
    fn to_string(&self) -> Option<String> {
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
struct WideColorGamut(TripleState);

impl WideColorGamut {
    fn to_string(&self) -> Option<String> {
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
struct HighDynamicRange(TripleState);

impl HighDynamicRange {
    fn to_string(&self) -> Option<String> {
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
struct ScreenRound(TripleState);

impl ScreenRound {
    fn to_string(&self) -> Option<String> {
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

named!(pub parse_resource_table_config<&[u8], Configuration>, do_parse!(
    size: le_u32 >>
    imsi_mcc: le_u16 >>
    imsi_mnc: le_u16 >>
    language: be_u16 >>
    country: be_u16 >>
    orientation: le_u8 >>
    touchscreen: le_u8 >>
    density: le_u16 >>
    keyboard: le_u8 >>
    navigation: le_u8 >>
    input_flags: le_u8 >>
    take!(1) >>
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
    take!(2) >>
    (Configuration {
        imsi_mcc,
        imsi_mnc,
        language: language,
        country: country,
        orientation: orientation.into(),
        touchscreen: touchscreen.into(),
        density: density.into(),
        keyboard: keyboard.into(),
        navigation: navigation.into(),
        keys_hidden: input_flags.into(),
        nav_hidden: input_flags.into(),
        screen_width,
        screen_height,
        sdk_version: sdk_version.into(),
        minor_version,

        screen_size: screen_layout.into(),
        screen_long: screen_layout.into(),
        layout_direction: screen_layout.into(),
        ui_mode: ui_mode.into(),
        night_mode: ui_mode.into(),
        smallest_screen_width_dp,
        screen_width_dp: screen_width_dp.into(),
        screen_height_dp: screen_height_dp.into(),
        locale_script: convert_zero_terminated_u8(locale_script),
        locale_variant: convert_zero_terminated_u8(locale_variant),

        screen_layout2,
        screen_round: screen_layout2.into(),
        wide_color_gamut: color_mode.into(),
        hdr: color_mode.into(),
    })
));


#[derive(Debug)]
pub struct Configuration {
    imsi_mcc: u16,
    imsi_mnc: u16,
    //locale
    language: u16,
    country: u16,
    //screen
    orientation: Orientation,
    touchscreen: Touchscreen,
    density: Density,
    //input
    keyboard: Keyboard,
    navigation: Navigation,
    keys_hidden: KeysHidden,
    nav_hidden: NavigationHidden,

    screen_width: u16,
    screen_height: u16,
    //version
    sdk_version: SdkVersion,
    minor_version: u16,

    screen_size: ScreenSize,
    screen_long: ScreenLong,
    layout_direction: LayoutDirection,
    ui_mode: UiMode,
    night_mode: NightMode,
    smallest_screen_width_dp: u16,

    screen_width_dp: ScreenWidthDp,
    screen_height_dp: ScreenHeightDp,

    locale_script: String,
    locale_variant: String,

    screen_layout2: u8,
    screen_round: ScreenRound,
    wide_color_gamut: WideColorGamut,
    hdr: HighDynamicRange,
}

fn language_or_locale_to_string(v: u16) -> Option<String> {
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

impl Configuration {
    pub fn to_configuration_name(&self) -> Option<String> {
        let parts = self.get_configuration_parts();
        if parts.len() == 0 {
            return None;
        }

        let mut s = String::new();
        for (i, p) in parts.iter().enumerate() {
            if i != 0 {
                s.push('-');
            }
            s.push_str(&p[..]);
        }

        Some(s)
    }

    fn get_configuration_parts(&self) -> Vec<String> {
        let mut parts: Vec<String> = Vec::new();
        //TODO: add MCC and MNC
        if let Some(l) = self.language() {
            parts.push(l);
        }

        if let Some(c) = self.country() {
            parts.push(format!("r{}", c));
        }

        if let Some(ld) = self.layout_direction.to_string() {
            parts.push(ld);
        }

        if let Some(sw) = self.smallest_screen_width_dp() {
            parts.push(format!("sw{}dp", sw));
        }

        if let Some(sw) = self.screen_width_dp.to_string() {
            parts.push(sw);
        }

        if let Some(sh) = self.screen_height_dp.to_string() {
            parts.push(sh);
        }

        if let Some(s) = self.screen_size.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.screen_long.to_string() {
            parts.push(s);
        }

        if let Some(o) = self.screen_round.to_string() {
            parts.push(o);
        }

        if let Some(o) = self.wide_color_gamut.to_string() {
            parts.push(o);
        }

        if let Some(o) = self.hdr.to_string() {
            parts.push(o);
        }

        if let Some(o) = self.orientation.to_string() {
            parts.push(o);
        }

        if let Some(s) = self.ui_mode.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.night_mode.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.density.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.touchscreen.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.keys_hidden.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.keyboard.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.nav_hidden.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.navigation.to_string() {
            parts.push(s);
        }

        if let Some(s) = self.sdk_version.to_string() {
            parts.push(s);
        }


        parts
    }

    pub fn language(&self) -> Option<String> {
        language_or_locale_to_string(self.language)
    }

    pub fn country(&self) -> Option<String> {
        language_or_locale_to_string(self.country)
    }

    pub fn screen_width(&self) -> Option<u16> {
        if self.screen_width != 0 {
            Some(self.screen_width)
        } else {
            None
        }
    }

    pub fn screen_height(&self) -> Option<u16> {
        if self.screen_height != 0 {
            Some(self.screen_height)
        } else {
            None
        }
    }

    pub fn minor_version(&self) -> Option<u16> {
        if self.minor_version != 0 {
            Some(self.minor_version)
        } else {
            None
        }
    }

    pub fn smallest_screen_width_dp(&self) -> Option<u16> {
        if self.smallest_screen_width_dp != 0 {
            Some(self.smallest_screen_width_dp)
        } else {
            None
        }
    }
}
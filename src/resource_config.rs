use nom::*;
use crate::resources::convert_zero_terminated_u8;

#[derive(Debug)]
pub enum Orientation {
    Any,
    Portrait,
    Landscape,
    Square,
}

#[derive(Debug)]
pub enum Touchscreen {
    Any,
    NoTouch,
    Stylus,
    Finger,
}

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

pub enum Keyboard {
    Any,
    NoKeys,
    QWERTY,
    TwelveKey,
}

pub enum Navigation {
    Any,
    NoNav,
    DPad,
    Trackball,
    Wheel,
}

pub enum KeysHidden {
    Any,
    No,
    Yes,
    Soft,
}

pub enum ScreenWidth {
    Any,
    Some(u16),
}

pub enum ScreenHeight {
    Any,
    Some(u16),
}

pub enum SdkVersion {
    Any,
    Some(u16),
}

pub enum MinorVersion {
    Any,
    Some(u16),
}

pub enum ScreenSize {
    Any,
    Small,
    Normal,
    Large,
    XLarge,
}

pub enum LayoutDirection {
    Any,
    LeftToRight,
    RightToLeft,
}

pub enum Mode {
    Any,
    Normal,
    Desk,
    Car,
    Television,
    Appliance,
    Watch,
    VRHeadset,
}

pub enum ScreenWidthDp {
    Any,
    Some(u16),
}

pub enum ScreenHeightDp {
    Any,
    Some(u16),
}


/**
For:
NavigationHidden
ScreenLong
ModeNight
ScreenRound
WideColorGamut
HDR
*/
pub enum TripleState {
    Any,
    Yes,
    No,
}

named!(pub parse_resource_table_config<&[u8], Configuration>, do_parse!(
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
/*
    locale_script_was_computed: le_u8 >>
    locale_numbering_system: take!(8) >>
    */
    (Configuration {
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
/*
        locale_script_was_computed: locale_script_was_computed == 1,
        locale_numbering_system: convert_zero_terminated_u8(locale_numbering_system),
        */
    })
));


#[derive(Debug)]
pub struct Configuration {
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
    /*
    locale_script_was_computed: bool,

    locale_numbering_system: String,
    */
}

impl Configuration {
    fn to_configuration_name(&self) -> String {
        "".to_string()
    }

    fn get_configuration_parts(&self) -> Vec<String> {
        vec!["".to_string()]
    }

    pub fn orientation(&self) -> Orientation {
        match self.orientation {
            0x00 => Orientation::Any,
            0x01 => Orientation::Portrait,
            0x02 => Orientation::Landscape,
            0x03 => Orientation::Square,
            n => unimplemented!("unknown orientation {}", n),
        }
    }

    pub fn touchscreen(&self) -> Touchscreen {
        match self.touchscreen {
            0x00 => Touchscreen::Any,
            0x01 => Touchscreen::NoTouch,
            0x02 => Touchscreen::Stylus,
            0x03 => Touchscreen::Finger,
            n => unimplemented!("unknown touchscreen {}", n),
        }
    }

    pub fn density(&self) -> Density {
        match self.density {
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

    pub fn keyboard(&self) -> Keyboard {
        match self.keyboard {
            0x00 => Keyboard::Any,
            0x01 => Keyboard::NoKeys,
            0x02 => Keyboard::QWERTY,
            0x03 => Keyboard::TwelveKey,
            n => unimplemented!("unknown keyboard {}", n),
        }
    }

    pub fn navigation(&self) -> Navigation {
        match self.navigation {
            0x00 => Navigation::Any,
            0x01 => Navigation::NoNav,
            0x02 => Navigation::DPad,
            0x03 => Navigation::Trackball,
            0x04 => Navigation::Wheel,
            n => unimplemented!("unknown navigation {}", n),
        }
    }

    pub fn keys_hidden(&self) -> KeysHidden {
        match self.input_flags & 0x03 {
            0 => KeysHidden::Any,
            1 => KeysHidden::No,
            2 => KeysHidden::Yes,
            3 => KeysHidden::Soft,
            n => unimplemented!("unknown keys_hidden {}", n),
        }
    }

    pub fn nav_hidden(&self) -> TripleState {
        match (self.input_flags & 0x0c) >> 2 {
            0 => TripleState::Any,
            1 => TripleState::No,
            2 => TripleState::Yes,
            n => unimplemented!("unknown nav_hidden {}", n),
        }
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

    pub fn sdk_version(&self) -> Option<u16> {
        if self.sdk_version != 0 {
            Some(self.sdk_version)
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

    pub fn screen_size(&self) -> ScreenSize {
        match self.screen_layout & 0xf {
            0x00 => ScreenSize::Any,
            0x01 => ScreenSize::Small,
            0x02 => ScreenSize::Normal,
            0x03 => ScreenSize::Large,
            0x04 => ScreenSize::XLarge,
            n => unimplemented!("unknown screen size {}", n),
        }
    }

    pub fn screen_long(&self) -> TripleState {
        match (self.screen_layout & 0x30) >> 4 {
            0x00 => TripleState::Any,
            0x01 => TripleState::No,
            0x02 => TripleState::Yes,
            n => unimplemented!("unknown screen size {}", n),
        }
    }

    pub fn screen_layout_direction(&self) -> LayoutDirection {
        match (self.screen_layout & 0xC0) >> 6 {
            0x00 => LayoutDirection::Any,
            0x01 => LayoutDirection::LeftToRight,
            0x02 => LayoutDirection::RightToLeft,
            n => unimplemented!("unknown layout direction {}", n),
        }
    }

    pub fn mode(&self) -> Mode {
        match self.ui_mode & 0x0f {
            0x00 => Mode::Any,
            0x01 => Mode::Normal,
            0x02 => Mode::Desk,
            0x03 => Mode::Car,
            0x04 => Mode::Television,
            0x05 => Mode::Appliance,
            0x06 => Mode::Watch,
            0x07 => Mode::VRHeadset,
            n => unimplemented!("unknown ui mode {}", n),
        }
    }

    pub fn night_mode(&self) -> TripleState {
        match (self.ui_mode & 0x30) >> 4 {
            0x00 => TripleState::Any,
            0x01 => TripleState::No,
            0x02 => TripleState::Yes,
            n => unimplemented!("unknown night mode {}", n),
        }
    }

    pub fn smallest_screen_width_dp(&self) -> Option<u16> {
        if self.smallest_screen_width_dp != 0 {
            Some(self.smallest_screen_width_dp)
        } else {
            None
        }
    }

    pub fn screen_width_dp(&self) -> Option<u16> {
        if self.screen_width_dp != 0 {
            Some(self.screen_width_dp)
        } else {
            None
        }
    }

    pub fn screen_height_dp(&self) -> Option<u16> {
        if self.screen_height_dp != 0 {
            Some(self.screen_height_dp)
        } else {
            None
        }
    }

    pub fn screen_round(&self) -> TripleState {
        match self.screen_layout2 & 0x03 {
            0x00 => TripleState::Any,
            0x01 => TripleState::No,
            0x02 => TripleState::Yes,
            n => unimplemented!("unknown screen round {}", n),
        }
    }

    pub fn wide_color_gamut(&self) -> TripleState {
        match (self.color_mode & 0x0c) >> 2 {
            0x00 => TripleState::Any,
            0x01 => TripleState::No,
            0x02 => TripleState::Yes,
            n => unimplemented!("unknown screen round {}", n),
        }
    }
}

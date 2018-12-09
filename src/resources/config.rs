use nom::*;
use crate::resources::resources::convert_zero_terminated_u8;
use crate::resources::config_qualifiers::*;


named!(pub parse_resource_table_config<&[u8], Configuration>, do_parse!(
    size: le_u32 >>
    imsi_mcc: le_u16 >>
    imsi_mnc: le_u16 >>
    language: be_u16 >>
    region: be_u16 >>
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
        imsi_mcc: imsi_mcc.into(),
        imsi_mnc: imsi_mnc.into(),
        language: language.into(),
        region: region.into(),
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
        smallest_screen_width_dp: smallest_screen_width_dp.into(),
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
    imsi_mcc: MCC,
    imsi_mnc: MNC,
    //locale
    language: Language,
    region: Region,
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
    smallest_screen_width_dp: SmallestWidthDp,

    screen_width_dp: ScreenWidthDp,
    screen_height_dp: ScreenHeightDp,

    locale_script: String,
    locale_variant: String,

    screen_layout2: u8,
    screen_round: ScreenRound,
    wide_color_gamut: WideColorGamut,
    hdr: HighDynamicRange,
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
        if let Some(mcc) = self.imsi_mcc.to_string() {
            parts.push(mcc);
        }

        if let Some(mnc) = self.imsi_mnc.to_string() {
            parts.push(mnc);
        }

        if let Some(l) = self.language.to_string() {
            parts.push(l);
        }

        if let Some(c) = self.region.to_string() {
            parts.push(c);
        }

        if let Some(ld) = self.layout_direction.to_string() {
            parts.push(ld);
        }

        if let Some(sw) = self.smallest_screen_width_dp.to_string() {
            parts.push(sw);
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
}

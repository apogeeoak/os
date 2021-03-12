#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub(super) enum Color {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub(super) struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub(super) struct ScreenChar {
    character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(character: u8, color_code: ColorCode) -> ScreenChar {
        ScreenChar {
            character,
            color_code,
        }
    }

    pub fn blank(color_code: ColorCode) -> ScreenChar {
        ScreenChar {
            character: b' ',
            color_code
        }
    }

    pub fn character(&self) -> u8 {
        self.character
    }
}

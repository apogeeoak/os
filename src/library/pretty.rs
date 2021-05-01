use core::fmt;

pub struct Color<T> {
    item: T,
    color: ColorCode,
}

impl<T: fmt::Display> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}m", self.color.to_str())?; // Prefix ANSI escape code.
        self.item.fmt(f)?;
        write!(f, "\x1B[0m")?; // Reset ANSI escape code.
        Ok(())
    }
}

impl<T> Color<T> {
    pub fn new(item: T, color: ColorCode) -> Color<T> {
        Color { item, color }
    }

    pub fn bright_green(item: T) -> Color<T> {
        Color::new(item, ColorCode::BrightGreen)
    }

    pub fn bright_red(item: T) -> Color<T> {
        Color::new(item, ColorCode::BrightRed)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorCode {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl ColorCode {
    pub fn to_str(self) -> &'static str {
        use ColorCode::*;
        match self {
            Black => "30",
            Red => "31",
            Green => "32",
            Yellow => "33",
            Blue => "34",
            Magenta => "35",
            Cyan => "36",
            White => "37",
            BrightBlack => "90",
            BrightRed => "91",
            BrightGreen => "92",
            BrightYellow => "93",
            BrightBlue => "94",
            BrightMagenta => "95",
            BrightCyan => "96",
            BrightWhite => "97",
        }
    }
}

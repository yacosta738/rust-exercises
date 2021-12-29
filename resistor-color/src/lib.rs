use std::fmt;
use std::fmt::Formatter;

use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoEnumIterator, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}
impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let color = match self {
            ResistorColor::Black => "Black",
            ResistorColor::Brown => "Brown",
            ResistorColor::Red => "Red",
            ResistorColor::Orange => "Orange",
            ResistorColor::Yellow => "Yellow",
            ResistorColor::Green => "Green",
            ResistorColor::Blue => "Blue",
            ResistorColor::Violet => "Violet",
            ResistorColor::Grey => "Grey",
            ResistorColor::White => "White",
        };
        write!(f, "{}", color)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    println!("convert a color into a numerical representation");
    _color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    println!(
        "convert the value {} into a string representation of color",
        value
    );
   let result = ResistorColor::from_int(value as u8);
    // Result<Self, IntEnumError<Self>>
    match result {
        Ok(color) => color.to_string(),
        Err(_error) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    println!("return a list of all the colors ordered by resistance");
    // to list the different band colors. return a list of all the colors ordered by resistance
    ResistorColor::into_enum_iter().collect()
}

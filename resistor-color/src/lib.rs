use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::marker::Copy;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoEnumIterator, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    let res = match _color{
        ResistorColor::Black => ResistorColor::Black.int_value(),
        ResistorColor::Blue => ResistorColor::Blue.int_value(),
        ResistorColor::Brown => ResistorColor::Brown.int_value(),
        ResistorColor::Green => ResistorColor::Green.int_value(),
        ResistorColor::Grey => ResistorColor::Grey.int_value(),
        ResistorColor::Orange => ResistorColor::Orange.int_value(),
        ResistorColor::Red => ResistorColor::Red.int_value(),
        ResistorColor::Violet => ResistorColor::Violet.int_value(),
        ResistorColor::White => ResistorColor::White.int_value(),
        ResistorColor::Yellow => ResistorColor::Yellow.int_value(),
    };
    return res;
}

pub fn value_to_color_string(value: usize) -> String {
    let mut color_string = String::new();
    let mut value_color: Option<ResistorColor> = None;
    for resistor_color in ResistorColor::into_enum_iter(){
        if value == resistor_color.int_value(){
            value_color = Some(resistor_color.clone())
        }
    }
    if let Some(matched_color) = value_color{
        match matched_color{
            ResistorColor::Black => color_string.push_str("Black"),
            ResistorColor::Blue => color_string.push_str("Blue"),
            ResistorColor::Brown => color_string.push_str("Brown"),
            ResistorColor::Green => color_string.push_str("Green"),
            ResistorColor::Grey => color_string.push_str("Grey"),
            ResistorColor::Orange => color_string.push_str("Orange"),
            ResistorColor::Red => color_string.push_str("Red"),
            ResistorColor::Violet => color_string.push_str("Violet"),
            ResistorColor::White => color_string.push_str("White"),
            ResistorColor::Yellow => color_string.push_str("Yellow"),
        }
    }
    return color_string;
}

pub fn colors() -> Vec<ResistorColor> {
    let mut unsorted_colors: Vec<ResistorColor> = ResistorColor::into_enum_iter().collect();
    bubble_sort_resistor_color_by_value(&mut unsorted_colors);
    return  unsorted_colors;
    
}

fn bubble_sort_resistor_color_by_value(resistor_colors: &mut Vec<ResistorColor>){
    for i in 0..(resistor_colors.len()-2){
        for j in i..resistor_colors.len()-1{
            if resistor_colors[j].int_value() > resistor_colors[j+1].int_value(){
                {
                    let temp_val = resistor_colors[j];
                    resistor_colors[j] = resistor_colors[j+1];
                    resistor_colors[j+1] = temp_val
                }
            } 
        }
    }
}
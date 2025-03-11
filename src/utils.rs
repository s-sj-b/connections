
use wasm_bindgen::prelude::*;

// colors for the squares
pub const SQUARE_BACKGROUND: u32 = 0xCCCCCC;
pub const SQUARE_SELECTED  : u32 = 0xAAAAAA;
pub const SQUARE_YELLOW    : u32 = 0xF9DF6D;
pub const SQUARE_GREEN     : u32 = 0xA0C35A;
pub const SQUARE_BLUE      : u32 = 0xB0C4EF;
pub const SQUARE_PURPLE    : u32 = 0xBA81C5;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub enum AnswerGroup {
    Yellow,
    Green,
    Blue,
    Purple,
}

impl AnswerGroup {
    pub fn color(&self) -> u32 {
        match &self {
            AnswerGroup::Yellow => SQUARE_YELLOW,
            AnswerGroup::Green => SQUARE_GREEN,
            AnswerGroup::Blue => SQUARE_BLUE,
            AnswerGroup::Purple => SQUARE_PURPLE,
        }
    }
}

pub const ANSWERS: [(AnswerGroup, &str, [&str; 4]); 4] = [
    (AnswerGroup::Yellow, "Names for the evening meal", [
        "Supper", "Tea", "Dinner", "Banquet"
    ]),
    (AnswerGroup::Green, "Palindromes", [
        "Bob", "Dad", "Hannah", "Racecar"
    ]),
    (AnswerGroup::Blue, "Features of a wave", [
        "Crest", "Speed", "Trough", "Break"
    ]),
    (AnswerGroup::Purple, "Words that become names without the E", [
        "Seam", "Jean", "Bread", "Tome"
    ])
];

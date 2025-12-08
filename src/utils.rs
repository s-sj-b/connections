
use wasm_bindgen::prelude::*;

// colors for the squares
pub const SQUARE_BACKGROUND: u32 = 0xCCCCCC;
pub const SQUARE_SELECTED  : u32 = 0xAAAAAA;
pub const SQUARE_YELLOW    : u32 = 0xF9DF6D;
pub const SQUARE_GREEN     : u32 = 0xA0C35A;
pub const SQUARE_BLUE      : u32 = 0xB0C4EF;
pub const SQUARE_PURPLE    : u32 = 0xBA81C5;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f32;
}

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
            AnswerGroup::Green  => SQUARE_GREEN,
            AnswerGroup::Blue   => SQUARE_BLUE,
            AnswerGroup::Purple => SQUARE_PURPLE,
        }
    }
}

pub const ANSWERS: [(AnswerGroup, &str, [&str; 4]); 4] = [
    (AnswerGroup::Yellow, "Reversible Letters", [
        "Y", "O", "M", "H"
    ]),
    (AnswerGroup::Green, "Text Abbreviations", [
        "TTYL", "IYKYK", "ROFL", "LOL"
    ]),
    (AnswerGroup::Purple, "Firsts", [
        "Melania Trump", "January", "A", "1"
    ]),
    (AnswerGroup::Blue, "Names for People and Months", [
        "August", "May", "April", "Julio"
    ])
];

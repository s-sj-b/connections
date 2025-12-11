use wasm_bindgen::prelude::*;

// colors for the squares
pub const SQUARE_BACKGROUND: u32 = 0xCCCCCC;
pub const SQUARE_SELECTED: u32 = 0xAAAAAA;
pub const SQUARE_YELLOW: u32 = 0xF9DF6D;
pub const SQUARE_GREEN: u32 = 0xA0C35A;
pub const SQUARE_BLUE: u32 = 0xB0C4EF;
pub const SQUARE_PURPLE: u32 = 0xBA81C5;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f32;
}

// Alias AnswerGroup for ease
type A = AnswerGroup;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[wasm_bindgen]
pub enum AnswerGroup {
    Easy,
    Medium,
    Hard,
    VeryHard,
}

impl AnswerGroup {
    pub fn color(&self) -> u32 {
        match self {
            A::Easy => SQUARE_YELLOW,
            A::Medium => SQUARE_GREEN,
            A::Hard => SQUARE_BLUE,
            A::VeryHard => SQUARE_PURPLE,
        }
    }
}

impl ToString for AnswerGroup {
    fn to_string(&self) -> String {
        match self {
            A::Easy => String::from("easy"),
            A::Medium => String::from("medium"),
            A::Hard => String::from("hard"),
            A::VeryHard => String::from("very-hard"),
        }
    }
}

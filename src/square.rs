
use wasm_bindgen::prelude::*;
use crate::utils;

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub enum SquareState {
    Plain,
    Selected,
    Answered,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Square {
    row: usize,
    column: usize,
    state: SquareState,
    answer_group: utils::AnswerGroup,
    text: String,
}

#[wasm_bindgen]
impl Square {
    pub fn new(x: usize, y: usize, answer_group: utils::AnswerGroup, text: &str) -> Self {
        Self {
            row: y,
            column: x,
            state: SquareState::Plain,
            answer_group,
            text: text.to_string(),
        }
    }

    pub fn id(&self) -> String {
        format!("{}{}", self.column, self.row)
    }

    #[wasm_bindgen]
    pub fn toggle(&mut self) {
        // if plain, set the current selecion state to selected;
        // if selected, do the opposite;
        // if answered, do nothing!
        self.state = match self.state {
            SquareState::Plain => SquareState::Selected,
            SquareState::Selected => SquareState::Plain,
            _ => self.state,
        };

        self.update_color();
    }

    pub fn state(&self) -> SquareState {
        self.state.clone()
    }

    pub fn answer_group(&self) -> utils::AnswerGroup {
        self.answer_group
    }

    pub fn solve(&mut self) {
        self.state = SquareState::Answered;
        
        // set the html element to the answer group color
        let color = match self.answer_group {
            utils::AnswerGroup::Yellow => utils::SQUARE_YELLOW,
            utils::AnswerGroup::Green => utils::SQUARE_GREEN,
            utils::AnswerGroup::Blue => utils::SQUARE_BLUE,
            utils::AnswerGroup::Purple => utils::SQUARE_PURPLE,
        };

        let square_id = self.id();
        let square = web_sys::window()
            .expect("")
            .document()
            .expect("")
            .get_element_by_id(&square_id)
            .unwrap();
        square.set_attribute("style", &format!("background-color: #{:x}", color)).expect("");

        let doc = web_sys::window().expect("").document().expect("");
        let grid_container = doc.get_element_by_id("grid-container").unwrap();
        let node = doc.get_element_by_id(&square_id).unwrap();
        grid_container.remove_child(&node);
        // console_log!("removed {:?}", node);
    }

    pub fn update_color(&mut self) {
        let string = match self.state {
            SquareState::Plain | SquareState::Answered => "button",
            SquareState::Selected => "button-selected",
        };

        web_sys::window()
            .expect("error")
            .document()
            .expect("another error")
            .get_element_by_id(&self.id())
            .expect("another another error")
            .set_class_name(string);
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}

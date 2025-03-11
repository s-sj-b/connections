
mod utils;

const GRID_WIDTH: usize = 4;
const GRID_HEIGHT: usize = 4;
const DIV_SIZE_PX: usize = 100;


use std::ops::Index;

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};
use utils::AnswerGroup;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(s);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct AnswerGrid {

    width: usize,
    height: usize,

    squares: Vec<Square>,

    // selected is a vector of indices which point to squares in
    // `self.squares`
    selected: Vec<usize>,
    lives: usize,
    solved_count: usize,

}


#[wasm_bindgen]
pub fn remove_element_by_id(id: &str) {
    let doc = web_sys::window()
        .expect("")
        .document()
        .expect("");

    let element = doc.get_element_by_id(id).unwrap();
    doc.remove_child(&element).expect("");
}

#[wasm_bindgen]
impl AnswerGrid {
    pub fn new(width: usize, height: usize) -> Self {
        // create a scrambled array of indices for the squares
        // so that we can assign each square its data
        let mut square_idcs: Vec<(usize, usize)> = (0..width*height).map(|i| (i % width, i / height)).collect();


        let mut squares: Vec<Square> = Vec::new();
        for x in 0..width {
            for y in 0..height {

                // grab the data from the answers list and create the square with it
                let retrieval_idx = (square_idcs.len() as f32 * random()).floor() as usize;
                let (group_idx, data_idx) = square_idcs[retrieval_idx];
                let (answer_group, _, answers) = utils::ANSWERS[group_idx];
                let answer = answers[data_idx].to_uppercase();

                squares.push(Square::new(x, y, answer_group, &answer));

                square_idcs.remove(retrieval_idx);
            }
        }

        Self {
            width,
            height,

            squares,

            selected: Vec::new(),
            lives: 4,
            solved_count: 0,
        }
    }

    fn lose_life(&mut self) -> bool {
        self.lives -= 1;

        // update the lives counter
        let lc = web_sys::window()
            .expect("")
            .document()
            .expect("")
            .get_element_by_id("lives-counter")
            .unwrap();

        lc.set_inner_html(&self.lives_string());
        
        self.lives < 1
    }

    pub fn setup(&mut self) -> Result<(), JsValue> {

        let document = web_sys::window()
            .expect("could not find the window")
            .document()
            .expect("could not find the document");

        let body = document.body()
            .expect("could not find document body");

        let grid_container = document.create_element("div")
            .expect("failed to create grid-container");

        grid_container.set_id("grid-container");
        grid_container.set_class_name("answer-grid");

        for i in 0..self.width() {
            for j in 0..self.height() {
                // grab the square at the self position (i, j)
                let idx = self.get_idx(i, j);
                let square = &self.squares[idx];

                let div = document.create_element("div")?;
                div.set_text_content(Some(&square.text()));
                div.set_class_name("button");
                div.set_id(&square.id());

                // console_log!("Created {:?} with text {}", div, div.text_content().unwrap());

                // console_log!("{:?}", grid_container);

                grid_container.append_child(&div)?;
            }
        }

        body.append_child(&grid_container)?;

        let submit_button = document.create_element("button")?;
        submit_button.set_inner_html("SUBMIT");
        submit_button.set_id("submit-button");
        body.append_child(&submit_button)?;

        let lives_counter = document.create_element("div")?;
        lives_counter.set_inner_html(&self.lives_string());
        lives_counter.set_id("lives-counter");
        body.append_child(&lives_counter)?;

        Ok(())
    }

    pub fn lives_string(&self) -> String {
        let life_char = "&#9829;";
        (0..self.lives).map(|_| life_char).collect()
    }


    pub fn submit_selection(&mut self) -> Option<bool> {
        if self.selected.len() < 4 || self.lives < 1 {
            return None 
        }
        
        let check_group = self.squares[self.selected[0]].answer_group;
        let selected_squares = self.selected.iter().all(|i| self.squares[*i].answer_group == check_group);

        if selected_squares {
            // update the squares to answered and change their colors
            let mut answer_group = AnswerGroup::Yellow;
            let mut answer_desc = String::new();
            for square_idx in self.selected.iter() {
                self.squares[*square_idx].solve();
                answer_group = self.squares[*square_idx].answer_group;
                answer_desc = utils::ANSWERS.iter()
                    .find(|x| x.0 == answer_group)
                    .unwrap()
                    .1
                    .to_uppercase();


                // let square_id = self.squares[*square_idx].id();
                // remove_element_by_id(&square_id);
                // console_log!("removed {}", square_id);
            }

            // create a new div for the answered group
            let doc = web_sys::window().expect("")
                .document().expect("");
            let answered_div = doc
                .create_element("div").expect("");
            answered_div.set_class_name("button-solved");
            let color = answer_group.color();
            answered_div.set_attribute("style", &format!("background-color: #{:x};order:{}", color, self.solved_count));
            answered_div.set_inner_html(&answer_desc);
                
            doc.get_element_by_id("grid-container").unwrap().append_child(&answered_div);

            self.selected = vec![];
            self.solved_count += 1;
        } else {
            if self.lose_life() {
                alert("Game Over!");
            }
        }

        return Some(selected_squares);
    }

    // #[wasm_bindgen]
    pub fn toggle_square(&mut self, x: usize, y: usize) {
        
        // only toggle the square state if we have 
        // "space" in our seletion
        
        let idx = self.get_idx(x, y);

        match self.squares[idx].state {
            // if the square is Plain --before-- the toggle, it was selected before
            SquareState::Plain => {
                if self.selected.len() < 4 {
                    self.selected.push(idx);
                    self.squares[idx].toggle();
                }
            },
            SquareState::Selected => {
                let point = self.get_idx(x, y);
                let rem_idx = self.selected.iter().position(|s_id| s_id == &point).unwrap();
                self.selected.remove(rem_idx);
                self.squares[idx].toggle();
            },
            _ => {},
        }

        console_log!("{:?}", self.selected);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_idx(&self, width: usize, height: usize) -> usize {
        return width + height * self.width()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GridPoint {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
enum SquareState {
    Plain,
    Selected,
    Answered,
}



#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Square {
    position: GridPoint,
    state: SquareState,
    answer_group: AnswerGroup,
    text: String,
}

#[wasm_bindgen]
impl Square {
    pub fn new(x: usize, y: usize, answer_group: AnswerGroup, text: &str) -> Self {
        Self {
            position: GridPoint { x, y },
            state: SquareState::Plain,
            answer_group,
            text: text.to_string(),
        }
    }

    pub fn id(&self) -> String {
        format!("{}{}", self.position.x, self.position.y)
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

    pub fn solve(&mut self) {
        self.state = SquareState::Answered;
        
        // set the html element to the answer group color
        let color = match self.answer_group {
            AnswerGroup::Yellow => utils::SQUARE_YELLOW,
            AnswerGroup::Green => utils::SQUARE_GREEN,
            AnswerGroup::Blue => utils::SQUARE_BLUE,
            AnswerGroup::Purple => utils::SQUARE_PURPLE,
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
        console_log!("removed {:?}", node);
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


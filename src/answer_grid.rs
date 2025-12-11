use crate::square::*;
use crate::utils;
use wasm_bindgen::prelude::*;
use crate::html::*;

#[wasm_bindgen]
pub struct Answer {
    group: utils::AnswerGroup,
    description: String,
    values: Vec<String>,
}

/// Answers are groups of four values (the text in the webpage boxes) that belong to the same
/// AnswerGroup (e.g., Yellow, Green). This struct stores collects the AnswerGroup, description,
/// and a Vec<String> with the answer values for easy operability.
#[wasm_bindgen]
impl Answer {
    pub fn new(group: String, description: String, values: Vec<String>) -> Result<Self, String> {
        let group = match group.as_str() {
            "easy"      => utils::AnswerGroup::Easy,
            "medium"    => utils::AnswerGroup::Medium,
            "hard"      => utils::AnswerGroup::Hard,
            "very-hard" => utils::AnswerGroup::VeryHard,
            _ => return Err("Invalid AnswerGroup".to_string()),
        };

        Ok(Self {
            group,
            description,
            values,
        })
    }

    pub fn group_string(&self) -> String {
        self.group.to_string()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn values(&self) -> Vec<String> {
        self.values.clone()
    }

    /// Format the answer values into a comma-separated string to be displayed in p-tag
    /// in the answered div container.
    ///  
    /// NOTE: There is probably a better way to do this
    pub fn values_to_string(&self) -> String {
        let mut string = String::new();
        for (i, v) in self.values.iter().enumerate() {
            string.push_str(v);

            // push a ", " to separate the answers UNLESS it is the last answer
            if i < self.values.len() - 1 {
                string.push_str(", ");
            }
        }
        string
    }
}

/// Handles the state of the AnswerGrid and interoperability with the JavaScript.
///
/// Since `selected` stores positions of the selected squares within the `squares` vector,
/// `width` and `height` are needed to calculate from the from the grid row and column.
///
/// `lives` tracks how many incorrect guesses have been submitted.
#[wasm_bindgen]
pub struct AnswerGrid {
    width: usize,
    height: usize,

    squares: Vec<Square>,
    answers: Vec<Answer>,

    // selected is a vector of indices which point to squares in
    // `self.squares`
    selected: Vec<usize>,
    lives: usize,
    solved_count: usize,
}

#[wasm_bindgen]
impl AnswerGrid {
    pub fn new(width: usize, height: usize, answer_groups: Vec<Answer>) -> Self {
        // create a scrambled array of indices for the squares
        // so that we can assign each square its data
        let mut square_idcs: Vec<(usize, usize)> = (0..width * height)
            .map(|i| (i % width, i / height))
            .collect();

        let mut squares: Vec<Square> = Vec::new();
        for x in 0..width {
            for y in 0..height {
                // grab the data from the answers list and create the square with it
                let retrieval_idx = (square_idcs.len() as f32 * utils::random()).floor() as usize;
                let (group_idx, data_idx) = square_idcs[retrieval_idx];
                let answer_group = answer_groups[group_idx].group;
                let answers = &answer_groups[group_idx].values;
                let answer = answers[data_idx].to_uppercase();

                squares.push(Square::new(x, y, answer_group, &answer));

                square_idcs.remove(retrieval_idx);
            }
        }

        Self {
            width,
            height,

            squares,
            answers: answer_groups,

            selected: Vec::with_capacity(4),
            lives: 4,
            solved_count: 0,
        }
    }

    /// When an incorrect guess is submitted, remove a life.
    ///
    /// Uses web_sys to update the `lives-counter` div with the correct number of hearts.
    ///
    /// TODO: UI overhaul.
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

    /// Handles the creation and updating of elements within the webpage.
    ///
    /// Creates the `grid-container` div on the webpage and populates it with `Square` instances
    /// containing the answer information.
    pub fn setup(&mut self) -> Result<(), JsValue> {
        let document = web_sys::window()
            .expect("could not find the window")
            .document()
            .expect("could not find the document");

        let body = document.body().expect("could not find document body");

        let grid_container = document
            .create_element("div")
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

    /// Returns a `String` of `n` hearts, where `n` is the number of lives remaining.
    pub fn lives_string(&self) -> String {
        let life_char = "&#9829;";
        (0..self.lives).map(|_| life_char).collect()
    }

    /// Submits the selected squares for review.
    ///
    /// If the selection is invalid (fewer than 4 were chosen), returns `None`.
    /// If the selection was valid, returns `Some(true)` if the selection was correct, or `Some(false)` if it was not.
    pub fn submit_selection(&mut self) -> Option<bool> {
        if self.selected.len() < 4 || self.lives < 1 {
            return None;
        }

        // Check that all the values in the selected squares are within the same answer group
        let check_group = self.squares[self.selected[0]].answer_group();
        let selected_squares = self
            .selected
            .iter()
            .all(|i| self.squares[*i].answer_group() == check_group);

        if selected_squares {
            // If all the squares are part of the same group, update the table
            // by getting the answers for those squares
            let answer = self
                .answers
                .iter()
                .find(|a| a.group == check_group)
                .unwrap();

            // update the squares to answered and change their colors
            for square_idx in self.selected.iter() {
                self.squares[*square_idx].solve();

                // grab the answer description for the corresponding answer group
                // let square_id = self.squares[*square_idx].id();
                // remove_element_by_id(&square_id);
                // console_log!("removed {}", square_id);
            }

            // create a new div for the answered group with INNER HTML set to the
            // answer group's description
            // and a <p> within containing a list of the selected answers
            // Create 3 tags:
            // - <b>{answer.description}</b>
            // - <p>{list of the answer values}</p>
            // - <div> to contain the above two </div>
            // To be displayed like so:
            // <div>
            //      <b>{answer.description}</b>
            //      <p>{list of answer values, separated by commans}</p>
            // </div>
            let doc = web_sys::window().expect("").document().expect("");

            let answer_html_instance = HtmlParser::answer_to_element(&answer, &doc).unwrap();

            doc.get_element_by_id("grid-container")
                .unwrap()
                .append_child(&answer_html_instance)
                .unwrap();

            self.selected = vec![];
            self.solved_count += 1;
        } else {
            if self.lose_life() {
                utils::alert("Game Over!");
            }
        }

        return Some(selected_squares);
    }

    // #[wasm_bindgen]
    pub fn toggle_square(&mut self, x: usize, y: usize) {
        // only toggle the square state if we have
        // "space" in our seletion

        let idx = self.get_idx(x, y);

        match self.squares[idx].state() {
            // if the square is Plain --before-- the toggle, it was selected before
            SquareState::Plain => {
                if self.selected.len() < 4 {
                    self.selected.push(idx);
                    self.squares[idx].toggle();
                }
            }
            SquareState::Selected => {
                let point = self.get_idx(x, y);
                let rem_idx = self
                    .selected
                    .iter()
                    .position(|s_id| s_id == &point)
                    .unwrap();
                self.selected.remove(rem_idx);
                self.squares[idx].toggle();
            }
            _ => {}
        }

        // console_log!("{:?}", self.selected);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_idx(&self, width: usize, height: usize) -> usize {
        return width + height * self.width();
    }
}

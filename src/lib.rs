
mod answer_grid;
mod square;
mod utils;

use wasm_bindgen::prelude::*;
use answer_grid::*;

#[wasm_bindgen]
pub fn greet(s: &str) {
    utils::alert(s);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
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


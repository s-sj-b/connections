use wasm_bindgen::JsValue;
use web_sys::{Document, Element};
use wasm_bindgen::prelude::*;

/// Handle the transformation of crate structures into `Element` instances
/// from the `web_sys` crate.
/// `wasm-bindgen` does not yet support trait implementations so we are using an
/// `HtmlParser` to create the html elements needed to represent our different Rust
/// structures.
#[wasm_bindgen]
pub struct HtmlParser;

#[wasm_bindgen]
impl HtmlParser {
    /// Answer has 3 properties that will be displayed as
    /// <section id="answer-{answer.group}">
    ///     <h2 id="answer-{answer.group}-description">{answer.description}</h3>
    ///     <p class="answer-group-values">{answer-{answer.group}-values}</p>
    /// </section>
    pub fn answer_to_element(answer: &crate::Answer, document: &Document) -> Result<Element, JsValue> {
        let group_string = answer.group_string();
        // Create the paragraph element from the values of the answer
        let answer_values_p = document.create_element("p")?;
        answer_values_p.set_attribute("class", "answer-group-values")?;
        answer_values_p.set_inner_html(&answer.values_to_string());

        // Create a section heading with the appropriate text.
        // The inner html should be the answer description and the ID should be description-{answer.group}
        let answer_description_heading = document.create_element("h2")?;
        answer_description_heading
            .set_attribute("id", &format!("answer-group-{group_string}-description"))?;
        answer_description_heading.set_attribute("class", "answer-group-heading")?;

        // Create the
        let answer_group_section = document.create_element("section")?;
        answer_group_section.set_attribute("", &format!("answer-group-{group_string}"))?;

        // Append the heading and paragraph nodes as children
        answer_group_section.append_child(&answer_description_heading)?;
        answer_group_section.append_child(&answer_values_p)?;

        Ok(answer_group_section)
    }
}
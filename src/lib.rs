use pulldown_cmark::{Options, Parser, html};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn render(markdown_str: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parse = Parser::new_ext(markdown_str, options);

    let mut html_str = String::new();

    html::push_html(&mut html_str, parse);

    html_str
}

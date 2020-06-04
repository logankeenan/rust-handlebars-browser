use wasm_bindgen::prelude::*;
use handlebars::{Handlebars};
use std::include_str;

#[macro_use]
extern crate serde_json;

#[wasm_bindgen]
pub fn render_template() -> String {
    let reg = Handlebars::new();
    let template_string = include_str!("templates/index.hbs");

    let result = reg.render_template(template_string, &json!({"dynamic_text": "Rust is pretty nice!"}));
    match result {
        Ok(template) => {
            template
        }
        Err(_) => {
            "".to_string()
        }
    }
}
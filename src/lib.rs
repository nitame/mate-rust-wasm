use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn prompt(s: &str) -> String;
}

#[wasm_bindgen]
pub fn hi_mate() {
    let name = prompt("What's ye name?");
    alert(&format!("Ho hi, {}!", name));
}
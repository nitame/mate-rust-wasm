use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global `window` found");
    let name = window.prompt_with_message("What's ye name?").unwrap();
    if let Some(v) = name {
        window.alert_with_message(&format!("Ho hi, {}!", v));
    }
    Ok(())
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_message_len(s: &str) -> usize {
    s.len()
}

#[wasm_bindgen]
pub fn greetings(name: &str) {
    let message = format!("Hello, {}!", name);

    log(&message);
    alert(&message);
}

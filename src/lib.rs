use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "hello, wasm".to_string()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = wasmEnv, js_name = log)]
    fn log(o: JsValue);
}

#[wasm_bindgen(js_name = getAnswer)]
pub fn get_answer(s: &str) {
    log(match s {
        "hello, wasm" => JsValue::from("hello, browser"),
        _ => JsValue::from(JsError::new("Invalid Token!")),
    })
}




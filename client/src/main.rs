use common::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    pub fn supatest();
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    supatest();
    log(&test123());
    Ok(())
}

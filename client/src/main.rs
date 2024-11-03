use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/index.js")]
extern "C" {}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    log("#[wasm_bindgen(start)]");
    Ok(())
}

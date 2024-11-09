use common::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

// handling networking in rust seems cancer. i will use rust client side only as a way to process
// the game logic using the common lib. all the fetching, displaying, etc, should be js

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
	log("#[wasm_bindgen(start)]");
	Ok(())
}

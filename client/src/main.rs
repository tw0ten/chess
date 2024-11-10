use common::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
	log("#[wasm_bindgen(start)]");
	let board = def::Board::default();
	log(&format!("{}", &board));
	log(&format!("{}", &board.legal_moves().len()));
	Ok(())
}

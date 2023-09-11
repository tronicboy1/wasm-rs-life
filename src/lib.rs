mod table;
mod utils;

use table::BooleanTable;
pub use table::Table;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

//#[wasm_bindgen]
pub fn tick(u: BooleanTable) -> BooleanTable {
    let mut table = Table::from(u);

    table.tick().into()
}

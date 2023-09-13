mod table;
mod utils;

pub use table::Table;

use table::cell_state::CellState;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

/// Creates a table from a javascript Uint8Array
#[wasm_bindgen]
pub fn create_table(values: Box<[u8]>, width: u32, height: u32) -> Table {
    let len = values.len();

    assert!(len == (width * height) as usize);

    let mut table = Table::of_size(width, height);

    for (i, cell) in table.iter_mut().enumerate() {
        let js_val = values[i];
        *cell = CellState::from(js_val);
    }

    table
}

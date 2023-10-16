use std::fmt::Display;
use wasm_bindgen::prelude::*;

use crate::table::cell_state::CellState;

use self::{point::Point, rows::Rows};

pub mod cell_state;
pub mod point;
mod rows;

type Row = Vec<CellState>;

#[wasm_bindgen]
pub struct Table {
    height: usize,
    width: usize,
    values: Vec<CellState>,
}

#[wasm_bindgen]
impl Table {
    /// Creates new square lable of size n
    ///
    /// # Panics!
    /// If size is less than 3
    pub fn new(size: usize) -> Self {
        // Allow console.error logs of panic
        crate::utils::set_panic_hook();

        assert!(size >= 3);

        let rows = (0..(size * size)).map(|_| CellState::Dead).collect();

        Self {
            height: size,
            width: size,
            values: rows,
        }
    }

    pub fn of_size(width: u32, height: u32) -> Self {
        assert!(width > 3 && height > 3);

        let values = (0..(height * width)).map(|_| CellState::Dead).collect();

        Self {
            height: height as usize,
            width: width as usize,
            values,
        }
    }

    pub fn tick(&mut self) {
        let blocks = self.blocks();

        for (col, block) in self.values.iter_mut().zip(blocks) {
            *col = match block {
                // Rule 1: Any live cell with fewer than two live neighbours
                // dies, as if caused by underpopulation.
                // Rule 3: Any live cell with more than three live
                // neighbours dies, as if by overpopulation.
                Block {
                    value: CellState::Alive,
                    live_count,
                } if live_count < 2 || live_count > 3 => CellState::Dead,
                // Rule 2: Any live cell with two or three live neighbours
                // lives on to the next generation.
                Block {
                    value: CellState::Alive,
                    live_count: 2 | 3,
                } => CellState::Alive,
                // Rule 4: Any dead cell with exactly three live neighbours
                // becomes a live cell, as if by reproduction.
                Block {
                    value: CellState::Dead,
                    live_count,
                } if live_count == 3 => CellState::Alive,
                _ => CellState::Dead,
            }
        }

        // Can log into JS directly
        // web_sys::console::log_1(&"Ticked!".into());
    }

    pub fn set(&mut self, i: usize, value: CellState) {
        self[i] = value;
    }

    pub fn set_point(&mut self, p: &Point, value: CellState) {
        let row_i = p.y * self.width;
        let i = row_i + p.x;
        self[i] = value;
    }

    fn set_points(&mut self, points: &[Point], value: CellState) {
        for p in points {
            self.set_point(p, value);
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn is_alive(&self) -> bool {
        self.values.iter().any(|c| c == &CellState::Alive)
    }

    pub fn cells(&self) -> *const CellState {
        std::vec::Vec::as_ptr(&self.values)
    }

    fn rows(&self) -> Rows<'_> {
        Rows::new(&self.values, self.width)
    }

    fn blocks(&self) -> Vec<Block> {
        let row_count = self.height;
        let cols = self.width;

        let prev_row = self.rows().cycle().skip(row_count - 1);
        let curr_row = self.rows();
        let next_row = self.rows().cycle().skip(row_count + 1);

        let block_rows = curr_row
            .zip(prev_row)
            .zip(next_row)
            .map(|((curr_row, prev_row), next_row)| (prev_row, curr_row, next_row));

        block_rows
            .enumerate()
            .map(move |(i, (prev_row, curr_row, next_row))| {
                curr_row.iter().enumerate().map(move |(i, value)| {
                    let (prev, curr, next) = (wrap_prev(i, cols), i, wrap_next(i, cols));

                    // Add all Alive cells around the given cell
                    let live_count = prev_row[prev]
                        + prev_row[curr]
                        + prev_row[next]
                        + curr_row[prev]
                        + curr_row[next]
                        + next_row[prev]
                        + next_row[curr]
                        + next_row[next];

                    Block {
                        value: *value,
                        live_count,
                    }
                })
            })
            .flatten()
            .collect()
    }
}

pub type BooleanTable = Vec<Vec<bool>>;

impl From<BooleanTable> for Table {
    /// Converts to table from array of array of bools
    ///
    /// # Panics!
    /// If the row length is less than 3.
    fn from(value: Vec<Vec<bool>>) -> Self {
        assert!(value.len() >= 3);

        let height = value.len();
        let width = value[0].len();

        let rows: Vec<CellState> = value
            .into_iter()
            .map(|row| row.into_iter().map(|state| state.into()))
            .flatten()
            .collect();

        Self {
            height,
            width,
            values: rows,
        }
    }
}

impl Into<BooleanTable> for Table {
    fn into(self) -> BooleanTable {
        self.values
            .chunks(self.width)
            .map(|row| row.into_iter().map(|col| (*col).into()).collect())
            .collect()
    }
}

impl std::ops::Deref for Table {
    type Target = Vec<CellState>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl std::ops::DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.width;
        writeln!(f, "+-{}-+", "-".repeat(len))?;
        let rows = self.rows();
        for row in rows {
            write!(f, "| ")?;
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f, " |")?;
        }
        writeln!(f, "+-{}-+", "-".repeat(len))?;

        Ok(())
    }
}

/// An array of 3x3 blocks with the center being the target cell. Used to calculate the
/// value in the next tick of a table for a given cell.
#[derive(Debug)]
struct Block {
    value: CellState,
    live_count: u8,
}

fn wrap_prev(i: usize, len: usize) -> usize {
    if i as i32 - 1 < 0 {
        len - 1
    } else {
        i - 1
    }
}

fn wrap_next(i: usize, len: usize) -> usize {
    if i >= len - 1 {
        0
    } else {
        i + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_table_of_n_size() {
        let table = Table::new(42);

        assert_eq!(table.height, 42);
        assert_eq!(table.width, 42);
    }

    #[test]
    fn can_convert_to_blocks() {
        let table = Table::new(5);
        let blocks = table.blocks();

        assert_eq!(blocks.len(), 5 * 5);
    }

    #[test]
    fn count_is_correct() {
        let mut table = Table::new(5);

        table.values[0] = CellState::Alive;
        table.values[24] = CellState::Alive;

        let blocks = table.blocks();

        assert_eq!(blocks.len(), 5 * 5);
        assert_eq!(blocks[0].live_count, 1);
        assert_eq!(blocks[1].live_count, 1);
        assert_eq!(blocks[2].live_count, 0);
        assert_eq!(blocks[5].live_count, 1);
        assert_eq!(blocks[6].live_count, 1);
        assert_eq!(blocks[7].live_count, 0);

        assert_eq!(blocks[24].live_count, 1);
        assert_eq!(blocks[23].live_count, 1);
    }

    #[test]
    fn can_tick_block_table_to_table() {
        let mut table = Table::new(5);
        table.tick();

        assert_eq!(table.width, 5);
        assert_eq!(table.height, 5);
        assert_eq!(table.values.len(), 25);
    }

    #[test]
    fn tick_calc_is_correct() {
        let mut table = Table::new(5);

        table.values[0] = CellState::Alive;
        table.values[5] = CellState::Alive;
        table.values[10] = CellState::Alive;
        table.values[15] = CellState::Alive;
        table.values[20] = CellState::Alive;

        table.tick();

        assert_eq!(table.len(), 5 * 5);

        assert_eq!(table[0], CellState::Alive);
        assert_eq!(table[5], CellState::Alive);
        assert_eq!(table[10], CellState::Alive);
        assert_eq!(table[15], CellState::Alive);
        assert_eq!(table[20], CellState::Alive);

        assert_eq!(table[1], CellState::Alive);
        assert_eq!(table[6], CellState::Alive);
        assert_eq!(table[11], CellState::Alive);
        assert_eq!(table[16], CellState::Alive);
        assert_eq!(table[21], CellState::Alive);

        assert_eq!(table[4], CellState::Alive);
        assert_eq!(table[9], CellState::Alive);
        assert_eq!(table[14], CellState::Alive);
        assert_eq!(table[19], CellState::Alive);
        assert_eq!(table[24], CellState::Alive);
    }

    #[test]
    fn can_convert_string() {
        let table = Table::new(5);

        let s = table.to_string();

        assert_eq!(s.lines().count(), 7);
    }
}

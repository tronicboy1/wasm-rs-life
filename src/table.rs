use std::{fmt::Display, slice::Chunks};

use crate::table::cell_state::CellState;

pub mod cell_state;

type Row = Vec<CellState>;

pub struct Table {
    height: usize,
    width: usize,
    values: Vec<CellState>,
}

impl Table {
    /// Creates new square lable of size n
    ///
    /// # Panics!
    /// If size is less than 3
    pub fn new(size: usize) -> Self {
        assert!(size >= 3);

        let rows = (0..(size * size)).map(|_| CellState::Dead).collect();

        Self {
            height: size,
            width: size,
            values: rows,
        }
    }

    pub fn tick(self) -> Self {
        let block_table: BlockTable = self.into();

        block_table.tick()
    }

    fn rows(&self) -> Chunks<'_, CellState> {
        self.values.chunks(self.width)
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

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.width;
        println!("+-{}-+", "-".repeat(len));
        let rows = self.rows();
        for row in rows {
            print!("| ");
            for cell in row {
                print!("{}", cell);
            }
            println!(" |");
        }
        println!("+-{}-+", "-".repeat(len));

        Ok(())
    }
}

impl Into<BlockTable> for Table {
    fn into(self) -> BlockTable {
        let row_count = self.height;
        let cols = self.width;

        let prev_row = self.rows().cycle().skip(row_count - 1);
        let curr_row = self.rows();
        let next_row = self.rows().cycle().skip(row_count + 1);

        let block_rows = curr_row
            .zip(prev_row)
            .zip(next_row)
            .map(|((curr_row, prev_row), next_row)| (prev_row, curr_row, next_row));

        let blocks = block_rows
            .enumerate()
            .map(|(i, (prev_row, curr_row, next_row))| {
                curr_row.iter().enumerate().map(move |(i, value)| {
                    let (prev, curr, next) = (wrap_prev(i, cols), i, wrap_next(i, cols));

                    dbg!(prev);
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
            .collect();

        BlockTable {
            heigth: self.height,
            width: self.width,
            values: blocks,
        }
    }
}

#[derive(Debug)]
struct Block {
    value: CellState,
    live_count: u8,
}

#[derive(Debug)]
struct BlockTable {
    heigth: usize,
    width: usize,
    values: Vec<Block>,
}

impl BlockTable {
    fn tick(self) -> Table {
        let rows: Vec<CellState> = self
            .values
            .chunks(self.heigth)
            .into_iter()
            .map(|row| {
                row.into_iter().map(|block| match block {
                    Block {
                        value: CellState::Alive,
                        live_count,
                    } if *live_count < 2 || *live_count > 3 => CellState::Dead,
                    Block {
                        value: CellState::Dead,
                        live_count,
                    } if *live_count == 3 => CellState::Alive,
                    _ => CellState::Dead,
                })
            })
            .flatten()
            .collect();

        Table {
            height: self.heigth,
            width: self.width,
            values: rows,
        }
    }
}

impl std::ops::Deref for BlockTable {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
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
        let blocks: BlockTable = table.into();

        assert_eq!(blocks.heigth, 5);
        assert_eq!(blocks.width, 5);
    }

    #[test]
    fn count_is_correct() {
        let mut table = Table::new(5);

        table.values[0] = CellState::Alive;
        table.values[24] = CellState::Alive;

        let blocks: BlockTable = table.into();

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
        let table = Table::new(5);
        let blocks: BlockTable = table.into();

        let table = blocks.tick();

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

        let blocks: BlockTable = table.into();

        let table = blocks.tick();

        println!("{}", table);

        assert_eq!(table.len(), 5 * 5);

        assert_eq!(table[0], CellState::Dead);
        assert_eq!(table[5], CellState::Dead);
        assert_eq!(table[10], CellState::Dead);
        assert_eq!(table[15], CellState::Dead);
        assert_eq!(table[20], CellState::Dead);

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
}

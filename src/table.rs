use std::fmt::Display;

use crate::table::cell_state::CellState;

pub mod cell_state;

type Row = Vec<CellState>;

pub struct Table(Vec<Row>);

impl Table {
    /// Creates new square lable of size n
    ///
    /// # Panics!
    /// If size is less than 3
    pub fn new(size: usize) -> Self {
        assert!(size >= 3);

        let rows: Vec<Row> = (0..size)
            .map(|_| (0..size).map(|_| CellState::Dead).collect())
            .collect();

        Self(rows)
    }

    pub fn tick(self) -> Self {
        let block_table: BlockTable = self.into();

        block_table.tick()
    }
}

impl From<Vec<Vec<bool>>> for Table {
    /// Converts to table from array of array of bools
    ///
    /// # Panics!
    /// If the row length is less than 3.
    fn from(value: Vec<Vec<bool>>) -> Self {
        assert!(value.len() >= 3);

        let rows: Vec<Row> = value
            .into_iter()
            .map(|row| row.into_iter().map(|state| state.into()).collect())
            .collect();

        Self(rows)
    }
}

impl std::ops::Deref for Table {
    type Target = Vec<Row>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self[0].len();
        println!("+-{}-+", "-".repeat(len));
        for row in self.iter() {
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
        let rows = self.0.len();
        let cols = self.0[0].len();

        let prev_row = self.0.iter().cycle().skip(rows - 1);
        let curr_row = self.0.iter();
        let next_row = self.0.iter().cycle().skip(rows + 1);

        let block_rows = curr_row
            .zip(prev_row)
            .zip(next_row)
            .map(|((curr_row, prev_row), next_row)| (prev_row, curr_row, next_row));

        let blocks = block_rows
            .enumerate()
            .map(move |(i, (prev_row, curr_row, next_row))| {
                curr_row
                    .iter()
                    .enumerate()
                    .map(|(i, ..)| {
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
                            value: curr_row[curr],
                            live_count,
                        }
                    })
                    .collect()
            })
            .collect();

        BlockTable(blocks)
    }
}

#[derive(Debug)]
struct Block {
    value: CellState,
    live_count: usize,
}

#[derive(Debug)]
struct BlockTable(Vec<Vec<Block>>);

impl BlockTable {
    fn tick(self) -> Table {
        let rows: Vec<Row> = self
            .0
            .into_iter()
            .map(|row| -> Vec<CellState> {
                row.into_iter()
                    .map(|block| match block {
                        Block {
                            value: CellState::Alive,
                            live_count,
                        } if live_count < 2 || live_count > 3 => CellState::Dead,
                        Block {
                            value: CellState::Dead,
                            live_count,
                        } if live_count == 3 => CellState::Alive,
                        _ => CellState::Dead,
                    })
                    .collect()
            })
            .collect();

        Table(rows)
    }
}

impl std::ops::Deref for BlockTable {
    type Target = Vec<Vec<Block>>;

    fn deref(&self) -> &Self::Target {
        &self.0
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

        assert_eq!(table.0.len(), 42);
        assert_eq!(table[0].len(), 42);
    }

    #[test]
    fn can_convert_to_blocks() {
        let table = Table::new(5);
        let blocks: BlockTable = table.into();

        assert_eq!(blocks.len(), 5);
        assert_eq!(blocks[0].len(), 5);
    }

    #[test]
    fn count_is_correct() {
        let mut table = Table::new(5);

        table.0[0][0] = CellState::Alive;
        table.0[4][4] = CellState::Alive;

        let blocks: BlockTable = table.into();

        assert_eq!(blocks.len(), 5);
        assert_eq!(blocks[0].len(), 5);
        assert_eq!(blocks[0][0].live_count, 1);
        assert_eq!(blocks[0][1].live_count, 1);
        assert_eq!(blocks[0][2].live_count, 0);
        assert_eq!(blocks[1][0].live_count, 1);
        assert_eq!(blocks[1][1].live_count, 1);
        assert_eq!(blocks[1][2].live_count, 0);

        assert_eq!(blocks[4][4].live_count, 1);
        assert_eq!(blocks[4][3].live_count, 1);
        assert_eq!(blocks[3][4].live_count, 1);
        assert_eq!(blocks[3][2].live_count, 0);
    }

    #[test]
    fn can_tick_block_table_to_table() {
        let table = Table::new(5);
        let blocks: BlockTable = table.into();

        let table = blocks.tick();

        assert_eq!(table.len(), 5);
        assert_eq!(table[0].len(), 5);
    }

    #[test]
    fn tick_calc_is_correct() {
        let mut table = Table::new(5);

        table.0[0][0] = CellState::Alive;
        table.0[1][0] = CellState::Alive;
        table.0[2][0] = CellState::Alive;
        table.0[3][0] = CellState::Alive;
        table.0[4][0] = CellState::Alive;

        let blocks: BlockTable = table.into();

        let table = blocks.tick();

        println!("{}", table);

        assert_eq!(table.len(), 5);
        assert_eq!(table[0].len(), 5);

        assert_eq!(table[0][0], CellState::Dead);
        assert_eq!(table[1][0], CellState::Dead);
        assert_eq!(table[2][0], CellState::Dead);
        assert_eq!(table[3][0], CellState::Dead);
        assert_eq!(table[4][0], CellState::Dead);

        assert_eq!(table[0][1], CellState::Alive);
        assert_eq!(table[1][1], CellState::Alive);
        assert_eq!(table[2][1], CellState::Alive);
        assert_eq!(table[3][1], CellState::Alive);
        assert_eq!(table[4][1], CellState::Alive);

        assert_eq!(table[0][4], CellState::Alive);
        assert_eq!(table[1][4], CellState::Alive);
        assert_eq!(table[2][4], CellState::Alive);
        assert_eq!(table[3][4], CellState::Alive);
        assert_eq!(table[4][4], CellState::Alive);
    }
}

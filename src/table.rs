#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

impl From<bool> for CellState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Alive,
            false => Self::Dead,
        }
    }
}

type Row = Vec<CellState>;
type Block = [[CellState; 3]; 3];

struct Table(Vec<Row>);

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

    fn tick(self) {
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
                let (prev, curr, next) = (wrap_prev(i, cols), i, wrap_next(i, cols));

                dbg!(&prev_row);
                dbg!(&curr_row);
                dbg!(&next_row);

                [
                    [prev_row[prev], prev_row[curr], prev_row[next]],
                    [curr_row[prev], curr_row[curr], curr_row[next]],
                    [next_row[prev], next_row[curr], next_row[next]],
                ]
            });
    }
}

impl From<Vec<Vec<bool>>> for Table {
    fn from(value: Vec<Vec<bool>>) -> Self {
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

                        dbg!(&prev_row);
                        dbg!(&curr_row);
                        dbg!(&next_row);

                        [
                            [prev_row[prev], prev_row[curr], prev_row[next]],
                            [curr_row[prev], curr_row[curr], curr_row[next]],
                            [next_row[prev], next_row[curr], next_row[next]],
                        ]
                    })
                    .collect()
            })
            .collect();

        BlockTable(blocks)
    }
}

#[derive(Debug)]
struct BlockTable(Vec<Vec<Block>>);

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
        let mut table = Table::new(5);

        table.0[0][0] = CellState::Alive;
        table.0[4][4] = CellState::Alive;

        let blocks: BlockTable = table.into();
        dbg!(&blocks);
        assert_eq!(blocks.len(), 5);
        assert_eq!(blocks[0].len(), 5);
        assert_eq!(blocks[0][0][1][1], CellState::Alive);
        assert_eq!(blocks[4][4][1][1], CellState::Alive);
        assert_eq!(blocks[0][1][1][0], CellState::Alive);
        assert_eq!(blocks[0][2][1][0], CellState::Dead);
        assert_eq!(blocks[1][0][0][1], CellState::Alive);
        assert_eq!(blocks[1][1][0][0], CellState::Alive);
        assert_eq!(blocks[1][2][0][0], CellState::Dead);

        assert_eq!(blocks[4][3][1][2], CellState::Alive);
    }
}

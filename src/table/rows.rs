use super::cell_state::CellState;

pub struct Rows<'a> {
    all_rows: &'a [CellState],
    cursor: usize,
    width: usize,
    length: usize,
}

impl<'a> Rows<'a> {
    pub fn new(rows: &'a [CellState], width: usize) -> Self {
        Self {
            all_rows: rows,
            width,
            cursor: 0,
            length: rows.len(),
        }
    }
}

impl<'a> Iterator for Rows<'a> {
    type Item = &'a [CellState];

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor != self.length {
            let start_i = self.cursor;
            self.cursor += self.width;

            Some(&self.all_rows[start_i..self.cursor])
        } else {
            None
        }
    }
}

impl<'a> Clone for Rows<'a> {
    fn clone(&self) -> Self {
        Self {
            all_rows: self.all_rows,
            cursor: 0,
            width: self.width,
            length: self.all_rows.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_iter_over_rows() {
        let values = vec![CellState::Alive; 50];
        let rows = Rows::new(&values, 10);
        assert_eq!(rows.fold(0, |acc, row| acc + row.len()), 50);
    }

    #[test]
    fn row_count_is_width() {
        let values = vec![CellState::Alive; 50];
        let rows = Rows::new(&values, 10);
        assert_eq!(rows.count(), 5);
    }
}

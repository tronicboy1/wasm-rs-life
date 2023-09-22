use super::cell_state::CellState;

pub struct Rows<'a> {
    all_rows: &'a [CellState],
    cursor: usize,
    reverse_cursor: usize,
    width: usize,
    length: usize,
}

impl<'a> Rows<'a> {
    pub fn new(rows: &'a [CellState], width: usize) -> Self {
        Self {
            all_rows: rows,
            width,
            cursor: 0,
            reverse_cursor: rows.len(),
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

impl<'a> DoubleEndedIterator for Rows<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.reverse_cursor != 0 {
            let end_i = self.reverse_cursor;
            self.reverse_cursor -= self.width;

            Some(&self.all_rows[self.reverse_cursor..end_i])
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
            reverse_cursor: self.all_rows.len(),
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

    #[test]
    fn can_flatten() {
        let values = vec![CellState::Alive; 50];
        let rows = Rows::new(&values, 10);

        let len = rows.length;
        assert_eq!(rows.flatten().count(), len);
    }

    #[test]
    fn can_reverse() {
        let mut values = vec![CellState::Alive; 10];

        let last = values.last_mut().unwrap();
        *last = CellState::Dead;

        let mut rows = Rows::new(&values, 5);
        let last_row = rows.next_back().unwrap();
        let second_to_last_row = rows.next_back().unwrap();

        assert_eq!(last_row.last().unwrap(), &CellState::Dead);
        assert_eq!(second_to_last_row.len(), rows.width);
        assert_eq!(second_to_last_row.last().unwrap(), &CellState::Alive);
        assert!(dbg!(rows.next_back()).is_none());
    }
}

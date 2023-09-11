use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Dead,
    Alive,
}

impl Into<usize> for CellState {
    fn into(self) -> usize {
        match self {
            Self::Dead => 0,
            Self::Alive => 1,
        }
    }
}

impl std::ops::Add for CellState {
    type Output = usize;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs: usize = self.into();
        let rhs: usize = rhs.into();

        lhs + rhs
    }
}

impl std::ops::Add<CellState> for usize {
    type Output = usize;

    fn add(self, rhs: CellState) -> Self::Output {
        let rhs: usize = rhs.into();

        self + rhs
    }
}

impl From<bool> for CellState {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Alive,
            false => Self::Dead,
        }
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alive => '*',
                Self::Dead => ' ',
            }
        )
    }
}

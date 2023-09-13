use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Dead,
    Alive,
}

impl Into<u8> for CellState {
    fn into(self) -> u8 {
        match self {
            Self::Dead => 0,
            Self::Alive => 1,
        }
    }
}

impl From<u8> for CellState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Dead,
            1 => Self::Alive,
            _ => Self::Dead,
        }
    }
}

impl<T> std::ops::Add<T> for CellState
where
    T: Into<u8>,
{
    type Output = u8;

    fn add(self, rhs: T) -> Self::Output {
        let lhs: u8 = self.into();
        let rhs: u8 = rhs.into();

        lhs + rhs
    }
}

impl std::ops::Add<CellState> for u8 {
    type Output = u8;

    fn add(self, rhs: CellState) -> Self::Output {
        let rhs: u8 = rhs.into();

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

impl Into<bool> for CellState {
    fn into(self) -> bool {
        match self {
            Self::Alive => true,
            Self::Dead => false,
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

use std::ops::{
    Index,
    IndexMut,
};


type Row = [Option<bool>; Board::SIZE];
type Grid = [Row; Board::SIZE];

#[derive(Default)]
pub struct Board(Grid);

impl Index<usize> for Board {
    type Output = Option<bool>;
    fn index(&self, idx: usize) -> &Self::Output {
        let i = idx / Self::SIZE;
        let j = idx % Self::SIZE;
        &self.0[i][j]
    }
}
impl IndexMut<usize> for Board {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        let i = idx / Self::SIZE;
        let j = idx % Self::SIZE;
        &mut self.0[i][j]
    }
}

impl Board {
    pub(crate) const SIZE: usize = 3;
    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Row> {
        self.0.iter()
    }
    pub(crate) fn is_full(&self) -> bool {
        self.iter().all(|row| row.iter().all(Option::is_some))
    }
    pub(crate) fn has_won(&self, turn: bool) -> bool {
        // rows check
        let rows = self.iter().any(|row| {
            row.iter().all(|cell| {
                matches!(cell, Some(v) if *v == turn)
            })
        });
        // cols check
        let cols = 
            (0..Self::SIZE).any(|i| 
                self.iter().all(|row| matches!(row[i], Some(v) if v == turn))
            );

        // diagonals check
        let diagonals =
            (0..Self::SIZE).all(|i| matches!(self.0[i][i], Some(v) if v == turn)) ||
                (0..Self::SIZE).all(|i| matches!(self.0[i][Self::SIZE - i - 1], Some(v) if v == turn));
        
        rows || cols || diagonals
    }
}

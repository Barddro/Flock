use std::{usize};

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl <T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![default; rows * cols],
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[self.index(row, col)].clone()
    }

    pub fn put(&mut self, row: usize, col: usize, e: T) {
        let i = self.index(row, col);
        self.data[i] = e;
    }

}






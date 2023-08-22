// TODO: Optimise, Logger, Rayon::scope, documentation, Serde serialization

use rand::{distributions::Uniform, Rng};

pub mod generator;
pub mod solver;

/// Represents square grid in KenKen as `Vec<u8>` of size `u8`*`u8`
#[derive(Clone, Debug)]
pub struct Grid(Vec<u8>, u8);
impl Grid {
    /// Generates new valid grid KenKen grid
    pub fn new(size: u8) -> Self {
        let mut grid: Vec<u8> = Vec::new();
        for i in 0..size {
            for j in 0..size {
                let value = (i + j) % size + 1;
                grid.push(value);
            }
        }

        Self(grid, size)
    }
    pub fn print(&self) {
        let size = self.1 as usize;
        let mut offset = 0;
        if size > 9 {
            offset = 1;
        }
        println!("/{:-^1$}\\", "", (2 + offset) * size + 1);
        for i in (0..self.0.len()).step_by(size) {
            let row = &self.0[i..i + size];
            print!("| ");
            for cell in row {
                print!("{:01$} ", cell, offset + 1);
            }
            println!("|");
        }
        println!("\\{:-^1$}/", "", (2 + offset) * size + 1);
    }
    pub fn shuffle(&mut self, count: u32) {
        let mut rng = rand::thread_rng();
        let uni = Uniform::new(0, self.1);
        for _ in 0..count {
            self.swap_row(rng.sample(uni), rng.sample(uni));
            self.swap_col(rng.sample(uni), rng.sample(uni));
            self.transpose()
        }
    }
    fn swap_row(&mut self, row1: u8, row2: u8) {
        let size = self.1 as usize;
        let start1 = row1 as usize * size;
        let start2 = row2 as usize * size;

        for i in 0..size {
            self.0.swap(start1 + i, start2 + i);
        }
    }
    fn swap_col(&mut self, col1: u8, col2: u8) {
        let size = self.1 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        for row in 0..size {
            let index1 = row * size + col1;
            let index2 = row * size + col2;
            self.0.swap(index1, index2);
        }
    }
    fn transpose(&mut self) {
        let size = self.1 as usize;
        for i in 0..size {
            for j in (i + 1)..size {
                let index1 = i * size + j;
                let index2 = j * size + i;
                self.0.swap(index1, index2);
            }
        }
    }
}

/// Variants of operations in KenKen cage
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Free,
}
/// Data type for KenKen cage
#[derive(Clone, Debug)]
pub struct Cage {
    pub target: u32,
    pub operation: MathOp,
    ///indexes, Need to be ordered start-end or end-start
    pub cells: Vec<usize>,
}

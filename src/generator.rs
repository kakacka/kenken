use super::{Cage, Grid, MathOp};
use crate::solver::KenkenPuzzle;
use rand::{distributions::WeightedIndex, prelude::Distribution, seq::SliceRandom, Rng};
use std::cmp::{max, min};

///`Difficulty` defines target difficulty for puzzle from generator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
    Any, //nolimit
}
impl Difficulty {
    //Difficulty settings based on depth
    fn from_depth(depth: usize) -> Self {
        if depth == 0 {
            return Difficulty::Easy;
        }
        if depth >= 1 && depth <= 2 {
            return Difficulty::Medium;
        }
        if depth >= 3 && depth <= 5 {
            return Difficulty::Hard;
        }

        return Difficulty::Extreme;
    }
    fn test_depth(&self, depth: usize) -> bool {
        match self {
            Difficulty::Any => {
                return true;
            }
            d => {
                return *d == Difficulty::from_depth(depth);
            }
        }
    }
}
enum Directions {
    Up,
    Down,
    Left,
    Right,
}
/// Structure with parameters for KenKen generator
/// Used to generate new puzzles
pub struct KenkenGenerator {
    pub size: u8,
    pub difficulty: Difficulty,
    pub max_depth: usize,
    pub unique: bool,
    pub max_cage_size: usize,
    pub operation_weight: [f64; 5], //for each option in MathOp,
}

impl KenkenGenerator {
    pub fn new(
        size: u8,
        difficulty: Difficulty,
        max_depth: usize,
        unique: bool,
        max_cage_size: usize,
        operations: Option<[f64; 5]>,
    ) -> Self {
        Self {
            size,
            difficulty,
            max_depth,
            unique,
            max_cage_size,
            operation_weight: match operations {
                Some(x) => x,
                None => [1.0, 1.3, 1.0, 1.6, 0.15],
            },
        }
    }
    ///Generate KenKen puzzles with current generator instance.
    /// # Arguments
    /// * `count` - Target number of puzzles to return.
    /// * `validate` - Throw away every puzzle that isn't valid KenKen puzzle and doesn't satisfy `difficulty` and 'unique'.
    /// * `grid` - If specified, this grid will be used to create cages, instead of random grid.
    pub fn generate_puzzles(
        &self,
        count: u32,
        validate: bool,
        grid: Option<&Grid>,
    ) -> Vec<KenkenPuzzle> {
        let mut counter = 0;
        let mut total = 0;
        let mut puzzles = Vec::<KenkenPuzzle>::new();
        while counter != count {
            let puzzle;
            if let Some(grid) = grid {
                puzzle = self.generate_puzzle_with_grid(grid);
            } else {
                puzzle = self.generate_puzzle();
            };
            total += 1;
            if !validate || self.validate_puzzle(&puzzle) {
                puzzles.push(puzzle);
                counter += 1;
            }
        }
        println!("Generated: {total} / Valid {counter}");
        return puzzles;
    }
    fn validate_puzzle(&self, puzzle: &KenkenPuzzle) -> bool {
        if let Ok(solutions) = puzzle.solve(&self.max_depth, &2) {
            if let Some(solutions) = solutions {
                println!("sol count: {}", solutions.len());
                for sol in &solutions {
                    println!("depth: {}", sol.depth);
                }
                if solutions.len() != 1 && self.unique {
                    return false;
                }

                return self.difficulty.test_depth(solutions[0].depth);
            }
        } else {
            println!("too deep")
        }
        println!("no sol");
        return false;
    }
    fn generate_puzzle(&self) -> KenkenPuzzle {
        let mut grid = self.create_grid();
        grid.shuffle(self.size as u32 * 2);
        grid.print();
        return self.generate_puzzle_with_grid(&grid);
    }
    fn generate_puzzle_with_grid(&self, grid: &Grid) -> KenkenPuzzle {
        let mut unallocated_cells: Vec<usize> = (0..(self.size as usize).pow(2)).collect();
        let mut cages = Vec::<Cage>::new();
        while unallocated_cells.len() > 0 {
            cages.push(self.generate_cage(&grid, &mut unallocated_cells));
        }

        return KenkenPuzzle::new(self.size, cages);
    }
    fn create_grid(&self) -> Grid {
        Grid::new(self.size)
    }

    fn generate_cage(&self, grid: &Grid, unallocated: &mut Vec<usize>) -> Cage {
        let mut cells = Vec::<usize>::new();
        let mut rng = rand::thread_rng();
        let mut last = unallocated.remove(0);
        cells.push(last);
        let directions = [
            Directions::Up,
            Directions::Down,
            Directions::Left,
            Directions::Right,
        ]; //up,down,left,right
        let size = self.size as usize;
        let chance = 1.0 - (self.operation_weight[4] / self.operation_weight.iter().sum::<f64>());
        'cage_grow: while cells.len() < self.max_cage_size
            && rng.gen_bool(chance / cells.len() as f64)
        {
            for dir in directions.choose_multiple(&mut rng, directions.len()) {
                let neighbor = match dir {
                    Directions::Up => {
                        if last < size {
                            continue;
                        }
                        last - size
                    }
                    Directions::Down => {
                        if last + size >= size.pow(2) {
                            continue;
                        }
                        last + size
                    }
                    Directions::Left => {
                        if last % size == 0 {
                            continue;
                        }
                        last - 1
                    }
                    Directions::Right => {
                        if (last + 1) % size == 0 {
                            continue;
                        }
                        last + 1
                    }
                };
                if let Ok(idx) = unallocated.binary_search(&neighbor) {
                    last = unallocated.remove(idx);
                    cells.push(last);
                    continue 'cage_grow; //found new cell so continue cycle
                }
            }
            if chance == 1.0 && cells.len() == 1 {
                unallocated.push(last);
                return self.generate_cage(grid, unallocated); //retry
            }
            break; //no available cell found, break cycle
        }

        let mut operation = MathOp::Free;
        let clen = cells.len();
        let operations = [
            MathOp::Add,
            MathOp::Sub,
            MathOp::Mul,
            MathOp::Div,
            MathOp::Free,
        ];
        let mut weights = WeightedIndex::new(&self.operation_weight).unwrap();
        loop {
            let op = operations[weights.sample(&mut rng)];
            match op {
                MathOp::Add => {
                    if clen > 1 {
                        operation = MathOp::Add;
                        break;
                    }
                    weights.update_weights(&[(0, &0f64)]).unwrap();
                }
                MathOp::Sub => {
                    if clen == 2 {
                        operation = MathOp::Sub;
                        break;
                    }
                    weights.update_weights(&[(1, &0f64)]).unwrap()
                }
                MathOp::Mul => {
                    if clen > 1 {
                        operation = MathOp::Mul;
                        break;
                    }
                    weights.update_weights(&[(2, &0f64)]).unwrap()
                }
                MathOp::Div => {
                    if clen == 2 {
                        let (n, m) = (grid.0[cells[0]], grid.0[cells[1]]);
                        if max(n, m) % min(n, m) != 0 {
                            continue;
                        }
                        operation = MathOp::Div;
                        break;
                    }
                    weights.update_weights(&[(3, &0f64)]).unwrap()
                }
                MathOp::Free => {
                    if clen == 1 {
                        break;
                    }
                    weights.update_weights(&[(4, &0f64)]).unwrap()
                }
            }
        }
        let target = match operation {
            MathOp::Add => {
                let mut sum = 0;
                for idx in &cells {
                    sum += grid.0[*idx] as u32;
                }
                sum
            }
            MathOp::Sub => {
                let (n, m) = (grid.0[cells[0]], grid.0[cells[1]]);
                (max(n, m) - min(n, m)) as u32
            }
            MathOp::Mul => {
                let mut product = 1;
                for idx in &cells {
                    product *= grid.0[*idx] as u32;
                }
                product
            }
            MathOp::Div => {
                let (n, m) = (grid.0[cells[0]], grid.0[cells[1]]);
                (max(n, m) / min(n, m)) as u32
            }
            MathOp::Free => grid.0[cells[0]] as u32,
        };
        return Cage {
            target,
            operation,
            cells,
        };
    }
}

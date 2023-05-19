use super::Grid;

/// Stores grid and depth where the solution was found
#[derive(Clone, Debug)]
pub struct Solution {
    pub grid: Grid,
    pub depth: usize,
}
impl Solution {
    pub fn from_area(area: &Area, size: u8, depth: usize) -> Self {
        let mut solved = vec![];
        for cell in area {
            match cell {
                Cell::Solution(n) => solved.push(*n),
                Cell::Possible(_) => panic!("Area is unsolved"),
            }
        }
        return Self {
            grid: Grid(solved, size),
            depth,
        };
    }
    pub fn print(&self) {
        let size = (self.grid.0.len() as f64).sqrt() as usize;
        println!("/{:-^1$}\\", "", 2 * size + 1);
        for i in (0..self.grid.0.len()).step_by(size) {
            let row = &self.grid.0[i..i + size];
            print!("| ");
            for cell in row {
                print!("{cell} ");
            }
            println!("|");
        }
        println!("\\{:-^1$}/", "", 2 * size + 1);
    }
}

#[derive(Clone, Debug)]
pub enum Cell {
    Solution(u8),
    Possible(Vec<u8>),
}
type Area = Vec<Cell>; //I am NOT changing this to tuple struct
                       //EDIT: okay i will rewrite this one day.....

trait ApplyPossibilities {
    fn apply_sequences(&mut self, sequences: &Vec<Vec<u8>>);
    fn apply_pairs(&mut self, pairs: &Vec<(u8, u8)>);
}
trait BestCandidate {
    fn get_best_candidate(&self) -> Option<(usize, usize)>;
}
impl ApplyPossibilities for Area {
    fn apply_sequences(&mut self, sequences: &Vec<Vec<u8>>) {
        //Add, Mul
        for (e, cell) in self.iter_mut().enumerate() {
            match cell {
                Cell::Possible(v) => {
                    v.clear();
                    for seq in sequences.iter() {
                        if !v.contains(&seq[e]) {
                            v.push(seq[e]);
                        }
                    }
                }
                _ => (),
            }
        }
    }
    fn apply_pairs(&mut self, pairs: &Vec<(u8, u8)>) {
        //Div, Sub
        if self.len() != 2 {
            panic!("Area needs to have just 2 cells");
        }
        match &mut self[0] {
            Cell::Possible(v) => {
                v.clear();
                for pair in pairs {
                    if !v.contains(&pair.0) {
                        v.push(pair.0);
                    }
                }
            }
            _ => (),
        }
        match &mut self[1] {
            Cell::Possible(v) => {
                v.clear();
                for pair in pairs {
                    if !v.contains(&pair.1) {
                        v.push(pair.1);
                    }
                }
            }
            _ => (),
        }
    }
}
impl BestCandidate for Area {
    fn get_best_candidate(&self) -> Option<(usize, usize)> {
        let mut best_candidate: Option<(usize, usize)> = None; //len, index
        for (i, cell) in self.iter().enumerate() {
            match cell {
                Cell::Possible(v) => {
                    if let Some(x) = best_candidate {
                        if x.0 > v.len() {
                            best_candidate = Some((v.len(), i));
                        }
                    } else {
                        best_candidate = Some((v.len(), i));
                    }
                }
                _ => (),
            }
        }
        return best_candidate;
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
    pub cells: Vec<usize>, //indexes, Need to be ordered start-end or end-start
}
impl Cage {
    fn solve(&self, area: &mut Area, size: u8) {
        let len = area.len();
        match self.operation {
            MathOp::Add => {
                let seq = sequence_gen::generate_sequences_sum(len, size, self.target, Some(area));
                area.apply_sequences(&seq);
            }
            MathOp::Mul => {
                let seq = sequence_gen::generate_sequences_mul(len, size, self.target, Some(area));
                area.apply_sequences(&seq);
            }
            MathOp::Div => {
                if len != 2 {
                    panic!(
                        "Division can't be applied to {len} cells. Only 2-cell cage can divide."
                    );
                }
                let a = Some((area.get(0).unwrap(), area.get(1).unwrap()));
                let seq = sequence_gen::generate_sequences_div(size, self.target, a);
                area.apply_pairs(&seq);
            }
            MathOp::Sub => {
                if len != 2 {
                    panic!("Subtraction can't be applied to {len} cells. Only 2-cell cage can subtract.");
                }
                let a = Some((area.get(0).unwrap(), area.get(1).unwrap()));
                let seq = sequence_gen::generate_sequences_sub(size, self.target, a);
                area.apply_pairs(&seq);
            }
            MathOp::Free => {
                if len != 1 {
                    panic!("Free cage can't be applied to {len} cells. Only 1-cell cage can be a freebie.");
                }
                let cell = &mut area[0];
                if let Cell::Possible(v) = cell {
                    v.clear();
                    v.push(self.target as u8);
                }
            }
        }
    }
}

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Err(e),
        }
    };
}
#[derive(Clone, Copy, Debug)]
pub enum SolverError {
    /// Exceeded `maximum_depth`
    DepthExceeded,
}
/// Structure defining puzzle in KenKen
/// Used to find solutions for puzzle
#[derive(Clone, Debug)]
pub struct KenkenPuzzle {
    pub size: u8,
    pub cages: Vec<Cage>,
}
impl KenkenPuzzle {
    pub fn new(size: u8, cages: Vec<Cage>) -> Self {
        Self { size, cages }
    }
    pub fn format(&self) -> String {
        let mut string = format!("{}<", self.size);
        for cage in &self.cages {
            let mut cell_str = String::from("");
            for id in &cage.cells {
                cell_str += &format!("{},", id);
            }
            cell_str.remove(cell_str.len() - 1);
            string += &format!(
                "{}.{}.{}>",
                cage.target,
                match cage.operation {
                    MathOp::Add => "a",
                    MathOp::Mul => "m",
                    MathOp::Div => "d",
                    MathOp::Sub => "s",
                    MathOp::Free => "f",
                },
                cell_str
            );
        }
        string
    }
    /// Finds and returns solutions to puzzle.
    /// Time to solve rapidly grows with puzzle size.
    /// # Arguments
    /// * `max_depth` - Limits `depth` when solving puzzle. If exceeded search will be aborted and return `None`. (`depth` counts recursion cycles which occurs when guessing solutions is needed)
    /// * 'max_solutions' - Stops search early if found enough solutions. When set to `0` solutions are unlimited.
    pub fn solve(
        &self,
        max_depth: &usize,
        max_solutions: &usize,
    ) -> Result<Option<Vec<Solution>>, SolverError> {
        //Returns all found solutions
        return self.find_solutions(self.get_area(), 0, max_depth, max_solutions);
    }
    fn get_area(&self) -> Area {
        let possible = Vec::from_iter(1..self.size + 1);
        vec![Cell::Possible(possible); self.size as usize * self.size as usize]
    }
    /// Main
    /// recursive function
    fn find_solutions(
        &self,
        mut board: Area,
        depth: usize,
        max_depth: &usize,
        max_solutions: &usize,
    ) -> Result<Option<Vec<Solution>>, SolverError> {
        if depth > *max_depth {
            return Err(SolverError::DepthExceeded); //too deep, stop search
        }
        let mut progress = true;
        while progress {
            //Loop will continue as long it can solve cells with just deduction
            progress = false;
            self.deduction(board.as_mut());
            for cell in board.iter_mut() {
                match cell {
                    Cell::Possible(v) => {
                        if v.len() == 0 {
                            //Solution impossible in this state
                            return Ok(None);
                        } else if v.len() == 1 {
                            //Cell solved
                            *cell = Cell::Solution(v[0]);
                            progress = true;
                        }
                    }
                    _ => (),
                }
            }
        }
        //Candidate with least possibilities
        if let Option::Some(candidate) = board.get_best_candidate() {
            //guess
            let mut guess = board.clone();
            if let Cell::Possible(v) = &mut board[candidate.1] {
                let mut solutions = vec![];
                //To make a guess put possibility as solution into board clone and try to solve that
                let num = v[0];
                guess[candidate.1] = Cell::Solution(num);
                match unwrap_or_return!(self.find_solutions(
                    guess,
                    depth + 1,
                    max_depth,
                    max_solutions
                )) {
                    Some(mut sol1) => {
                        //Guess was correct
                        solutions.append(&mut sol1);
                        if solutions.len() == *max_solutions {
                            //found enough solutions
                            return Ok(Some(solutions));
                        }
                        v.remove(0);
                        //try remaining guesses
                        match unwrap_or_return!(self.find_solutions(
                            board,
                            depth + 1,
                            max_depth,
                            max_solutions
                        )) {
                            Some(mut sol2) => {
                                //Don't pass analysis, because first solution was already found
                                solutions.append(&mut sol2);
                                return Ok(Some(solutions));
                            }
                            None => return Ok(Some(solutions)),
                        }
                    }
                    None => {
                        //Ruled out a possibility, so rerun this function with currect board
                        v.remove(0);
                        return self.find_solutions(board, depth + 1, max_depth, max_solutions);
                    }
                }
            } else {
                //unreachable
                panic!("Cosmic ray detected.");
            }
        } else {
            println!("sol");
            return Ok(Some(vec![Solution::from_area(&board, self.size, depth)]));
            //no more candidates => all solved
        }
    }
    fn deduction(&self, board: &mut Area) {
        //find possibilities within cages
        for cage in self.cages.iter() {
            let mut area = Vec::<Cell>::with_capacity(cage.cells.len());
            for i in cage.cells.iter() {
                area.push(board[*i].clone());
            }
            cage.solve(&mut area, self.size);
            for (e, i) in cage.cells.iter().enumerate() {
                board[*i].clone_from(&area[e]);
            }
        }
        //check rows
        for i in (0..board.len()).step_by(self.size as usize) {
            let row = &mut board[i..i + self.size as usize];
            let mut filter_out = Vec::new();
            for cell in row.iter() {
                if let Cell::Solution(n) = cell {
                    filter_out.push(*n);
                }
            }
            for cell in row.iter_mut() {
                if let Cell::Possible(v) = cell {
                    v.retain(|x| !filter_out.contains(x));
                }
            }
        }
        //check columns
        for col in 0..self.size as usize {
            let mut contained = Vec::new();
            for i in (col..board.len()).step_by(self.size as usize) {
                let cell = &board[i];
                if let Cell::Solution(n) = cell {
                    contained.push(*n);
                }
            }
            for i in (col..board.len()).step_by(self.size as usize) {
                let cell = &mut board[i];
                if let Cell::Possible(v) = cell {
                    v.retain(|x| !contained.contains(x));
                }
            }
        }
    }
}
// Generator functions to generate possible values in cage
mod sequence_gen {

    use super::Cell;
    pub fn generate_sequences_sub(
        size: u8,
        target: u32,
        area: Option<(&Cell, &Cell)>,
    ) -> Vec<(u8, u8)> {
        let t = target as u8;
        let mut pairs = Vec::new();
        for n in 1..=size {
            let d = n.checked_sub(t);
            if let Some(d) = d {
                if d > 0 && d <= size && d != n {
                    pairs.push((n, d));
                }
            }
        }
        let mut sequences = Vec::new();
        for p in pairs.iter() {
            let mut pass = (true, true);
            if let Some(a) = area {
                match a.0 {
                    //Check if number is possible within a cell
                    Cell::Solution(n) => {
                        if *n != p.0 {
                            pass.0 = false;
                        }
                        if *n != p.1 {
                            pass.1 = false;
                        }
                    }
                    Cell::Possible(v) => {
                        if !v.contains(&p.0) {
                            pass.0 = false;
                        }
                        if !v.contains(&p.1) {
                            pass.1 = false;
                        }
                    }
                }
                match a.1 {
                    //Check if number is possible within a cell
                    Cell::Solution(n) => {
                        if *n != p.1 {
                            pass.0 = false;
                        }
                        if *n != p.0 {
                            pass.1 = false;
                        }
                    }
                    Cell::Possible(v) => {
                        if !v.contains(&p.1) {
                            pass.0 = false;
                        }
                        if !v.contains(&p.0) {
                            pass.1 = false;
                        }
                    }
                }
            }
            if pass.0 {
                sequences.push(*p);
            }
            if pass.1 {
                sequences.push((p.1, p.0));
            }
        }
        sequences
    }
    pub fn generate_sequences_div(
        size: u8,
        target: u32,
        area: Option<(&Cell, &Cell)>,
    ) -> Vec<(u8, u8)> {
        //DIV must be only on two cells
        let t = target as u8;
        let mut pairs = Vec::new();
        for i in 1..=size {
            for j in 1..=size {
                if i != j && i % j == 0 && i / j == t {
                    pairs.push((i, j));
                }
            }
        }
        let mut sequences = Vec::new();
        for p in pairs.iter() {
            let mut pass = (true, true);
            if let Some(a) = area {
                match a.0 {
                    //Check if number is possible within a cell
                    Cell::Solution(n) => {
                        if *n != p.0 {
                            pass.0 = false;
                        }
                        if *n != p.1 {
                            pass.1 = false;
                        }
                    }
                    Cell::Possible(v) => {
                        if !v.contains(&p.0) {
                            pass.0 = false;
                        }
                        if !v.contains(&p.1) {
                            pass.1 = false;
                        }
                    }
                }
                match a.1 {
                    //Check if number is possible within a cell
                    Cell::Solution(n) => {
                        if *n != p.1 {
                            pass.0 = false;
                        }
                        if *n != p.0 {
                            pass.1 = false;
                        }
                    }
                    Cell::Possible(v) => {
                        if !v.contains(&p.1) {
                            pass.0 = false;
                        }
                        if !v.contains(&p.0) {
                            pass.1 = false;
                        }
                    }
                }
            }
            if pass.0 {
                sequences.push(*p);
            }
            if pass.1 {
                sequences.push((p.1, p.0));
            }
        }
        sequences
    }
    pub fn generate_sequences_mul(
        len: usize,
        max: u8,
        target: u32,
        area: Option<&Vec<Cell>>,
    ) -> Vec<Vec<u8>> {
        let mut sequences = Vec::new();
        let mut sequence = Vec::new();
        gen_seq_mul_recursive(len, max, target, area, &mut sequence, &mut sequences);
        sequences
    }
    fn gen_seq_mul_recursive(
        len: usize,
        max: u8,
        target: u32,
        area: Option<&Vec<Cell>>,
        sequence: &mut Vec<u8>,
        sequences: &mut Vec<Vec<u8>>,
    ) {
        if sequence.len() == len {
            let mut product = 1u32;
            for n in sequence.iter() {
                product *= *n as u32;
            }
            if product == target {
                sequences.push(sequence.clone());
            }
            return;
        }
        for num in 1..=max {
            if sequence.len() > 0 && sequence[sequence.len() - 1] == num {
                continue; // Avoid adjacent identical numbers
            }
            if let Some(a) = area {
                match a.get(sequence.len()).unwrap() {
                    //Check if number is possible within a cell
                    Cell::Solution(n) => {
                        if *n != num {
                            continue;
                        }
                    }
                    Cell::Possible(v) => {
                        if !v.contains(&num) {
                            continue;
                        }
                    }
                }
            }
            sequence.push(num);
            gen_seq_mul_recursive(len, max, target, area, sequence, sequences);
            sequence.pop();
        }
    }
    pub fn generate_sequences_sum(
        len: usize,
        max: u8,
        target: u32,
        area: Option<&Vec<Cell>>,
    ) -> Vec<Vec<u8>> {
        let mut sequences = Vec::new();
        gen_seq_sum_recursive(&mut sequences, &mut Vec::new(), len, max, target, area, 0);
        sequences
    }
    fn gen_seq_sum_recursive(
        sequences: &mut Vec<Vec<u8>>,
        sequence: &mut Vec<u8>,
        len: usize,
        max: u8,
        target: u32,
        area: Option<&Vec<Cell>>,
        sum: u32,
    ) {
        if sequence.len() == len {
            if sum == target {
                sequences.push(sequence.clone());
            }
            return;
        }
        for num in 1..=max {
            if sequence.len() > 0 && sequence[sequence.len() - 1] == num {
                continue;
            }
            if let Some(a) = area {
                match a.get(sequence.len()).unwrap() {
                    //Check if number is possible within a cell
                    Cell::Solution(n) => {
                        if *n != num {
                            continue;
                        }
                    }
                    Cell::Possible(v) => {
                        if !v.contains(&num) {
                            continue;
                        }
                    }
                }
            }
            if sum + num as u32 + (len - sequence.len() - 1) as u32 * 1 > target {
                continue;
            }
            if sum + num as u32 + (len - sequence.len() - 1) as u32 * (max as u32) < target {
                continue;
            }
            sequence.push(num);
            gen_seq_sum_recursive(
                sequences,
                sequence,
                len,
                max,
                target,
                area,
                sum + (num as u32),
            );
            sequence.pop();
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sequences_mul() {
        let seq = sequence_gen::generate_sequences_mul(3, 5, 18, None); //Seq.0 [3, 2, 3]
        assert_eq!(seq.len(), 1);
        assert_eq!(seq[0].len(), 3);
        assert_eq!(seq[0], vec![3, 2, 3]);
        let mut area = vec![Cell::Possible(vec![1,2,3,4,5]);3];
        area[0] = Cell::Solution(3);
        let seq = sequence_gen::generate_sequences_mul(3, 5, 15, Some(&area));
        assert_eq!(seq.len(), 2);
        area[0] = Cell::Possible(vec![3,4,5]);
        let seq = sequence_gen::generate_sequences_mul(3, 5, 15, Some(&area));
        assert_eq!(seq.len(), 4);
        area[0] = Cell::Possible(vec![2,4]);
        let seq = sequence_gen::generate_sequences_mul(3, 5, 15, Some(&area));
        assert_eq!(seq.len(), 0);
    }
    #[test]
    fn  generate_sequences_div() {
        assert_eq!(sequence_gen::generate_sequences_div(5, 2, None).len(), 4);
    }
    #[test]
    fn solve_test() {
        let board = KenkenPuzzle::new(3, vec![ //kenken tutorial puzzle
            Cage {target: 5, operation: MathOp::Add, cells: vec![0,1]},
            Cage {target: 3, operation: MathOp::Add, cells: vec![2,5]},
            Cage {target: 4, operation: MathOp::Add, cells: vec![3,6]},
            Cage {target: 3, operation: MathOp::Add, cells: vec![4,7]},
            Cage {target: 3, operation: MathOp::Free, cells: vec![8]}
            ]);
        solve_test_board(&board, vec![2,3,1,3,1,2,1,2,3]);
        let board = KenkenPuzzle::new(3, vec![
            Cage {target: 5, operation: MathOp::Add, cells: vec![0,1]},
            Cage {target: 1, operation: MathOp::Sub, cells: vec![2,5]},
            Cage {target: 3, operation: MathOp::Div, cells: vec![3,4]},
            Cage {target: 2, operation: MathOp::Free, cells: vec![6]},
            Cage {target: 3, operation: MathOp::Div, cells: vec![7,8]}
        ]);
        solve_test_board(&board, vec![3,2,1,1,3,2,2,1,3]);
        let board = KenkenPuzzle::new(4, vec![
            Cage {target: 24, operation: MathOp::Mul, cells: vec![0,4,5]},
            Cage {target: 2, operation: MathOp::Sub, cells: vec![1,2]},
            Cage {target: 7, operation: MathOp::Add, cells: vec![3,7,11]},
            Cage {target: 12, operation: MathOp::Add, cells: vec![6,10,14,15]},
            Cage {target: 2, operation: MathOp::Div, cells: vec![8,12]},
            Cage {target: 3, operation: MathOp::Sub, cells: vec![9,13]}
            ]);
        solve_test_board(&board, vec![4,3,1,2,3,2,4,1,2,1,3,4,1,4,2,3]);
        let board = KenkenPuzzle::new(5, vec![
            Cage {target: 3, operation: MathOp::Sub, cells: vec![0,5]},
            Cage {target: 12, operation: MathOp::Add, cells: vec![1,2,3]},
            Cage {target: 10, operation: MathOp::Mul, cells: vec![4,9]},
            Cage {target: 6, operation: MathOp::Add, cells: vec![6,7,8]},
            Cage {target: 3, operation: MathOp::Sub, cells: vec![10,15]},
            Cage {target: 2, operation: MathOp::Div, cells: vec![11,16]},
            Cage {target: 9, operation: MathOp::Add, cells: vec![12,13,14]},
            Cage {target: 40, operation: MathOp::Mul, cells: vec![17,22,21]},
            Cage {target: 2, operation: MathOp::Sub, cells: vec![18,23]},
            Cage {target: 3, operation: MathOp::Sub, cells: vec![19,24]},
            Cage {target: 3, operation: MathOp::Free, cells: vec![20]}
            ]);
        solve_test_board(&board, vec![1,3,5,4,2,4,1,3,2,5,2,4,1,5,3,5,2,4,3,1,3,5,2,1,4]);
        let board = KenkenPuzzle::new(9, vec![
            Cage {target: 8, operation: MathOp::Sub, cells: vec![0,1]},
            Cage {target: 7, operation: MathOp::Free, cells: vec![2]},
            Cage {target: 2, operation: MathOp::Sub, cells: vec![3,4]},
            Cage {target: 3, operation: MathOp::Sub, cells: vec![5,6]},
            Cage {target: 10, operation: MathOp::Add, cells: vec![7,16,25]},
            Cage {target: 90, operation: MathOp::Mul, cells: vec![8,17,26]},
            Cage {target: 3, operation: MathOp::Div, cells: vec![9, 10]},
            Cage {target: 11, operation: MathOp::Add, cells: vec![11,20]},
            Cage {target: 7, operation: MathOp::Add, cells: vec![12,21]},
            Cage {target: 1, operation: MathOp::Sub, cells: vec![13,22]},
            Cage {target: 2, operation: MathOp::Div, cells: vec![14,15]},
            Cage {target: 2, operation: MathOp::Sub, cells: vec![18,19]},
            Cage {target: 24, operation: MathOp::Mul, cells: vec![23,32,41]},
            Cage {target: 9, operation: MathOp::Free, cells: vec![24]},
            Cage {target: 5, operation: MathOp::Sub, cells: vec![27, 28]},
            Cage {target: 8, operation: MathOp::Sub, cells: vec![29,38]},
            Cage {target: 15, operation: MathOp::Add, cells: vec![30, 31]},
            Cage {target: 60, operation: MathOp::Mul, cells: vec![33,34,35]},
            Cage {target: 70, operation: MathOp::Mul, cells: vec![36,37,46,45]},
            Cage {target: 7, operation: MathOp::Sub, cells: vec![39,40]},
            Cage {target: 2, operation: MathOp::Div, cells: vec![42,51]},
            Cage {target: 2, operation: MathOp::Sub, cells: vec![43,52]},
            Cage {target: 56, operation: MathOp::Mul, cells: vec![44,53]},
            Cage {target: 1, operation: MathOp::Sub, cells: vec![47,48]},
            Cage {target: 54, operation: MathOp::Mul, cells: vec![50,49,58]},
            Cage {target: 30, operation: MathOp::Mul, cells: vec![54,55]},
            Cage {target: 3, operation: MathOp::Sub, cells: vec![56,57]},
            Cage {target: 8, operation: MathOp::Sub, cells: vec![59,60]},
            Cage {target: 31, operation: MathOp::Add, cells: vec![61,70,79,80]},
            Cage {target: 2, operation: MathOp::Div, cells: vec![62,71]},
            Cage {target: 2, operation: MathOp::Div, cells: vec![63,72]},
            Cage {target: 4, operation: MathOp::Div, cells: vec![64,65]},
            Cage {target: 3, operation: MathOp::Div, cells: vec![66,75]},
            Cage {target: 5, operation: MathOp::Free, cells: vec![67]},
            Cage {target: 15, operation: MathOp::Add, cells: vec![68,77,76]},
            Cage {target: 11, operation: MathOp::Add, cells: vec![69,78]},
            Cage {target: 2, operation: MathOp::Div, cells: vec![73,74]}
            ]);
            solve_test_board(&board, vec![9,1,7,6,4,5,8,2,3,3,9,8,5,7,4,2,1,6,6,4,3,2,8,1,9,7,5,
                7,2,1,9,6,8,5,3,4,2,5,9,8,1,3,6,4,7,1,7,5,4,9,2,3,6,8,
                5,6,4,7,3,9,1,8,2,4,8,2,3,5,6,7,9,1,8,3,6,1,2,7,4,5,9]);
    }
    fn solve_test_board(board: &KenkenPuzzle, expected: Vec<u8>) {
        if let Some(x) = board.solve(&40, &1).unwrap() {
            assert_eq!(x[0].grid.0, expected);
        }
        else {
            assert!(false);
        }
    }
}

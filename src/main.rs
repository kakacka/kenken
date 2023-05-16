
fn main() {
    println!("Kenken solver:");
    use kenken::solver::*;
    // let board = KenkenBoard::with(3, vec![
    //     Cage {target: 6, operation: MathOp::Add, cells: vec![0,3,6]},
    //     Cage {target: 1, operation: MathOp::Sub, cells: vec![1,2]},
    //     Cage {target: 7, operation: MathOp::Add, cells: vec![4,6,8,7]}
    //     ]);
    let board = KenkenBoard::with(9, vec![
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

    let solutions = board.solve_all();
    if let Some(x) = solutions {
        println!("Solutions: {}", x.len());
        for sol in x {
            sol.print_solution();
            println!("{}", sol.depth);
        }
    }
    else {
        println!("No solution");
    }

}
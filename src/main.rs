#[rustfmt::skip]
fn main() {
    println!("Kenken solver:");
    use kenken::solver::*;
    use kenken::generator::*;
    let gen = KenkenGenerator::new(6, Difficulty::Extreme, 24, true, 5, None);
    println!("{:?}",gen.generate_puzzles(1, true, None)[0]);

}

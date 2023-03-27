mod solver;
mod parser;

fn main() {
    let (file, num_sol) = parser::parse_args();
    let mut grid = parser::parse_file(file);
    let mut counter: u32 = 0;
    solver::solve_sudoku(&mut grid, num_sol, &mut counter);
}

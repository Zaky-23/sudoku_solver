use std::process::exit;

pub fn is_val_possible(grid: &[[u8; 9]; 9], x: usize, y: usize, val: u8) ->  bool {
    
    // Check row
    for i in 0..9 {
        if grid[y][i] == val {
            return false;
        }
    }

    // Check column
    for i in 0..9 {
        if grid[i][x] == val {
            return false;
        }
    }

    // Check inner-grid
    let xc = (x / 3) * 3;
    let yc = (y / 3) * 3;

    for j in 0..3 {
        for i in 0..3 {
            if grid[yc + j][xc + i] == val {
                return false;
            }
        }
    }
    true
}

pub fn solve_sudoku(grid: &mut [[u8; 9]; 9], num_sol: u32, counter: &mut u32) {

    for y in 0..9 {
        for x in 0..9 {
            if grid[y][x] != 0 {
                continue;
            }

            for val in 0..=9 {
                if !is_val_possible(grid, x, y, val) {
                    continue;
                }

                grid[y][x] = val;
                // Branch off and solve for the new grid state
                solve_sudoku(grid, num_sol, counter);
                // Undo the change when it returns; backtrack 
                grid[y][x] = 0;
            }

            if *counter == num_sol {
                exit(0)
            }

            // When there is no possible values left aka
            // backtracking
            return;
        }
    }

    *counter += 1;

    print_grid(grid);
}

fn print_grid(grid: &[[u8; 9]; 9]) {
    let mut x = 0;
    let mut y = 0;

    loop {
        print!("{} ", grid[y][x]);

        x+= 1;

        if x == 9 {
            println!();
            x = 0;
            y += 1;
        }

        if y == 9 {
            break;
        }

        if x == 3 || x == 6 {
            print!("| ");
        }

        if (y == 3 || y == 6) && x == 0 {
            println!("=====================");
        }
    }
}
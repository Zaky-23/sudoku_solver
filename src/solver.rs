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
                // Undo the change when it returns
                // and backtrack
                grid[y][x] = 0;
            }

            if *counter == num_sol {
                exit(0)
            }

            // When there is no possible values left,
            // badcktrack
            return;
        }
    }

    *counter += 1;
    dbg!(counter);

    println!("{:?}", grid[0]);
    println!("{:?}", grid[1]);
    println!("{:?}", grid[2]);
    println!("{:?}", grid[3]);
    println!("{:?}", grid[4]);
    println!("{:?}", grid[5]);
    println!("{:?}", grid[6]);
    println!("{:?}", grid[7]);
    println!("{:?}", grid[8]);
    println!();
}

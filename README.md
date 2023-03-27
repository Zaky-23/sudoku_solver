# sudoku_solver

A simple recursive sudoku solver written in Rust.

## Usage
```bash
cargo run -- [file_path] [number_of_solutions]
 ```
 
 ##### Example
 ``` bash
 cargo run -- testing/input.txt 1
 ```
 
 ##### Example Input
 ``` bash
0 0 0 0 0 0 2 0 0 
0 8 0 0 0 7 0 9 0 
6 0 2 0 0 0 5 0 0 
0 7 0 0 6 0 0 0 0
0 0 0 9 0 1 0 0 0 
0 0 0 0 2 0 0 0 0 
0 0 5 0 0 0 0 0 0
0 9 0 4 0 0 0 0 0 
0 0 6 0 0 0 0 0 0
 ```
 ##### Example Output
 ``` bash
[7, 1, 9, 3, 5, 6, 2, 4, 8]
[5, 8, 3, 2, 4, 7, 1, 9, 6]
[6, 4, 2, 1, 8, 9, 5, 3, 7]
[2, 7, 1, 5, 6, 4, 3, 8, 9]
[3, 5, 8, 9, 7, 1, 4, 6, 2]
[9, 6, 4, 8, 2, 3, 7, 1, 5]
[1, 2, 5, 6, 3, 8, 9, 7, 4]
[8, 9, 7, 4, 1, 2, 6, 5, 3]
[4, 3, 6, 7, 9, 5, 8, 2, 1]
```

use sudoku::{backtrack, sets, bits};
use std::time::Instant;
use std::env;

fn main() {
    let grid: [[u8; 9]; 9] = [
        [0, 0, 8,  0, 0, 9,  0, 6, 2],
        [0, 0, 0,  0, 0, 0,  0, 0, 5],
        [1, 0, 2,  5, 0, 0,  0, 0, 0],

        [0, 0, 0,  2, 1, 0,  0, 9, 0],
        [0, 5, 0,  0, 0, 0,  6, 0, 0],
        [6, 0, 0,  0, 0, 0,  0, 2, 8],

        [4, 1, 0,  6, 0, 8,  0, 0, 0],
        [8, 6, 0,  0, 3, 0,  1, 0, 0],
        [0, 0, 0,  0, 0, 0,  4, 0, 0],
    ];
    // let grid: [[u8; 9]; 9] = [
    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
        
    //     [1, 2, 3,  0, 0, 0,  0, 0, 0],
    //     [4, 5, 6,  0, 0, 0,  0, 0, 0],
    //     [0, 0, 0,  7, 0, 0,  0, 0, 0],

    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    // ];
    let args: Vec<String> = env::args().collect();
    let test_grids = sudoku_from_file(&args[1]);

    let mut d1 = Vec::new();
    let mut d2 = Vec::new();
    let mut d3 = Vec::new();

    for grid in &test_grids {
        d1.push(measure(|| { backtrack::Sudoku::from(*grid).unwrap().solve(); }));
        d2.push(measure(|| { sets::Sudoku::from(*grid).unwrap().solve(); }));
        d3.push(measure(|| { bits::Sudoku::from(*grid).unwrap().solve(); }));
    }

    println!("The backtracking algorithm took: {} s", d1.iter().sum::<f64>() / test_grids.len() as f64);
    println!("The backtracking + sets algorithm: {} s", d2.iter().sum::<f64>() / test_grids.len() as f64);
    println!("The backtracking + bit-sets algorithm: {} s", d3.iter().sum::<f64>() / test_grids.len() as f64);
}

fn measure(f: impl FnOnce()) -> f64 {
    let t = Instant::now();
    f();
    Instant::now().duration_since(t).as_secs_f64()
}

fn sudoku_from_file(filename: &String) -> Vec<[[u8; 9]; 9]> {
    let contents = std::fs::read_to_string(filename)
        .expect("could not read file");
    let contents: Vec<&str> =
        contents.split("\n\n")
        .collect();
    let mut grids: Vec<[[u8; 9]; 9]> = Vec::new();
    contents.into_iter().for_each(|grid| {
        let mut tmp = [[0u8; 9]; 9];
        grid.split('\n').enumerate().for_each(|(row, line)| {
            line.split(' ').enumerate().for_each(|(col, elem)| {
                tmp[row][col] = elem.parse().expect("failed to parse");
            });
        });
        grids.push(tmp);
    });
    grids
}
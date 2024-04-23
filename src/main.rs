use sudoku::{backtrack, sets, bits, wavefunc};
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
    // let mut sud = backtrack::Sudoku::from(grid).unwrap();
    // println!("{sud}");
    // sud.solve();
    // println!("\n{sud}");
    
    
    let args: Vec<String> = env::args().collect();
    let test_grids = sudoku_from_file(&args[1]);
    // performance_test(&test_grids);
    if args.len() >= 4 {
        performance_single_grid(
            test_grids[args[2].parse::<usize>().expect("Could not parse grid index")],
            args[3].parse().expect("Could not parse the number of repetitions")
        );
    } else {
        performance_multiple_grids(&test_grids);
    }
}

fn measure(f: impl FnOnce()) -> u128 {
    let t = Instant::now();
    f();
    Instant::now().duration_since(t).as_nanos()
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

fn performance_test(test_grids: &Vec<[[u8; 9]; 9]>) {
    let mut delta1 = Vec::new();
    let mut delta2 = Vec::new();
    let mut delta3 = Vec::new();

    for grid in test_grids {
        delta1.push(measure(|| { backtrack::Sudoku::from(*grid).unwrap().solve(); }));
        delta2.push(measure(|| { sets::Sudoku::from(*grid).unwrap().solve(); }));
        delta3.push(measure(|| { bits::Sudoku::from(*grid).unwrap().solve(); }));
    }

    println!("The backtracking algorithm took: {:.4} s", delta1.iter().sum::<u128>() / test_grids.len() as u128);
    println!("The backtracking + sets algorithm: {:.4} s", delta2.iter().sum::<u128>() / test_grids.len() as u128);
    println!("The backtracking + bit-sets algorithm: {:.4} s", delta3.iter().sum::<u128>() / test_grids.len() as u128);
}

fn performance_multiple_grids(test_grids: &[[[u8; 9]; 9]]) {
    println!("försök,tid");
    for (i, grid) in test_grids.iter().enumerate() {
        let mut sud = backtrack::Sudoku::from(*grid).unwrap();
        let delta = measure(|| { sud.solve(); });
        println!("{i},{delta}");
    }
}

fn performance_single_grid(test_grid: [[u8; 9]; 9], repetitions: u32) {
    println!("försök,tid");
    for i in 0..repetitions {
        let mut sud = backtrack::Sudoku::from(test_grid).unwrap();
        let delta = measure(|| { sud.solve(); });
        println!("{i},{delta}");
    }
}
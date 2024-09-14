#![allow(unused)]

use colored::Colorize;
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
    //     [8, 9, 0,  7, 0, 0,  0, 0, 0],

    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    //     [0, 0, 0,  0, 0, 0,  0, 0, 0],
    // ];
    // let mut sud = backtrack::Sudoku::from(grid).unwrap();
    // println!("{sud}");
    // sud.solve();
    // println!("\n{sud}");

    // let sud = wavefunc::Sudoku::from(grid);
    // println!("{sud}");

    // let solution = sud.solved();
    // match solution {
    //     Some(solved) => println!("sudoku solved:\n{solved}"),
    //     None => println!("the sudoku could not be solved"),
    // }
    
    
    let args: Vec<String> = env::args().collect();
    let test_grids = sudoku_from_file(&args[1]);
    performance_test(&test_grids, vec![
        (
            "backtracking",
            Box::new(|grid| { assert!(backtrack::Sudoku::from(*grid).unwrap().solve()); }),
        ), (
            "backtracking + sets",
            Box::new(|grid| { assert!(sets::Sudoku::from(*grid).unwrap().solve()); }),
        ), (
            "backtracking + bitsets",
            Box::new(|grid| { assert!(bits::Sudoku::from(*grid).unwrap().solve()); }),
        ), (
            "wavefunc",
            Box::new(|grid| { wavefunc::Sudoku::from(*grid).solved().unwrap(); }),
        )
    ]);
    // if args.len() >= 4 {
    //     performance_single_grid(
    //         test_grids[args[2].parse::<usize>().expect("Could not parse grid index")],
    //         args[3].parse().expect("Could not parse the number of repetitions")
    //     );
    // } else {
    //     performance_multiple_grids(&test_grids);
    // }
}

fn measure(f: impl FnOnce()) -> u128 {
    let t = Instant::now();
    f();
    Instant::now().duration_since(t).as_micros()
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

fn performance_test(test_grids: &Vec<[[u8; 9]; 9]>, solvers: Vec<(&str, Box<dyn Fn(&[[u8; 9]; 9])>)>) {
    let mut deltasums = vec![0u128; solvers.len()];

    for grid in test_grids {
        println!("\n\nNow solving:");
        print_grid(grid);
        println!();
        for (i, (label, solver)) in solvers.iter().enumerate() {
            let mut total_delta = 0;
            let n = 1000;
            for _ in 0..n {
                total_delta += measure(|| { solver(grid); });
            }
            let average = total_delta / n;
            deltasums[i] += average;
            println!("{label}: {:.4}", average);
        }
    }

    println!("{}", "\n\nThe total sum for all solvers were as follows:".green());
    for (sum, (label, _))
    in deltasums.iter().zip(solvers) {
        println!("{label}: {:.4}", sum);
    }
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

fn print_grid(grid: &[[u8; 9]; 9]) {
    for cells in grid {
        for cell in cells {
            print!("{}", cell.to_string().dimmed());
        }
        println!();
    }
}
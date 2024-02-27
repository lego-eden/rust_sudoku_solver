use sudoku::sudoku::Sudoku;
fn main() {
    let grid: Vec<Vec<u8>> = vec![
        vec![0, 0, 8,  0, 0, 9,  0, 6, 2],
        vec![0, 0, 0,  0, 0, 0,  0, 0, 5],
        vec![1, 0, 2,  5, 0, 0,  0, 0, 0],

        vec![0, 0, 0,  2, 1, 0,  0, 9, 0],
        vec![0, 5, 0,  0, 0, 0,  6, 0, 0],
        vec![6, 0, 0,  0, 0, 0,  0, 2, 8],

        vec![4, 1, 0,  6, 0, 8,  0, 0, 0],
        vec![8, 6, 0,  0, 3, 0,  1, 0, 0],
        vec![0, 0, 0,  0, 0, 0,  4, 0, 0],
    ];
    let mut sud = into_sudoku(grid);
    let mut sud = Sudoku::new();
    let solved = sud.solve();
    println!("{}", sud);
    println!("Is the sudoku solved? {solved}")
}

fn into_sudoku(grid: Vec<Vec<u8>>) -> Sudoku {
    let mut arr_2d = [[0u8; 9]; 9];
    for (i, row) in grid.into_iter().enumerate() {
        for (j, element) in row.into_iter().enumerate() {
            arr_2d[i][j] = element;
        }
    }
    Sudoku::from(arr_2d).unwrap()
}
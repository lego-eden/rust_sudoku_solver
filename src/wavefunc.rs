use std::{collections::HashSet, fmt};
use colored::Colorize;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BitSet {
    set: u16, // the unsigned int containing the numbers
    size: u8, // how many numbers are currently contained in the set?
}
impl BitSet {
    pub fn contains(&self, x: u8) -> bool {
        if !(1..=9).contains(&x) { return false; }
        (self.set & (1 << x)) != 0
    }
    
    pub fn insert(&self, x: u8) -> BitSet {
        let new_size = {
            if !self.contains(x) { self.size + 1 }
            else { self.size }
        };
        BitSet {
            set: self.set | 1 << x,
            size: new_size,
        }
    }

    pub fn remove(&self, x: u8) -> BitSet {
        let new_size = {
            if self.contains(x) { self.size - 1 }
            else { self.size }
        };
        BitSet {
            set: self.set & !(1 << x),
            size: new_size,
        }
    }
    
    #[allow(clippy::new_without_default)]
    pub fn new() -> BitSet {
        BitSet {
            set: 0u16,
            size: 0u8,
        }
    }

    pub fn full() -> BitSet {
        BitSet {
            set: 0b1111111110u16,
            size: 9u8,
        }
    }

    pub fn from_single(x: u8) -> BitSet {
        let set = BitSet::new();
        set.insert(x)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sudoku {
    grid: [[Cell; 9]; 9],
}

trait CellIter = Iterator<Item = (Cell, usize, usize)>;

impl Sudoku {
    pub fn solved(&self) -> Option<Sudoku> {
        match self.lowest_entropy() {
            _ if !self.is_valid() => None,
            None => Some(*self),
            Some((cell, min_row, min_col)) => {
                let possible_values_set = cell.possible_values();
                for val in 1..=9 {
                    if !possible_values_set.contains(val) { continue; }
                    let updated_grid = self.set(min_row, min_col, val);
                    // updated_grid.print_affected_cells(min_row, min_col);
                    // println!("\n");

                    match updated_grid.solved() {
                        solution@Some(_) => return solution,
                        None => { continue; },
                    }
                }
                None
            },
        }
    }

    fn is_valid(&self) -> bool {
        self.cells_with_indexes()
            .all(|(cell, _, _)| cell.is_valid())
    }

    fn lowest_entropy(&self) -> Option<(Cell, usize, usize)> {
        self.cells_with_indexes()
            .filter(|(cell, _, _)| {
                match cell {
                    Cell::Unknown(_) => true,
                    _ => false,
                }
            })
            .min_by(|this, other|
                this.0
                    .possible_values()
                    .size()
                    .cmp(&other.0.possible_values().size())
            )
    }

    fn cells_with_indexes(&self) -> impl CellIter {
        self.grid.into_iter()
            .enumerate()
            .flat_map(|(row, cells)|
                cells.into_iter()
                    .enumerate()
                    .map(move |(col, cell)| (cell, row, col))
            )
    }

    fn affected_cells(&self, row: usize, col: usize) -> impl CellIter {
        self.cells_with_indexes()
            .filter(move |(_, r, c)| {
                *r == row || *c == col
                || (row / 3 == r / 3 && col / 3 == c / 3) 
            })
    }

    pub fn set(&self, row: usize, col: usize, val: u8) -> Sudoku {
        let mut new_sudoku =
            self.updated(row, col, self.grid[row][col].set(val));

        for (cell, r, c) in new_sudoku.affected_cells(row, col) {
            new_sudoku =
                new_sudoku.updated(r, c, cell.without(val));
        }

        new_sudoku
    }

    fn updated(&self, row: usize, col: usize, cell: Cell) -> Sudoku {
        let mut grid = self.grid;
        grid[row][col] = cell;
        Sudoku { grid }
    }

    pub fn empty() -> Sudoku {
        let grid = [[Cell::new(); 9]; 9];
        Sudoku { grid }
    }

    pub fn from(grid: [[u8; 9]; 9]) -> Sudoku {
        let mut sudoku = Sudoku::empty();
        for (row_idx, row) in grid.into_iter().enumerate() {
            for (col_idx, num) in row.into_iter().enumerate() {
                if num == 0 { continue; }
                sudoku = sudoku.set(row_idx, col_idx, num);
            }
        }
        sudoku
    }

    pub fn print_affected_cells(&self, row: usize, col: usize) {
        let affected_cells: HashSet<(usize, usize)> = HashSet::from_iter(
            self.affected_cells(row, col)
                .map(|(_, r, c)| (r, c))
        );
        
        for row in 0..9 {
            for col in 0..9 {
                let text = 
                    if affected_cells.contains(&(row, col)) {
                        self.grid[row][col].to_string().reversed()
                    } else {
                        self.grid[row][col].to_string().normal()
                    };
                print!("{text} ");
                if col % 3 == 2 { print!(" "); }
            }
            println!();
            if row % 3 == 2 { println!() }
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in 0..9 {
            for col in 0..9 {
                s.push_str(&format!("{} ", self.grid[row][col]));
                if col % 3 == 2 { s.push(' ')}
            }
            s.push('\n');
            if row % 3 == 2 { s.push('\n'); }
        }
        write!(f, "{s}")
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Cell {
    Known(u8),
    Unknown(BitSet),
    Invalid(Option<u8>)
}

impl Cell {
    fn is_valid(&self) -> bool {
        match self {
            Cell::Invalid(_) => false,
            _ => true,
        }
    }

    fn set(&self, val: u8) -> Self {
        match self {
            Cell::Known(_) => *self,
            Cell::Unknown(values) => {
                if values.contains(val) { Cell::Known(val) }
                else { Cell::Invalid(Some(val)) }
            },
            Cell::Invalid(_) => *self,
        }
    }

    fn without(&self, val: u8) -> Self {
        match self {
            Cell::Known(_) => *self,
            Cell::Unknown(values) => {
                let new_values: BitSet = values.remove(val);
                if new_values.is_empty() { Cell::Invalid(None) }
                else { Cell::Unknown(new_values) }
            },
            Cell::Invalid(_) => *self,
        }
    }

    fn possible_values(&self) -> BitSet {
        match self {
            Cell::Known(val) => BitSet::from_single(*val),
            Cell::Unknown(values) => *values,
            Cell::Invalid(_) => BitSet::new(),
        }
    }

    fn default_values() -> BitSet {
        BitSet::full()
    }

    fn new() -> Cell {
        Cell::Unknown(Cell::default_values())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Cell::*;
        match self {
            Known(val) => {
                write!(f, "{}", val.to_string().green())
            },
            Unknown(_) => write!(f, "-"),
            Invalid(Some(val)) => write!(f, "{}", val.to_string().red()),
            Invalid(None) => write!(f, "{}", "X".red()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_set() {
        assert_eq!(0b1111111110, 1022);
        assert_eq!(BitSet::full().set, 1022);
    }

    #[test]
    fn full_cell() {
        assert_eq!(Cell::new().possible_values(), BitSet::full());
    }

    #[test]
    fn cell_with_missing_values() {
        let mut cell = Cell::new();
        assert!(!cell.without(1).possible_values().contains(1));
        for i in 1..=9 {
            cell = cell.without(i);
        }
        assert!(cell.possible_values().is_empty());
        println!("{cell}");
    }

    #[test]
    fn cell_invalid_set() {
        let cell = Cell::new().without(1);
        assert_eq!(cell.set(1), Cell::Invalid(Some(1)));

        let mut cell = Cell::new();
        for i in 1..=9 {
            cell = cell.without(i);
        }
        assert_eq!(cell, Cell::Invalid(None));
    }
}
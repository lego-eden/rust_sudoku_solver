use std::{backtrace::{self, Backtrace}, fmt};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct BitSet {
    set: u16, // the unsigned int containing the numbers
    size: u8, // how many numbers are currently contained in the set?
}
impl BitSet {
    pub fn contains(&self, x: u8) -> bool {
        if !(1..=9).contains(&x) { return false; }
        (self.set & (1 << x)) != 0
    }
    
    pub fn insert(&mut self, x: u8) -> bool {
        if !(1..=9).contains(&x) { return false; }
        if !self.contains(x) { self.size += 1; }
        self.set |= 1 << x;
        true
    }

    pub fn remove(&mut self, x: u8) -> bool {
        if !(1..=9).contains(&x) { return false; }
        if self.contains(x) { self.size -= 1; }
        self.set &= !(1 << x);
        true
    }

    pub fn set_to(&mut self, x: BitSet) {
        *self = x;
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
        let mut set = BitSet::new();
        set.insert(x);
        set
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}

#[derive(Debug)]
pub struct Sudoku {
    grid: [[BitSet; 9]; 9],
}

#[allow(clippy::new_without_default)]
impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku {
            grid: [[BitSet::full(); 9]; 9],
        }
    }

    pub fn from(grid: [[u8; 9]; 9]) -> Result<Sudoku, &'static str> {
        let mut sud = Sudoku::new();
        #[allow(clippy::needless_range_loop)]
        for r in 0..9 {
            for c in 0..9 {
                let val = grid[r][c];
                if val == 0u8 { continue; }
                if !(1..=9).contains(&val) { return Err("Invalid number")}
                sud.set(r, c, grid[r][c])?;
            }
        }
        Ok(sud)
    }

    pub fn set(&mut self, r: usize, c: usize, x: u8) -> Result<bool, &'static str> {
        restrain_index(r, c)?;
        if !(1..=9).contains(&x) { return Err("x not in 1..=9"); }
        if !self.grid[r][c].contains(x) { return Ok(false) }
        let backup = self.grid;
        self.grid[r][c] = BitSet::from_single(x);
        if !self.propagate_set(r, c, x) {
            self.grid = backup;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn get(&self, r: usize, c: usize) -> Result<u8, &'static str> {
        restrain_index(r, c)?;
        let set = self.grid[r][c];
        Ok(
            if set.size > 1 { return Ok(0) }
            else {
                let mut i = 1u8;
                while i <= 9 {
                    if set.contains(i) { break; }
                    i += 1;
                }
                i
            }
        )
    }

    #[allow(clippy::needless_range_loop)]
    pub fn get_grid(&self) -> [[u8; 9]; 9] {
        let mut res = [[0u8; 9]; 9];
        for r in 0..9 {
            for c in 0..9 {
                res[r][c] = self.get(r, c).unwrap();
            }
        }
        res
    }

    #[allow(clippy::needless_range_loop)]
    pub fn clear(&mut self, r: usize, c: usize) -> Result<(), &'static str> {
        restrain_index(r, c)?;
        let tmp = self.get_grid();
        self.clear_all();
        for ir in 0..9 {
            for ic in 0..9 {
                if ir != r && ic != c { self.set(ir, ic, tmp[ir][ic])?; }
            }
        }
        Ok(())
    }

    pub fn clear_all(&mut self) {
        *self = Sudoku::new();
    }

    #[allow(clippy::needless_range_loop)]
    pub fn set_grid(&mut self, grid: [[u8; 9]; 9]) -> Result<bool, &'static str> {
        self.clear_all();
        for (r, row) in grid.into_iter().enumerate() {
            for (c, elem) in row.into_iter().enumerate() {
                if !self.set(r, c, elem)? { return Ok(false); }
            }
        }
        Ok(true)
    }
    
    fn propagate_set(&mut self, r: usize, c: usize, x: u8) -> bool {
        let backup = self.grid;
        for i in 0..9 {
            if i != r {
                let cell = &mut self.grid[i][c];
                cell.remove(x);
                if cell.is_empty() || !self.propagate_set(i, c, x) {
                    self.grid = backup;
                    return false;
                } // backtrack, make sure to revert to pre-set state
            }
            
            if i != c {
                let cell = &mut self.grid[r][i];
                cell.remove(x);
                if cell.is_empty() || !self.propagate_set(r, i, x){
                    self.grid = backup;
                    return false;
                } // backtrack
            }
            
            let ir = r / 3 + i / 3;
            let ic = c / 3 + i % 3;
            
            if (ir != r) && (ic != c) {
                let cell = &mut self.grid[ir][ic];
                cell.remove(x);
                if cell.is_empty() || !self.propagate_set(ir, ic, x) {
                    self.grid = backup;
                    return false;
                } // backtrack
            }
        }
        true // The loop did not make any cell have zero
             // possible numbers => the algorithm can continue.
    }

    fn lowest_global_entropy(&self) -> Option<(usize, usize)> {
        let mut lowest_coords = None;
        let mut lowest_size = 10;
        for row in 0..9 {
            for col in 0..9 {
                let cell = self.grid[row][col];
                if (2..lowest_size).contains(&cell.size()) {
                    lowest_coords = Some((row, col));
                    lowest_size = cell.size();
                }
            }
        }
        lowest_coords
    }

    pub fn solve(&mut self) -> bool {
        match self.lowest_global_entropy() {
            Some((row, col)) => self.solve_at(row, col),
            None => true,
        }
    }

    fn solve_at(&mut self, r: usize, c: usize) -> bool {
        let (lowest_entropy_row, lowest_entropy_col) = match self.lowest_global_entropy() {
            Some(point) => point,
            None => return true,
        };
        
        for i in 1..=9 {
            if !self.set(r, c, i).unwrap() { continue; }
            if self.solve_at(lowest_entropy_row, lowest_entropy_col) { return true; }
        }
        false
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        (0..9).for_each(|r| {
            (0..9).for_each(|c| {
                s.push_str(&format!("{} ", self.get(r, c).unwrap()));
                if c % 3 == 2 { s.push(' '); };
            });
            s.push('\n');
            if r % 3 == 2 { s.push('\n'); };
        });
        write!(f, "{}", s)
    }
}

fn restrain_index(r: usize, c: usize) -> Result<(), &'static str> {
    if (0..9).contains(&r) && (0..9).contains(&c) { Ok(()) }
    else { Err("Index greater than 9") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_set() {
        assert_eq!(0b1111111110, 1022);
        assert_eq!(BitSet::full().set, 1022);
    }
}
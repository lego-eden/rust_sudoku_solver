pub mod sudoku {
    use std::fmt;

    #[derive(Debug)]
    pub struct Sudoku {
        grid: [[u8; 9]; 9],
    }
    
    #[allow(clippy::new_without_default)]
    impl Sudoku {
        pub fn new() -> Sudoku {
            Sudoku {
                grid: [[0u8; 9]; 9]
            }
        }
    
        pub fn from(grid: [[u8; 9]; 9]) -> Result<Sudoku, &'static str> {
            let mut new_sudoku = Sudoku {
                grid: [[0u8; 9]; 9],
            };
            new_sudoku.set_grid(grid)?;
            Ok(new_sudoku)
        }
    
        fn restrain_index(r: usize, c: usize) -> Result<(), &'static str> {
            if (0..9).contains(&r) && (0..9).contains(&c) { Ok(()) }
            else { Err("Index greater than 9") }
        }
    
        pub fn set(&mut self, r: usize, c: usize, n: u8) -> Result<(), &'static str> {
            Sudoku::restrain_index(r, c)?;
            if !(1..=9).contains(&n) { return Err("Value is not in range [1..=9]"); }
            self.grid[r][c] = n;
            Ok(())
        }

        pub fn get(&self, r: usize, c: usize) -> Result<u8, &'static str> {
            Sudoku::restrain_index(r, c)?;
            Ok(self.grid[r][c])
        }
    
        pub fn clear(&mut self, r: usize, c: usize) -> Result<(), &'static str> {
            Sudoku::restrain_index(r, c)?;
            self.grid[r][c] = 0;
            Ok(())
        }
    
        pub fn clear_all(&mut self) {
            self.grid = [[0u8; 9]; 9];
        }
    
        pub fn is_valid(&self, r: usize, c: usize) -> Result<bool, &'static str> {
            Sudoku::restrain_index(r, c)?;
            let this_num = self.get(r, c)?;
            if this_num == 0 { return Ok(true); }
    
            // check row
            for cc in 0..9 {
                if cc == c { continue; }
                if self.get(r, cc)? == this_num { return Ok(false); }
            }
            // check col
            for rr in 0..9 {
                if rr == r { continue; }
                if self.get(rr, c)? == this_num { return Ok(false); }
            }
            // check zone
            let zone_r = r / 3;
            let zone_c = c / 3;
            for rr in (3 * zone_r)..((zone_r + 1) * 3) {
                for cc in (3 * zone_c)..((zone_c + 1) * 3) {
                    if rr == r && cc == c { continue; }
                    if self.get(rr, cc)? == this_num { return Ok(false);}
                }
            }
            // if all above checks pass then the cell is valid
            Ok(true)
        }
    
        pub fn is_all_valid(&self) -> bool {
            for r in 0..9 {
                for c in 0..9 {
                    if !self.is_valid(r, c).unwrap() { return false; }
                }
            }
            // if all cells are valid return true
            true
        }
    
        pub fn set_grid(&mut self, grid: [[u8; 9]; 9]) -> Result<(), &'static str> {
            for r in grid {
                for v in r {
                    if !(0..=9).contains(&v) { return Err("grid contains value outside [0..=9]") }
                }
            }
            self.grid = grid;
            Ok(())
        }
    
        pub fn solve(&mut self) -> bool {
            self.solve_at(0, 0)
        }
    
        fn solve_at(&mut self, r: usize, c: usize) -> bool {
            if Sudoku::restrain_index(r, c).is_err() {
                return true;
            }

            let new_c = (c + 1) % 9;
            let new_r = r + ((c + 1) / 9);

            if self.get(r, c).unwrap() != 0 {
                if !self.is_all_valid() { return false; }
                else { return self.solve_at(new_r, new_c); }
            }

            for n in 1..=9 {
                self.set(r, c, n).unwrap();
                if self.is_all_valid() && self.solve_at(new_r, new_c) { return true; }
            }

            self.clear(r, c).unwrap();
            false
        }
    
        pub fn grid(&self) -> &[[u8; 9]; 9] {
            &self.grid
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
}
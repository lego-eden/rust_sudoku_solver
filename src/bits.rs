use std::fmt;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct BitSet(u16);
impl BitSet {
    pub fn contains(&self, x: u8) -> bool {
        if !(1..=9).contains(&x) { return false; }
        (self.0 & (1 << x)) != 0
    }
    
    pub fn insert(&mut self, x: u8) -> bool {
        if !(1..=9).contains(&x) { return false; }
        self.0 |= 1 << x;
        true
    }

    pub fn remove(&mut self, x: u8) -> bool {
        if !(1..=9).contains(&x) { return false; }
        self.0 &= !(1 << x);
        true
    }
    
    #[allow(clippy::new_without_default)]
    pub fn new() -> BitSet {
        BitSet(0u16)
    }

    pub fn full() -> BitSet {
        BitSet(0b1111111110)
    }
}

#[derive(Debug)]
pub struct Sudoku {
    grid: [[u8; 9]; 9],
    r_sets: [BitSet; 9],
    c_sets: [BitSet; 9],
    z_sets: [[BitSet; 3]; 3],
}

#[allow(clippy::new_without_default)]
impl Sudoku {
    pub fn new() -> Sudoku {
        let r_sets: [BitSet; 9] = vec![Sudoku::init_set(); 9]
            .try_into().unwrap();
        let c_sets = vec![Sudoku::init_set(); 9]
            .try_into().unwrap();
        let z_sets: [[BitSet; 3]; 3] = vec![vec![Sudoku::init_set(); 3]; 3]
            .into_iter().map(|v| v.try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into().unwrap();

        Sudoku {
            grid: [[0u8; 9]; 9],
            r_sets,
            c_sets,
            z_sets,
        }
    }

    pub fn from(grid: [[u8; 9]; 9]) -> Result<Sudoku, &'static str> {
        let mut new_sudoku = Sudoku {
            grid: [[0u8; 9]; 9],
            ..Sudoku::new()
        };
        new_sudoku.set_grid(grid)?;
        Ok(new_sudoku)
    }

    pub fn set(&mut self, r: usize, c: usize, n: u8) -> Result<bool, &'static str> {
        if n == 0 { self.clear(r, c)?; return Ok(true); }
        if !(1..=9).contains(&n) { return Err("Value is not in range 1..=9, was {n}"); }
        if !self.is_valid(r, c, n)? { return Ok(false); }
        self.grid[r][c] = n;
        self.r_sets[r].remove(n);
        self.c_sets[c].remove(n);
        self.z_sets[r / 3][c / 3].remove(n);
        
        Ok(true)
    }

    pub fn get(&self, r: usize, c: usize) -> Result<u8, &'static str> {
        Sudoku::restrain_index(r, c)?;
        Ok(self.grid[r][c])
    }

    pub fn clear(&mut self, r: usize, c: usize) -> Result<(), &'static str> {
        let n = self.get(r, c)?;
        if n == 0 { return Ok(()); }
        self.r_sets[r].insert(n);
        self.c_sets[c].insert(n);
        self.z_sets[r / 3][c / 3].insert(n);
        self.grid[r][c] = 0;
        Ok(())
    }

    pub fn clear_all(&mut self) {
        *self = Sudoku::new();
    }

    #[allow(clippy::needless_range_loop)]
    pub fn set_grid(&mut self, grid: [[u8; 9]; 9]) -> Result<bool, &'static str> {
        self.clear_all();
        for i in 0..9 {
            for j in 0..9 {
                if !self.set(i, j, grid[i][j])? { return Ok(false); }
            }
        }
        Ok(true)
    }
    
    fn init_set() -> BitSet {
        BitSet::full()
    }
    
    fn is_valid(&self, r: usize, c: usize, n: u8) -> Result<bool, &'static str> {
        Sudoku::restrain_index(r, c)?;
        Ok(
            self.r_sets[r].contains(n) &&
            self.c_sets[c].contains(n) &&
            self.z_sets[r / 3][c / 3].contains(n)
        )
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
            return self.solve_at(new_r, new_c);
        }

        for n in 1..=9 {
            if !self.set(r, c, n).unwrap() { continue; }
            if self.solve_at(new_r, new_c) { return true; }
            self.clear(r, c).unwrap();
        }

        false
    }

    fn restrain_index(r: usize, c: usize) -> Result<(), &'static str> {
        if (0..9).contains(&r) && (0..9).contains(&c) { Ok(()) }
        else { Err("Index greater than 9") }
    }

    pub fn print_sets(&self) {
        println!("Row vacant: ");
        for row in &self.r_sets {
            println!("{:?}", row);
        }
        println!("Col vacant: ");
        for col in &self.c_sets {
            println!("{:?}", col);
        }
        println!("Zone vacant: ");
        for row in &self.z_sets {
            for zone in row {
                println!("{:?}", zone);
            }
        }
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
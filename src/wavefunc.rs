use std::fmt;

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

    pub fn get(&self, r: usize, c: usize) -> Result<u8, &'static str> {
        todo!();
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
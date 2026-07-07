use std::fmt;
use rand::RngExt;

#[derive(Debug)]
struct Cell {
    is_alive: bool,
    x: u32,
    y: u32,
}

impl Cell {
    fn new(is_alive: bool, x: u32, y: u32) -> Cell {
        Cell { is_alive, x, y }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Cell({0}, ({1}, {2}))", self.is_alive, self.x, self.y)
}
}
fn main() {
    let (weight, height): (u32, u32) = (10, 10);
    let mut cells: Vec<Cell> = vec![];
    let mut rng = rand::rng();

    for x in 0..10 {
        for y in 0..10 {
            cells.push(Cell::new(rng.random_bool(0.25), x, y,));
        }
    }

    for i in cells.iter() {
        println!("{i}");
    }
}

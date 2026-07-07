use rand::RngExt;
use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
struct Cell {
    is_alive: bool,
}

impl Cell {
    fn new(is_alive: bool) -> Cell {
        Cell { is_alive }
    }
}

const WEIGHT: u32 = 100;
const HEIGHT: u32 = 30;

const WAIT_TIME: u64 = 100;

const ALIVE_CELL: char = 'O';
const DEAD_CELL: char = '.';

fn display_cells(cells: &HashMap<(u32, u32), Cell>) {
    let mut output = String::with_capacity((WEIGHT * HEIGHT + HEIGHT) as usize);
    for y in 0..HEIGHT {
        for x in 0..WEIGHT {
            let cell = cells.get(&(x, y)).unwrap();
            output.push(if cell.is_alive { ALIVE_CELL } else { DEAD_CELL });
        }
        output.push('\n');
    }
    print!("{}", output);
}

fn update_cells(cells: HashMap<(u32, u32), Cell>) -> HashMap<(u32, u32), Cell> {
    let mut new_cells: HashMap<(u32, u32), Cell> = HashMap::new();

    for y in 0..HEIGHT {
        for x in 0..WEIGHT {
            let option_cell: Option<&Cell> = cells.get(&(x, y));
            let cell: &Cell = option_cell.unwrap();

            let left_x = (x + WEIGHT - 1) % WEIGHT;
            let right_x = (x + 1) % WEIGHT;
            let above_y = (y + HEIGHT - 1) % HEIGHT;
            let below_y = (y + 1) % HEIGHT;

            let neighbors_variant = [
                (left_x, above_y),
                (x, above_y),
                (right_x, above_y),
                (left_x, y),
                (right_x, y),
                (left_x, below_y),
                (x, below_y),
                (right_x, below_y),
            ];

            let mut neighbors_number: u8 = 0;

            for i in &neighbors_variant {
                if let Some(cell) = cells.get(&i) {
                    if cell.is_alive {
                        neighbors_number += 1;
                    }
                }
            }

            if cell.is_alive && (neighbors_number == 2 || neighbors_number == 3) {
                new_cells.insert((x, y), Cell::new(true));
            } else if !cell.is_alive && neighbors_number == 3 {
                new_cells.insert((x, y), Cell::new(true));
            } else {
                new_cells.insert((x, y), Cell::new(false));
            }
        }
    }
    new_cells
}

fn generate_cells() -> HashMap<(u32, u32), Cell> {
    let mut rng = rand::rng();
    let mut cells: HashMap<(u32, u32), Cell> = HashMap::new();
    for x in 0..WEIGHT {
        for y in 0..HEIGHT {
            cells.insert((x, y), Cell::new(rng.random_bool(0.5)));
        }
    }
    cells
}
fn clear_terminal() {
    print!("{}[2J", 27 as char);
}
fn main() {
    let mut cells = generate_cells();

    loop {
        clear_terminal();
        display_cells(&cells);
        thread::sleep(Duration::from_millis(WAIT_TIME));
        cells = update_cells(cells);
    }
}

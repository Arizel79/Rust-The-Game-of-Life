use rand::RngExt;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

const WEIGHT: u32 = 100;
const HEIGHT: u32 = 30;

const WAIT_TIME: u64 = 100;

const ALIVE_CELL: char = 'O';
const DEAD_CELL: char = '.';

fn display_cells(cells: &HashMap<(u32, u32), bool>) {
    let mut output = String::with_capacity((WEIGHT * HEIGHT + HEIGHT) as usize);
    for y in 0..HEIGHT {
        for x in 0..WEIGHT {
            let cell = cells.get(&(x, y)).unwrap();
            output.push(if *cell { ALIVE_CELL } else { DEAD_CELL });
        }
        output.push('\n');
    }
    print!("{}", output);
}

fn update_cells(cells: HashMap<(u32, u32), bool>) -> HashMap<(u32, u32), bool> {
    let mut new_cells: HashMap<(u32, u32), bool> = HashMap::new();

    for y in 0..HEIGHT {
        for x in 0..WEIGHT {
            let cell = *cells.get(&(x, y)).unwrap();

            let left_x = (x + WEIGHT - 1) % WEIGHT;
            let right_x = (x + 1) % WEIGHT;
            let above_y = (y + HEIGHT - 1) % HEIGHT;
            let below_y = (y + 1) % HEIGHT;

            let neighbors = [
                (left_x, above_y),
                (x, above_y),
                (right_x, above_y),
                (left_x, y),
                (right_x, y),
                (left_x, below_y),
                (x, below_y),
                (right_x, below_y),
            ];

            let neighbors_number = neighbors
                .iter()
                .filter(|&pos| *cells.get(pos).unwrap_or(&false))
                .count() as u8;

            let alive = neighbors_number == 3 || (cell && neighbors_number == 2);

            new_cells.insert((x, y), alive);
        }
    }
    new_cells
}

fn generate_cells() -> HashMap<(u32, u32), bool> {
    let mut rng = rand::rng();
    let mut cells: HashMap<(u32, u32), bool> = HashMap::new();
    for x in 0..WEIGHT {
        for y in 0..HEIGHT {
            cells.insert((x, y), rng.random_bool(0.5));
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

use rand::Rng;
use std::{thread, time};

use game_of_life_logic::types::*;

fn main() {
    let mut universe = get_random_universe();
    let two_secs = time::Duration::from_secs(1);

    loop {
        let previous_universe = universe.clone();
        println!("{}", universe);
        universe.next_generation();

        if universe == previous_universe {
            break;
        }

        thread::sleep(two_secs);
    }

    println!("that's all");
}

fn get_random_universe() -> Universe {
    let mut universe = Universe::new(40, 10);
    for index in 0..universe.cells.len() {
        let sectret_number = rand::thread_rng().gen_range(0, 6);
        if sectret_number % 6 == 0 {
            universe.toggle_cells_state(vec![index]);
        }
    }

    universe
}

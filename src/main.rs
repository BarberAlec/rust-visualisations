mod engines;
mod utils;

use utils::user_input;
use engines::{game_of_life};


fn main() {
    // Conways Game of Life with R-Pentomino start initialisation
    let mut gol = game_of_life::r_pentomino();

    // ask user for iteration count
    let num_iterations = user_input::user_input_with_parse("How many iterations for game of life: ");

    // loop and print current alive count
    for ind in 0..num_iterations {
        println!("After {} iterations, alive: {}", ind, gol.alive_count());
        gol.iterate();
    }
}

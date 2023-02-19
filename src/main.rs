mod engines;
mod utils;

use utils::user_input;
use engines::{fibonacci, game_of_life::{self, GOL}};


fn play_fibonacci() {
    let count = user_input::user_input_with_parse::<i32>("How many fibonacci numbers would you like to print: ");

    let fib_nums = fibonacci::get_fibonacci(count).expect(&format!(
        "Fibinanci function failed with count input {}",
        count
    ));

    println!("{:?}", fib_nums);
}

fn play_game_of_life() {

    // Conways Game of Life with R-Pentomino start initialisation
    let mut gol: GOL;

    // ask user for init conditions
    loop {
        let init_type = user_input::user_input_with_parse::<char>("Select initial conditions: \nR-Pentomino\t\t(R)");
        match init_type {
            'R' => {
                gol = game_of_life::r_pentomino();
                break;
            },
            _ => {println!("Unknown code {}.", init_type)}
        }
    }

    // ask user for iteration count
    let num_iterations = user_input::user_input_with_parse::<i32>("How many iterations for game of life: ");

    // loop and print current alive count
    for ind in 0..num_iterations {
        println!("After {} iterations, alive: {}", ind, gol.alive_count());
        gol.iterate();
    }
}

fn main() {
    let game_code = user_input::user_input_with_parse::<char>("What game would you like to play?\nConway's Game of Life\t(G)\nFibonacci\t\t(F)");

    match game_code {
        'F' => {play_fibonacci();},
        'G' => {play_game_of_life();},
        _ => {println!("Unrecognised code {}, exiting.", game_code)}
    }
}

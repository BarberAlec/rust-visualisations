mod engines;
mod utils;
pub mod gui;

use utils::user_input;
use engines::{fibonacci, game_of_life::{self, GOL}};
// use gui::window;
use gui::gif;


#[allow(dead_code)]
fn play_fibonacci() {
    let count = user_input::user_input_with_parse::<i32>("How many fibonacci numbers would you like to print: ");

    let fib_nums = fibonacci::get_fibonacci(count).expect(&format!(
        "Fibinanci function failed with count input {}",
        count
    ));

    println!("{:?}", fib_nums);
}

#[allow(dead_code)]
fn play_game_of_life() {
    // Conways Game of Life with R-Pentomino start initialisation
    let mut gol: GOL;
    let gif_name: &str;

    // 2d vector to store state at each iteration
    let mut game_states: Vec<Vec<Vec<bool>>> = Vec::new();

    // ask user for init conditions
    loop {
        let init_type = user_input::user_input_with_parse::<char>("Select initial conditions:\
                                                                                \nR-Pentomino\t\t\t(R)\
                                                                                \nDiehard\t\t\t\t(D)\
                                                                                \nAcorn\t\t\t\t(A)\
                                                                                \nBlock Layering Switch Engine 1\t(B)\
                                                                                \nBlock Layering Switch Engine 2\t(Z)");
        match init_type {
            'R' => {gol = game_of_life::r_pentomino(); gif_name="R-Pentomino_game_of_life.gif"; break;},
            'D' => {gol = game_of_life::diehard(); gif_name="Diehard_game_of_life.gif"; break;},
            'A' => {gol = game_of_life::acorn(); gif_name="Acorn_game_of_life.gif"; break;},
            'B' => {gol = game_of_life::block_laying_switch_engine_1(); gif_name="BlockLayeringSwitchEngine_1_game_of_life.gif"; break;},
            'Z' => {gol = game_of_life::block_laying_switch_engine_2(); gif_name="BlockLayeringSwitchEngine_2_game_of_life.gif"; break;},
            _ => {println!("Unknown code {}.", init_type)}
        }
    }

    // ask user for iteration count
    let num_iterations = user_input::user_input_with_parse::<i32>("How many iterations for game of life: ");

    game_states.push(gol.current_state());

    // loop and print current alive count
    for ind in 0..num_iterations {
        println!("After {} iterations, alive: {}", ind, gol.alive_count());
        gol.iterate();
        game_states.push(gol.current_state());
    }

    match gif::create_gif(gif_name, game_states) {
        Ok(_) => {}
        Err(e) => {println!("{}", e)}
    };
}



fn main() {
    let game_code = user_input::user_input_with_parse::<char>("What game would you like to play?\nConway's Game of Life\t(G)\nFibonacci\t\t(F)");

    match game_code {
        'F' => {play_fibonacci();},
        'G' => {play_game_of_life();},
        _ => {println!("Unrecognised code {}, exiting.", game_code)}
    }
}

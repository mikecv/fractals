// Application menu functions.

use log::{info};
use inline_colorization::*;
use std::io::{self, Write};

use crate::AppState;
use crate::fractal::Fractal;

// Print the menu selection.
// This should be statefull assuming possible selections.
pub fn print_menu(state: &AppState) {
    println!("{color_green}{style_bold}\n====\nMenu\n====\n{style_reset}{color_reset}");
    match state {
        AppState::AppStart => {
            info!("Application state at menu: START");
            println!("1) Initialise new fractal");
        },
        AppState::NewFractal => {
            info!("Application state: NEW FRACTAL");
            println!("2) Option Two");
        },
    }
    println!("Q) Quit\n");
}

// Get the user input(s) for the menu selection.
// Inputs returned as string(s).
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    input
}

// User selected option to initialise new fractal.
// Parameters"
// rows : u32 - number of rows in image.
// cols : u32 - number of columns in image.
// max_its : u32 - max number of iterations to escape.
pub fn new_fractal(fractals : &mut Fractal) {
    info!("Initialising new fractal.");
    let rows = get_user_input("Number of rows: ");
    let cols = get_user_input("Number of columns: ");
    let max_its = get_user_input("Max iterations: ");
    fractals.init_fractal_image(rows.trim().parse().unwrap(), cols.trim().parse().unwrap());
    fractals.max_its = max_its.trim().parse().unwrap();
    info!("Fractal with, rows: {}, cols: {}, max iterations: {}", fractals.rows, fractals.cols, fractals.max_its);
}

// User selected option option 2.
pub fn option_two(_fractals : &mut Fractal) {
    println!("You selected Option Two.");
}

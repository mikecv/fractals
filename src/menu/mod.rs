// Application menu functions.

use log::{info};
use inline_colorization::*;
use std::io::{self, Write};

use crate::AppState;

// Print the menu selection.
// This should be statefull assuming possible selections.
pub fn print_menu(state: &AppState) {
    println!("{color_green}{style_bold}====\nMenu\n====\n{style_reset}{color_reset}");
    match state {
        AppState::AppStart => {
            info!("Application state at menu: START");
            println!("1) Option One");
        },
        AppState::NewFractal => {
            info!("Application state: NEW FRACTAL");
            println!("2) Option Two");
        },
    }
    println!("Q) Quit");
}

// Get the user selection.
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

// User selected option option 1.
pub fn option_one() {
    println!("You selected Option One.");
    let param = get_user_input("Enter a parameter: ");
    println!("Parameter received: {}", param.trim());
}

// User selected option option 2.
pub fn option_two() {
    println!("You selected Option Two.");
    let param1 = get_user_input("Enter the first parameter: ");
    let param2 = get_user_input("Enter the second parameter: ");
    println!("Parameters received: {}, {}", param1.trim(), param2.trim());
}

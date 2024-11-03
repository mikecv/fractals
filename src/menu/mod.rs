// Application menu functions.

use log::{info};

use inline_colorization::*;
use num_complex::Complex;
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
            println!("N) Initialise new fractal");
        },
        AppState::NewFractal => {
            info!("Application state: NEW FRACTAL");
            println!("N) Initialise new fractal");
            println!("C) Calculate fractal divergence");
        },
    }
    println!("P) Print class variables");
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
// mid_pt_r : f64 - real part of image centrepoint.
// mid_pt_i : f64 - imaginary part of image centrepoint.
// pt_div : f64 - division of points in BOTH axis.
// max_its : u32 - max number of iterations to escape.
pub fn new_fractal(fractals : &mut Fractal) {
    info!("Initialising new fractal.");
    let rows = get_user_input("Number of rows: ");
    let cols = get_user_input("Number of columns: ");
    let mid_pt_r = get_user_input("Midpoint Real axis: ");
    let mid_pt_i = get_user_input("Midpoint Imaginary axis: ");
    let pt_div = get_user_input("Point division: ");
    let max_its = get_user_input("Max iterations: ");
    fractals.mid_pt = Complex::new(mid_pt_r.trim().parse().unwrap(), mid_pt_i.trim().parse().unwrap());
    fractals.max_its = max_its.trim().parse().unwrap();
    fractals.pt_div = pt_div.trim().parse().unwrap();
    fractals.init_fractal_image(rows.trim().parse().unwrap(),
                                cols.trim().parse().unwrap(),
                                fractals.mid_pt,
                                fractals.pt_div);
    info!("Fractal rows: {}, cols: {}", fractals.rows, fractals.cols);
    info!("Fractal centrepoint: {}", fractals.mid_pt);
    info!("Fractal point division: {}", fractals.pt_div);
    info!("Fractal max iterations: {}", fractals.max_its);
}

// Function to print out the state of the class variables.
pub fn print_class(fractals : &mut Fractal) {
    println!("App state      : {}", fractals.state);
    println!("Rows           : {}", fractals.rows);
    println!("Columns        : {}", fractals.cols);
    println!("Centre point   : {}", fractals.mid_pt);
    println!("Point division : {}", fractals.pt_div);
    println!("Max iterations : {}", fractals.max_its);
    println!("Left limit     : {}", fractals.left_lim);
    println!("Top limit      : {}", fractals.top_lim);
}

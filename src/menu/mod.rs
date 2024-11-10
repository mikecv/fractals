// Application menu functions.

use log::{info};

use inline_colorization::*;
use num_complex::Complex;
use std::io::{self, Write};
use std::time::{Instant};

use crate::AppState;
use crate::fractal::Fractal;

// Print the menu selection.
// This should be statefull assuming possible selections.
pub fn print_menu(state: &AppState) {
    println!("{color_green}{style_bold}\n====\nMenu\n====\n{style_reset}{color_reset}");
    match state {
        AppState::AppStart => {
            info!("Application state at menu: START");
            println!("E) Enter new fractal settings");
            println!("F) Initialise fractal from file");
        },
        AppState::NewFractal => {
            info!("Application state: NEW FRACTAL");
            println!("E) Enter new fractal settings");
            println!("F) Initialise fractal from file");
            println!("C) Calculate fractal divergence");
            println!("S) Save fractal settings to file");
        },
        AppState::DivComplete => {
            info!("Application state: DIVERGENCE DONE");
            println!("E) Enter new fractal settings");
            println!("F) Initialise fractal from file");
            println!("C) Calculate fractal divergence");
            println!("S) Save fractal settings to file");
        },
    }
    println!("P) Print class variables");
    println!("Q) Quit\n");
}

// Get the user input(s) for the menu selection.
// Inputs returned as string(s).
pub fn get_user_input(prompt: &str) -> String {
    info!("Displing menu and getting user response.");

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
pub fn enter_fractal(fractals : &mut Fractal) {
    info!("Initialising new fractal by user.");

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

    // At this point we have an initialised fractal.
    // From here we can re-initialise a new fractal or proceed to calculate
    // point divergence for the initialised fractal.
    fractals.state = AppState::NewFractal;
}

// User selected option to initialise new fractal.
// As for function enter_fractal except settings read from file.
pub fn load_settings(fractals : &mut Fractal) {
    info!("Initialising new fractal from file.");

    // Clone the file path to avoid simultaneous mutable and immutable borrows.
    let file_path = fractals.settings.fractal_file.clone();

    // Save fractal settings to toml file.
    // Now call load_config without conflicting borrows.
    let _load_status = fractals.load_config(&file_path);   

    // At this point we have an initialised fractal.
    // From here we can re-initialise a new fractal or proceed to calculate
    // point divergence for the initialised fractal.
    fractals.state = AppState::NewFractal;
}

// Save fractal settings to file.
pub fn save_settings(fractals: &mut Fractal) {
    info!("Saving fractal settings to file.");

    // Clone the file path to avoid simultaneous mutable and immutable borrows.
    let file_path = fractals.settings.fractal_file.clone();

    // Now call save_config without conflicting borrows.
    let _save_status = fractals.save_config(&file_path);
}

// Function to calculate divergence at all points in fractal.
// Do it row by row, leaves posibility of using multiple works to calculate
// divergence on more than one row at a time.
pub fn cal_divergence(fractals : &mut Fractal) {
    info!("Calculating fractal divergence.");

    // Initialise timer for divergence caluclation.
    let calc_start = Instant::now();

    // Start with the left top point.
    let mut st_c: Complex<f64> = fractals.pt_lt;

    // Iterate calculation over rows.
    for row in 0..fractals.rows {
        // Calculate the starting point for the row.
        // Lust need to deduct incremental distance from
        // efery row after the first (top) row.
        if row > 0 {
            st_c.im -= fractals.pt_div;
        }

        // Calculate divergence for row.
        fractals.cal_row_divergence(row, st_c);
    }

    // Determine delta time for divergence calculation.
    fractals.calc_duration = calc_start.elapsed();
    info!("Divergence calculations in: {:?}", fractals.calc_duration);

    // At this point we have divergence iterations at every point.
    fractals.state = AppState::DivComplete;
}

// Function to print out the state of the class variables.
pub fn print_class(fractals : &mut Fractal) {
    println!("App state      : {:?}", fractals.state);
    println!("Rows           : {:?}", fractals.rows);
    println!("Columns        : {:?}", fractals.cols);
    println!("Centre point   : {:?}", fractals.mid_pt);
    println!("Point division : {:?}", fractals.pt_div);
    println!("Max iterations : {:?}", fractals.max_its);
    println!("Left limit     : {:?}", fractals.left_lim);
    println!("Top limit      : {:?}", fractals.top_lim);
    println!("Left top point : {:?}", fractals.pt_lt);
}

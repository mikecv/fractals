// Application menu functions.

use log::{info};

use inline_colorization::*;
use num_complex::Complex;
use std::io::{self, Write};
use std::str::FromStr;
use std::time::{Instant};

use crate::fractal::Fractal;

// Print the menu prompt / selections.
pub fn print_menu() {
    println!("{color_green}{style_bold}\n====\nMenu\n====\n{style_reset}{color_reset}");
    println!("A) Enter new fractal settings");
    println!("B) Initialise fractal from file");
    println!("C) Calculate fractal divergence");
    println!("D) Save fractal settings & results to file");
    println!("E) Print class variables");
    println!("{color_red}{style_bold}Q) Quit\n{style_reset}{color_reset}");
}

// Get user input for string input.
// Used for menu selection.
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Get the user input(s0) for the menu selection.
// Check input against required type.
pub fn get_user_input_numeric<T: FromStr>(prompt: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        // Attempt to parse the input to the expected type `T`.
        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid value."),
        }
    }
}

// User selected option to initialise new fractal.
// Does type checking.
pub fn enter_fractal(fractals : &mut Fractal) {
    info!("Initialising new fractal by user.");

    let rows: u32 = get_user_input_numeric("Number of rows: ");
    let cols: u32 = get_user_input_numeric("Number of columns: ");
    let mid_pt_r: f64  = get_user_input_numeric("Midpoint Real axis: ");
    let mid_pt_i: f64 = get_user_input_numeric("Midpoint Imaginary axis: ");
    let pt_div: f64 = get_user_input_numeric("Point division: ");
    let max_its: u32 = get_user_input_numeric("Max iterations: ");
    fractals.mid_pt = Complex::new(mid_pt_r, mid_pt_i);
    fractals.max_its = max_its;
    fractals.pt_div = pt_div;
    fractals.init_fractal_image(rows,
                                cols,
                                fractals.mid_pt,
                                fractals.pt_div);
    info!("Fractal rows: {}, cols: {}", fractals.rows, fractals.cols);
    info!("Fractal centrepoint: {}", fractals.mid_pt);
    info!("Fractal point division: {}", fractals.pt_div);
    info!("Fractal max iterations: {}", fractals.max_its);

    // At this point we have an initialised fractal.
    // From here we can re-initialise a new fractal or proceed to calculate
    // point divergence for the initialised fractal.
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
    // The next stage will be to configure the rendering.
}

// Function to print out the state of most of the class variables.
pub fn print_class(fractals : &mut Fractal) {
    println!("Rows           : {:?}", fractals.rows);
    println!("Columns        : {:?}", fractals.cols);
    println!("Centre point   : {:?}", fractals.mid_pt);
    println!("Point division : {:?}", fractals.pt_div);
    println!("Max iterations : {:?}", fractals.max_its);
    println!("Left limit     : {:?}", fractals.left_lim);
    println!("Top limit      : {:?}", fractals.top_lim);
    println!("Left top point : {:?}", fractals.pt_lt);
}

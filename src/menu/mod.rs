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
    println!("D) Define colour profile");
    println!("E) Render fractal image");

    println!("F) Save fractal settings & results to file");
    println!("G) Print class variables");

    println!("{color_red}{style_bold}\nQ) Quit\n{style_reset}{color_reset}");
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
}

// User selected option to initialise new fractal.
// As for function enter_fractal except settings read from file.
pub fn load_settings(fractals : &mut Fractal) {
    info!("Initialising new fractal from file.");

    // Default filename.
    let default_file_name = &fractals.settings.fractal_file;

    // Prompt the user for the filename.
    print!("Enter the filename (without path) [default: {}]: ", default_file_name);
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the user's entry.
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read filename");
    let file_name = file_name.trim();

    // Use the default filename if the user enters nothing.
    let file_name = if file_name.is_empty() {
        default_file_name.to_string()
    } else if file_name.contains('.') {
        file_name.to_string()
    } else {
        format!("{}.toml", file_name)
    };

    // Construct the full file path.
    let file_path = format!("{}/{}", fractals.settings.fractals_folder, file_name);

    // Now call load_config without conflicting borrows.
    let _load_status = fractals.load_config(&file_path);   
    match _load_status {
        Ok(_load_status) => println!("Settings loaded from: {}", file_path),
        Err(_) => println!("Failed to read from file: {:?}", file_path),
    }
}

// Save fractal settings to file.
pub fn save_settings(fractals: &mut Fractal) {
    // Default filename.
    let default_file_name = &fractals.settings.fractal_file;

    // Prompt the user for the filename.
    print!("Enter the filename (without path) [default: {}]: ", default_file_name);
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the user's entry.
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read filename");
    let file_name = file_name.trim();

    // Use the default filename if the user enters nothing.
    let file_name = if file_name.is_empty() {
        default_file_name.to_string()
    } else if file_name.contains('.') {
        file_name.to_string()
    } else {
        format!("{}.toml", file_name)
    };

    // Construct the full file path.
    let file_path = format!("{}/{}", fractals.settings.fractals_folder, file_name);

    // Save the configuration to the file.
    let _save_status = fractals.save_config(&file_path);

    match _save_status {
        Ok(_save_status) => println!("Settings saved to: {}", file_path),
        Err(_) => println!("Failed to save to file: {:?}", file_path),
    }
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
        // Just need to deduct incremental distance from
        // every row after the first (top) row.
        if row > 0 {
            st_c.im -= fractals.pt_div;
        }

        // Calculate divergence for row.
        fractals.cal_row_divergence(row, st_c);
    }

    // Determine delta time for divergence calculation.
    fractals.calc_duration = calc_start.elapsed();
    info!("Divergence calculations in: {:?}", fractals.calc_duration);
}

// Function to define the colour profile to use
// when rendering images.
// Defined as an array of inflection points with a
// corresponding rgb value.
pub fn def_col_profile(_fractals : &mut Fractal) {
    info!("Defining colour profile.");

}

// Function to render the image according to the
// defined colour profile..
pub fn render_image(fractals : &mut Fractal) {
    info!("Rendering image according to profile.");

    // Initialise timer for image renderingn.
    let render_start = Instant::now();

    // Determine delta time for rendering.
    fractals.render_duration = render_start.elapsed();
    info!("Image rendering in: {:?}", fractals.render_duration);
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
    println!("Colour profile : {:?}", fractals.col_palete);
}

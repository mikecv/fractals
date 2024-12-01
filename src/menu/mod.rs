// Application menu functions.

use log::{info};

use image::{Rgb, RgbImage};
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
    println!("D) Define colour palete");
    println!("E) Render fractal image");
    println!("F) Generate iterations historgram");

    println!("G) Save fractal settings & results to file");
    println!("H) Print class variables");

    println!("{color_red}{style_bold}\nQ) Quit\n{style_reset}{color_reset}");
}

// Get user input for string input.
// Used for menu selection.
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string();
    input.to_lowercase()
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

    info!("Initialising new fractal from {:?}", file_path);
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
    println!("Divergence calculations in: {:?}", fractals.calc_duration);
}

// Function to define the colour palete to use
// when rendering images.
// Defined as an array of iteration boundary limits and a
// corresponding rgb value at that boundary.
pub fn def_col_palete(fractals : &mut Fractal) {
    info!("Defining colour palete.");

    println!("Enter palete boundary details.");
    println!("Enter iteration count, followed by RGB colour at boundary.");
    println!("End with boundary at max iterations: {:?}", fractals.max_its);

    // Number of index boundary.
    let mut idx: u8 = 0;

    // First colour boundary at 0 iterations.
    let mut its_bound: u32 = 1;
    println!("({:02}) Iteration boundary: {:?}", idx, its_bound);
    let mut red: u8 = get_user_input_numeric("     RED colour component: ");
    let mut green: u8 = get_user_input_numeric("     GREEN colour component: ");
    let mut blue: u8 = get_user_input_numeric("     BLUE colour component: ");
    add_colour_to_palete(fractals, its_bound, (red, green, blue));

    // Increment bountary index.
    idx += 1;

    // Keep prompting for palete boundary details until max iterations is reached.
    while its_bound < fractals.max_its {

        let its_bound_prompt = format!("({:02}) Iterations boundary: ", idx);
        its_bound = get_user_input_numeric(&its_bound_prompt);
        red = get_user_input_numeric("     RED colour component: ");
        green = get_user_input_numeric("     GREEN colour component: ");
        blue = get_user_input_numeric("     BLUE colour component: ");    

        // Need to check if boundary outside max bounds.
        // If greater that bounds set to max iterations.
        if its_bound > fractals.max_its {
            its_bound = fractals.max_its;
        }   

        // Add next colour boundary and colour to array.
        add_colour_to_palete(fractals, its_bound, (red, green, blue));

        // Increment boundary count and loop.
        idx += 1;
    }
}

// Function to add an entry to the colour palete array.
// This is colour at a particular number  of iterations
pub fn add_colour_to_palete(fractals : &mut Fractal, its_bound: u32, color: (u8, u8, u8)) {
    fractals.col_palete.push((its_bound, color));
}

// Function to render the image according to the
// defined colour palete.
pub fn render_image(fractals : &mut Fractal) {
    info!("Rendering image according to colour palete.");

    print!("Enter the image filename (ext .png): ");
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the user's entry.
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read filename");
    let file_name = file_name.trim();

    // Construct the full file path.
    let file_path = format!("{}/{}", fractals.settings.fractals_folder, file_name);
    info!("Saving image to file: {:?}", file_path);

    // Initialise timer for image renderingn.
    let render_start = Instant::now();

    // Define an image of the right size.
    let rows = fractals.rows;
    let cols = fractals.cols;
    let mut img = RgbImage::new(cols, rows);

    // Iterate through rows and columuns and
    // set the pixel colour accordingly.
    for y in 0..rows {
        for x in 0..cols{
            let pt_its: u32 = fractals.escape_its[y as usize][x as usize];
            let px_col: Rgb<u8> = det_px_col(pt_its, &fractals.col_palete);
            img.put_pixel(x, y, px_col);
        }
    }

    // Save the image.
    let _ = img.save(file_path);

    // Determine delta time for rendering.
    fractals.render_duration = render_start.elapsed();
    info!("Image rendering in: {:?}", fractals.render_duration);
    println!("Image rendering in: {:?}", fractals.render_duration);
}

// Function to determine the colour of the pixel.
// Based on linear interpolation of colour palete.
pub fn det_px_col(its: u32, col_pal: &Vec<(u32, (u8, u8, u8))>) -> Rgb<u8> {

    // Iterate through the boundaries to find where `its` fits
    // between consecutive boundaries.
    for i in 0..col_pal.len() - 1 {
        let (lower_bound, lower_color) = col_pal[i];
        let (upper_bound, upper_color) = col_pal[i + 1];

        if its > lower_bound && its <= upper_bound {
            // Perform linear interpolation between the two colours.
            let t = (its - lower_bound) as f32 / (upper_bound - lower_bound) as f32;
            let r = (1.0 - t) * lower_color.0 as f32 + t * upper_color.0 as f32;
            let g = (1.0 - t) * lower_color.1 as f32 + t * upper_color.1 as f32;
            let b = (1.0 - t) * lower_color.2 as f32 + t * upper_color.2 as f32;

            // Return interpolated colour for the pixel.
            return Rgb([r as u8, g as u8, b as u8]);
        }
    }

    // Handle the case where `its` doesn't fit into any range.
    // For simplicity, return the last colour in the palette.
    if let Some(&(last_bound, last_color)) = col_pal.last() {
        if its > last_bound {
            return Rgb([last_color.0, last_color.1, last_color.2]);
        }
    }

    // Default fallback colour (e.g., black).
    Rgb([0, 0, 0])
}

// Generate iterations count histogram plot.
// Useful tell when generating colour paletes as shows
// iteration hot spots.
pub fn generate_histogram(_fractals : &mut Fractal) {
    info!("Generating iterations histogram.");
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
    println!("Colour palete  : {:?}", fractals.col_palete);
}

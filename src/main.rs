use log::info;
use log4rs;

use std::path::PathBuf;
use std::fs::create_dir_all;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::settings::Settings;
use crate::fractal::Fractal;

pub mod settings;
pub mod fractal;

mod menu;

// Load program settings.
async fn load_settings() -> Settings {
    // Deserialise settings file into settings struct.
    let mut file = File::open("settings.yml").await.expect("Unable to open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.expect("Unable to read file.");
    let settings: Settings = serde_yaml::from_str(&contents).expect("Unable to parse YAML.");
    settings
}

fn main() {
    // Logging configuration held in log4rs.yml.
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Create the async runtime and run only the settings loading async block.
    let rt = tokio::runtime::Runtime::new().unwrap();
    let settings = rt.block_on(load_settings());

    // Check if folder for results exists, if not, create it.
    let mut wrt_path = PathBuf::new();       
    wrt_path.push(&settings.fractals_folder);
    if !wrt_path.exists() {
        create_dir_all(&wrt_path).unwrap();
    }

    // Now that settings have been loaded asynchronously, run the rest of the program synchronously.
    info!("Application started: {} v({})", settings.program_name, settings.program_ver);

    // Create fractals class instance.
    let mut fractals: Fractal = Fractal::init(settings);

    // Command line application menu.
    // Keep looping until user selects the quit option.
    loop {
        // Display the menu applicable to the application state.
        menu::print_menu();

        // Get the user's parameter(s) selection.
        let choice = menu::get_user_input("Option: ");

        // Apply the users selection.
        match choice.trim() {
            // Initialise new fractal (user entry).
            "a" => menu::enter_fractal(&mut fractals),

            // Initialise new fractal from file.
            "b" => menu::load_settings(&mut fractals),

            // Calculate fractal divergence.
            "c" => menu::cal_divergence(&mut fractals),

            // Define colour palete for rendering.
            "d" => menu::def_col_palete(&mut fractals),

            // Render image according to palete.
            "e" => menu::render_image(&mut fractals),

            // Save fractal settings and results to files.
            "f" => menu::save_settings(&mut fractals),

            // Print class variables.
            "g" => menu::print_class(&mut fractals),

            // Quitting application.
            "q" => {
                println!("Quitting...");
                break;
            }

            // Invalid option selected.
            _ => println!("Invalid option."),
        }
    }
}

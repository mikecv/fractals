use log::info;
use log4rs;

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

// Define constance for program state.
pub enum AppState {
    AppStart,
    NewFractal,
}

fn main() {
    // Logging configuration held in log4rs.yml.
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Create the async runtime and run only the settings loading async block.
    let rt = tokio::runtime::Runtime::new().unwrap();
    let settings = rt.block_on(load_settings());

    // Now that settings have been loaded asynchronously, run the rest of the program synchronously.
    info!("Application started: {} v({})", settings.program_name, settings.program_ver);

    // create fractals class instance.
    let mut fractals: Fractal = Fractal::init(settings);

    // Command line application menu.
    // Keep looping until user selects the quit option.
    loop {
        // Display the menu applicable to the application state.
        menu::print_menu(&fractals.state);

        // Get the user's parameter(s) selection.
        let choice = menu::get_user_input("Option: ");

        // Apply the users selection.
        match choice.trim() {
            // Initialise new fractal.
            "1" => menu::new_fractal(&mut fractals),

            // Option 2 selected.
            "2" => menu::option_two(&mut fractals),

            // Quitting application.
            "q" | "Q" => {
                println!("\nQuitting...");
                break;
            }

            // Invalid option selected.
            _ => println!("Invalid option.\n"),
        }
    }
}

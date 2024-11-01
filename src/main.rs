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

fn main() {
    // Logging configuration held in log4rs.yml.
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Create the async runtime and run only the settings loading async block.
    let rt = tokio::runtime::Runtime::new().unwrap();
    let settings = rt.block_on(load_settings());

    // Now that settings have been loaded asynchronously, run the rest of the program synchronously.
    info!("Application started: {} v({})", settings.program_name, settings.program_ver);

    // create fractals class instance.
    let _fractals: Fractal = Fractal::init(settings);

    // Command line application menu.
    // Keep looping until user selects the quit option.
    loop {
        menu::print_menu();
        let choice = menu::get_user_input("\nOption: ");

        // Get the user's choice and apply.
        match choice.trim() {
            // Option 1 selected.
            "1" => menu::option_one(),

            // Option 2 selected.
            "2" => menu::option_two(),

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

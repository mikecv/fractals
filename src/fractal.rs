// Fractals data structure and methods.

use log::{info};
use crate::settings::Settings;

// Struct of parameters fractals generation.
pub struct Fractal {
    pub settings: Settings,
}

// Initialise all struct variables.
// This method called at the start.
impl Fractal {
    pub fn init(settings: Settings) -> Self {
        info!("Initialising Fractal struct.");

        Fractal {
            settings: settings,
        }
    }
}

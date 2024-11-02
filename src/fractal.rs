// Fractals data structure and methods.

use log::{info};
use crate::settings::Settings;
use crate::AppState;

// Struct of parameters fractals generation.
pub struct Fractal {
    pub settings: Settings,
    pub state: AppState,
    pub rows: u32,
    pub cols: u32,
    pub max_its: u32,
}

// Initialise all struct variables.
// This method called at the start.
impl Fractal {
    pub fn init(settings: Settings) -> Self {
        info!("Initialising Fractal struct.");

        Fractal {
            settings: settings,
            state: AppState::AppStart,
            rows: 0,
            cols: 0,
            max_its: 0,
        }
    }
}

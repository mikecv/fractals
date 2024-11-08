// Fractals data structure and methods.

use log::{info};

use num_complex::Complex;

use crate::settings::Settings;
use crate::AppState;

// Struct of parameters fractals generation.
pub struct Fractal {
    pub settings: Settings,
    pub state: AppState,
    pub rows: u32,
    pub cols: u32,
    pub mid_pt: Complex<f64>,
    pub pt_div: f64,
    pub max_its: u32,
    pub left_lim: f64,
    pub top_lim: f64,
    pub escape_its: Vec<Vec<u32>>,
    pub pt_lt: Complex<f64>,
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
            mid_pt: Complex::new(0.0, 0.0),
            pt_div: 0.0,
            max_its: 0,
            left_lim: 0.0,
            top_lim: 0.0,
            escape_its: Vec::new(),
            pt_lt: Complex::new(0.0, 0.0),
        }
    }
 
    // Method to initialize fractal image size,
    // and declare array size for interation counts.
    // Also calculate the left / top limits for iteration start points.
    pub fn init_fractal_image(&mut self, rows: u32, cols: u32, _mid_pt: Complex<f64>, _pt_div: f64) {
        // Rows and column size, and matching arrays.
        self.rows = rows;
        self.cols = cols;
        self.escape_its = vec![vec![0; cols as usize]; rows as usize];

        // Left vertice for iterating across columns.
        let left_offset: f64 = self.mid_pt.re - (self.cols as f64 / 2.0) * self.pt_div;
        self.left_lim = left_offset;

        // Top vertice for iterating across rows.
        let top_offset: f64 = self.mid_pt.im + (self.rows as f64 / 2.0) * self.pt_div;
        self.top_lim = top_offset;

        // Left top vertice.
        self.pt_lt.re = self.left_lim;
        self.pt_lt.im = self.top_lim;
    }

    // Methed to calculate fractal divergence at a single point.
    // TODO
    pub fn cal_row_divergence(&mut self, _row: u32, st_c: Complex<f64>) {
        println!("Divergence: {:?}", st_c);
    }
}

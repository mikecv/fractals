// Fractals data structure and methods.

use log::{info};

use num_complex::Complex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::f64::consts;
use std::io::{self};
use std::time::{Duration};
use toml;

use crate::settings::Settings;

// Struct of parameters for fractals generation.
pub struct Fractal {
    pub settings: Settings,
    pub rows: u32,
    pub cols: u32,
    pub mid_pt: Complex<f64>,
    pub pt_div: f64,
    pub max_its: u32,
    pub left_lim: f64,
    pub top_lim: f64,
    pub escape_its: Vec<Vec<u32>>,
    pub pt_lt: Complex<f64>,
    pub col_palete: Vec<(f32, (u8, u8, u8))>,
    pub calc_duration: Duration,
    pub render_duration: Duration,
}

// Sub-Struct of parameters for fractal setting.
// These are the parameters saved to file.
#[derive(Serialize, Deserialize)]
pub struct FractalConfig {
    pub rows: u32,
    pub cols: u32,
    pub mid_pt: (f64, f64),
    pub pt_div: f64,
    pub max_its: u32,
    pub col_palete: Vec<(f32, (u8, u8, u8))>,
    pub escape_its: Vec<Vec<u32>>,
}

// Initialise all struct variables.
// This method called at the start.
impl Fractal {
    pub fn init(settings: Settings) -> Self {
        info!("Initialising Fractal struct.");

        Fractal {
            settings: settings,
            rows: 0,
            cols: 0,
            mid_pt: Complex::new(0.0, 0.0),
            pt_div: 0.0,
            max_its: 0,
            left_lim: 0.0,
            top_lim: 0.0,
            escape_its: Vec::new(),
            pt_lt: Complex::new(0.0, 0.0),
            col_palete: Vec::new(),
            calc_duration: Duration::new(0, 0),
            render_duration: Duration::new(0, 0),
        }
    }
 
    // Save fractal settings to FractalConfig.
    pub fn to_config(&self) -> FractalConfig {
        FractalConfig {
            rows: self.rows,
            cols: self.cols,
            mid_pt: (self.mid_pt.re, self.mid_pt.im),
            pt_div: self.pt_div,
            max_its: self.max_its,
            col_palete: self.col_palete.clone(),
            escape_its: self.escape_its.clone(),
        }
    }

    // Load FractalConfig and update Fractal (self).
    pub fn from_config(&mut self, config: FractalConfig) {
        self.rows = config.rows;
        self.cols = config.cols;
        self.mid_pt = Complex::new(config.mid_pt.0, config.mid_pt.1);
        self.pt_div = config.pt_div;
        self.max_its = config.max_its;
        self.col_palete = config.col_palete;
    }

    // Save FractalConfig to a TOML file.
    pub fn save_config(&mut self, path: &str) -> io::Result<()> {
        let config = self.to_config();
        let toml_str = toml::to_string(&config).expect("Failed to serialize config");
        fs::write(path, toml_str)?;
        Ok(())
    }

    // Load FractalConfig from a TOML file.
    pub fn load_config(&mut self, path: &str) -> io::Result<()> {
        let toml_str = fs::read_to_string(path)?;
        let config: FractalConfig = toml::from_str(&toml_str).expect("Failed to deserialize config");
        self.from_config(config);
        Ok(())
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
    // For points that reach the iteration count caculate
    // fractional divergence.
    pub fn cal_row_divergence(&mut self, row: u32, st_c: Complex<f64>) {
        // Iterante over all the columns in the row.
        // Starting point is left of the row.
        let mut pt_row: Complex<f64> = st_c;

        for col in 0..self.cols {
            // Iterate point along th
            if col > 0 {
                pt_row.re += self.pt_div;
            }

            // Define diverges flag and set to false.
            let mut diverges: bool = false;

            // Initialise divergence result to complex 0.
            let mut px_fn: Complex<f64> = Complex::new(0.0, 0.0);

            // Initialise number of iterations.
            let mut num_its: u32 = 1;

            // Keep iterating until function diverges.
            while !diverges && (num_its < self.max_its) {
                // Perform Mandelbrot function Fn+1 = Fn^2 + pt_row.
                px_fn = (px_fn * px_fn) + pt_row;
                // Check if function diverges.
                // Will diverge if modulus equal or greater than 2.
                if px_fn.norm() >= 2.0 {
                    diverges = true;
                }
                else {
                    num_its += 1;
                }
            }

            // Calculate fractional divergence for higher definition.
            let mod_fn = px_fn.norm();
            let mu_log = if mod_fn > consts::E {
                (mod_fn.ln().ln()) / consts::LN_2
            } else {
                0.0
            };
            let mut mu = num_its as f64 + 1.0 - mu_log;

            // Limit fractional divergence to maximum iterations
            if mu > self.max_its as f64 {
                mu = self.max_its as f64;
            }
            num_its = mu as u32;

            // Save number of iterations for point.
            self.escape_its[row as usize][col as usize] = num_its;
        }
    }
}

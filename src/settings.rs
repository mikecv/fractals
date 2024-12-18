use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub program_name: String,
    pub program_ver: String,
    pub program_devs: Vec<String>,
    pub program_web: String,
    pub fractals_folder: String,
    pub fractal_file: String,
    pub hist_plot_log: bool,
}

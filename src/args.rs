use std::path::PathBuf;

use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about)]
pub struct Arguments {
    /// Gerber / Drill file to process
    pub input: PathBuf,
    /// Optional outline layer
    #[clap(long, short)]
    pub outline: Option<PathBuf>,
    /// SVG file to output
    pub output: PathBuf,

    #[clap(flatten)]
    pub config: Configuration,
}

#[derive(Args)]
pub struct Configuration {
    /// Aperture radius multiplier
    #[clap(long, short, default_value_t = 1.0)]
    pub aperture_thickness: f64,

    /// Trace thickness multiplier
    #[clap(long, short, default_value_t = 1.0)]
    pub trace_thickness: f64,

    /// Ignore traces, only export apatures
    #[clap(long, short, default_value_t = false)]
    pub pads_only: bool,
}

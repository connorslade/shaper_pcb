use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Gerber file to process
    pub input: PathBuf,

    /// Optional outline layer
    #[clap(short)]
    pub outline: Option<PathBuf>,

    /// SVG file to output
    pub output: PathBuf,
}

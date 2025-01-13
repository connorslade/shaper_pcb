use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use anyhow::{bail, Result};
use args::Arguments;
use clap::Parser;
use format::drill::DrillFile;
use gerber_parser::{gerber_doc::GerberDoc, parser::parse_gerber};
use pcb::Pcb;

mod args;
mod format;
mod geometry;
mod pcb;
mod point;

const CIRCLE_SIDES: u32 = 50;

fn main() -> Result<()> {
    let args = Arguments::parse();

    let mut pcb = Pcb::new(args.config);

    let extension = file_extension(&args.input);
    match extension.as_str() {
        "gbr" => pcb.add_traces(load_gerber(&args.input)?),
        "drl" => pcb.add_drill(DrillFile::parse(&fs::read_to_string(args.input)?)?),
        _ => bail!("Unknown file format: .{extension}"),
    }

    if let Some(outline) = args.outline {
        pcb.add_guide(load_gerber(&outline)?);
    }

    let svg = pcb.into_svg();
    fs::write(args.output, svg.to_string())?;
    Ok(())
}

fn file_extension(path: &Path) -> String {
    path.extension().unwrap().to_string_lossy().into_owned()
}

fn load_gerber(path: &Path) -> Result<GerberDoc> {
    let file = File::open(path)?;
    Ok(parse_gerber(BufReader::new(file)))
}

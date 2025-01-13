use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

use anyhow::Result;
use args::Arguments;
use clap::Parser;
use format::drill::DrillFile;
use gerber_parser::gerber_doc::GerberDoc;
use pcb::Pcb;

mod args;
mod format;
mod geometry;
mod pcb;
mod point;

const CIRCLE_SIDES: u32 = 50;

fn main() -> Result<()> {
    let drill = DrillFile::parse(&fs::read_to_string("/home/connorslade/Documents/LibrePCB/projects/Relay Logic/Better_XOR_Gate/output/v1/gerber/Better_XOR_Gate_DRILLS-PTH.drl")?)?;

    let args = Arguments::parse();

    let mut pcb = Pcb::new(args.config);
    pcb.add_traces(load_gerber(&args.input)?);
    pcb.add_drill(drill);

    if let Some(outline) = args.outline {
        pcb.add_guide(load_gerber(&outline)?);
    }

    let svg = pcb.into_svg();
    fs::write(args.output, svg.to_string())?;
    Ok(())
}

fn load_gerber(path: &Path) -> Result<GerberDoc> {
    let file = File::open(path)?;
    Ok(gerber_parser::parser::parse_gerber(BufReader::new(file)))
}

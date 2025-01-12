use std::{
    fs::{self, File},
    io::BufReader,
    mem,
};

use anyhow::Result;
use gerber_types::{Command, DCode, FunctionCode, Operation};
use i_overlay::{
    core::{fill_rule::FillRule, overlay_rule::OverlayRule},
    float::single::SingleFloatOverlay,
};
use svg::{node::element::Polygon, Document};

mod geometry;
mod point;
use geometry::{close_path, generate_circle};

const CIRCLE_SIDES: u32 = 20;

fn main() -> Result<()> {
    let input = File::open("/home/connorslade/Documents/LibrePCB/projects/Relay Logic/RS_Latch/output/v1/gerber/Relay_Logic_COPPER-TOP.gbr")?;
    let doc = gerber_parser::parser::parse_gerber(BufReader::new(input));

    let path_thickness = 0.5 / 2.0;

    let mut path = Vec::new();
    let mut paths = Vec::new();

    for cmd in doc.commands {
        match cmd {
            Command::FunctionCode(FunctionCode::DCode(code)) => match code {
                DCode::Operation(Operation::Move(mov)) => {
                    if !path.is_empty() {
                        paths.push(close_path(mem::take(&mut path), path_thickness));
                    }

                    let point = mov.into();
                    paths.push(generate_circle(point, path_thickness / 2.0, CIRCLE_SIDES));
                    path.push(point);
                }
                DCode::Operation(Operation::Interpolate(pos, _offset)) => {
                    let point = pos.into();
                    paths.push(generate_circle(point, path_thickness / 2.0, CIRCLE_SIDES));
                    path.push(point);
                }
                _ => {}
            },
            _ => {}
        }
    }

    if !path.is_empty() {
        paths.push(close_path(path, path_thickness));
    }

    let mut union = vec![vec![paths.remove(0)]];
    for path in paths.into_iter() {
        union = union.overlay(&[path], OverlayRule::Union, FillRule::EvenOdd);
    }

    let mut svg = Document::new();

    for shape in union.into_iter().flatten() {
        let node = Polygon::new().set(
            "points",
            shape.iter().map(|x| (x[0], x[1])).collect::<Vec<_>>(),
        );
        svg = svg.add(node);
    }

    fs::write("out.svg", svg.to_string())?;
    Ok(())
}

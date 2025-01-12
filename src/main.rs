use std::{
    f64::{self, consts::PI},
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
use itertools::Itertools;
use svg::{node::element::Polygon, Document};

type Point = [f64; 2];
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
                        paths.push(close_path(&mem::take(&mut path), path_thickness));
                    }

                    let x: f64 = mov.x.unwrap().into();
                    let y: f64 = mov.y.unwrap().into();
                    paths.push(generate_circle([x, y], path_thickness / 2.0, CIRCLE_SIDES));
                    path.push([x, y]);
                }
                DCode::Operation(Operation::Interpolate(pos, _offset)) => {
                    let x: f64 = pos.x.unwrap().into();
                    let y: f64 = pos.y.unwrap().into();
                    paths.push(generate_circle([x, y], path_thickness / 2.0, CIRCLE_SIDES));
                    path.push([x, y]);
                }
                _ => {}
            },
            _ => {}
        }
    }

    if !path.is_empty() {
        paths.push(close_path(&path, path_thickness));
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

/// Converts the input line into a polygon with the defined thickness
fn close_path(path: &Vec<Point>, path_thickness: f64) -> Vec<Point> {
    let mut out = Vec::new();
    let half_thickness = path_thickness / 2.0;

    for (p1, p2) in path.iter().tuple_windows() {
        let dx = p2[0] - p1[0];
        let dy = p2[1] - p1[1];
        let length = (dx * dx + dy * dy).sqrt();

        let px = -dy / length * half_thickness;
        let py = dx / length * half_thickness;

        out.push([p1[0] + px, p1[1] + py]);
        out.push([p1[0] - px, p1[1] - py]);
        out.push([p2[0] - px, p2[1] - py]);
        out.push([p2[0] + px, p2[1] + py]);
    }

    out
}

/// Generates a polygon approxapating a circle with the defined number of sides.
fn generate_circle(center: Point, radius: f64, sides: u32) -> Vec<Point> {
    let mut out = Vec::new();

    for i in 0..sides {
        let angle = (i as f64 / sides as f64) * 2.0 * PI;
        out.push([
            center[0] + angle.cos() * radius,
            center[1] + angle.sin() * radius,
        ]);
    }

    out
}

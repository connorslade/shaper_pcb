use core::f64;
use std::{
    fs::{self, File},
    io::BufReader,
    mem,
};

use anyhow::Result;
use args::Args;
use clap::Parser;
use gerber_types::{Aperture, Command, DCode, FunctionCode, Operation};
use i_overlay::{
    core::{fill_rule::FillRule, overlay_rule::OverlayRule},
    float::single::SingleFloatOverlay,
};
use point::Point;
use svg::{node::element::Polygon, Document};

mod args;
mod geometry;
mod point;
use geometry::{close_path, generate_circle, generate_rectangle};

const CIRCLE_SIDES: u32 = 100;

fn main() -> Result<()> {
    let args = Args::parse();

    let input = File::open(args.input)?;
    let doc = gerber_parser::parser::parse_gerber(BufReader::new(input));

    let mut aperture: Option<&Aperture> = None;
    let mut thickness = 0.0;

    let mut path = Vec::new();
    let mut paths = Vec::new();

    for cmd in doc.commands {
        match cmd {
            Command::FunctionCode(FunctionCode::DCode(code)) => match code {
                DCode::Operation(Operation::Move(mov)) => {
                    thickness = match aperture.unwrap() {
                        Aperture::Circle(circle) => circle.diameter,
                        _ => 0.0,
                    };

                    if !path.is_empty() {
                        paths.push(close_path(mem::take(&mut path), thickness));
                    }

                    let point = mov.into();
                    paths.push(generate_circle(point, thickness / 2.0, CIRCLE_SIDES));
                    path.push(point);
                }
                DCode::Operation(Operation::Interpolate(pos, _offset)) => {
                    let point = pos.into();
                    paths.push(generate_circle(point, thickness / 2.0, CIRCLE_SIDES));
                    path.push(point);
                }

                DCode::SelectAperture(x) => aperture = doc.apertures.get(&x),
                DCode::Operation(Operation::Flash(flash)) => {
                    let pos = flash.into();
                    match aperture {
                        Some(Aperture::Circle(circle)) => {
                            paths.push(generate_circle(pos, circle.diameter / 2.0, CIRCLE_SIDES))
                        }
                        Some(Aperture::Rectangle(rect) | Aperture::Obround(rect)) => {
                            paths.push(generate_rectangle(pos, Point::new(rect.x, rect.y)));
                        }
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }

    if !path.is_empty() {
        paths.push(close_path(path, thickness));
    }

    let mut union = vec![vec![paths.remove(0)]];
    for path in paths.into_iter() {
        union = union.overlay(&[path], OverlayRule::Union, FillRule::EvenOdd);
    }

    let (mut min, mut max) = (Point::repeat(f64::MAX), Point::repeat(f64::MIN));
    for point in union.iter().flatten().flatten() {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    }

    let mut svg = Document::new().set("viewBox", (min.x, min.y, max.x - min.x, max.y - min.y));

    for shape in union.iter().flatten() {
        let node = Polygon::new()
            .set("fill", "black")
            .set("stroke", "black")
            .set("stroke-width", "0")
            .set(
                "points",
                shape
                    .iter()
                    .map(|p| (p.x, -p.y + max.y + min.y))
                    .collect::<Vec<_>>(),
            );
        svg = svg.add(node);
    }

    fs::write(args.output, svg.to_string())?;
    Ok(())
}

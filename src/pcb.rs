use std::mem;

use gerber_parser::gerber_doc::GerberDoc;
use gerber_types::{Aperture, Command, DCode, FunctionCode, Operation};
use svg::{node::element::Polygon, Document};

use crate::{
    geometry::{bounds, close_path, generate_circle, generate_rectangle, union_shapes},
    point::Point,
    CIRCLE_SIDES,
};

#[derive(Default)]
pub struct Pcb {
    paths: Vec<Vec<Point>>,
    guides: Vec<Vec<Point>>,
}

impl Pcb {
    pub fn add_guide(&mut self, gerber: GerberDoc) {
        process_gerber(&mut self.guides, gerber);
    }

    pub fn add_traces(&mut self, gerber: GerberDoc) {
        process_gerber(&mut self.paths, gerber);
    }

    pub fn to_svg(self) -> Document {
        let trace_union = union_shapes(self.paths);
        let guide_union = union_shapes(self.guides);

        let (min, max) = bounds(
            trace_union
                .iter()
                .chain(guide_union.iter())
                .flatten()
                .flatten(),
        );

        let mut svg = Document::new().set("viewBox", (min.x, min.y, max.x - min.x, max.y - min.y));

        for shape in trace_union.iter().flatten() {
            svg = svg.add(
                create_polygon(shape, (min, max))
                    .set("fill", "black")
                    .set("stroke", "black")
                    .set("stroke-width", "0"),
            );
        }

        for shape in guide_union.iter().flatten() {
            svg = svg.add(
                create_polygon(shape, (min, max))
                    .set("fill", "none")
                    .set("stroke", "#0068FF")
                    .set("stroke-width", "0.1"),
            );
        }

        svg
    }
}

fn create_polygon(shape: &[Point], (min, max): (Point, Point)) -> Polygon {
    Polygon::new().set(
        "points",
        shape
            .iter()
            .map(|p| (p.x, -p.y + max.y + min.y))
            .collect::<Vec<_>>(),
    )
}

fn process_gerber(shapes: &mut Vec<Vec<Point>>, gerber: GerberDoc) {
    let mut aperture: Option<&Aperture> = None;
    let mut thickness = 0.0;
    let mut path = Vec::new();

    for cmd in gerber.commands {
        match cmd {
            Command::FunctionCode(FunctionCode::DCode(code)) => match code {
                DCode::Operation(Operation::Move(mov)) => {
                    thickness = match aperture.unwrap() {
                        Aperture::Circle(circle) => circle.diameter,
                        _ => 0.0,
                    };

                    if !path.is_empty() {
                        shapes.push(close_path(mem::take(&mut path), thickness));
                    }

                    let point = mov.into();
                    shapes.push(generate_circle(point, thickness / 2.0, CIRCLE_SIDES));
                    path.push(point);
                }
                DCode::Operation(Operation::Interpolate(pos, _offset)) => {
                    let point = pos.into();
                    shapes.push(generate_circle(point, thickness / 2.0, CIRCLE_SIDES));
                    path.push(point);
                }

                DCode::SelectAperture(x) => aperture = gerber.apertures.get(&x),
                DCode::Operation(Operation::Flash(flash)) => {
                    let pos = flash.into();
                    match aperture {
                        Some(Aperture::Circle(circle)) => {
                            shapes.push(generate_circle(pos, circle.diameter / 2.0, CIRCLE_SIDES))
                        }
                        Some(Aperture::Rectangle(rect) | Aperture::Obround(rect)) => {
                            shapes.push(generate_rectangle(pos, Point::new(rect.x, rect.y)));
                        }
                        _ => {}
                    }
                }
            },
            _ => {}
        }
    }

    if !path.is_empty() {
        shapes.push(close_path(path, thickness));
    }
}

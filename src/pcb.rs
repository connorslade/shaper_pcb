use std::mem;

use gerber_parser::gerber_doc::GerberDoc;
use gerber_types::{Aperture, Command, DCode, FunctionCode, Operation};
use svg::{node::element::Polygon, Document};

use crate::{
    args::Configuration,
    format::drill::DrillFile,
    geometry::{bounds, close_path, generate_circle, generate_rectangle, union_shapes},
    point::Point,
    CIRCLE_SIDES,
};

pub struct Pcb {
    config: Configuration,
    paths: Vec<Vec<Point>>,
    guides: Vec<Vec<Point>>,
}

impl Pcb {
    pub fn new(config: Configuration) -> Self {
        Self {
            config,
            paths: Vec::new(),
            guides: Vec::new(),
        }
    }

    pub fn add_guide(&mut self, gerber: GerberDoc) {
        process_gerber(&self.config, &mut self.guides, gerber, true);
    }

    pub fn add_traces(&mut self, gerber: GerberDoc) {
        process_gerber(&self.config, &mut self.paths, gerber, false);
    }

    pub fn add_drill(&mut self, drill: DrillFile) {
        for (_tool, operations) in drill.holes.iter() {
            let radius = operations.diameter * self.config.aperture_thickness / 2.0;
            for hole in operations.holes.iter() {
                self.paths
                    .push(generate_circle(*hole, radius, CIRCLE_SIDES));
            }
        }
    }

    pub fn into_svg(self) -> Document {
        let trace_union = union_shapes(self.paths);
        let guide_union = union_shapes(self.guides);

        let (min, max) = bounds(
            trace_union
                .iter()
                .chain(guide_union.iter())
                .flatten()
                .flatten(),
        );

        let (width, height) = (max.x - min.x, max.y - min.y);
        let mut svg = Document::new()
            .set("viewBox", (min.x, min.y, width, height))
            .set("width", format!("{width}mm"))
            .set("height", format!("{height}mm"));

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

fn process_gerber(
    config: &Configuration,
    shapes: &mut Vec<Vec<Point>>,
    gerber: GerberDoc,
    guide: bool,
) {
    let traces = !config.pads_only || guide;

    let mut aperture: Option<&Aperture> = None;
    let mut thickness = 0.0;
    let mut path = Vec::new();

    for cmd in gerber.commands {
        let Ok(Command::FunctionCode(FunctionCode::DCode(code))) = cmd else {
            continue;
        };

        match code {
            DCode::Operation(Operation::Move(mov)) if traces => {
                if !path.is_empty() {
                    shapes.push(close_path(mem::take(&mut path), thickness));
                }

                let point = mov.into();
                shapes.push(generate_circle(point, thickness / 2.0, CIRCLE_SIDES));
                path.push(point);
            }
            DCode::Operation(Operation::Interpolate(pos, _offset)) if traces => {
                let point = pos.into();
                shapes.push(generate_circle(point, thickness / 2.0, CIRCLE_SIDES));
                path.push(point);
            }

            DCode::SelectAperture(x) => {
                aperture = gerber.apertures.get(&x);
                if let Some(Aperture::Circle(circle)) = aperture {
                    thickness = circle.diameter * config.trace_thickness;
                }
            }
            DCode::Operation(Operation::Flash(flash)) => {
                let pos = flash.into();
                match aperture {
                    Some(Aperture::Circle(circle)) => shapes.push(generate_circle(
                        pos,
                        circle.diameter * config.aperture_thickness / 2.0,
                        CIRCLE_SIDES,
                    )),
                    Some(Aperture::Rectangle(rect)) => {
                        shapes.push(generate_rectangle(
                            pos,
                            Point::new(rect.x, rect.y) * config.aperture_thickness,
                        ));
                    }
                    Some(Aperture::Obround(rect)) => {
                        let rect = Point::new(rect.x, rect.y) * config.aperture_thickness;
                        let radius = rect.x.min(rect.y) / 2.0;
                        let mut circle = |offset: Point| {
                            shapes.push(generate_circle(pos + offset, radius, CIRCLE_SIDES));
                        };

                        let size = if rect.y < rect.x {
                            circle(Point::new(-rect.x / 2.0 + radius, 0.0));
                            circle(Point::new(rect.x / 2.0 - radius, 0.0));
                            Point::new(rect.x - rect.y, rect.y)
                        } else {
                            circle(Point::new(0.0, -rect.x / 2.0 + radius));
                            circle(Point::new(0.0, rect.x / 2.0 - radius));
                            Point::new(rect.x, rect.x)
                        };
                        shapes.push(generate_rectangle(pos, size));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    if !path.is_empty() {
        shapes.push(close_path(path, thickness));
    }
}

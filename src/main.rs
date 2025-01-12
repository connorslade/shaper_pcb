use std::{
    f64,
    fs::{self, File},
    io::BufReader,
};

use anyhow::Result;
use gerber_types::{Aperture, Command, DCode, FunctionCode, Operation};
use svg::{
    node::element::{path::Data, Circle, Path, Rectangle},
    Document,
};

fn main() -> Result<()> {
    let input = File::open("/home/connorslade/Documents/LibrePCB/projects/Relay_Logic/output/v1/gerber/Relay_Logic_COPPER-TOP.gbr")?;
    let doc = gerber_parser::parser::parse_gerber(BufReader::new(input));

    let mut svg = Document::new();

    let mut aperture = None;
    let mut path = Data::new();

    for cmd in doc.commands {
        match cmd {
            Command::FunctionCode(FunctionCode::DCode(code)) => match code {
                DCode::SelectAperture(x) => aperture = doc.apertures.get(&x),
                DCode::Operation(Operation::Flash(flash)) => {
                    let x: f64 = flash.x.unwrap().into();
                    let y: f64 = flash.y.unwrap().into();
                    match aperture {
                        Some(Aperture::Circle(circle)) => {
                            let node = Circle::new()
                                .set("cx", x)
                                .set("cy", y)
                                .set("r", circle.diameter / 2.0)
                                .set("fill", "black")
                                .set("stroke", "black")
                                .set("stroke-width", "0");
                            svg = svg.add(node);
                        }
                        Some(Aperture::Rectangle(rect) | Aperture::Obround(rect)) => {
                            let node = Rectangle::new()
                                .set("x", x)
                                .set("y", y)
                                .set("width", rect.x)
                                .set("height", rect.y)
                                .set("fill", "black")
                                .set("stroke", "black")
                                .set("stroke-width", "0");
                            svg = svg.add(node);
                        }
                        _ => {}
                    }
                }
                DCode::Operation(Operation::Move(mov)) => {
                    let x: f64 = mov.x.unwrap().into();
                    let y: f64 = mov.y.unwrap().into();

                    if !path.is_empty() {
                        let node = Path::new()
                            .set("fill", "none")
                            .set("stroke", "black")
                            .set("stroke-width", "0.5")
                            .set("stroke-linecap", "round")
                            .set("d", path);
                        svg = svg.add(node);
                    }

                    path = Data::new().move_to((x, y));
                }
                DCode::Operation(Operation::Interpolate(pos, _offset)) => {
                    let x: f64 = pos.x.unwrap().into();
                    let y: f64 = pos.y.unwrap().into();
                    path = path.line_to((x, y));
                }
            },
            _ => {}
        }
    }

    if !path.is_empty() {
        let node = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.5")
            .set("stroke-linecap", "round")
            .set("d", path);
        svg = svg.add(node);
    }

    fs::write("out.svg", svg.to_string())?;
    Ok(())
}

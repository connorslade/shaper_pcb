use std::collections::HashMap;

use anyhow::Result;

use crate::{format::parser::Parser, point::Point};

#[derive(Debug)]
pub struct DrillFile {
    pub holes: HashMap<u32, DrillOperations>,
}

#[derive(Debug)]
pub struct DrillOperations {
    pub diameter: f64,
    pub holes: Vec<Point>,
}

impl DrillFile {
    pub fn parse(file: &str) -> Result<Self> {
        let mut parser = Parser::new(file);
        parser.expect("M48")?;
        parser.next_line();

        let mut holes = HashMap::new();
        let mut selected_tool = 0;

        while !parser.is_eof() {
            let operation = parser.next();

            match operation {
                'T' => {
                    let id = parser.parse_int()?;
                    if parser.peek() == 'C' {
                        parser.next();
                        let diameter = parser.parse_float()?;
                        holes.insert(id, DrillOperations::new(diameter));
                    } else {
                        selected_tool = id;
                    }
                }
                'X' => {
                    let x = parser.parse_float()?;
                    parser.expect("Y")?;
                    let y = parser.parse_float()?;

                    let tools = holes.get_mut(&selected_tool).unwrap();
                    tools.holes.push(Point::new(x, y));
                }
                _ => {}
            }
            parser.next_line();
        }

        Ok(Self { holes })
    }
}

impl DrillOperations {
    fn new(diameter: f64) -> Self {
        Self {
            diameter,
            holes: Vec::new(),
        }
    }
}

use derive_more::derive::{Add, Deref, DerefMut, Div, Mul, Sub};
use gerber_types::Coordinates;
use i_float::float::compatible::FloatPointCompatible;
use nalgebra::Vector2;

#[derive(Deref, DerefMut, Add, Sub, Mul, Div, Clone, Copy)]
pub struct Point(Vector2<f64>);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2::new(x, y).into()
    }
}

impl From<Vector2<f64>> for Point {
    fn from(value: Vector2<f64>) -> Self {
        Self(value)
    }
}

impl From<Coordinates> for Point {
    fn from(value: Coordinates) -> Self {
        Vector2::new(value.x.unwrap().into(), value.y.unwrap().into()).into()
    }
}

impl FloatPointCompatible<f64> for Point {
    fn from_xy(x: f64, y: f64) -> Self {
        Vector2::new(x, y).into()
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

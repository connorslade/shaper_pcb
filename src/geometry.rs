use std::f64::consts::PI;

use i_overlay::{
    core::{fill_rule::FillRule, overlay_rule::OverlayRule},
    float::single::SingleFloatOverlay,
};
use itertools::Itertools;
use nalgebra::Vector2;

use crate::point::Point;

/// Converts the input line into a polygon with the defined thickness
pub fn close_path(path: Vec<Point>, path_thickness: f64) -> Vec<Point> {
    let mut out = Vec::with_capacity(4 * path.len());
    let half_thickness = path_thickness / 2.0;

    for (p1, p2) in path.into_iter().tuple_windows() {
        let delta = p2 - p1;
        let direction = delta.normalize();
        let normal = Vector2::new(-direction.y, direction.x)
            .scale(half_thickness)
            .into();
        out.extend_from_slice(&[p1 + normal, p1 - normal, p2 - normal, p2 + normal]);
    }

    out
}

/// Generates a polygon approxapating a circle with the defined number of sides.
pub fn generate_circle(center: Point, radius: f64, sides: u32) -> Vec<Point> {
    let mut out = Vec::new();

    for i in 0..sides {
        let angle = (i as f64 / sides as f64) * 2.0 * PI;
        let circle = Vector2::new(angle.cos(), angle.sin()) * radius;
        out.push(center + circle.into());
    }

    out
}

pub fn generate_rectangle(center: Point, size: Point) -> Vec<Point> {
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;

    let top_left = Point::new(center.x - half_width, center.y + half_height);
    let top_right = Point::new(center.x + half_width, center.y + half_height);
    let bottom_right = Point::new(center.x + half_width, center.y - half_height);
    let bottom_left = Point::new(center.x - half_width, center.y - half_height);

    vec![top_left, top_right, bottom_right, bottom_left]
}

pub fn union_shapes(mut shapes: Vec<Vec<Point>>) -> Vec<Vec<Vec<Point>>> {
    if shapes.is_empty() {
        return Vec::new();
    }

    let mut union = vec![vec![shapes.remove(0)]];
    for path in shapes.into_iter() {
        union = union.overlay(&[path], OverlayRule::Union, FillRule::EvenOdd);
    }
    union
}

pub fn bounds<'a>(points: impl Iterator<Item = &'a Point>) -> (Point, Point) {
    let (mut min, mut max) = (Point::repeat(f64::MAX), Point::repeat(f64::MIN));
    for point in points {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    }

    (min, max)
}

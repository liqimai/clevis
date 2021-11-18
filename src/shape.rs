use core::fmt::Debug;
use std::collections::HashMap;

pub trait Shape: Debug {}
pub type Shapes = HashMap<String, Box<dyn Shape>>;

pub type DataType = i32;

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: DataType,
    pub y: DataType,
}
impl Shape for Point {}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rectangle(pub Point, pub Point);
impl Shape for Rectangle {}

#[derive(Debug, Copy, Clone, Default)]
pub struct Line(pub Point, pub Point);
impl Shape for Line {}

#[derive(Debug, Copy, Clone, Default)]
pub struct Circle {
    pub center: Point,
    pub radius: DataType,
}
impl Shape for Circle {}

#[derive(Debug, Copy, Clone, Default)]
pub struct Square {
    pub corner: Point,
    pub side: DataType,
}
impl Shape for Square {}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_shapes() -> Shapes {
        let shapes: Shapes = HashMap::from([
            (
                "PointName".to_string(),
                Box::new(Point::default()) as Box<dyn Shape>,
            ),
            (
                "RectangleName".to_string(),
                Box::new(Rectangle::default()) as Box<dyn Shape>,
            ),
            (
                "LineName".to_string(),
                Box::new(Line::default()) as Box<dyn Shape>,
            ),
            (
                "CircleName".to_string(),
                Box::new(Circle::default()) as Box<dyn Shape>,
            ),
            (
                "SquareName".to_string(),
                Box::new(Square::default()) as Box<dyn Shape>,
            ),
        ]);

        shapes
    }
}

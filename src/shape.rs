use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

pub trait Shape<RenderType>: Debug {
    fn draw(&self, render: &mut RenderType) -> Result<(), Box<dyn Error>>;
}
pub type Shapes<RenderType> = HashMap<String, Box<dyn Shape<RenderType>>>;

pub type DataType = i32;

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: DataType,
    pub y: DataType,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Rectangle(pub Point, pub Point);

#[derive(Debug, Copy, Clone, Default)]
pub struct Line(pub Point, pub Point);

#[derive(Debug, Copy, Clone, Default)]
pub struct Circle {
    pub center: Point,
    pub radius: DataType,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Square {
    pub corner: Point,
    pub side: DataType,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_shapes<RenderType>() -> Shapes<RenderType>
    where
        Point: Shape<RenderType>,
        Rectangle: Shape<RenderType>,
        Line: Shape<RenderType>,
        Circle: Shape<RenderType>,
        Square: Shape<RenderType>,
    {
        let shapes: Shapes<RenderType> = HashMap::from([
            (
                std::any::type_name::<Point>().into(),
                Box::new(Point::default()) as Box<dyn Shape<RenderType>>,
            ),
            (
                std::any::type_name::<Rectangle>().into(),
                Box::new(Rectangle::default()) as Box<dyn Shape<RenderType>>,
            ),
            (
                std::any::type_name::<Line>().into(),
                Box::new(Line::default()) as Box<dyn Shape<RenderType>>,
            ),
            (
                std::any::type_name::<Circle>().into(),
                Box::new(Circle::default()) as Box<dyn Shape<RenderType>>,
            ),
            (
                std::any::type_name::<Square>().into(),
                Box::new(Square::default()) as Box<dyn Shape<RenderType>>,
            ),
        ]);

        shapes
    }
}

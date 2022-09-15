use crate::render::Renderer;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

pub trait Shape: Debug + Send {
    fn move_by(&mut self, x: DataType, y: DataType);
    fn draw_on(&self, render: &mut dyn Renderer) -> Result<(), Box<dyn Error>>;
}

pub type Shapes = HashMap<String, Box<dyn Shape>>;

pub type DataType = i32;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Point {
    pub x: DataType,
    pub y: DataType,
}
impl Shape for Point {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.x += x;
        self.y += y;
    }
    fn draw_on(&self, render: &mut dyn Renderer) -> Result<(), Box<dyn Error>> {
        render.draw_point(self)
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Rectangle {
    pub corner: Point,
    pub w: DataType,
    pub h: DataType,
}
impl Shape for Rectangle {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.corner.move_by(x, y);
    }
    fn draw_on(&self, render: &mut dyn Renderer) -> Result<(), Box<dyn Error>> {
        render.draw_rectangle(self)
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Line(pub Point, pub Point);
impl Shape for Line {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.0.move_by(x, y);
        self.1.move_by(x, y);
    }
    fn draw_on(&self, render: &mut dyn Renderer) -> Result<(), Box<dyn Error>> {
        render.draw_line(self)
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: DataType,
}
impl Shape for Circle {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.center.move_by(x, y);
    }
    fn draw_on(&self, render: &mut dyn Renderer) -> Result<(), Box<dyn Error>> {
        render.draw_circle(self)
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Square {
    pub corner: Point,
    pub side: DataType,
}
impl Shape for Square {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.corner.move_by(x, y);
    }
    fn draw_on(&self, render: &mut dyn Renderer) -> Result<(), Box<dyn Error>> {
        render.draw_square(self)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_shapes() -> Shapes
    where
        Point: Shape,
        Rectangle: Shape,
        Line: Shape,
        Circle: Shape,
        Square: Shape,
    {
        let shapes: Shapes = HashMap::from([
            (
                std::any::type_name::<Point>().into(),
                Box::new(Point::default()) as Box<dyn Shape>,
            ),
            (
                std::any::type_name::<Rectangle>().into(),
                Box::new(Rectangle::default()) as Box<dyn Shape>,
            ),
            (
                std::any::type_name::<Line>().into(),
                Box::new(Line::default()) as Box<dyn Shape>,
            ),
            (
                std::any::type_name::<Circle>().into(),
                Box::new(Circle::default()) as Box<dyn Shape>,
            ),
            (
                std::any::type_name::<Square>().into(),
                Box::new(Square::default()) as Box<dyn Shape>,
            ),
        ]);

        shapes
    }

    #[test]
    fn test_move_by() {
        let mut point = Point::default();
        point.move_by(2, 3);
        assert_eq!(point, Point { x: 2, y: 3 });

        let mut rect = Rectangle::default();
        rect.move_by(2, 3);
        assert_eq!(
            rect,
            Rectangle {
                corner: Point { x: 2, y: 3 },
                w: rect.w,
                h: rect.h
            }
        );

        let mut line = Line(Point { x: 1, y: 2 }, Point { x: 3, y: 4 });
        line.move_by(2, 3);
        assert_eq!(line, Line(Point { x: 3, y: 5 }, Point { x: 5, y: 7 },));

        let mut circle = Circle {
            center: Point { x: 1, y: 2 },
            radius: 5,
        };
        circle.move_by(2, 3);
        assert_eq!(
            circle,
            Circle {
                center: Point { x: 3, y: 5 },
                radius: circle.radius,
            }
        );

        let mut square = Square {
            corner: Point { x: 1, y: 2 },
            side: 5,
        };
        square.move_by(2, 3);
        assert_eq!(
            square,
            Square {
                corner: Point { x: 3, y: 5 },
                side: square.side,
            }
        );
    }
}

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;

pub trait Shape<RenderType>: Renderable<RenderType> + Movable + Debug + Send {}
impl<T, RenderType> Shape<RenderType> for T where T: Renderable<RenderType> + Movable + Debug + Send {}

pub trait Renderable<RenderType> {
    fn draw(&self, render: &mut RenderType) -> Result<(), Box<dyn Error>>;
}
pub trait Movable {
    fn move_by(&mut self, x: DataType, y: DataType);
}

pub type Shapes<RenderType> = HashMap<String, Box<dyn Shape<RenderType>>>;

pub type DataType = i32;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Point {
    pub x: DataType,
    pub y: DataType,
}
impl Movable for Point {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Rectangle {
    pub corner: Point,
    pub w: DataType,
    pub h: DataType,
}
impl Movable for Rectangle {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.corner.move_by(x, y);
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Line(pub Point, pub Point);
impl Movable for Line {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.0.move_by(x, y);
        self.1.move_by(x, y);
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: DataType,
}
impl Movable for Circle {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.center.move_by(x, y);
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Square {
    pub corner: Point,
    pub side: DataType,
}
impl Movable for Square {
    fn move_by(&mut self, x: DataType, y: DataType) {
        self.corner.move_by(x, y);
    }
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

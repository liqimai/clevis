use crate::shape::{Shape, Shapes};
use std::fmt;

pub trait Command: fmt::Debug + fmt::Display {
    fn execute(&mut self, shapes: &mut Shapes);
    fn undo(&mut self, shapes: &mut Shapes);
}

#[derive(Debug)]
pub struct DrawShape<T: Shape> {
    name: String,
    shape: T,
}

impl<T: Shape + Default> Default for DrawShape<T> {
    fn default() -> Self {
        Self {
            name: std::any::type_name::<T>().into(),
            shape: T::default(),
        }
    }
}

impl<T: Shape> fmt::Display for DrawShape<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.shape)
    }
}

impl<T: 'static + Shape + Copy> Command for DrawShape<T> {
    fn execute(&mut self, shapes: &mut Shapes) {
        shapes.insert(self.name.clone(), Box::new((self.shape).clone()));
    }
    fn undo(&mut self, shapes: &mut Shapes) {
        shapes.remove(&self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::*;

    #[test]
    fn execute() {
        let mut cmd1 = DrawShape {
            shape: Point::default(),
            name: "p1".to_string(),
        };
        let mut cmd2 = DrawShape {
            shape: Rectangle::default(),
            name: "p2".to_string(),
        };
        let mut shapes = Shapes::default();

        cmd1.execute(&mut shapes);
        assert_eq!(
            format!("{:?}", shapes[&cmd1.name]),
            format!("{:?}", cmd1.shape),
        );

        cmd1.undo(&mut shapes);
        assert!(shapes.get(&cmd1.name).is_none());
        assert_eq!(shapes.len(), 0);

        cmd1.execute(&mut shapes);
        cmd2.execute(&mut shapes);
        assert_eq!(
            format!("{:?}", shapes[&cmd1.name]),
            format!("{:?}", cmd1.shape),
        );
        assert_eq!(
            format!("{:?}", shapes[&cmd2.name]),
            format!("{:?}", cmd2.shape),
        );

        cmd1.undo(&mut shapes);
        assert_eq!(shapes.len(), 1);
        assert_eq!(
            format!("{:?}", shapes[&cmd2.name]),
            format!("{:?}", cmd2.shape),
        );

        cmd2.undo(&mut shapes);
        assert_eq!(shapes.len(), 0);
    }
}

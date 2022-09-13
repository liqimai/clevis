use super::{Command, Error, Executor};
use crate::shape::{Shape, Shapes};
use std::fmt;

pub struct DrawShape<ShapeType>
where
    ShapeType: Shape,
{
    name: String,
    shape: ShapeType,
}

impl<ShapeType> DrawShape<ShapeType>
where
    ShapeType: Shape,
{
    pub fn new(name: String, shape: ShapeType) -> Self {
        Self {
            name,
            shape,
        }
    }
}

impl<ShapeType> Clone for DrawShape<ShapeType>
where
    ShapeType: Shape + Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.name.clone(), self.shape.clone())
    }
}

impl<ShapeType> Default for DrawShape<ShapeType>
where
    ShapeType: Shape + Default,
{
    fn default() -> Self {
        Self {
            name: std::any::type_name::<ShapeType>().into(),
            shape: ShapeType::default(),
        }
    }
}

impl<ShapeType> fmt::Display for DrawShape<ShapeType>
where
    ShapeType: Shape,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.name, self.shape)
    }
}

impl<ShapeType> Command for DrawShape<ShapeType>
where
    ShapeType: 'static + Shape + Clone,
{
    fn execute(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        shapes.insert(self.name.clone(), Box::new((self.shape).clone()));
        Ok(())
    }
    fn undo(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        shapes.remove(&self.name);
        Ok(())
    }
    fn after_execute(
        &mut self,
        _executor: &mut Executor,
        _shapes: &mut Shapes,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(true)
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

        cmd1.execute(&mut shapes).unwrap();
        assert_eq!(
            format!("{:?}", shapes[&cmd1.name]),
            format!("{:?}", cmd1.shape),
        );

        cmd1.undo(&mut shapes).unwrap();
        assert!(shapes.get(&cmd1.name).is_none());
        assert_eq!(shapes.len(), 0);

        cmd1.execute(&mut shapes).unwrap();
        cmd2.execute(&mut shapes).unwrap();
        assert_eq!(
            format!("{:?}", shapes[&cmd1.name]),
            format!("{:?}", cmd1.shape),
        );
        assert_eq!(
            format!("{:?}", shapes[&cmd2.name]),
            format!("{:?}", cmd2.shape),
        );

        cmd1.undo(&mut shapes).unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(
            format!("{:?}", shapes[&cmd2.name]),
            format!("{:?}", cmd2.shape),
        );

        cmd2.undo(&mut shapes).unwrap();
        assert_eq!(shapes.len(), 0);
    }
}

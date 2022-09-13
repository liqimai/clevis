use super::{Command, Error, Executor};
use crate::shape::DataType;
use crate::shape::Shapes;
use std::fmt;

#[derive(Clone)]
pub struct MoveBy {
    name: String,
    dx: DataType,
    dy: DataType,
}
impl MoveBy {
    pub fn new(name: String, dx: DataType, dy: DataType) -> Self {
        Self { name, dx, dy }
    }
}
impl fmt::Display for MoveBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} {} {}", self.name, self.dx, self.dy)
    }
}
impl Command for MoveBy {
    fn execute(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        let err_msg = format!("Shape {:?} is not found.", self.name);
        let shape = shapes.get_mut(&self.name).ok_or(err_msg)?;
        shape.move_by(self.dx, self.dy);

        Ok(())
    }
    fn undo(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        let err_msg = format!("Shape {:?} is not found.", self.name);
        let shape = shapes.get_mut(&self.name).ok_or(err_msg)?;
        shape.move_by(-self.dx, -self.dy);

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
pub mod tests {
    use super::*;
    use crate::render::tests::get_writer_render_result;
    use crate::shape::*;

    #[test]
    fn test_move_by() {
        let mut cmd = MoveBy {
            name: "shape_name".into(),
            dx: 3,
            dy: 5,
        };

        let mut shapes = Shapes::from([(
            "shape_name".to_string(),
            Box::new(Point { x: 2, y: 4 }) as Box<dyn Shape>,
        )]);

        cmd.execute(&mut shapes).unwrap();
        let result = get_writer_render_result(&shapes);
        assert_eq!(result, "\nshape_name Point { x: 5, y: 9 }\n");

        cmd.undo(&mut shapes).unwrap();
        let result = get_writer_render_result(&shapes);
        assert_eq!(result, "\nshape_name Point { x: 2, y: 4 }\n");

        cmd.name = "aaa".into();
        assert_eq!(
            format!("{}", cmd.undo(&mut shapes).unwrap_err()),
            r#"Shape "aaa" is not found."#
        );
    }
}

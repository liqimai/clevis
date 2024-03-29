use super::{Command, Error, Executor};
use crate::shape::{Shape, Shapes};
use std::fmt;
use std::mem;

#[derive(Default)]
pub struct Delete {
    name: String,
    deleted: Option<Box<dyn Shape>>,
}

impl Delete {
    pub fn new(name: String) -> Self {
        Self {
            name,
            deleted: None,
        }
    }
}

impl fmt::Display for Delete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Delete {:?} with deleted {:?}", self.name, self.deleted)
    }
}

impl Command for Delete {
    fn execute(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        let err_msg = format!("Shape {:?} is not found.", self.name);
        let shape = shapes.remove(&self.name).ok_or(err_msg)?;
        self.deleted = Some(shape);
        // shapes.insert(self.name.clone(), Box::new((self.shape).clone()));
        Ok(())
    }
    fn undo(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        let shape = mem::replace(&mut self.deleted, None);
        let shape = shape.ok_or("No shape was deleted.")?;
        shapes.insert(self.name.clone(), shape);
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
    fn test_delete() {
        let mut del1 = Delete::new("name".to_string());
        let shapes = &mut Shapes::from([(
            "name".to_string(),
            Box::new(Point::default()) as Box<dyn Shape>,
        )]);
        assert_eq!(del1.to_string(), r#"Delete "name" with deleted None"#);

        del1.execute(shapes).unwrap();
        assert_eq!(get_writer_render_result(shapes), "\n");
        assert!(del1.deleted.is_some());
        assert_eq!(
            del1.to_string(),
            r#"Delete "name" with deleted Some(Point { x: 0, y: 0 })"#
        );

        del1.undo(shapes).unwrap();
        assert_eq!(
            get_writer_render_result(shapes),
            "\nname Point { x: 0, y: 0 }\n"
        );
        assert!(del1.deleted.is_none());
        assert_eq!(del1.to_string(), r#"Delete "name" with deleted None"#);
    }
}

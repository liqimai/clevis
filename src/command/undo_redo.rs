use super::{Command, Error, Executor};
use crate::shape::Shapes;
use std::fmt;

pub enum Control {
    Redo,
    Undo,
}
impl fmt::Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Redo => "redo",
                Self::Undo => "undo",
            }
        )
    }
}
impl Command for Control {
    fn execute(&mut self, _shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn undo(&mut self, _shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }
    fn after_execute(
        &mut self,
        executor: &mut Executor,
        shapes: &mut Shapes,
    ) -> Result<bool, Box<dyn Error>> {
        match self {
            Control::Redo => executor.redo(shapes)?,
            Control::Undo => executor.undo(shapes)?,
        };

        Ok(false)
    }
}

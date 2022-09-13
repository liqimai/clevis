use crate::command::Command;
use crate::shape::Shapes;
use std::error::Error;
use std::fmt;

#[derive(Default)]
pub struct Executor {
    pub executed: Vec<Box<dyn Command>>,
    pub undone: Vec<Box<dyn Command>>,
}

#[derive(Debug)]
pub enum ExecutionError {
    NoCmdToUndo,
    NoCmdToRedo,
}
impl Error for ExecutionError {}
impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExecutionError::NoCmdToUndo => "No command to undo.",
                ExecutionError::NoCmdToRedo => "No command to redo.",
            }
        )
    }
}

impl Executor {
    pub fn execute(
        &mut self,
        mut cmd: Box<dyn Command>,
        shapes: &mut Shapes,
    ) -> Result<(), Box<dyn Error>> {
        cmd.execute(shapes)?;
        if cmd.after_execute(self, shapes)? {
            self.undone.clear();
            self.executed.push(cmd);
        }
        Ok(())
    }
    pub fn undo(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.executed.pop().ok_or(ExecutionError::NoCmdToUndo)?;
        cmd.undo(shapes)?;
        self.undone.push(cmd);

        Ok(())
    }

    pub fn redo(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.undone.pop().ok_or(ExecutionError::NoCmdToRedo)?;
        cmd.execute(shapes)?;
        self.executed.push(cmd);

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::command::Control;
    use crate::command::*;
    use crate::render::tests::get_writer_render_result;
    use crate::shape::*;

    #[test]
    fn test_redo_undo() {
        let mut shapes = Shapes::new();
        let mut executor = Executor::default();

        let undo = Box::new(Control::Undo);
        assert_eq!(
            executor.execute(undo, &mut shapes).unwrap_err().to_string(),
            "No command to undo."
        );
        assert_eq!(get_writer_render_result(&shapes), "\n");

        let cmd1 = Box::new(DrawShape::new("p1".to_string(), Point::default()));
        executor.execute(cmd1, &mut shapes).unwrap();
        assert_eq!(
            get_writer_render_result(&shapes),
            "\np1 Point { x: 0, y: 0 }\n"
        );

        let undo = Box::new(Control::Undo);
        executor.execute(undo, &mut shapes).unwrap();
        assert_eq!(get_writer_render_result(&shapes), "\n");

        let redo = Box::new(Control::Redo);
        executor.execute(redo, &mut shapes).unwrap();
        assert_eq!(
            get_writer_render_result(&shapes),
            "\np1 Point { x: 0, y: 0 }\n"
        );

        let undo = Box::new(Control::Undo);
        executor.execute(undo, &mut shapes).unwrap();
        assert_eq!(get_writer_render_result(&shapes), "\n");

        let cmd1 = Box::new(DrawShape::new("p1".to_string(), Point::default()));
        executor.execute(cmd1, &mut shapes).unwrap();
        assert_eq!(
            get_writer_render_result(&shapes),
            "\np1 Point { x: 0, y: 0 }\n"
        );

        let redo = Box::new(Control::Redo);
        assert_eq!(
            executor.execute(redo, &mut shapes).unwrap_err().to_string(),
            "No command to redo."
        );
        assert_eq!(
            get_writer_render_result(&shapes),
            "\np1 Point { x: 0, y: 0 }\n"
        );
    }
}

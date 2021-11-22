use super::command::Command;
use super::shape::Shapes;
use std::error::Error;
use std::fmt;

pub struct Executor<RenderType> {
    pub executed: Vec<Box<dyn Command<RenderType>>>,
    pub undone: Vec<Box<dyn Command<RenderType>>>,
}

impl<RenderType> Default for Executor<RenderType> {
    fn default() -> Self {
        Self {
            executed: Vec::default(),
            undone: Vec::default(),
        }
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    NoCmdToUndo,
    NoCmdToRedo,
}
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
impl Error for ExecutionError {}

impl<RenderType> Executor<RenderType> {
    pub fn execute(
        &mut self,
        cmd: Box<dyn Command<RenderType>>,
        shapes: &mut Shapes<RenderType>,
    ) -> Result<(), Box<dyn Error>> {
        self.executed.push(cmd);
        self.executed.last_mut().unwrap().execute(shapes)?;
        // cmd.execute(&mut self.shapes);
        Ok(())
    }
    pub fn undo(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>> {
        let cmd = self.executed.pop().ok_or(ExecutionError::NoCmdToUndo)?;
        cmd.undo(shapes)?;
        self.undone.push(cmd);

        Ok(())
    }

    pub fn redo(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>> {
        let cmd = self.undone.pop().ok_or(ExecutionError::NoCmdToRedo)?;
        cmd.execute(shapes)?;
        self.executed.push(cmd);

        Ok(())
    }
}

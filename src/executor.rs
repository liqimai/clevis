use super::command::Command;
use super::shape::Shapes;

#[derive(Debug)]
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

pub enum ExecutionError {
    NoCmdToUndo,
    NoCmdToRedo,
}

impl<RenderType> Executor<RenderType> {
    pub fn execute(&mut self, cmd: Box<dyn Command<RenderType>>, shapes: &mut Shapes<RenderType>) {
        self.executed.push(cmd);
        self.executed.last_mut().unwrap().execute(shapes);
        // cmd.execute(&mut self.shapes);
    }
    pub fn undo(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), ExecutionError> {
        let mut cmd = self.executed.pop().ok_or(ExecutionError::NoCmdToUndo)?;
        cmd.undo(shapes);
        self.undone.push(cmd);

        Ok(())
    }

    pub fn redo(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), ExecutionError> {
        let mut cmd = self.undone.pop().ok_or(ExecutionError::NoCmdToRedo)?;
        cmd.execute(shapes);
        self.executed.push(cmd);

        Ok(())
    }
}

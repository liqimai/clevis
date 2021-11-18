use super::command::Command;
use super::shape::Shapes;

#[derive(Default, Debug)]
pub struct Executor {
    pub executed: Vec<Box<dyn Command>>,
    pub undone: Vec<Box<dyn Command>>,
}

pub enum ExecutionError {
    NoCmdToUndo,
    NoCmdToRedo,
}

impl Executor {
    pub fn execute(&mut self, cmd: Box<dyn Command>, shapes: &mut Shapes) {
        self.executed.push(cmd);
        self.executed.last_mut().unwrap().execute(shapes);
        // cmd.execute(&mut self.shapes);
    }
    pub fn undo(&mut self, shapes: &mut Shapes) -> Result<(), ExecutionError> {
        let mut cmd = self.executed.pop().ok_or(ExecutionError::NoCmdToUndo)?;
        cmd.undo(shapes);
        self.undone.push(cmd);

        Ok(())
    }

    pub fn redo(&mut self, shapes: &mut Shapes) -> Result<(), ExecutionError> {
        let mut cmd = self.undone.pop().ok_or(ExecutionError::NoCmdToRedo)?;
        cmd.execute(shapes);
        self.executed.push(cmd);

        Ok(())
    }
}

use crate::executor::Executor;
use crate::shape::Shapes;
use std::error::Error;
use std::fmt;

pub trait Command: fmt::Display {
    fn execute(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>>;
    fn undo(&mut self, shapes: &mut Shapes) -> Result<(), Box<dyn Error>>;
    fn after_execute(
        &mut self,
        executor: &mut Executor,
        shapes: &mut Shapes,
    ) -> Result<bool, Box<dyn Error>>;
}

mod draw_shape;
pub use draw_shape::DrawShape;

mod delete;
pub use delete::Delete;

mod move_by;
pub use move_by::MoveBy;

mod undo_redo;
pub use undo_redo::Control;

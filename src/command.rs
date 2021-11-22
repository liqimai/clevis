use crate::shape::Shapes;
use std::error::Error;
use std::fmt;

pub trait Command<RenderType>: fmt::Display {
    fn execute(&self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>>;
    fn undo(&self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>>;
}

mod draw_shape;
pub use draw_shape::DrawShape;

mod move_by;
pub use move_by::MoveBy;

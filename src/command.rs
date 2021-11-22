use crate::shape::Shapes;
use std::fmt;

pub trait Command<RenderType>: fmt::Display {
    fn execute(&mut self, shapes: &mut Shapes<RenderType>);
    fn undo(&mut self, shapes: &mut Shapes<RenderType>);
}

mod draw_shape;
pub use draw_shape::DrawShape;

use crate::executor::Executor;
use crate::shape::Shapes;
use std::error::Error;
use std::fmt;

pub trait Command<RenderType>: fmt::Display {
    fn execute(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>>;
    fn undo(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>>;
    fn after_execute(
        &mut self,
        executor: &mut Executor<RenderType>,
        shapes: &mut Shapes<RenderType>,
    ) -> Result<bool, Box<dyn Error>>;
}

// trait ShapeCommand<RenderType>: Command<RenderType> {
//     fn execute(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>>;
//     fn undo(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>>;
// }

// impl<ShapeCommandType, RenderType> Command<RenderType> for ShapeCommandType
// where
//     ShapeCommandType: 'static + ShapeCommand<RenderType>,
// {
//     fn execute(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>> {
//         ShapeCommand::<RenderType>::execute(self, shapes)
//     }
//     fn undo(&mut self, shapes: &mut Shapes<RenderType>) -> Result<(), Box<dyn Error>> {
//         ShapeCommand::<RenderType>::undo(self, shapes)
//     }
//     fn after_execute(
//         &mut self,
//         executor: &mut Executor<RenderType>,
//         _shapes: &mut Shapes<RenderType>,
//     ) -> Result<bool, Box<dyn Error>> {
//         Ok(true)
//     }
// }

mod draw_shape;
pub use draw_shape::DrawShape;

mod delete;
pub use delete::Delete;

mod move_by;
pub use move_by::MoveBy;

mod undo_redo;
pub use undo_redo::Control;

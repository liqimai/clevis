use crate::command::Command;

pub trait Commander<RenderType>: IntoIterator<Item = Box<dyn Command<RenderType>>> {}

impl<T, RenderType> Commander<RenderType> for T where
    T: IntoIterator<Item = Box<dyn Command<RenderType>>>
{
}

mod cli_commander;
pub use cli_commander::CliCommander;

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::command::DrawShape;
    use crate::executor::Executor;
    use crate::render::DummyRenderer;
    use crate::shape::*;
    use std::fmt::Debug;

    pub fn get_cmd_vec<RenderType: 'static + Debug>() -> Vec<Box<dyn Command<RenderType>>>
    where
        Point: Shape<RenderType>,
        Rectangle: Shape<RenderType>,
        Line: Shape<RenderType>,
        Circle: Shape<RenderType>,
        Square: Shape<RenderType>,
    {
        vec![
            Box::new(DrawShape::<RenderType, Point>::default()),
            Box::new(DrawShape::<RenderType, Circle>::default()),
            Box::new(DrawShape::<RenderType, Line>::default()),
            Box::new(DrawShape::<RenderType, Rectangle>::default()),
            Box::new(DrawShape::<RenderType, Square>::default()),
        ]
    }

    #[test]
    fn test_cmd_vec() {
        let commands = get_cmd_vec();
        let mut exe = Executor::<DummyRenderer>::default();
        let mut shapes = Shapes::default();
        for cmd in commands {
            exe.execute(cmd, &mut shapes).unwrap();
        }
        assert_eq!(shapes.len(), 5);
    }
}

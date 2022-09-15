use crate::command::Command;

pub trait Commander: IntoIterator<Item = Box<dyn Command>> {}

impl<T> Commander for T where T: IntoIterator<Item = Box<dyn Command>> {}

pub mod cli_commander;
pub use cli_commander::CliCommander;

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::command::DrawShape;
    use crate::executor::Executor;
    use crate::shape::*;

    pub fn get_cmd_vec() -> Vec<Box<dyn Command>>
    where
        Point: Shape,
        Rectangle: Shape,
        Line: Shape,
        Circle: Shape,
        Square: Shape,
    {
        vec![
            Box::new(DrawShape::<Point>::default()),
            Box::new(DrawShape::<Circle>::default()),
            Box::new(DrawShape::<Line>::default()),
            Box::new(DrawShape::<Rectangle>::default()),
            Box::new(DrawShape::<Square>::default()),
        ]
    }

    #[test]
    fn test_cmd_vec() {
        let commands = get_cmd_vec();
        let mut exe = Executor::default();
        let mut shapes = Shapes::default();
        for cmd in commands {
            exe.execute(cmd, &mut shapes).unwrap();
        }
        assert_eq!(shapes.len(), 5);
    }
}

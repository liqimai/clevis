use crate::command::Command;

pub trait Commander: IntoIterator<Item = Box<dyn Command>> {}

impl<T: IntoIterator<Item = Box<dyn Command>>> Commander for T {}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::command::DrawShape;
    use crate::executor::Executor;
    use crate::shape::*;

    pub fn get_cmd_vec() -> Vec<Box<dyn Command>> {
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
        let cmds = get_cmd_vec();
        let mut exe = Executor::default();
        let mut shapes = Shapes::default();
        for cmd in cmds {
            exe.execute(cmd, &mut shapes);
        }
        assert_eq!(shapes.len(), 5);
    }
}

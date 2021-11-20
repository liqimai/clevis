use crate::shape::{Shape, Shapes};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::Debug;

pub trait Renderer<RenderType> {
    fn init_frame(&mut self) -> Result<(), Box<dyn Error>>;
    fn finish_frame(&mut self) -> Result<(), Box<dyn Error>>;
    fn render(&mut self, name: &str, shape: &dyn Shape<RenderType>) -> Result<(), Box<dyn Error>>;
    fn render_shapes(&mut self, shapes: &Shapes<RenderType>) -> Result<(), Box<dyn Error>> {
        self.init_frame()?;
        for (name, shape) in shapes {
            self.render(name, shape.borrow())?;
        }
        self.finish_frame()?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DummyRenderer;

impl<T> Renderer<T> for DummyRenderer {
    fn render(&mut self, _name: &str, _shape: &dyn Shape<T>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl<S: Debug> Shape<DummyRenderer> for S {
    fn draw(&self, _render: &mut DummyRenderer) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub mod file_renderer;
pub use file_renderer::FileRenderer;

// pub mod html_renderer;
// pub use html_renderer::HtmlRenderer;

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use crate::shape::tests::get_shapes;
    pub use std::collections::HashMap;
    use std::fmt::Debug;
    use std::io::Write;

    impl<S, W> Shape<W> for S
    where
        S: Debug,
        W: Write,
    {
        fn draw(&self, render: &mut W) -> Result<(), Box<dyn Error>> {
            render.write_all(format!("{:?}", &self).as_bytes())?;

            Ok(())
        }
    }

    impl<W: Write> Renderer<W> for W {
        fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
            self.write_all(b"\n")?;
            Ok(())
        }
        fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
            self.flush()?;
            Ok(())
        }
        fn render(&mut self, name: &str, shape: &dyn Shape<W>) -> Result<(), Box<dyn Error>> {
            self.write_all(name.as_bytes())?;
            self.write_all(b" ")?;
            shape.draw(self)?;
            self.write_all(b"\n")?;
            Ok(())
        }
    }

    #[test]
    fn test_writer_renderer() {
        let shapes = get_shapes();
        // std::io::stderr().render_shapes(&shapes);

        let mut buff = Vec::<u8>::new();
        buff.render_shapes(&shapes).unwrap();
        let render_result = String::from_utf8(buff).unwrap(); // buffer to string
        println!("{}", &render_result);
        check_string_render(&get_answers(), &render_result);
    }

    pub fn get_answers() -> HashMap<String, String> {
        use crate::shape::*;

        HashMap::from([
            (
                std::any::type_name::<Line>().into(),
                "Line(Point { x: 0, y: 0 }, Point { x: 0, y: 0 })".into(),
            ),
            (
                std::any::type_name::<Square>().into(),
                "Square { corner: Point { x: 0, y: 0 }, side: 0 }".into(),
            ),
            (
                std::any::type_name::<Circle>().into(),
                "Circle { center: Point { x: 0, y: 0 }, radius: 0 }".into(),
            ),
            (
                std::any::type_name::<Point>().into(),
                "Point { x: 0, y: 0 }".into(),
            ),
            (
                std::any::type_name::<Rectangle>().into(),
                "Rectangle(Point { x: 0, y: 0 }, Point { x: 0, y: 0 })".into(),
            ),
        ])
    }

    pub fn check_string_render(answer: &HashMap<String, String>, render_result: &str) {
        let mut cnt = 0;
        for line in render_result.split('\n') {
            if line.is_empty() {
                continue;
            }
            let v: Vec<&str> = line.splitn(2, " ").collect();
            match v[..] {
                [name, debug_info] => {
                    assert_eq!(answer[name], debug_info);
                }
                _ => panic!("each line should be in '<name> <shape debug>' format."),
            }
            cnt += 1;
        }
        assert_eq!(answer.len(), cnt);
    }
}

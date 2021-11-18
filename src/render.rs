use crate::shape::{Shape, Shapes};
use std::borrow::Borrow;
use std::error::Error;

pub trait Renderer {
    fn init_frame(&mut self) -> Result<(), Box<dyn Error>>;
    fn finish_frame(&mut self) -> Result<(), Box<dyn Error>>;
    fn render(&mut self, name: &str, shape: &dyn Shape) -> Result<(), Box<dyn Error>>;
    fn render_shapes(&mut self, shapes: &Shapes) -> Result<(), Box<dyn Error>> {
        self.init_frame()?;
        for (name, shape) in shapes {
            self.render(name, shape.borrow())?;
        }
        self.finish_frame()?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::shape::tests::get_shapes;
    use std::io::Write;

    #[derive(Debug)]
    pub struct DummyRenderer;

    impl Renderer for DummyRenderer {
        fn render(&mut self, _name: &str, _shape: &dyn Shape) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
        fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
    }

    impl<T: Write> Renderer for T {
        fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
            self.write_all(b"\n")?;
            Ok(())
        }
        fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
            self.flush()?;
            Ok(())
        }
        fn render(&mut self, name: &str, shape: &dyn Shape) -> Result<(), Box<dyn Error>> {
            let s = format!("{} {:?}\n", name, shape);
            self.write_all(s.as_bytes())?;
            Ok(())
        }
    }

    #[test]
    fn test_writer_renderer() {
        let shapes = get_shapes();
        // std::io::stderr().render_shapes(&shapes);

        let mut buff = Vec::<u8>::new();
        buff.render_shapes(&shapes).unwrap();
        let string_render = String::from_utf8(buff).unwrap(); // buffer to string
        check_string_render(&shapes, &string_render);
    }

    fn check_string_render(shapes: &Shapes, string_render: &str) {
        let mut cnt = 0;
        for line in string_render.split('\n') {
            if line.is_empty() {
                continue;
            }
            let v: Vec<&str> = line.splitn(2, " ").collect();
            match v[..] {
                [name, debug_info] => assert_eq!(&format!("{:?}", shapes[name]), debug_info),
                _ => panic!("each line should be in '<name> <shape debug>' format."),
            }
            cnt += 1;
        }
        assert_eq!(shapes.len(), cnt);
    }
}

use crate::shape::{Shape, Shapes};
use std::borrow::Borrow;

pub trait Renderer {
    fn init_frame(&mut self);
    fn finish_frame(&mut self);
    fn render(&mut self, name: &str, shape: &dyn Shape);
    fn render_shapes(&mut self, shapes: &Shapes) {
        self.init_frame();
        for (name, shape) in shapes {
            self.render(name, shape.borrow());
        }
        self.finish_frame();
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
        fn render(&mut self, _name: &str, _shape: &dyn Shape) {}
        fn init_frame(&mut self) {}
        fn finish_frame(&mut self) {}
    }

    impl<T: Write> Renderer for T {
        fn init_frame(&mut self) {
            self.write_all(b"\n").unwrap();
        }
        fn finish_frame(&mut self) {
            self.flush().unwrap();
        }
        fn render(&mut self, name: &str, shape: &dyn Shape) {
            let s = format!("{} {:?}\n", name, shape);
            self.write_all(s.as_bytes()).unwrap();
        }
    }

    #[test]
    fn test_writer_renderer() {
        let shapes = get_shapes();
        // std::io::stderr().render_shapes(&shapes);

        let mut buff = Vec::<u8>::new();
        buff.render_shapes(&shapes);
        let s = String::from_utf8(buff).unwrap(); // buffer to string
        for line in s.split('\n') {
            if line.is_empty() {
                continue;
            }
            let v: Vec<&str> = line.splitn(2, " ").collect();
            match v[..] {
                [name, debug_info] => assert_eq!(&format!("{:?}", shapes[name]), debug_info),
                _ => panic!(),
            }
        }
    }
}

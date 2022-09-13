use crate::shape::{Shape, Shapes};
use std::borrow::Borrow;
use std::error::Error;
use crate::shape::*;

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

    #[allow(unused_variables)]
    fn draw_point(&mut self, point: &Point) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn draw_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
    
    #[allow(unused_variables)]
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
    
    #[allow(unused_variables)]
    fn draw_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
    
    #[allow(unused_variables)]
    fn draw_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>> {
        unimplemented!();
    }
}

mod dummy_renderer;
pub use dummy_renderer::DummyRenderer;

mod file_renderer;
pub use file_renderer::FileRenderer;

mod html_renderer;
pub use html_renderer::HtmlRenderer;

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use crate::shape::tests::get_shapes;
    pub use std::collections::HashMap;
    use std::io::Write;
    
    fn draw_shape_to_writer<W: Write>(writer: &mut W, shape: &dyn Shape) -> Result<(), Box<dyn Error>> {
        writer.write_all(format!("{:?}", &shape).as_bytes())?;
        Ok(())
    }

    impl<W: Write> Renderer for W {
        fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
            self.write_all(b"\n")?;
            Ok(())
        }
        fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
            self.flush()?;
            Ok(())
        }
        fn render(&mut self, name: &str, shape: &dyn Shape) -> Result<(), Box<dyn Error>> {
            self.write_all(name.as_bytes())?;
            self.write_all(b" ")?;
            shape.draw_on(self)?;
            self.write_all(b"\n")?;
            Ok(())
        }

        fn draw_point(&mut self, point: &Point) -> Result<(), Box<dyn Error>> {
            draw_shape_to_writer(self, point)
        }

        fn draw_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>> {
            draw_shape_to_writer(self, line)
        }
        
        fn draw_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>> {
            draw_shape_to_writer(self, rectangle)
        }
        
        fn draw_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>> {
            draw_shape_to_writer(self, circle)
        }
        
        fn draw_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>> {
            draw_shape_to_writer(self, square)
        }
    }

    pub fn get_writer_render_result(shapes: &Shapes) -> String {
        let mut buff = Vec::<u8>::new();
        buff.render_shapes(&shapes).unwrap();
        String::from_utf8(buff).unwrap()
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
                "Rectangle { corner: Point { x: 0, y: 0 }, w: 0, h: 0 }".into(),
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

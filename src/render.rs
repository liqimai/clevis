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

use std::fs::File;
use std::io;
#[derive(Debug)]
pub struct FileRenderer {
    file: File,
    filename: String,
}

impl FileRenderer {
    pub fn new(filename: &str) -> Result<FileRenderer, io::Error> {
        Ok(FileRenderer {
            filename: filename.to_string(),
            file: File::create(filename)?,
        })
    }
}

use std::io::prelude::*;
impl Renderer for FileRenderer {
    fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.file = File::create(&self.filename)?;
        self.file.rewind()?;

        Ok(())
    }
    fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
        let len = self.file.stream_position()?;
        self.file.set_len(len)?;
        self.file.sync_all()?;

        Ok(())
    }
    fn render(&mut self, name: &str, shape: &dyn Shape) -> Result<(), Box<dyn Error>> {
        let s = format!("{} {:?}\n", name, shape);
        self.file.write_all(s.as_bytes())?;

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

    #[test]
    fn test_file_renderer() {
        use crate::shape::*;
        use std::collections::HashMap;

        let screen_file_name = "crate::render::tests::test_file_renderer.screen";
        let mut render = FileRenderer::new(screen_file_name).unwrap();
        let shape_list = [
            (
                "PointName".to_string(),
                Box::new(Point::default()) as Box<dyn Shape>,
            ),
            (
                "RectangleName".to_string(),
                Box::new(Rectangle::default()) as Box<dyn Shape>,
            ),
            (
                "LineName".to_string(),
                Box::new(Line::default()) as Box<dyn Shape>,
            ),
            (
                "CircleName".to_string(),
                Box::new(Circle::default()) as Box<dyn Shape>,
            ),
            (
                "SquareName".to_string(),
                Box::new(Square::default()) as Box<dyn Shape>,
            ),
        ];
        let mut shapes: Shapes = HashMap::new();

        let mut answer = "".to_string();
        for (n, s) in shape_list {
            answer += &format!("{} {:?}\n", &n, &s);

            shapes.insert(n, s);
            render.render_shapes(&shapes).unwrap();

            let mut string_render = String::new();
            File::open(screen_file_name)
                .unwrap()
                .read_to_string(&mut string_render)
                .unwrap();
            check_string_render(&shapes, &string_render);

            // use std::{thread, time};
            // thread::sleep(time::Duration::from_secs(1));
        }
        std::fs::remove_file(screen_file_name).unwrap();
    }
}

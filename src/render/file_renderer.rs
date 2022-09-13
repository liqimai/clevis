use super::{Error, Renderer, Shape};
use crate::shape::*;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::prelude::*;

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

    pub fn draw_shape(&mut self, shape: &dyn Shape) -> Result<(), Box<dyn Error>> {
        self.file.write_all(format!("{:?}", &shape).as_bytes())?;
        Ok(())
    }
}

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
    fn render(
        &mut self,
        name: &str,
        shape: &dyn Shape,
    ) -> Result<(), Box<dyn Error>> {
        self.file.write_all(name.as_bytes())?;
        self.file.write_all(b" ")?;
        shape.draw_on(self)?;
        self.file.write_all(b"\n")?;

        Ok(())
    }

    fn draw_point(&mut self, point: &Point) -> Result<(), Box<dyn Error>> {
        self.draw_shape(point)
    }

    fn draw_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>> {
        self.draw_shape(line)
    }
    
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>> {
        self.draw_shape(rectangle)
    }
    
    fn draw_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>> {
        self.draw_shape(circle)
    }
    
    fn draw_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>> {
        self.draw_shape(square)
    }
}

impl Drop for FileRenderer {
    fn drop(&mut self) {
        std::fs::remove_file(&self.filename).unwrap();
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::tests::{check_string_render, get_answers, get_shapes};
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_file_renderer() {
        use crate::shape::*;
        use std::fs::File;

        let screen_file_name = "crate::render::file_renderer::tests::test_file_renderer.screen";
        let full_answer = get_answers();
        let full_shapes = get_shapes();

        let mut render = FileRenderer::new(screen_file_name).unwrap();
        let mut answer = HashMap::<String, String>::new();
        let mut shapes = Shapes::new();
        for (n, s) in full_shapes {
            shapes.insert(n.clone(), s);
            render.render_shapes(&shapes).unwrap();

            answer.insert(n.clone(), full_answer.get(&n).unwrap().to_string());

            // read the file and check
            let mut string_render = String::new();
            File::open(screen_file_name)
                .unwrap()
                .read_to_string(&mut string_render)
                .unwrap();
            check_string_render(&answer, &string_render);

            // use std::{thread, time};
            // thread::sleep(time::Duration::from_secs(1));
        }
    }
}

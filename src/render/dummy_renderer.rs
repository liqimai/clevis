use super::*;
use std::fmt::Debug;

#[derive(Debug, Default)]
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

    #[allow(unused_variables)]
    fn draw_point(&mut self, point: &Point) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

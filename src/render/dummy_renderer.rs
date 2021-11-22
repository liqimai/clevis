use super::*;
use std::fmt::Debug;

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

impl<S: Debug> Renderable<DummyRenderer> for S {
    fn draw(&self, _render: &mut DummyRenderer) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

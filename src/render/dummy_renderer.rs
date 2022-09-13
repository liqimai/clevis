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
}

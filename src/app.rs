use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;

use crate::command::Command;
use crate::commander::Commander;
use crate::executor::Executor;
use crate::log::Logger;
use crate::render::Renderer;
use crate::shape::Shapes;

#[derive(Debug)]
pub struct App<LoggerType, RendererType>
where
    LoggerType: Logger,
    RendererType: Renderer,
{
    // store shapes
    shapes: RefCell<Shapes>,

    // draw shapes in its own way
    renderer: RendererType,

    // a list of logger
    logger: LoggerType,

    // execute command
    executor: Executor,
}

impl<LoggerType, RendererType> App<LoggerType, RendererType>
where
    LoggerType: Logger,
    RendererType: Renderer,
{
    pub fn new(logger: LoggerType, renderer: RendererType) -> Self {
        App {
            shapes: RefCell::new(Shapes::default()),
            executor: Executor::default(),
            renderer,
            logger,
        }
    }
    pub fn execute(&mut self, cmd: Box<dyn Command>) {
        self.executor
            .execute(cmd, self.shapes.borrow_mut().borrow_mut());
    }
    pub fn render_shapes(&mut self) {
        self.renderer.render_shapes(self.shapes.borrow().borrow());
    }
    pub fn run<CommanderType: Commander>(&mut self, commander: CommanderType) {
        for cmd in commander {
            self.logger.log(&cmd.to_string());
            self.execute(cmd);
            self.render_shapes();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::commander::tests::get_cmd_vec;
    use crate::log::tests::DummyLogger;
    use crate::render::tests::DummyRenderer;

    pub fn get_test_app() -> App<DummyLogger, DummyRenderer> {
        App::new(DummyLogger, DummyRenderer)
    }

    #[test]
    fn test_mock_app() {
        let commander = get_cmd_vec();
        let mut app = get_test_app();
        app.run(commander);
        dbg!(app);
    }
}

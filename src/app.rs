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
pub struct App<LoggerType, RenderType>
where
    LoggerType: Logger,
    RenderType: Renderer<RenderType>,
{
    // store shapes
    shapes: RefCell<Shapes<RenderType>>,

    // draw shapes in its own way
    renderer: RenderType,

    // a list of logger
    logger: LoggerType,

    // execute command
    executor: Executor<RenderType>,
}

impl<LoggerType, RenderType> App<LoggerType, RenderType>
where
    LoggerType: Logger,
    RenderType: Renderer<RenderType>,
{
    pub fn new(logger: LoggerType, renderer: RenderType) -> Self {
        App {
            shapes: RefCell::new(Shapes::default()),
            executor: Executor::default(),
            renderer,
            logger,
        }
    }
    pub fn execute(&mut self, cmd: Box<dyn Command<RenderType>>) {
        self.executor
            .execute(cmd, self.shapes.borrow_mut().borrow_mut());
    }
    pub fn render_shapes(&mut self) {
        match self.renderer.render_shapes(self.shapes.borrow().borrow()) {
            Err(error) => self
                .logger
                .log(&format!("{}:{}:{}", file!(), line!(), error)),
            Ok(()) => (),
        }
    }
    pub fn run<CommanderType: Commander<RenderType>>(&mut self, commander: CommanderType) {
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
    use crate::render::DummyRenderer;

    pub fn get_test_app() -> App<DummyLogger, DummyRenderer> {
        App::new(DummyLogger, DummyRenderer)
    }

    #[test]
    fn test_mock_app() {
        let commander = get_cmd_vec();
        let mut app = get_test_app();
        app.run(commander);
        assert_eq!(app.executor.executed.len(), app.shapes.borrow().len());
    }

    #[test]
    fn test_file_renderer() {
        use crate::render::FileRenderer;
        let screen_file_name = "crate::app::tests::test_file_renderer.screen";
        let mut app = App::new(DummyLogger, FileRenderer::new(screen_file_name).unwrap());
        let commander = get_cmd_vec();
        app.run(commander);
    }
}

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};
use std::{thread, time};

use crate::command::Command;
use crate::commander::Commander;
use crate::executor::Executor;
use crate::log::{DummyLogger, Logger};
use crate::render::{DummyRenderer, Renderer};
use crate::shape::Shapes;

pub struct App {
    // store shapes
    shapes: Arc<Mutex<Shapes>>,

    // draw shapes in its own way
    renderer: Arc<Mutex<dyn Renderer>>,

    // logger
    logger: Box<dyn Logger>,

    // execute command
    executor: Executor,

    fps: u64,
    async_render: bool,
}

impl App {
    pub fn new<LoggerType, RendererType>(
        logger: LoggerType,
        renderer: RendererType,
        async_render: bool,
    ) -> Self
    where
        LoggerType: 'static + Logger,
        RendererType: 'static + Renderer,
    {
        let mut app = App::default();
        app.set_logger(logger);
        app.set_renderer(renderer);
        app.set_async_render(async_render);
        app
    }

    pub fn set_renderer<RendererType>(&mut self, renderer: RendererType)
    where
        RendererType: 'static + Renderer,
    {
        self.renderer = Arc::new(Mutex::new(renderer));
    }
    pub fn set_logger<LoggerType>(&mut self, logger: LoggerType)
    where
        LoggerType: 'static + Logger,
    {
        self.logger = Box::new(logger);
    }
    pub fn set_fps(&mut self, fps: u64) {
        self.fps = fps;
    }
    pub fn set_async_render(&mut self, async_render: bool) {
        self.async_render = async_render;
    }
    pub fn execute(&mut self, cmd: Box<dyn Command>) -> Result<(), Box<dyn Error + '_>> {
        self.executor
            .execute(cmd, self.shapes.lock()?.borrow_mut())?;
        Ok(())
    }
    pub fn render_shapes(&mut self) -> Result<(), Box<dyn Error + '_>> {
        self.renderer
            .lock()?
            .render_shapes(self.shapes.lock()?.borrow())?;

        Ok(())
    }
    fn start_render_thread(&mut self) -> (thread::JoinHandle<()>, mpsc::Sender<()>) {
        let shapes = Arc::clone(&self.shapes);
        let renderer = Arc::clone(&self.renderer);
        let (tx, rx) = mpsc::channel::<()>();
        let fps = self.fps;

        let join_handle = thread::spawn(move || {
            let render = || -> Result<(), Box<dyn Error + '_>> {
                renderer.lock()?.render_shapes(shapes.lock()?.borrow())?;
                thread::sleep(time::Duration::from_millis(1000 / fps));
                Ok(())
            };
            while let Ok(_) = rx.recv() {
                while let Ok(_) = rx.try_recv() {}
                match render() {
                    Err(error) => log::error!("{}", error),
                    Ok(_) => (),
                };
            }
        });

        return (join_handle, tx);
    }
    pub fn run<CommanderType: Commander>(&mut self, commander: CommanderType) {
        if self.async_render {
            let (join_handle, render_signal) = self.start_render_thread();
            for cmd in commander {
                self.logger.log(&cmd.to_string());
                match self.execute(cmd) {
                    Err(error) => log::error!("{}", error),
                    Ok(()) => (),
                }
                render_signal.send(()).unwrap();
            }
            drop(render_signal);
            join_handle.join().unwrap();
        } else {
            for cmd in commander {
                self.logger.log(&cmd.to_string());
                match self.execute(cmd) {
                    Err(error) => log::error!("{}", error),
                    Ok(()) => (),
                }
                match self.render_shapes() {
                    Err(error) => log::error!("{}", error),
                    Ok(()) => (),
                }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            shapes: Arc::new(Mutex::new(Shapes::default())),
            executor: Executor::default(),
            renderer: Arc::new(Mutex::new(DummyRenderer)),
            logger: Box::new(DummyLogger),
            fps: 10,
            async_render: false,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::commander::tests::get_cmd_vec;
    use crate::log::DummyLogger;
    use crate::render::DummyRenderer;

    pub fn get_test_app() -> App {
        App::new(DummyLogger, DummyRenderer, true)
    }

    #[test]
    fn test_mock_app() {
        let commander = get_cmd_vec();
        let mut app = get_test_app();
        app.run(commander);
        assert_eq!(
            app.executor.executed.len(),
            app.shapes.lock().unwrap().borrow().len()
        );
    }

    #[test]
    fn test_file_renderer() {
        use crate::render::FileRenderer;
        let screen_file_name = "crate::app::tests::test_file_renderer.screen";
        let mut app = App::new(
            DummyLogger,
            FileRenderer::new(screen_file_name).unwrap(),
            true,
        );
        let commander = get_cmd_vec();
        app.run(commander);
    }
}

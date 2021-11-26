use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::error::Error;
use std::sync::{Arc, Mutex, mpsc};
use std::{thread, time};

use crate::command::Command;
use crate::commander::Commander;
use crate::executor::Executor;
use crate::log::Logger;
use crate::render::Renderer;
use crate::shape::Shapes;

pub struct App<LoggerType, RenderType>
where
    LoggerType: Logger,
    RenderType: Renderer<RenderType>,
{
    // store shapes
    shapes: Arc<Mutex<Shapes<RenderType>>>,

    // draw shapes in its own way
    renderer: Arc<Mutex<RenderType>>,

    // a list of logger
    logger: LoggerType,

    // execute command
    executor: Executor<RenderType>,

    pub fps: u64,
    pub async_render: bool,
}

impl<LoggerType, RenderType> App<LoggerType, RenderType>
where
    LoggerType: Logger,
    RenderType: Renderer<RenderType> + 'static + Send,
{
    pub fn new(logger: LoggerType, renderer: RenderType) -> Self {
        App {
            shapes: Arc::new(Mutex::new(Shapes::default())),
            executor: Executor::default(),
            renderer: Arc::new(Mutex::new(renderer)),
            logger,
            fps: 10,
            async_render: true,
        }
    }
    pub fn execute(&mut self, cmd: Box<dyn Command<RenderType>>) -> Result<(), Box<dyn Error + '_>> {
        self.executor.execute(cmd, self.shapes.lock()?.borrow_mut())?;
        Ok(())
    }
    pub fn render_shapes(&mut self) -> Result<(), Box<dyn Error + '_>> {
        self.renderer.lock()?.render_shapes(self.shapes.lock()?.borrow())?;

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
                thread::sleep(time::Duration::from_millis(1000/fps));
                Ok(())
            };
            while let Ok(_) = rx.recv() {
                while let Ok(_) = rx.try_recv() {}
                match render() {
                    Err(error) => eprintln!("{}", error),
                    Ok(_) => (),
                };
            }
        });

        return (join_handle, tx)
    }
    pub fn run<CommanderType: Commander<RenderType>>(&mut self, commander: CommanderType) {
        if self.async_render {
            let (join_handle, render_signal) = self.start_render_thread();
            for cmd in commander {
                self.logger.log(&cmd.to_string());
                match self.execute(cmd) {
                    Err(error) => eprintln!("{}", error),
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
                    Err(error) => eprintln!("{}", error),
                    Ok(()) => (),
                }
                match self.render_shapes() {
                    Err(error) => eprintln!("{}", error),
                    Ok(()) => (),
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::commander::tests::get_cmd_vec;
    use crate::log::DummyLogger;
    use crate::render::DummyRenderer;

    pub fn get_test_app() -> App<DummyLogger, DummyRenderer> {
        App::new(DummyLogger, DummyRenderer)
    }

    #[test]
    fn test_mock_app() {
        let commander = get_cmd_vec();
        let mut app = get_test_app();
        app.run(commander);
        assert_eq!(app.executor.executed.len(), app.shapes.lock().unwrap().borrow().len());
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

use clap::Parser;
use super::app::{App};
use super::render::{FileRenderer, HtmlRenderer, DummyRenderer};
use super::log::DummyLogger;
use std::error::Error;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
enum RenderType{
    Html,
    Text,
    Dummy,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
enum LoggerType{
    Dummy,
}

/// A program to draw shapes
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
   /// Render type
   #[clap(long, value_parser, default_value = "html")]
   renderer: RenderType,

   /// Logger type
   #[clap(long, value_parser, default_value = "dummy")]
   logger: LoggerType,

   /// Async render or not
   #[clap(long, action = clap::ArgAction::StoreValue, default_value_t = true)]
   async_render: bool,
}

impl TryInto<App> for Args {
    type Error = Box<dyn Error>;
    fn try_into(self) -> Result<App, Self::Error> {
        let mut app = App::default();
        match self.renderer {
            RenderType::Html => app.set_renderer(HtmlRenderer::new("screen", true)?),
            RenderType::Text => app.set_renderer(FileRenderer::new("screen")?),
            RenderType::Dummy => app.set_renderer(DummyRenderer),
        };
        match self.logger {
            LoggerType::Dummy => app.set_logger(DummyLogger),
        };
        app.set_async_render(self.async_render);
        Ok(app)
    }
}

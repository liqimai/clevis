use std::error::Error;
use std::io;

use clevis::app::App;
use clevis::commander::CliCommander;
use clevis::log::DummyLogger;
use clevis::render::HtmlRenderer;

fn main() -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "macos") {
        println!("I'm running on a macos machine!");
    }

    let mut app = App::new(DummyLogger, HtmlRenderer::new("screen.html", true).unwrap());
    let commander = CliCommander::new(io::BufReader::new(io::stdin()), io::stdout());

    app.run(commander);

    Ok(())
}

use std::error::Error;

use clevis::app::App;
use clevis::commander::cli_commander::HELP_INFO;
use clevis::commander::CliCommander;
use clevis::log::DummyLogger;
use clevis::render::HtmlRenderer;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Available commands:");
    for (cmd, info) in HELP_INFO.iter() {
        println!("    {:15}{}", cmd, info);
    }

    let mut app = App::new(DummyLogger, HtmlRenderer::new("screen", true).unwrap());
    let commander = CliCommander::default();

    app.run(commander);

    Ok(())
}

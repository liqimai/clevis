use std::error::Error;

use clevis::app::App;
use clevis::commander::cli_commander::HELP_INFO;
use clevis::commander::CliCommander;
use clevis::log::DummyLogger;
use clevis::render::HtmlRenderer;
use std::io::Write;

fn init_logger() {
    env_logger::Builder::new()
    .format(|buf, record| {
        let mut level_style = buf.style();
        let level = record.level();

        match level {
            log::Level::Trace => level_style.set_color(env_logger::fmt::Color::White),
            log::Level::Debug => level_style.set_color(env_logger::fmt::Color::Blue),
            log::Level::Info => level_style.set_color(env_logger::fmt::Color::Green),
            log::Level::Warn => level_style.set_color(env_logger::fmt::Color::Yellow),
            log::Level::Error => level_style.set_color(env_logger::fmt::Color::Red).set_bold(true),
        };

        writeln!(
            buf,
            "[{} {} {}:{}] {}",
            level_style.value(level),
            buf.timestamp(),
            record.module_path().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    })
    .init();
}

fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
    println!("Available commands:");
    for (cmd, info) in HELP_INFO.iter() {
        println!("    {:15}{}", cmd, info);
    }

    let mut app = App::new(DummyLogger, HtmlRenderer::new("screen", true).unwrap());
    let commander = CliCommander::default();

    // uncomment following line to forbid asynchronous rendering
    // app.async_render = false;

    app.run(commander);

    // sleep(Duration::from_millis(100));

    Ok(())
}

use std::error::Error;

use clevis::app::App;
use clevis::commander::cli_commander::HELP_INFO;
use clevis::commander::CliCommander;
use clevis::args::Args;
use std::io::Write;
use clap::Parser;

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
    let args = Args::parse();

    init_logger();
    println!("Available commands:");
    for (cmd, info) in HELP_INFO.iter() {
        println!("    {:15}{}", cmd, info);
    }

    let mut app: App = args.try_into()?;

    let commander = CliCommander::default();
    app.run(commander);

    Ok(())
}

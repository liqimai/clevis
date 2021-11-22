pub trait Logger {
    fn log(&mut self, msg: &str);
}

pub struct DummyLogger;

impl Logger for DummyLogger {
    fn log(&mut self, _msg: &str) {}
}

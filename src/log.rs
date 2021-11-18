pub trait Logger {
    fn log(&mut self, msg: &str);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[derive(Debug)]
    pub struct DummyLogger;
    impl Logger for DummyLogger {
        fn log(&mut self, _msg: &str) {}
    }
}

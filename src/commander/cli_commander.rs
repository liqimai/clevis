use super::*;
use crate::command::DrawShape;
use crate::shape::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::{BufRead, Write};
use std::marker::PhantomData;

pub struct CliCommander<Reader, Writer, RenderType>
where
    Reader: BufRead,
    Writer: Write,
{
    // iterator over lines of the reader
    lines: io::Lines<Reader>,
    writer: Writer,
    parse_fn: HashMap<String, ParseFn<RenderType>>,
    phantom: PhantomData<RenderType>,
}

type ParseFn<RenderType> = fn(&str) -> Result<Box<dyn Command<RenderType>>, Box<dyn Error>>;

impl<Reader, Writer, RenderType> CliCommander<Reader, Writer, RenderType>
where
    RenderType: 'static,
    Reader: 'static + BufRead,
    Writer: Write,
    Point: Shape<RenderType>,
{
    pub fn new(reader: Reader, writer: Writer) -> Self {
        let mut this = Self {
            lines: reader.lines(),
            writer,
            parse_fn: HashMap::new(),
            phantom: PhantomData,
        };
        this.register_parse_fn("Point".to_string(), parse_cmd::point::<RenderType>);

        this
    }

    pub fn register_parse_fn(
        &mut self,
        cmd_name: String,
        func: ParseFn<RenderType>,
    ) -> Option<ParseFn<RenderType>> {
        self.parse_fn.insert(cmd_name, func)
    }

    fn parse_line(
        &mut self,
        line: Result<String, io::Error>,
    ) -> Result<Box<dyn Command<RenderType>>, Box<dyn Error>> {
        lazy_static! {
            static ref RE_POINT: Regex = Regex::new(r"^\s*(?P<cmd_name>\w+)(\s+(.*))?$").unwrap();
        }
        let line = line?;

        let err_msg = "usage: command args ...";
        let caps = RE_POINT.captures(&line).ok_or(&err_msg[..])?;
        let cmd_name = caps.name("cmd_name").ok_or(&err_msg[..])?.as_str();

        let err_msg = format!("{:?} is not a valid command.", cmd_name);
        let parse_fn = self.parse_fn.get(cmd_name).ok_or(&err_msg[..])?;

        parse_fn(&line)
    }

    fn next_line(&mut self) -> Option<Result<String, io::Error>> {
        match self.writer.write_all(b"> ") {
            Err(error) => return Some(Err(error)),
            Ok(_) => (),
        }
        match self.writer.flush() {
            Err(error) => return Some(Err(error)),
            Ok(_) => (),
        }

        self.lines.next()
    }
}

impl<Reader, Writer, RenderType> Iterator for CliCommander<Reader, Writer, RenderType>
where
    Reader: 'static + BufRead,
    Writer: Write,
    RenderType: 'static,
    Point: Shape<RenderType>,
{
    type Item = Box<dyn Command<RenderType>>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some(line) = self.next_line() {
            match self.parse_line(line) {
                Ok(cmd) => return Some(cmd),
                Err(error) => {
                    let res = self.writer.write_all(format!("{}\n", error).as_bytes());
                    match res {
                        Err(error) => io::stdout()
                            .lock()
                            .write_all(format!("{}\n", error).as_bytes())
                            .unwrap(),
                        Ok(_) => (),
                    }
                }
            }
        }
        None
    }
}

mod parse_cmd {
    use super::*;

    pub fn point<RenderType>(line: &str) -> Result<Box<dyn Command<RenderType>>, Box<dyn Error>>
    where
        RenderType: 'static,
        Point: Shape<RenderType>,
    {
        lazy_static! {
            static ref PATTERN_CMD_POINT: String =
                r"^\s*Point (?P<name>\w+) (?P<x>[[:digit:]]+) (?P<y>[[:digit:]]+)\s*$"
                    .split(' ')
                    .collect::<Vec<_>>()
                    .join(r"\s*");
            static ref RE_POINT: Regex = Regex::new(&PATTERN_CMD_POINT).unwrap();
        }
        let err_msg = format!(
            r#"The pattern should be like "Point <name> <x> <y>" but got {:?}"#,
            line
        );

        let caps = RE_POINT.captures(&line).ok_or(&err_msg[..])?;
        let name = caps.name("name").ok_or(&err_msg[..])?.as_str();
        let x = caps.name("x").ok_or(&err_msg[..])?.as_str().parse()?;
        let y = caps.name("y").ok_or(&err_msg[..])?.as_str().parse()?;

        Ok(Box::new(DrawShape::<RenderType, Point>::new(
            name.to_string(),
            Point { x, y },
        )))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::render::DummyRenderer;

    #[test]
    fn test_from_string() {
        let cmd = parse_cmd::point::<DummyRenderer>("Point p1 2 3").unwrap();
        assert_eq!(format!("{}", cmd), "p1 Point { x: 2, y: 3 }");
    }
}

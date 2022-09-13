use super::*;
use crate::shape::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::{BufRead, Write};

pub struct CliCommander<Reader, Stdout, Stderr>
where
    Reader: BufRead,
    Stdout: Write,
    Stderr: Write,
{
    // iterator over lines of the reader
    lines: io::Lines<Reader>,
    stdout: Stdout,
    stderr: Stderr,
    parse_fn: HashMap<String, ParseFn>,
}

type ParseFn = fn(&str) -> Result<Box<dyn Command>, Box<dyn Error>>;

impl<Reader, Stdout, Stderr> CliCommander<Reader, Stdout, Stderr>
where
    Reader: 'static + BufRead,
    Stdout: Write,
    Stderr: Write,
    Point: Shape,
    Rectangle: Shape,
    Line: Shape,
    Circle: Shape,
    Square: Shape,
{
    pub fn new(reader: Reader, stdout: Stdout, stderr: Stderr) -> Self {
        let mut this = Self {
            lines: reader.lines(),
            stdout,
            stderr,
            parse_fn: HashMap::new(),
        };
        this.register_parse_fn();

        this
    }

    fn register_parse_fn(&mut self) {
        self.register_parser("point".to_lowercase(), parse_cmd::point);
        self.register_parser(
            "rectangle".to_lowercase(),
            parse_cmd::rectangle,
        );
        self.register_parser("line".to_lowercase(), parse_cmd::line);
        self.register_parser("circle".to_lowercase(), parse_cmd::circle);
        self.register_parser("square".to_lowercase(), parse_cmd::square);

        self.register_parser("move".to_lowercase(), parse_cmd::move_by);
        self.register_parser("undo".to_lowercase(), parse_cmd::undo);
        self.register_parser("redo".to_lowercase(), parse_cmd::redo);
        self.register_parser("delete".to_lowercase(), parse_cmd::delete);
    }
}
impl<Reader, Stdout, Stderr> CliCommander<Reader, Stdout, Stderr>
where
    Reader: 'static + BufRead,
    Stdout: Write,
    Stderr: Write,
{
    pub fn register_parser(
        &mut self,
        cmd_name: String,
        func: ParseFn,
    ) -> Option<ParseFn> {
        self.parse_fn.insert(cmd_name, func)
    }

    fn parse_line(
        &mut self,
        line: Result<String, io::Error>,
    ) -> Result<Box<dyn Command>, Box<dyn Error>> {
        lazy_static! {
            static ref RE_POINT: Regex = Regex::new(r"^\s*(?P<cmd_name>\w+)(\s+(.*))?$").unwrap();
        }
        let line = line?;

        let err_msg = "usage: command args ...";
        let caps = RE_POINT.captures(&line).ok_or(&err_msg[..])?;
        let cmd_name = caps
            .name("cmd_name")
            .ok_or(&err_msg[..])?
            .as_str()
            .to_lowercase();

        let err_msg = format!("{:?} is not a valid command.", cmd_name);
        let parse_fn = self.parse_fn.get(&cmd_name).ok_or(&err_msg[..])?;

        parse_fn(&line)
    }

    fn next_line(&mut self) -> Option<Result<String, io::Error>> {
        match self.stdout.write_all(b"> ") {
            Err(error) => return Some(Err(error)),
            Ok(_) => (),
        }
        match self.stdout.flush() {
            Err(error) => return Some(Err(error)),
            Ok(_) => (),
        }

        self.lines.next()
    }
}

impl Default
    for CliCommander<io::BufReader<io::Stdin>, io::Stdout, io::Stderr>
where
    Point: Shape,
    Rectangle: Shape,
    Line: Shape,
    Circle: Shape,
    Square: Shape,
{
    fn default() -> Self {
        Self::new(io::BufReader::new(io::stdin()), io::stdout(), io::stderr())
    }
}

impl<Reader, Stdout, Stderr> Iterator
    for CliCommander<Reader, Stdout, Stderr>
where
    Reader: 'static + BufRead,
    Stdout: Write,
    Stderr: Write,
    Point: Shape,
    Rectangle: Shape,
    Line: Shape,
    Circle: Shape,
    Square: Shape,
{
    type Item = Box<dyn Command>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        while let Some(line) = self.next_line() {
            match self.parse_line(line) {
                Ok(cmd) => return Some(cmd),
                Err(error) => {
                    let res = self.stderr.write_all(format!("{}\n", error).as_bytes());
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

pub mod parse_cmd;
pub use parse_cmd::{HELP_INFO, READABLE_PATTERNS};

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::render::Renderer;
    use std::collections::HashSet;
    use std::str;

    #[test]
    fn test_draw_shape() {
        let input = b"\
        point p1 0 0\n\
        rectangle rect 2 3 4 5\n\
        line l1 2 3 4 5\n\
        circle cic 23 4 45\n\
        square sq 32 34 56\n\
        foo\n\
        bar foo";
        let input = io::BufReader::new(&input[..]);
        let mut stdout = Vec::<u8>::new();
        let mut stderr = Vec::<u8>::new();
        let commander = CliCommander::new(input, &mut stdout, &mut stderr);
        let mut shapes = Shapes::new();

        for mut cmd in commander {
            cmd.execute(&mut shapes).unwrap();
        }
        let mut buff = Vec::<u8>::new();
        buff.render_shapes(&shapes).unwrap();

        // check
        let correct_stdout = "> > > > > > > > ";
        assert_eq!(str::from_utf8(&stdout).unwrap(), correct_stdout);

        let correct_stderr = "\
            \"foo\" is not a valid command.\n\
            \"bar\" is not a valid command.\n\
            ";
        assert_eq!(str::from_utf8(&stderr).unwrap(), correct_stderr);

        let correct_buff = HashSet::from([
            "",
            "cic Circle { center: Point { x: 23, y: 4 }, radius: 45 }",
            "rect Rectangle { corner: Point { x: 2, y: 3 }, w: 4, h: 5 }",
            "l1 Line(Point { x: 2, y: 3 }, Point { x: 4, y: 5 })",
            "p1 Point { x: 0, y: 0 }",
            "sq Square { corner: Point { x: 32, y: 34 }, side: 56 }",
        ]);
        let buff = io::BufReader::new(&buff[..]);
        for line in buff.lines() {
            let line = line.unwrap();
            assert!(correct_buff.contains(&line[..]));
        }
    }
}

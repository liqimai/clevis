use super::*;
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
    Rectangle: Shape<RenderType>,
    Line: Shape<RenderType>,
    Circle: Shape<RenderType>,
    Square: Shape<RenderType>,
{
    pub fn new(reader: Reader, writer: Writer) -> Self {
        let mut this = Self {
            lines: reader.lines(),
            writer,
            parse_fn: HashMap::new(),
            phantom: PhantomData,
        };
        this.register_parse_fn("point".to_lowercase(), parse_cmd::point::<RenderType>);
        this.register_parse_fn(
            "rectangle".to_lowercase(),
            parse_cmd::rectangle::<RenderType>,
        );
        this.register_parse_fn("line".to_lowercase(), parse_cmd::line::<RenderType>);
        this.register_parse_fn("circle".to_lowercase(), parse_cmd::circle::<RenderType>);
        this.register_parse_fn("square".to_lowercase(), parse_cmd::square::<RenderType>);

        this.register_parse_fn("move".to_lowercase(), parse_cmd::move_by::<RenderType>);

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
    Rectangle: Shape<RenderType>,
    Line: Shape<RenderType>,
    Circle: Shape<RenderType>,
    Square: Shape<RenderType>,
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

mod parse_cmd;

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
        let mut output = Vec::<u8>::new();
        let commander = CliCommander::new(input, &mut output);
        let mut shapes = Shapes::new();

        for cmd in commander {
            cmd.execute(&mut shapes).unwrap();
        }
        let mut buff = Vec::<u8>::new();
        buff.render_shapes(&shapes).unwrap();

        // check
        let correct_output = "\
            > > > > > > \"foo\" is not a valid command.\n\
            > \"bar\" is not a valid command.\n\
            > ";
        assert_eq!(str::from_utf8(&output).unwrap(), correct_output);

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

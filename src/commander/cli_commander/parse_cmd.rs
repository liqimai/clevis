use super::*;
use crate::command::*;
use std::collections::HashMap;

macro_rules! err_msg_pattern {
    () => {
        "The pattern should be like {:?} but got {:?}"
    };
}

lazy_static! {
    pub static ref READABLE_PATTERNS: HashMap<&'static str, &'static str> = HashMap::from([
        ("point", "point <name> <x:i32> <y:i32>"),
        (
            "rectangle",
            "rectangle <name> <x:i32> <y:i32> <w:i32> <h:i32>"
        ),
        ("line", "line <name> <x1:i32> <y1:i32> <x2:i32> <y2:i32>"),
        ("circle", "circle <name> <x:i32> <y:i32> <r:i32>"),
        ("square", "square <name> <x:i32> <y:i32> <l:i32>"),
        ("move", "move <name> <dx:i32> <dy:i32>"),
        ("undo", "undo"),
        ("redo", "redo"),
        ("delete", "delete <name>"),
    ]);
    pub static ref HELP_INFO: HashMap<&'static str, &'static str> = HashMap::from([
        ("point", "Draw point"),
        ("rectangle", "Draw rectangle"),
        ("line", "Draw line"),
        ("circle", "Draw circle"),
        ("square", "Draw square"),
        ("move", "Move a shape"),
        ("undo", "Undo last command"),
        ("redo", "Redo last undone command"),
        ("delete", "Delete a shape by its name"),
    ]);
}

pub fn point(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>>
where
    Point: Shape,
{
    lazy_static! {
        static ref PATTERN_CMD_POINT: String = vec!(
            r"^\s*(?i:point)",
            r"(?P<name>\w+)",
            r"(?P<x>(\+|-)?[[:digit:]]+)",
            r"(?P<y>(\+|-)?[[:digit:]]+)\s*$",
        )
        .join(r"\s+");
        static ref RE_CMD_POINT: Regex = Regex::new(&PATTERN_CMD_POINT).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("point").unwrap(),
        line
    );

    let caps = RE_CMD_POINT.captures(&line).ok_or(&err_msg[..])?;
    let name = caps.name("name").ok_or(&err_msg[..])?.as_str();
    let x = caps.name("x").ok_or(&err_msg[..])?.as_str().parse()?;
    let y = caps.name("y").ok_or(&err_msg[..])?.as_str().parse()?;

    Ok(Box::new(DrawShape::new(
        name.to_string(),
        Point { x, y },
    )))
}

pub fn rectangle(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>>
where
    Rectangle: Shape,
{
    lazy_static! {
        static ref PATTERN_CMD_RECTANGLE: String = vec!(
            r"^\s*(?i:rectangle)",
            r"(?P<name>\w+)",
            r"(?P<x>(\+|-)?[[:digit:]]+)",
            r"(?P<y>(\+|-)?[[:digit:]]+)",
            r"(?P<w>(\+|-)?[[:digit:]]+)",
            r"(?P<h>(\+|-)?[[:digit:]]+)\s*$",
        )
        .join(r"\s+");
        static ref RE_CMD_RECTANGLE: Regex = Regex::new(&PATTERN_CMD_RECTANGLE).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("rectangle").unwrap(),
        line
    );

    let caps = RE_CMD_RECTANGLE.captures(&line).ok_or(&err_msg[..])?;
    let name = caps.name("name").ok_or(&err_msg[..])?.as_str();
    let x = caps.name("x").ok_or(&err_msg[..])?.as_str().parse()?;
    let y = caps.name("y").ok_or(&err_msg[..])?.as_str().parse()?;
    let w = caps.name("w").ok_or(&err_msg[..])?.as_str().parse()?;
    let h = caps.name("h").ok_or(&err_msg[..])?.as_str().parse()?;

    Ok(Box::new(DrawShape::new(
        name.to_string(),
        Rectangle {
            corner: Point { x: x, y: y },
            w,
            h,
        },
    )))
}

pub fn line(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>>
where
    Line: Shape,
{
    lazy_static! {
        static ref PATTERN_CMD_LINE: String = vec!(
            r"^\s*(?i:line)",
            r"(?P<name>\w+)",
            r"(?P<x1>(\+|-)?[[:digit:]]+)",
            r"(?P<y1>(\+|-)?[[:digit:]]+)",
            r"(?P<x2>(\+|-)?[[:digit:]]+)",
            r"(?P<y2>(\+|-)?[[:digit:]]+)\s*$",
        )
        .join(r"\s+");
        static ref RE_CMD_LINE: Regex = Regex::new(&PATTERN_CMD_LINE).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("line").unwrap(),
        line
    );

    let caps = RE_CMD_LINE.captures(&line).ok_or(&err_msg[..])?;
    let name = caps.name("name").ok_or(&err_msg[..])?.as_str();
    let x1 = caps.name("x1").ok_or(&err_msg[..])?.as_str().parse()?;
    let y1 = caps.name("y1").ok_or(&err_msg[..])?.as_str().parse()?;
    let x2 = caps.name("x2").ok_or(&err_msg[..])?.as_str().parse()?;
    let y2 = caps.name("y2").ok_or(&err_msg[..])?.as_str().parse()?;

    Ok(Box::new(DrawShape::new(
        name.to_string(),
        Line(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }),
    )))
}

pub fn circle(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>>
where
    Circle: Shape,
{
    lazy_static! {
        static ref PATTERN_CMD_CIRCLE: String = vec!(
            r"^\s*(?i:circle)",
            r"(?P<name>\w+)",
            r"(?P<x>(\+|-)?[[:digit:]]+)",
            r"(?P<y>(\+|-)?[[:digit:]]+)",
            r"(?P<r>(\+|-)?[[:digit:]]+)\s*$",
        )
        .join(r"\s+");
        static ref RE_CMD_CIRCLE: Regex = Regex::new(&PATTERN_CMD_CIRCLE).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("circle").unwrap(),
        line
    );

    let caps = RE_CMD_CIRCLE.captures(&line).ok_or(&err_msg[..])?;
    let name = caps.name("name").ok_or(&err_msg[..])?.as_str();
    let x = caps.name("x").ok_or(&err_msg[..])?.as_str().parse()?;
    let y = caps.name("y").ok_or(&err_msg[..])?.as_str().parse()?;
    let r = caps.name("r").ok_or(&err_msg[..])?.as_str().parse()?;

    Ok(Box::new(DrawShape::new(
        name.to_string(),
        Circle {
            center: Point { x, y },
            radius: r,
        },
    )))
}

pub fn square(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>>
where
    Square: Shape,
{
    lazy_static! {
        static ref PATTERN_CMD_SQUARE: String = vec!(
            r"^\s*(?i:square)",
            r"(?P<name>\w+)",
            r"(?P<x>(\+|-)?[[:digit:]]+)",
            r"(?P<y>(\+|-)?[[:digit:]]+)",
            r"(?P<l>(\+|-)?[[:digit:]]+)\s*$",
        )
        .join(r"\s+");
        static ref RE_CMD_SQUARE: Regex = Regex::new(&PATTERN_CMD_SQUARE).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("square").unwrap(),
        line
    );

    let caps = RE_CMD_SQUARE.captures(&line).ok_or(&err_msg[..])?;
    let name = caps.name("name").ok_or(&err_msg[..])?.as_str();
    let x = caps.name("x").ok_or(&err_msg[..])?.as_str().parse()?;
    let y = caps.name("y").ok_or(&err_msg[..])?.as_str().parse()?;
    let l = caps.name("l").ok_or(&err_msg[..])?.as_str().parse()?;

    Ok(Box::new(DrawShape::new(
        name.to_string(),
        Square {
            corner: Point { x, y },
            side: l,
        },
    )))
}

pub fn move_by(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>> {
    lazy_static! {
        static ref PATTERN_CMD_MOVE: String = vec!(
            r"^\s*(?i:move)",
            r"(?P<name>\w+)",
            r"(?P<dx>(\+|-)?[[:digit:]]+)",
            r"(?P<dy>(\+|-)?[[:digit:]]+)\s*$",
        )
        .join(r"\s+");
        static ref RE_CMD_MOVE: Regex = Regex::new(&PATTERN_CMD_MOVE).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("move").unwrap(),
        line
    );

    let caps = RE_CMD_MOVE.captures(&line).ok_or(&err_msg[..])?;
    let name = caps.name("name").ok_or(&err_msg[..])?.as_str();
    let dx = caps.name("dx").ok_or(&err_msg[..])?.as_str().parse()?;
    let dy = caps.name("dy").ok_or(&err_msg[..])?.as_str().parse()?;

    Ok(Box::new(MoveBy::new(name.to_string(), dx, dy)))
}

pub fn undo(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>> {
    lazy_static! {
        static ref PATTERN_CMD_UNDO: String = vec!(r"^\s*(?i:undo)\s*$",).join(r"\s+");
        static ref RE_CMD_UNDO: Regex = Regex::new(&PATTERN_CMD_UNDO).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("undo").unwrap(),
        line
    );

    RE_CMD_UNDO.captures(&line).ok_or(&err_msg[..])?;

    Ok(Box::new(Control::Undo))
}

pub fn redo(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>> {
    lazy_static! {
        static ref PATTERN_CMD_REDO: String = vec!(r"^\s*(?i:redo)\s*$",).join(r"\s+");
        static ref RE_CMD_REDO: Regex = Regex::new(&PATTERN_CMD_REDO).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("redo").unwrap(),
        line
    );

    RE_CMD_REDO.captures(&line).ok_or(&err_msg[..])?;

    Ok(Box::new(Control::Redo))
}

pub fn delete(line: &str) -> Result<Box<dyn Command>, Box<dyn Error>>
where
{
    lazy_static! {
        static ref PATTERN_CMD_DELETE: String =
            vec!(r"^\s*(?i:delete)", r"(?P<name>\w+)\s*$",).join(r"\s+");
        static ref RE_CMD_DELETE: Regex = Regex::new(&PATTERN_CMD_DELETE).unwrap();
    }
    let err_msg = format!(
        err_msg_pattern!(),
        READABLE_PATTERNS.get("delete").unwrap(),
        line
    );

    let caps = RE_CMD_DELETE.captures(&line).ok_or(&err_msg[..])?;
    let name = caps.name("name").ok_or(&err_msg[..])?.as_str();

    Ok(Box::new(Delete::new(name.to_string())))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        // point
        let cmd_point = point("Point p1 2 3").unwrap();
        assert_eq!(format!("{}", cmd_point), "p1 Point { x: 2, y: 3 }");
        let cmd_point = point("point p1 4 5").unwrap();
        assert_eq!(format!("{}", cmd_point), "p1 Point { x: 4, y: 5 }");
        assert!(!point("aaa bbb cc cc").is_ok());

        // rectangle
        let cmd_rectangle = rectangle("rectangle rect 2 3 4 5").unwrap();
        assert_eq!(
            format!("{}", cmd_rectangle),
            "rect Rectangle { corner: Point { x: 2, y: 3 }, w: 4, h: 5 }"
        );
        let cmd_rectangle = rectangle("Rectangle rect 4 5 3 2").unwrap();
        assert_eq!(
            format!("{}", cmd_rectangle),
            "rect Rectangle { corner: Point { x: 4, y: 5 }, w: 3, h: 2 }"
        );
        assert!(!rectangle("aaa bbb cc cc").is_ok());

        // line
        let cmd_line = line("line line1 2 3 4 5").unwrap();
        assert_eq!(
            format!("{}", cmd_line),
            "line1 Line(Point { x: 2, y: 3 }, Point { x: 4, y: 5 })"
        );
        let cmd_line = line("line line2 4 5 3 2").unwrap();
        assert_eq!(
            format!("{}", cmd_line),
            "line2 Line(Point { x: 4, y: 5 }, Point { x: 3, y: 2 })"
        );
        assert!(!line("aaa bbb cc cc").is_ok());

        // circle
        let cmd_circle = circle("circle circle1 2 3 4").unwrap();
        assert_eq!(
            format!("{}", cmd_circle),
            "circle1 Circle { center: Point { x: 2, y: 3 }, radius: 4 }"
        );
        let cmd_circle = circle("circle circle2 4 5 3").unwrap();
        assert_eq!(
            format!("{}", cmd_circle),
            "circle2 Circle { center: Point { x: 4, y: 5 }, radius: 3 }"
        );
        assert!(!circle("aaa bbb cc cc").is_ok());

        // square
        let cmd_square = square("square square1 2 3 4").unwrap();
        assert_eq!(
            format!("{}", cmd_square),
            "square1 Square { corner: Point { x: 2, y: 3 }, side: 4 }"
        );
        let cmd_square = square("sQuare square2 4 5 3").unwrap();
        assert_eq!(
            format!("{}", cmd_square),
            "square2 Square { corner: Point { x: 4, y: 5 }, side: 3 }"
        );
        assert!(!square("aaa bbb cc cc").is_ok());

        let cmd_move = move_by("move aaa 3 -5").unwrap();
        assert_eq!(format!("{}", cmd_move), "move aaa 3 -5");

        let undo = undo("undo").unwrap();
        assert_eq!(format!("{}", undo), "undo");
        let redo = redo("redo").unwrap();
        assert_eq!(format!("{}", redo), "redo");

        let delete = delete("delete name").unwrap();
        assert_eq!(format!("{}", delete), r#"Delete "name" with deleted None"#);
    }

    #[test]
    fn test_from_string_error() {
        let non_sense = "aaaaa";

        macro_rules! test {
            ($cmd: ident, $key: expr) => {
                match $cmd(non_sense) {
                    Err(error) => assert_eq!(
                        format!("{}", error),
                        format!(
                            err_msg_pattern!(),
                            *READABLE_PATTERNS.get($key).unwrap(),
                            non_sense
                        )
                    ),
                    Ok(_) => panic!(),
                }
            };
        }

        test!(point, "point");
        test!(rectangle, "rectangle");
        test!(line, "line");
        test!(circle, "circle");
        test!(square, "square");
        test!(move_by, "move");
        test!(undo, "undo");
        test!(redo, "redo");
        test!(delete, "delete");
    }
}

use super::{Error, Renderer, Shape};
use crate::shape::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

macro_rules! HTML_TEMPLATE {
    () => {
        r#"
<!DOCTYPE html>
<html>

<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Canvas</title>
    <style>
        body {{
            margin: 0;
            overflow: hidden;
        }}
    </style>
</head>

<body>
    <canvas class="myCanvas">
        <p>Canvas</p>
    </canvas>
    <script>
        var canvas = document.querySelector('.myCanvas');
        var width = canvas.width = window.innerWidth;
        var height = canvas.height = window.innerHeight;
        var ctx = canvas.getContext('2d');

        ctx.strokeStyle = 'rgb(0, 0, 0)';
        ctx.lineWidth = 2;

        function point(x, y) {{
            ctx.beginPath();
            ctx.arc(x, y, 1, 0, 2 * Math.PI, false);
            ctx.stroke();
        }}

        function rectangle(x, y, w, h) {{
            ctx.strokeRect(x, y, w, h);
        }}

        function circle(x, y, r) {{
            ctx.beginPath();
            ctx.arc(x, y, r, 0, 2 * Math.PI, false);
            ctx.stroke();
        }}

        function square(x, y, l) {{
            ctx.strokeRect(x, y, l, l);
        }}

        function line(x1, y1, x2, y2) {{
            ctx.beginPath();
            ctx.moveTo(x1, y1);
            ctx.lineTo(x2, y2);
            ctx.stroke();
        }}

    </script>
    <script src={}></script>
</body>

</html>"#
    };
}

pub struct HtmlRenderer {
    file: File,
    filename: String,
    auto_refresh: bool,
}

impl HtmlRenderer {
    pub fn html_file_path(&self) -> String {
        format!("{}.html", self.filename)
    }

    pub fn js_file_path(&self) -> String {
        format!("{}.js", self.filename)
    }

    pub fn new(filename: &str, auto_refresh: bool) -> Result<HtmlRenderer, io::Error> {
        let render = HtmlRenderer {
            filename: filename.to_string(),
            file: File::create(format!("{}.js", filename))?,
            auto_refresh,
        };
        File::create(render.html_file_path())?
            .write_all(format!(HTML_TEMPLATE!(), render.js_file_path()).as_bytes())?;

        if auto_refresh {
            render.fresh()?;
        }

        Ok(render)
    }

    pub fn fresh(&self) -> Result<(), io::Error> {
        if cfg!(target_os = "macos") {
            process::Command::new("open")
                .args([self.html_file_path()])
                .output()?;
        }
        if cfg!(target_os = "windows") {
            process::Command::new(self.html_file_path()).output()?;
        }

        Ok(())
    }
}

impl Renderer for HtmlRenderer {
    fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.file = File::create(self.js_file_path())?;
        self.file.rewind()?;

        Ok(())
    }
    fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
        let len = self.file.stream_position()?;
        self.file.set_len(len)?;
        self.file.sync_all()?;

        if self.auto_refresh {
            self.fresh()?;
        }

        Ok(())
    }
    fn render(&mut self, name: &str, shape: &dyn Shape) -> Result<(), Box<dyn Error>> {
        shape.draw_on(self)?;
        self.file.write_all(format!(" // {} \n", name).as_bytes())?;

        Ok(())
    }

    fn draw_point(&mut self, point: &Point) -> Result<(), Box<dyn Error>> {
        self.file
            .write_all(format!("point({}, {});", point.x, point.y).as_bytes())?;
        Ok(())
    }

    fn draw_line(&mut self, line: &Line) -> Result<(), Box<dyn Error>> {
        self.file.write_all(
            format!(
                "line({}, {}, {}, {});",
                line.0.x, line.0.y, line.1.x, line.1.y
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    fn draw_rectangle(&mut self, rectangle: &Rectangle) -> Result<(), Box<dyn Error>> {
        self.file.write_all(
            format!(
                "rectangle({}, {}, {}, {});",
                rectangle.corner.x, rectangle.corner.y, rectangle.w, rectangle.h
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    fn draw_circle(&mut self, circle: &Circle) -> Result<(), Box<dyn Error>> {
        self.file.write_all(
            format!(
                "circle({}, {}, {});",
                circle.center.x, circle.center.y, circle.radius
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    fn draw_square(&mut self, square: &Square) -> Result<(), Box<dyn Error>> {
        self.file.write_all(
            format!(
                "square({}, {}, {});",
                square.corner.x, square.corner.y, square.side
            )
            .as_bytes(),
        )?;
        Ok(())
    }
}

impl Drop for HtmlRenderer {
    fn drop(&mut self) {
        std::fs::remove_file(self.js_file_path()).unwrap();
        std::fs::remove_file(self.html_file_path()).unwrap();
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::tests::get_shapes;
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_html_renderer() {
        use crate::shape::*;
        use std::fs::File;

        let screen_file_name = "crate::render::html_renderer::tests::test_html_renderer";
        let mut render = HtmlRenderer::new(screen_file_name, false).unwrap();
        render.auto_refresh = false;

        let full_shapes = get_shapes();
        let answer = HashSet::from([
            "point(0, 0); // clevis::shape::Point ",
            "circle(0, 0, 0); // clevis::shape::Circle ",
            "square(0, 0, 0); // clevis::shape::Square ",
            "line(0, 0, 0, 0); // clevis::shape::Line ",
            "rectangle(0, 0, 0, 0); // clevis::shape::Rectangle ",
        ]);
        let mut shapes = Shapes::new();
        for (n, s) in full_shapes {
            shapes.insert(n.clone(), s);
            render.render_shapes(&shapes).unwrap();

            // read the file and check
            for line in io::BufReader::new(File::open(render.js_file_path()).unwrap()).lines() {
                let line = line.unwrap();
                dbg!(&line);
                assert!(answer.contains(&line[..]));
            }

            // use std::{thread, time};
            // thread::sleep(time::Duration::from_secs(1));
        }
    }
}

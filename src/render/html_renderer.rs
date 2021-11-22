use super::{Error, Renderer, Shape};
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

pub struct HtmlRenderer {
    file: File,
    filename: String,
    auto_refresh: bool,
}

impl HtmlRenderer {
    pub fn new(filename: &str, auto_refresh: bool) -> Result<HtmlRenderer, io::Error> {
        let render = HtmlRenderer {
            filename: filename.to_string(),
            file: File::create(filename)?,
            auto_refresh,
        };
        if auto_refresh {
            render.fresh()?;
        }

        Ok(render)
    }
    pub fn fresh(&self) -> Result<(), io::Error> {
        if cfg!(macos) {
            process::Command::new("open")
                .args([&self.filename])
                .output()?;
        }

        Ok(())
    }
}

impl Renderer<HtmlRenderer> for HtmlRenderer {
    fn init_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.file = File::create(&self.filename)?;
        self.file.rewind()?;
        self.file.write(b"<table>\n")?;

        Ok(())
    }
    fn finish_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.write(b"</table>\n")?;

        let len = self.file.stream_position()?;
        self.file.set_len(len)?;
        self.file.sync_all()?;

        if self.auto_refresh {
            self.fresh()?;
        }

        Ok(())
    }
    fn render(
        &mut self,
        name: &str,
        shape: &dyn Shape<HtmlRenderer>,
    ) -> Result<(), Box<dyn Error>> {
        let name = format!(
            "\
            \t<tr>\n\
            \t\t<td>{}</td>\n\
            \t\t",
            name
        );
        self.file.write_all(name.as_bytes())?;

        shape.draw(self)?;

        self.file.write_all(b"\n\t</tr>\n")?;

        Ok(())
    }
}

impl Drop for HtmlRenderer {
    fn drop(&mut self) {
        std::fs::remove_file(&self.filename).unwrap();
    }
}

impl<T> Shape<HtmlRenderer> for T
where
    T: Debug,
{
    fn draw(&self, render: &mut HtmlRenderer) -> Result<(), Box<dyn Error>> {
        render
            .file
            .write_all(format!("<td>{:?}</td>", &self).as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::super::tests::{get_answers, get_shapes};
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_html_renderer() {
        use crate::shape::*;
        use std::fs::File;

        let screen_file_name = "crate::render::html_renderer::tests::test_html_renderer.html";
        let mut render = HtmlRenderer::new(screen_file_name, false).unwrap();
        render.auto_refresh = false;

        let full_answer = get_answers();
        let full_shapes = get_shapes();

        let mut answer = HashMap::<String, String>::new();
        let mut shapes = Shapes::new();
        for (n, s) in full_shapes {
            shapes.insert(n.clone(), s);
            render.render_shapes(&shapes).unwrap();

            answer.insert(n.clone(), full_answer.get(&n).unwrap().to_string());

            // read the file and check
            let mut string_render = String::new();
            File::open(screen_file_name)
                .unwrap()
                .read_to_string(&mut string_render)
                .unwrap();
            check_html_render(&answer, &string_render);

            // use std::{thread, time};
            // thread::sleep(time::Duration::from_secs(1));
        }
    }

    fn check_html_render(answer: &HashMap<String, String>, render_result: &str) {
        use regex::Regex;
        let tr =
            Regex::new(r"<tr>\s*<td>(?P<name>.*)</td>\s*<td>(?P<value>.*)</td>\s*</tr>").unwrap();

        let mut cnt = 0;
        for tr_content in tr.captures_iter(render_result) {
            let name = tr_content.name("name").unwrap().as_str();
            let value = tr_content.name("value").unwrap().as_str();
            assert_eq!(answer[name], value);
            cnt += 1;
        }
        assert_eq!(answer.len(), cnt);
    }
}

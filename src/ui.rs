use anstyle::{AnsiColor, Color, Style};
use std::fmt::Display;
use std::io::Write;

pub struct Reporter<W: Write> {
    writer: W,
}

impl Reporter<anstream::AutoStream<std::io::Stdout>> {
    pub fn stdout() -> Self {
        Self {
            writer: anstream::stdout(),
        }
    }
}

impl Reporter<anstream::AutoStream<std::io::Stderr>> {
    pub fn stderr() -> Self {
        Self {
            writer: anstream::stderr(),
        }
    }
}

impl Reporter<Vec<u8>> {
    pub fn buffer() -> Self {
        Self { writer: Vec::new() }
    }

    pub fn into_string(self) -> String {
        String::from_utf8(self.writer).unwrap_or_default()
    }
}

impl<W: Write> Reporter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn success(&mut self, msg: impl Display) -> std::io::Result<()> {
        let green = Style::new()
            .fg_color(Some(Color::Ansi(AnsiColor::Green)))
            .bold();
        writeln!(self.writer, "{green}✔{green:#} {msg}")
    }

    pub fn warn(&mut self, msg: impl Display) -> std::io::Result<()> {
        let yellow = Style::new()
            .fg_color(Some(Color::Ansi(AnsiColor::Yellow)))
            .bold();
        writeln!(self.writer, "{yellow}⚠{yellow:#} {msg}")
    }

    pub fn error(&mut self, msg: impl Display) -> std::io::Result<()> {
        let red = Style::new()
            .fg_color(Some(Color::Ansi(AnsiColor::Red)))
            .bold();
        writeln!(self.writer, "{red}✖{red:#} {msg}")
    }

    pub fn info(&mut self, msg: impl Display) -> std::io::Result<()> {
        let cyan = Style::new()
            .fg_color(Some(Color::Ansi(AnsiColor::Cyan)))
            .bold();
        writeln!(self.writer, "{cyan}ℹ{cyan:#} {msg}")
    }

    pub fn section(&mut self, title: impl Display) -> std::io::Result<()> {
        let bold = Style::new().bold();
        writeln!(self.writer, "\n{bold}{title}{bold:#}")
    }

    pub fn item(&mut self, msg: impl Display) -> std::io::Result<()> {
        let dim = Style::new().dimmed();
        writeln!(self.writer, "  {dim}-{dim:#} {msg}")
    }

    pub fn key_value(&mut self, key: impl Display, val: impl Display) -> std::io::Result<()> {
        let bold = Style::new().bold();
        writeln!(self.writer, "  {bold}{key}:{bold:#} {val}")
    }

    pub fn raw_line(&mut self, line: impl Display) -> std::io::Result<()> {
        writeln!(self.writer, "{line}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reporter_formatting() {
        let mut reporter = Reporter::buffer();

        reporter.success("Completed successfully").unwrap();
        reporter.warn("Potential issue detected").unwrap();
        reporter.error("Critical failure").unwrap();
        reporter.info("Informational notice").unwrap();
        reporter.section("Setup steps").unwrap();
        reporter.item("Configured path").unwrap();

        let output = reporter.into_string();
        assert!(output.contains("Completed successfully"));
        assert!(output.contains("Potential issue detected"));
        assert!(output.contains("Critical failure"));
        assert!(output.contains("Informational notice"));
        assert!(output.contains("Setup steps"));
        assert!(output.contains("Configured path"));
    }
}

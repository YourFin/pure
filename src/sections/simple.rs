use crate::section_gen::{PromptSectionGenerator, Utility};
use std::io;
use std::io::Write;
use termcolor::WriteColor;

pub struct Section<'a> {
    color_spec: termcolor::ColorSpec,
    text: &'a str,
}

impl<'a> Section<'a> {
    pub fn from_text(bg: termcolor::Color, text: &'a str) -> Self {
        let mut spec: termcolor::ColorSpec = termcolor::ColorSpec::default();
        spec.set_fg(Some(termcolor::Color::Black));
        spec.set_bg(Some(bg));
        Self {
            color_spec: spec,
            text: text,
        }
    }
}

impl PromptSectionGenerator for Section<'_> {
    fn utility_for_chars(&self, _amount: i32) -> Utility {
        unimplemented!();
    }
    fn bg_color(&self) -> termcolor::Color {
        *self.color_spec.bg().unwrap()
    }
    fn write_section(&self, _length: i32, buffer: &mut termcolor::Buffer) -> Result<(), io::Error> {
        let _ = buffer.set_color(&self.color_spec);
        write!(buffer, "{}", self.text)?;

        return Ok(());
    }
}

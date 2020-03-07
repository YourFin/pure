use crate::section_gen::{PromptSectionGenerator, Utility};
use lazy_static::lazy_static;
use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use termcolor::WriteColor;

lazy_static! {
    static ref COLOR_SPEC: termcolor::ColorSpec = {
        let mut spec = termcolor::ColorSpec::new();
        spec.set_bg(Some(termcolor::Color::Cyan));
        spec.set_fg(Some(termcolor::Color::Black));
        spec
    };
}

pub struct Pwd {
    path: io::Result<PathBuf>,
}

impl Pwd {
    pub fn from_pwd() -> Pwd {
        Self {
            path: env::current_dir(),
        }
    }
}

impl PromptSectionGenerator for Pwd {
    fn utility_for_chars(&self, _amount: i32) -> Utility {
        unimplemented!();
    }
    fn bg_color(&self) -> termcolor::Color {
        *COLOR_SPEC.bg().unwrap()
    }
    fn write_section(&self, _length: i32, buffer: &mut termcolor::Buffer) -> Result<(), io::Error> {
        let _ = buffer.set_color(&COLOR_SPEC);
        let text = match &self.path {
            Ok(path) => path.display().to_string(),
            Err(err) => err.to_string(),
        };
        write!(buffer, "{}", text)?;
        return Ok(());
    }
}

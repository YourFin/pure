type Utility = i32;
use std::io;
use std::io::Write;
use termcolor::WriteColor;

pub trait PromptSectionGenerator {
    // Potentially relevent: https://www.mit.edu/~modiano/papers/C67.pdf
    fn utility_for_chars(&self, amount: i32) -> Utility;
    fn bg_color(&self) -> termcolor::Color;
    fn write_section(&self, buffer: &mut termcolor::Buffer) -> Result<(), io::Error>;
}

// Takes list of prompt section generators an their precedence, with smaller
// numbers being higher, and an available amount of prompt to fill
//
// Returns a
pub fn generate_prompt(
    seperator: &str,
    prompt_sections: &[(i32, &dyn PromptSectionGenerator)],
    _length: i32,
) -> Result<(termcolor::Buffer, termcolor::BufferWriter), io::Error> {
    let bufwtr = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    // TODO: shrink prompt sections by some heuristic
    //let prompt_sections_by_precidence = prompt_sections
    //.into_iter()
    //.enumerate()
    //.map(|(idx, (precidence, prompt_sec))| (precidence, idx, prompt_sec))
    //.collect()
    //.sort_unstable_by(|(precidence_a, _, _), (precidence_b, _, _)| {
    //precidence_a.cmp(precidence_b)
    //});

    assert!(prompt_sections.len() >= 1);
    for idx in 0..prompt_sections.len() {
        if idx != 0 {
            let mut sep_color = termcolor::ColorSpec::new();
            sep_color.set_fg(Some((prompt_sections[idx - 1].1).bg_color()));
            sep_color.set_bg(Some((prompt_sections[idx].1).bg_color()));
            buffer.set_color(&sep_color)?;
            write!(&mut buffer, "{}", seperator)?;
        }
    }

    return Ok((buffer, bufwtr));
}

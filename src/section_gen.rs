pub type Utility = i32;
use std::io;
use std::io::Write;
use termcolor::WriteColor;

pub trait PromptSectionGenerator {
    // Potentially relevent: https://www.mit.edu/~modiano/papers/C67.pdf
    fn utility_for_chars(&self, amount: i32) -> Utility;
    fn bg_color(&self) -> termcolor::Color;
    fn write_section(&self, length: i32, buffer: &mut termcolor::Buffer) -> Result<(), io::Error>;
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
            // Print out seperator
            let mut sep_color = termcolor::ColorSpec::new();
            sep_color.set_fg(Some((prompt_sections[idx - 1].1).bg_color()));
            sep_color.set_bg(Some((prompt_sections[idx].1).bg_color()));
            buffer.set_color(&sep_color)?;
            write!(&mut buffer, "{}", seperator)?;
        }
        let mut space_color = termcolor::ColorSpec::new();
        space_color.set_bg(Some((prompt_sections[idx].1).bg_color()));

        // Write colored space
        buffer.set_color(&space_color)?;
        write!(&mut buffer, " ")?;

        // Write prompt section
        (prompt_sections[idx].1).write_section(0, &mut buffer)?;

        // Write colored space
        buffer.set_color(&space_color)?;
        write!(&mut buffer, " ")?;
    }
    buffer.set_color(&termcolor::ColorSpec::default())?;

    return Ok((buffer, bufwtr));
}

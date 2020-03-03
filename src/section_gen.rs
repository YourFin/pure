type Utility = i32;

pub trait PromptSectionGenerator {
    // Potentially relevent: https://www.mit.edu/~modiano/papers/C67.pdf
    fn utility_for_chars(&self, amount: i32) -> Utility;
    fn fg_color(&self) -> termcolor::Color;
    fn bg_color(&self) -> termcolor::Color;
    fn gen_section(&self, length: i32);
}

// Takes list of prompt section generators an their precedence, with smaller
// numbers being higher, and an available amount of prompt to fill
//
// Returns a
pub fn generate_prompt(
    prompt_sections: Vec<(i32, &PromptSectionGenerator)>,
    length: i32,
) -> (termcolor::Buffer, termcolor::BufferWriter) {
    let mut bufwtr = termcolor::BufferWriter::stdout(termcolor::ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    let prompt_sections_by_precidence = prompt_sections
        .into_iter()
        .enumerate()
        .map(|(idx, (precidence, prompt_sec))| (precidence, idx, prompt_sec))
        .collect()
        .sort_unstable_by(|(precidence_a, _, _), (precidence_b, _, _)| {
            precidence_a.cmp(precidence_b)
        });
}

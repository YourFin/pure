pub trait PromptSectionGenerator {
	fn max_length() -> i32;
	fn fg_color() -> termcolor::Color;
	fn bg_color() -> termcolor::Color;
	fn gen_section(length: i32);
}

// Takes list of prompt section generators an their precedence, with smaller numbers
// being higher, and an available amount of prompt to fill
//
// Returns a
pub fn generate_prompt(prompt_sections: Vec<(i32, &PromptSectionGenerator)>,
					   length: i32) -> (termcolor::Buffer, termcolor::BufferWriter) {
	let mut buffer = bufwtr = termcolor::BufferWriter::
}

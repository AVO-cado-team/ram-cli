use ramemu::parser;
use ramemu::program::Program;
use std::path::PathBuf;
use colored::Colorize;

use crate::errors::RamCliError;
use crate::display_error::display_parsing_error;


pub fn create_program(
    source: &str,
    file_path: PathBuf,
) -> Result<Program, RamCliError> {
    let program: Vec<_> = parser::parse(source).collect();

    let errors = program
        .iter()
        .filter_map(|stmt| stmt.as_ref().err().map(|e| display_parsing_error(source, e)))
        .count();
    
    if errors == 0 {
        return Ok(
            Program::from(
                program
                    .into_iter()
                    .map(|stmt| stmt.expect("Should be no errors (checked above)"))
                    .collect()
            )
        )
    }
    
    Err(RamCliError::Parse(
        format!(
            "Found {} {} in file: {}",
            errors.to_string().bright_yellow().bold(),
            if errors == 1 { "error" } else { "errors" },
            file_path
                .to_str()
                .expect("That shouldn't happen. Please report this bug.")
                .bright_yellow()
                .bold()

        )
    ))
}

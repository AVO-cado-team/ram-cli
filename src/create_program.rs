use colored::Colorize;
use std::path::PathBuf;
use ramemu::parser;
use ramemu::stmt::Stmt;
use ramemu::program::Program;
use ramemu::errors::ParseError;

use crate::errors::RamCliError;
use crate::display_error::display_parsing_error;

pub fn create_program(
    source: &str,
    file_path: PathBuf,
) -> Result<Program, RamCliError> {
    let stmts_result: Vec<_> = parser::parse(source).collect();
    let errors: Vec<ParseError> = stmts_result
        .iter()
        .filter_map(|stmt|
            if let Err(e) = stmt {
                display_parsing_error(source, e).ok()?;
                Some(e.clone())
            } else {
                None
            }
        )
        .collect();

    if !errors.is_empty() {
        return Err(RamCliError::Parse(
            format!(
                "Found {} {} in file: {}",
                errors.len().to_string().bright_yellow().bold(),
                if errors.len() == 1 { "error" } else { "errors" },
                file_path
                    .to_str()
                    .expect("This should never happen")
                    .bright_yellow()
                    .bold()
            )
        ))
    }

    let stmts: Vec<Stmt> = stmts_result.into_iter().filter_map(|stmt| stmt.ok()).collect();
    Ok(Program::from(stmts))
}

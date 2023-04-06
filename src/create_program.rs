use crate::display_error::display_parsing_error;
use crate::errors::RamCliError;
use ramemu::parser;
use ramemu::program::Program;
use ramemu::stmt::Stmt;
use std::path::PathBuf;

pub fn create_program(source: &str, file_path: PathBuf) -> Result<Program, RamCliError> {
    let stmts_result: Vec<_> = parser::parse(source).collect();
    let errors: Vec<_> = stmts_result
        .iter()
        .filter_map(|stmt| stmt.as_ref().err().cloned())
        .collect();

    for error in errors.iter() {
        display_parsing_error(source, error)?;
    }

    if errors.is_empty() {
        let stmts: Vec<Stmt> = stmts_result
            .into_iter()
            .filter_map(|stmt| stmt.ok())
            .collect();
        return Ok(Program::from(stmts));
    }

    Err(RamCliError::Parse(format!(
        "Found {} {} in file: {}",
        errors.len(),
        if errors.len() == 1 { "error" } else { "errors" },
        file_path.to_str().expect("This should never happen").trim()
    )))
}

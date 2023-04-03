use std::path::PathBuf;
use ramemu::parser;
use colored::Colorize;


use crate::errors::RamCliError;
use crate::errors::RamCliErrorKind;
use crate::display_error::display_parsing_error;

pub fn check(
    file: PathBuf,
) -> Result<i32, RamCliError> {
    let path = std::path::Path::new(&file);
    let full_path = std::fs::canonicalize(path).map_err(|_| RamCliError::new(
        RamCliErrorKind::Other,
        format!("File not found: {:?}", path),
    ))?;
    
    if !full_path.is_file() {
        return Err(RamCliError::new(
            RamCliErrorKind::Other,
            format!("You must specify a file, not a directory: {:?}", full_path),
        ));
    }

    let source = std::fs::read_to_string(full_path.clone()).map_err(|_| RamCliError::new(
        RamCliErrorKind::Other,
        format!("Could not read file: {:?}", full_path),
    ))?;

    let program: Vec<_> = parser::parse(source.as_str()).collect();

    let errors: i32 = program.iter().map(
        |stmt| match stmt {
            Ok(_) => 0,
            Err(e) => {
                display_parsing_error(source.as_str(), e);
                1
            }
        }
    ).sum();

    

    if errors == 0 {
        println!("{}: No errors found {}", "Syntax analysis".cyan().bold(), "✓".green().bold());
        return Ok(0);
    }
    
    let error_message = if errors == 1 {
        format!("{}: Found {} error in file: {:?}",
            "Syntax analysis error".cyan().bold(),
            errors.to_string().bright_yellow().bold(),
            full_path
        )
    } else {
        format!(
            "{}: Found {} errors in file: {:?}",
            "Syntax analysis error".cyan().bold(),
            errors.to_string().bright_yellow().bold(),
            full_path
        )
    };

    Err(RamCliError::new(
        RamCliErrorKind::Parse,
        error_message,
    ))

}
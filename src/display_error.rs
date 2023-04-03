use ramemu::errors::ParseError;
use ramemu::errors::InvalidArgument;
use ramemu::errors::InterpretError;
use colored::Colorize;

use crate::errors::RamCliError;
use crate::errors::RamCliErrorKind;


fn get_line_from_source<'a>(source: &'a str, line_index: usize) -> Result<&'a str, RamCliError> {
    for (i, line) in source.lines().enumerate() {
        if i == line_index - 1 {
            return Ok(line);
        }
    }
    Err(RamCliError::new(
        RamCliErrorKind::Other,
        format!("Could not find line {} in source", line_index),
    ))
}

fn display_error_message(source: &str, line_index: usize, message: &str) {
    let line = get_line_from_source(source, line_index).unwrap();
    let (command, comment) = line.split_once('#').unwrap_or((line, ""));

    let comment = if !comment.is_empty() {
        format!("#{}", comment).as_str().truecolor(100, 100, 100)
    } else {
        "".normal()
    };
    
    println!(
        "{}: {}", 
        "Error".red().bold(),
        message
    );
    println!(
        "{}\t{} {}{}",
        line_index.to_string().bright_blue().bold(),
        '|'.to_string().bright_blue().bold(),
        command,
        comment
    );
    println!();
}

pub fn display_parsing_error(source: &str, error: &ParseError) {
    match error {
        ParseError::LabelIsNotValid (line_index ) => {
            display_error_message(source, *line_index, "Label is not valid");
        }
        ParseError::UnsupportedSyntax(line_index) => {
            display_error_message(source, *line_index, "Unsupported syntax");
        }
        ParseError::UnsupportedOpcode(line_index, opcode) => {
            display_error_message(source, *line_index, &format!("Unsupported opcode: {}", opcode.to_string().bright_blue().bold()));
        }
        ParseError::ArgumentIsRequired(line_index) => {
            display_error_message(source, *line_index, "Argument is required");
        }
        ParseError::ArgumentIsNotValid(line_index, argument) => {
            display_error_message(
                source, 
                *line_index, 
                &format!("Argument is not valid: {}", match argument {
                    InvalidArgument::LabelIsNotValid => "Label is not valid",
                    InvalidArgument::ArgumentIsRequired => "Argument is required",
                    InvalidArgument::ArgumentValueMustBeNumberic => "Argument value must be numeric",
                    InvalidArgument::PureArgumentIsNotAllowed => "Pure argument is not allowed in this context",
                    InvalidArgument::ArgumentIsNotValid => "Argument is not valid",
                }
        ));
        }
        ParseError::UnknownError(line_index) => {
            display_error_message(source, *line_index, "Unknown error");
        }
    }
}


pub fn display_runtime_error(source: &str, error: &InterpretError) {
    match error {
        InterpretError::SegmentationFault(line_index) => {
            display_error_message(source, *line_index, "Segmentation fault");
        },
        InterpretError::UnknownLabel(line_index) => {
            display_error_message(source, *line_index, "Unknown label");
        },
        InterpretError::InvalidInput(line_index, input) => {
            display_error_message(source, *line_index, &format!("Invalid input: {}", input));
        },
        InterpretError::InvalidLiteral(line_index) => {
            display_error_message(source, *line_index, "Invalid literal");
        },
        InterpretError::DivisionByZero(line_index) => {
            display_error_message(source, *line_index, "Division by zero");
        },
        InterpretError::WriteError(line_index) => {
            display_error_message(source, *line_index, "Write error");
        },
        InterpretError::Halted(line_index) => {
            display_error_message(source, *line_index, "Halted");
        }
    }
}
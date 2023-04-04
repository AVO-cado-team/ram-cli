use crate::errors::RamCliError;
use colored::Colorize;
use ramemu::errors::InterpretError;
use ramemu::errors::InvalidArgument;
use ramemu::errors::ParseError;

fn get_line_from_source(source: &str, line_index: usize) -> Result<&str, RamCliError> {
    source
        .lines()
        .nth(line_index - 1)
        .ok_or_else(|| RamCliError::Other(format!("Could not find line {} in source", line_index)))
}

fn display_error_message(
    source: &str,
    line_index: usize,
    message: &str,
) -> Result<(), RamCliError> {
    let line = get_line_from_source(source, line_index)?;
    let (command, comment) = line.split_once('#').unwrap_or((line, ""));

    let comment = if !comment.is_empty() {
        format!("#{}", comment).as_str().truecolor(100, 100, 100)
    } else {
        "".normal()
    };

    println!("{}: {}", "Error".red().bold(), message);
    println!(
        "{}\t{} {}{}\n",
        line_index.to_string().bright_blue().bold(),
        '|'.to_string().bright_blue().bold(),
        command,
        comment
    );
    Ok(())
}

pub fn display_parsing_error(source: &str, error: &ParseError) -> Result<(), RamCliError> {
    let (line_index, message) = match error {
        ParseError::LabelIsNotValid(line_index) => (
            *line_index,
            format!(
                "Label is not valid. Use only labels from {}:",
                "[a-zA-Z0-9_]+".bright_blue().bold()
            ),
        ),
        ParseError::UnsupportedSyntax(line_index) => (
            *line_index,
            "Unsupported syntax. Maybe you passed on an extra argument".to_string(),
        ),
        ParseError::UnsupportedOpcode(line_index, opcode) => (
            *line_index,
            format!(
                "Unsupported opcode: {}",
                opcode.to_string().bright_blue().bold()
            ),
        ),
        ParseError::ArgumentIsRequired(line_index) => {
            (*line_index, "Argument is required".to_string())
        }
        ParseError::ArgumentIsNotValid(line_index, argument) => (
            *line_index,
            format!(
                "Argument is not valid: {}",
                match argument {
                    InvalidArgument::LabelIsNotValid => "Label is not valid",
                    InvalidArgument::ArgumentIsRequired => "Argument is required",
                    InvalidArgument::ArgumentValueMustBeNumberic =>
                        "Argument value must be numeric",
                    InvalidArgument::PureArgumentIsNotAllowed =>
                        "Pure argument is not allowed in this context",
                    InvalidArgument::ArgumentIsNotValid => "Argument is not valid",
                }
            ),
        ),
        ParseError::UnknownError(line_index) => (*line_index, "Unknown error".to_string()),
    };

    display_error_message(source, line_index, &message)?;
    Ok(())
}

pub fn display_runtime_error(source: &str, error: &InterpretError) -> Result<(), RamCliError> {
    let (line_index, message) = match error {
        InterpretError::SegmentationFault(line_index) => (
            *line_index,
            "Segmentation fault. Look for possible reasons in the documentation".to_string(),
        ),
        InterpretError::UnknownLabel(line_index) => (
            *line_index,
            "Unknown label. Attempted to jump to unknown label".to_string(),
        ),
        InterpretError::InvalidInput(line_index, input) => {
            (*line_index, format!("Invalid input: {}", input))
        }
        InterpretError::InvalidLiteral(line_index) => (
            *line_index,
            "Invalid literal. Attempted to use invalid literal value".to_string(),
        ),
        InterpretError::DivisionByZero(line_index) => (
            *line_index,
            "Division by zero. Are you trying to divide by zero?".to_string(),
        ),
        InterpretError::WriteError(line_index) => (
            *line_index,
            "Output stream error. Can I write in the specified output?".to_string(),
        ),
        InterpretError::Halted(line_index) => (
            *line_index,
            "Halted! You have reached the end of the program. Don't try to continue execution!"
                .to_string(),
        ),
    };
    display_error_message(source, line_index, &message)?;
    Ok(())
}

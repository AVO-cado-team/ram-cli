use crate::colorize::styled_output;
use crate::colorize::STL;
use crate::errors::RamCliError;
use crate::highlighter::code_highlighter;
use crate::highlighter::PotentialProblem;
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
    pp: PotentialProblem,
) -> Result<(), RamCliError> {
    styled_output("Error: ", vec![STL::Red, STL::Bold]);
    styled_output(message, vec![]);
    styled_output("\n", vec![]);

    if line_index != 0 {
        code_highlighter(
            line_index - 1,
            get_line_from_source(source, line_index - 1)?,
            PotentialProblem::DistplayOnly,
        );
    }
    code_highlighter(line_index, get_line_from_source(source, line_index)?, pp);

    if line_index != source.lines().count() {
        code_highlighter(
            line_index + 1,
            get_line_from_source(source, line_index + 1)?,
            PotentialProblem::DistplayOnly,
        );
    }
    styled_output("\n", vec![]);
    Ok(())
}

pub fn display_parsing_error(source: &str, error: &ParseError) -> Result<(), RamCliError> {
    let (line_index, message, pp) = match error {
        ParseError::LabelIsNotValid(line_index) => (
            *line_index,
            format!(
                "Label is not valid. Use only labels from {}:",
                "(a-zA-Z_)[a-zA-Z0-9_]*"
            ),
            PotentialProblem::Head,
        ),
        ParseError::UnsupportedSyntax(line_index) => (
            *line_index,
            "Unsupported syntax. Maybe you passed on an extra argument".to_string(),
            PotentialProblem::Tail,
        ),
        ParseError::UnsupportedOpcode(line_index, opcode) => (
            *line_index,
            format!("Unsupported opcode: {}", opcode),
            PotentialProblem::Head,
        ),
        ParseError::ArgumentIsRequired(line_index) => (
            *line_index,
            "Argument is required".to_string(),
            PotentialProblem::Tail,
        ),
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
                },
            ),
            PotentialProblem::Tail,
        ),
        ParseError::UnknownError(line_index) => (
            *line_index,
            "Unknown error".to_string(),
            PotentialProblem::Head,
        ),
    };

    display_error_message(source, line_index, &message, pp)?;
    Ok(())
}

pub fn display_runtime_error(source: &str, error: &InterpretError) -> Result<(), RamCliError> {
    let (line_index, message, iip) = match error {
        InterpretError::SegmentationFault(line_index) => (
            *line_index,
            "Segmentation fault. Look for possible reasons in the documentation".to_string(),
            PotentialProblem::Unkn,
        ),
        InterpretError::UnknownLabel(line_index) => (
            *line_index,
            "Unknown label. Attempted to jump to unknown label".to_string(),
            PotentialProblem::Tail,
        ),
        InterpretError::InvalidInput(line_index, input) => (
            *line_index,
            format!("Invalid input: {}", input),
            PotentialProblem::Unkn,
        ),
        InterpretError::InvalidLiteral(line_index) => (
            *line_index,
            "Invalid literal. Attempted to use invalid literal value".to_string(),
            PotentialProblem::Unkn,
        ),
        InterpretError::DivisionByZero(line_index) => (
            *line_index,
            "Division by zero. Are you trying to divide by zero?".to_string(),
            PotentialProblem::Unkn,
        ),
        InterpretError::IOError(line_index) => (
            *line_index,
            "Invalid input/output. Check if you have provided valid input/output files".to_string(),
            PotentialProblem::Unkn,
        ),
        InterpretError::Halted(line_index) => (
            *line_index,
            "Halted! You have reached the end of the program. Don't try to continue execution!"
                .to_string(),
            PotentialProblem::Unkn,
        ),
    };
    display_error_message(source, line_index, &message, iip)?;
    Ok(())
}

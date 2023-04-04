use crate::colorize::Colorizable;
use crate::errors::RamCliError;
use ramemu::errors::InterpretError;
use ramemu::errors::InvalidArgument;
use ramemu::errors::ParseError;

/// Invalid Instruction Part
enum IIP {
    Opcode,
    Argument,
    Full,
    Nothing,
}

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
    iip: IIP,
) -> Result<(), RamCliError> {
    let line = get_line_from_source(source, line_index)?;
    let (command, comment) = line.split_once('#').unwrap_or((line, ""));
    let (mut opcode, mut argument) = command.split_once(' ').unwrap_or((command, ""));
    opcode = opcode.trim();
    argument = argument.trim();

    let command = match iip {
        IIP::Opcode => format!("{} {} ", format!(" {} ", opcode).bgred(), argument),
        IIP::Argument => format!("{} {} ", opcode, format!(" {} ", argument).bgred()),
        IIP::Full => format!(
            "{} {} ",
            format!(" {} ", opcode).bgred(),
            format!(" {} ", argument).bgred()
        ),
        IIP::Nothing => command.to_string(),
    };

    let comment = if !comment.is_empty() {
        format!("#{}", comment).as_str().fggray()
    } else {
        "".to_string()
    };

    println!("{}: {}", "Error".fgred().stbold(), message.trim());
    println!(
        "{}\t{} {}{}\n",
        line_index.to_string().fgbright_blue().stbold(),
        '|'.to_string().fgbright_blue().stbold(),
        command,
        comment
    );
    Ok(())
}

pub fn display_parsing_error(source: &str, error: &ParseError) -> Result<(), RamCliError> {
    let (line_index, message, iip) = match error {
        ParseError::LabelIsNotValid(line_index) => (
            *line_index,
            format!(
                "Label is not valid. Use only labels from {}:",
                "(a-zA-Z_)[a-zA-Z0-9_]*".fgbright_blue().stbold()
            ),
            IIP::Full,
        ),
        ParseError::UnsupportedSyntax(line_index) => (
            *line_index,
            "Unsupported syntax. Maybe you passed on an extra argument".to_string(),
            IIP::Argument,
        ),
        ParseError::UnsupportedOpcode(line_index, opcode) => (
            *line_index,
            format!(
                "Unsupported opcode: {}",
                opcode.to_string().fgbright_blue().stbold()
            ),
            IIP::Opcode,
        ),
        ParseError::ArgumentIsRequired(line_index) => (
            *line_index,
            "Argument is required".to_string(),
            IIP::Argument,
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
            IIP::Argument,
        ),
        ParseError::UnknownError(line_index) => {
            (*line_index, "Unknown error".to_string(), IIP::Opcode)
        }
    };

    display_error_message(source, line_index, &message, iip)?;
    Ok(())
}

pub fn display_runtime_error(source: &str, error: &InterpretError) -> Result<(), RamCliError> {
    let (line_index, message, iip) = match error {
        InterpretError::SegmentationFault(line_index) => (
            *line_index,
            "Segmentation fault. Look for possible reasons in the documentation".to_string(),
            IIP::Nothing,
        ),
        InterpretError::UnknownLabel(line_index) => (
            *line_index,
            "Unknown label. Attempted to jump to unknown label".to_string(),
            IIP::Argument,
        ),
        InterpretError::InvalidInput(line_index, input) => (
            *line_index,
            format!("Invalid input: {}", input),
            IIP::Nothing,
        ),
        InterpretError::InvalidLiteral(line_index) => (
            *line_index,
            "Invalid literal. Attempted to use invalid literal value".to_string(),
            IIP::Nothing,
        ),
        InterpretError::DivisionByZero(line_index) => (
            *line_index,
            "Division by zero. Are you trying to divide by zero?".to_string(),
            IIP::Nothing,
        ),
        InterpretError::IOError(line_index) => (
            *line_index,
            "Invalid input/output. Check if you have provided valid input/output files".to_string(),
            IIP::Nothing,
        ),
        InterpretError::Halted(line_index) => (
            *line_index,
            "Halted! You have reached the end of the program. Don't try to continue execution!"
                .to_string(),
            IIP::Nothing,
        ),
    };
    display_error_message(source, line_index, &message, iip)?;
    Ok(())
}

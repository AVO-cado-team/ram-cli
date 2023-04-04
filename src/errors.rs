use crate::colorize::Colorizable;
use ramemu::errors::InterpretError;
use std::error::Error;

pub trait DescribtibleError {
    fn get_error_reason(&self) -> String;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum RamCliError {
    Io(String),
    Parse(String),
    Runtime(String),
    Other(String),
}

impl std::fmt::Display for RamCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (kind, message) = match &self {
            RamCliError::Io(message) => ("IO error", message),
            RamCliError::Parse(message) => ("Parse error", message),
            RamCliError::Runtime(message) => ("Runtime error", message),
            RamCliError::Other(message) => ("Unknown error", message),
        };
        write!(f, "{}: {}", kind.fgred().stbold(), message)
    }
}

impl Error for RamCliError {}

impl DescribtibleError for RamCliError {
    fn get_error_reason(&self) -> String {
        match self {
            RamCliError::Io(_) => "IO error".to_string(),
            RamCliError::Parse(_) => "Parse error".to_string(),
            RamCliError::Runtime(_) => "Runtime error".to_string(),
            RamCliError::Other(_) => "Unknown error".to_string(),
        }
    }
}

impl DescribtibleError for InterpretError {
    fn get_error_reason(&self) -> String {
        match self {
            InterpretError::SegmentationFault(_) => "Segmentation fault".to_string(),
            InterpretError::UnknownLabel(_) => "Unknown label".to_string(),
            InterpretError::InvalidInput(_, _) => "Invalid input".to_string(),
            InterpretError::InvalidLiteral(_) => "Invalid literal".to_string(),
            InterpretError::DivisionByZero(_) => "Division by zero".to_string(),
            InterpretError::IOError(_) => "IO error".to_string(),
            InterpretError::Halted(_) => "Halted".to_string(),
        }
    }
}

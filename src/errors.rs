use std::error::Error;
use colored::Colorize;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum RamCliError {
    Io(String),
    Parse(String),
    Runtime(String),
    Other(String)
}

impl std::fmt::Display for RamCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (kind, message) = match &self {
            RamCliError::Io(message) => ("IO error", message),
            RamCliError::Parse(message) => ("Parse error", message),
            RamCliError::Runtime(message) => ("Runtime error", message),
            RamCliError::Other(message) => ("Unknown error", message),
        };
        write!(f, "{}: {}", kind.red().bold(), message)
    }
}

impl Error for RamCliError {}

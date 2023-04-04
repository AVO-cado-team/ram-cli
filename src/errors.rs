use std::error::Error;
use colored::Colorize;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum RamCliErrorKind {
    Io(String),
    Parse(String),
    Runtime(String),
    Other(String)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct RamCliError {
    pub kind: RamCliErrorKind,
}

impl RamCliError {
    pub fn new(kind: RamCliErrorKind) -> Self {
        Self {
            kind,
        }
    }
}

impl std::fmt::Display for RamCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (kind, message) = match &self.kind {
            RamCliErrorKind::Io(message) => ("IO error", message),
            RamCliErrorKind::Parse(message) => ("Parse error", message),
            RamCliErrorKind::Runtime(message) => ("Runtime error", message),
            RamCliErrorKind::Other(message) => ("Unknown error", message),
        };
        write!(f, "{}: {}", kind.red().bold(), message)
    }
}

impl Error for RamCliError {}

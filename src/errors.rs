use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub enum RamCliErrorKind {
    // Io,
    Parse,
    Other,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct RamCliError {
    pub kind: RamCliErrorKind,
    pub message: String
}

impl RamCliError {
    pub fn new(kind: RamCliErrorKind, message: String) -> Self {
        Self {
            kind,
            message
        }
    }
}

impl std::fmt::Display for RamCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}



impl Error for RamCliError {}
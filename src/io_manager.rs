use std::fs::canonicalize;
use std::fs::read_to_string;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use colored::Colorize;
use std::io::{stdin, stdout, Write};
use std::io::{BufReader, BufWriter};

use crate::errors::RamCliError;
use crate::errors::RamCliErrorKind;

struct RamCliStdin;
struct RamCliStdout;

impl std::io::Read for RamCliStdin {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        print!("{}", ">>> ".cyan().bold());
        stdout().flush().unwrap();
        stdin().read(buf)
    }
}

impl std::io::Write for RamCliStdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf == b"\n" || buf == b"\r" {
            println!();
            return Ok(buf.len());
        }
        print!("{} {}", "<<<".cyan().bold(), String::from_utf8_lossy(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        stdout().flush()
    }
}

fn ramcli_stdin() -> RamCliStdin {
    RamCliStdin
}

fn ramcli_stdout() -> RamCliStdout {
    RamCliStdout
}

pub fn read_source(file: PathBuf) -> Result<String, RamCliError> {
    let path = Path::new(&file);
    let full_path = canonicalize(path).map_err(|_| RamCliError::new(
        RamCliErrorKind::Io(format!("File not found: {:?}", path))
    ))?;
    
    if !full_path.is_file() {
        return Err(RamCliError::new(
            RamCliErrorKind::Io(format!("You must specify a file, not a directory: {:?}", full_path))
        ));
    }

    let source = read_to_string(full_path.clone()).map_err(|_| RamCliError::new(
        RamCliErrorKind::Io(format!("Could not read file: {:?}", full_path))
    ))?;

    Ok(source)
}

pub fn create_input_buf(input: Option<PathBuf>) -> Result<Box<dyn std::io::BufRead>, RamCliError> {
    if let Some(input) = input {
        let input_file = File::open(input).map_err(|_| RamCliError::new(
            RamCliErrorKind::Io("Could not open input file".to_string())
        ))?;
        Ok(Box::new(BufReader::new(input_file)))
    } else {
        Ok(Box::new(BufReader::new(ramcli_stdin())))
    }
}

pub fn create_output_buf(output: Option<PathBuf>) -> Result<Box<dyn std::io::Write>, RamCliError> {
    if let Some(output) = output {
        let output_file = File::create(output).map_err(|_| RamCliError::new(
            RamCliErrorKind::Io("Could not open output file".to_string())
        ))?;
        Ok(Box::new(BufWriter::new(output_file)))
    } else {
        Ok(Box::new(ramcli_stdout()))
    }
}
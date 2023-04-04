use colored::Colorize;
use std::fs::{canonicalize, read_to_string, File};
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Result as IoResult, Write};
use std::path::PathBuf;

use crate::errors::RamCliError;

struct RamCliStdin;
struct RamCliStdout;

impl Read for RamCliStdin {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        print!("{}", ">>> ".cyan().bold());
        stdout().flush().unwrap();
        stdin().read(buf)
    }
}

impl Write for RamCliStdout {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        if buf == b"\n" || buf == b"\r" {
            println!();
            return Ok(buf.len());
        }
        print!("{} {}", "<<<".cyan().bold(), String::from_utf8_lossy(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
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
    let file_path_str = if let Some(fps) = file.to_str() {
        fps
    } else {
        return Err(RamCliError::Io(
            "You specified a file with an invalid path".to_string(),
        ));
    };

    let abs_path = canonicalize(&file)
        .map_err(|_| RamCliError::Io(format!("File not found: {}", file_path_str)))?;

    if !abs_path.is_file() {
        return Err(RamCliError::Io(format!(
            "You must specify a file, not a directory: {}",
            file_path_str
        )));
    }

    let source = read_to_string(abs_path)
        .map_err(|_| RamCliError::Io(format!("Could not read file: {}", file_path_str)))?;

    Ok(source)
}

pub fn create_input_buf(input: Option<PathBuf>) -> Result<Box<dyn BufRead>, RamCliError> {
    if let Some(input) = input {
        let input_file = File::open(input)
            .map_err(|_| RamCliError::Io("Could not open input file".to_string()))?;
        Ok(Box::new(BufReader::new(input_file)))
    } else {
        Ok(Box::new(BufReader::new(ramcli_stdin())))
    }
}

pub fn create_output_buf(output: Option<PathBuf>) -> Result<Box<dyn std::io::Write>, RamCliError> {
    if let Some(output) = output {
        let output_file = File::create(output)
            .map_err(|_| RamCliError::Io("Could not open output file".to_string()))?;
        Ok(Box::new(BufWriter::new(output_file)))
    } else {
        Ok(Box::new(ramcli_stdout()))
    }
}

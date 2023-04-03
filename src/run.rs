use std::io::BufRead;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{stdin, stdout};
use ramemu::{program::Program, ram::Ram};
use colored::Colorize;

use crate::display_error::display_runtime_error;


pub enum RunStatusCodes {
    Success,
    Error,
}


pub fn run (
    file: PathBuf,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
) -> RunStatusCodes {
    let input_buf: Box<dyn BufRead> = if let Some(input) = input {
        let input_file = std::fs::File::open(input).expect("Unable to open input file");
        Box::new(BufReader::new(input_file))
    } else {
        Box::new(BufReader::new(stdin()))
    };

    let output_buf: Box<dyn std::io::Write> = if let Some(output) = output {
        let output_file = std::fs::File::create(output).expect("Unable to open output file");
        Box::new(BufWriter::new(output_file))
    } else {
        Box::new(BufWriter::new(stdout()))
    };

    let source = std::fs::read_to_string(file).expect("Unable to read file");
    let program = Program::from_source(source.as_str()).unwrap();
    
    let mut ram = Ram::new(
        program,
        input_buf,
        output_buf,
    );

    match ram.run() {
        Ok(_) => {
            println!(
                "{}: Program finished successfully {}",
                "Runtime".cyan().bold(),
                "✓".green().bold()
            );
        },
        Err(e) => {
            display_runtime_error(&source, &e);
            println!(
                "{}: Program finished with errors {}",
                "Runtime".cyan().bold(),
                "✗".red().bold()
            );
            return RunStatusCodes::Error;
        }
    }

    RunStatusCodes::Success
}
use std::path::PathBuf;
use ramemu::stmt::Stmt;
use ramemu::{program::Program, ram::Ram};
use crate::errors::RamCliError;
use crate::io_manager::create_input_buf;
use crate::io_manager::create_output_buf;
use crate::display_error::display_runtime_error;


pub fn run_source (
    source: &str,
    statements: Vec<Stmt>,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<(), RamCliError> {
    let input_buf = create_input_buf(input)?;
    let output_buf = create_output_buf(output)?;
    let program = Program::from(statements);
    
    let mut ram = Ram::new(
        program,
        input_buf,
        output_buf,
    );

    match ram.run() {
        Ok(_) => {},
        Err(e) => {
            display_runtime_error(source, &e)?;
            return Err(
                RamCliError::Runtime(format!("{:?}", e))
            );
        }
    }

    Ok(())
}
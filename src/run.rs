use crate::display_error::display_runtime_error;
use crate::errors::DescribtibleError;
use crate::errors::RamCliError;
use crate::io_manager::create_input_buf;
use crate::io_manager::create_output_buf;
use ramemu::{program::Program, ram::Ram};
use std::path::PathBuf;

pub fn run_source(
    source: &str,
    program: Program,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
) -> Result<(), RamCliError> {
    let input_buf = create_input_buf(input)?;
    let output_buf = create_output_buf(output)?;

    let mut ram = Ram::new(program, input_buf, output_buf);

    match ram.run() {
        Ok(_) => {}
        Err(e) => {
            display_runtime_error(source, &e)?;
            return Err(RamCliError::Runtime(e.get_error_reason()));
        }
    }

    Ok(())
}

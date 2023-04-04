use clap::Parser;
use colored::Colorize;
use ram_cli::run::run_source;
use ram_cli::check::create_source;
use ram_cli::io_manager::read_source;
use ram_cli::cli::{Subcommands, Cli};

fn main() {
    let args = Cli::parse();
    match args.subcommand {
        Subcommands::Check { file } => {
            let source = match read_source(file.clone()) {
                Ok(s) => s,
                Err(e) => 
                    return println!("{}", e)
            };
            
            match create_source(&source, file) {
                Ok(_) => 
                    println!("{}: No errors found {}", "Syntax analysis".cyan().bold(), "✓".green().bold()),
                Err(e) => 
                    println!("{}", e)
            }
        }

        Subcommands::Run { file, input, output } => {
            let source = match read_source(file.clone()) {
                Ok(s) => s,
                Err(e) => 
                    return println!("{}", e)
            };
            
            let statements = match create_source(&source, file) {
                Ok(program) => program,
                Err(e) =>
                    return println!("{}", e)
            };

            match run_source(&source, statements, input, output) {
                Ok(_) =>
                    println!("{}: Program finished with no errors {}", "Runtime".cyan().bold(), "✓".green().bold()),
                Err(e) => println!("{}", e)
            }
        }
    }
}

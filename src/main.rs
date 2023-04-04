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

            let Err(e) = create_source(&source, file) else {
                return println!("{}: No errors found {}", "Syntax analysis".cyan().bold(), "✓".green().bold())
            };
            println!("{}", e);
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

            let Err(e) = run_source(&source, statements, input, output) else {
                return println!("{}: Program finished with no errors {}", "Runtime".cyan().bold(), "✓".green().bold())
            };
            println!("{}", e);
        }
    }
}

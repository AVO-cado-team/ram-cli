mod cli;
mod errors;
mod check;
mod display_error;
mod run;

use clap::Parser;
use cli::Cli;
// use errors::RamCliError;


fn main() {
    let args = Cli::parse();
    match args.subcommand {
        cli::Subcommands::Check { file } => {
            let check_result = check::check(file);
            match check_result {
                Ok(_) => {
                }
                Err(e) => {
                    println!("{}", e.message);
                }
            }
        }
        cli::Subcommands::Run { file, input, output } => {
            let check_result = check::check(file.clone());
            match check_result {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e.message);
                    return;
                }
            }
            
            run::run(file, input, output);
            
        }
    }
}

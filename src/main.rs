use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use colored::Colorize;
use ram_cli::cli::{Cli, Subcommands};
use ram_cli::create_program::create_program;
use ram_cli::io_manager::read_source;
use ram_cli::run::run_source;

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        Subcommands::Check { file } => {
            let source = match read_source(file.clone()) {
                Ok(s) => s,
                Err(e) => return println!("{}", e),
            };

            let Err(e) = create_program(&source, file) else {
                return println!("{}: No errors found {}", "Syntax analysis".cyan().bold(), "✓".green().bold())
            };
            println!("{}", e);
        }

        Subcommands::Run {
            file,
            input,
            output,
        } => {
            let source = match read_source(file.clone()) {
                Ok(s) => s,
                Err(e) => return println!("{}", e),
            };

            let program = match create_program(&source, file) {
                Ok(program) => program,
                Err(e) => return println!("{}", e),
            };

            let Err(e) = run_source(&source, program, input, output) else {
                return println!("{}: Program finished with no errors {}", "Runtime".cyan().bold(), "✓".green().bold())
            };
            println!("{}", e);
        }

        Subcommands::GenCompletion { shell } => {
            clap_complete::generate_to(shell, &mut Cli::command(), "ram-cli", "./").unwrap();
            let path = match shell {
                Shell::Bash => "./procs.bash",
                Shell::Elvish => "./procs.elv",
                Shell::Fish => "./procs.fish",
                Shell::PowerShell => "./_procs.ps1",
                Shell::Zsh => "./_procs",
                _ => {
                    println!("Unknown shell type");
                    return;
                }
            };
            println!("Completion file is generated: {path}");
        }
    }
}

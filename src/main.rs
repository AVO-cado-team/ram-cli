use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use ram_cli::cli::{Cli, Subcommands};
use ram_cli::colorize::styled_output;
use ram_cli::colorize::STL;
use ram_cli::create_program::create_program;
use ram_cli::io_manager::read_source;
use ram_cli::run::run_source;

fn main() {
    let args = Cli::parse();

    match args.subcommand {
        Subcommands::Check { file } => {
            let source = match read_source(file.clone()) {
                Ok(s) => s,
                Err(e) => {
                    styled_output(format!("{}", e).as_str(), vec![]);
                    styled_output(" ✗\n", vec![STL::Red, STL::Bold]);
                    return;
                }
            };

            let Err(e) = create_program(&source, file) else {
                styled_output("Syntax analysis: ", vec![STL::Green, STL::Bold]);
                styled_output("No errors found ", vec![STL::Normal, STL::Bold]);
                styled_output("✓\n", vec![STL::Green, STL::Bold]);
                return;
            };
            styled_output(format!("{}", e).as_str(), vec![]);
            styled_output(" ✗\n", vec![STL::Red, STL::Bold]);
        }

        Subcommands::Run {
            file,
            input,
            output,
        } => {
            let source = match read_source(file.clone()) {
                Ok(s) => s,
                Err(e) => {
                    styled_output(format!("{}", e).as_str(), vec![]);
                    styled_output(" ✗\n", vec![STL::Red, STL::Bold]);
                    return;
                }
            };

            let program = match create_program(&source, file) {
                Ok(program) => program,
                Err(e) => {
                    styled_output(format!("{}", e).as_str(), vec![]);
                    styled_output(" ✗\n", vec![STL::Red, STL::Bold]);
                    return;
                }
            };

            let Err(e) = run_source(&source, program, input, output) else {
                styled_output("Runtime: ", vec![STL::Cyan, STL::Bold]);
                styled_output("Program finished with no errors ", vec![STL::Green, STL::Bold]);
                styled_output("✓\n", vec![STL::Green, STL::Bold]);
                return;
            };
            styled_output(format!("{}\n", e).as_str(), vec![]);
        }

        Subcommands::GenCompletion { shell } => {
            clap_complete::generate_to(shell, &mut Cli::command(), "ram-cli", "./").unwrap();
            let path = match shell {
                Shell::Bash => "./ram-cli.bash",
                Shell::Elvish => "./ram-cli.elv",
                Shell::Fish => "./ram-cli.fish",
                Shell::PowerShell => "./_ram-cli.ps1",
                Shell::Zsh => "./_ram-cli",
                _ => {
                    styled_output("Unsupported shell\n", vec![]);
                    return;
                }
            };
            styled_output(
                format!("Completion file is generated: {}\n", path).as_str(),
                vec![],
            );
        }
    }
}

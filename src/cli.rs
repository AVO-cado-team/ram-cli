use clap::{Parser, Subcommand, ValueHint};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, about = "CLI for ramemu (Random Access Machine Emulator)", version, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Check {
        #[clap(value_hint(ValueHint::DirPath))]
        file: PathBuf,
    },
    Run {
        #[clap(value_hint(ValueHint::DirPath))]
        file: PathBuf,

        #[clap(value_hint(ValueHint::DirPath), short, long)]
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        input: Option<PathBuf>,

        #[clap(value_hint(ValueHint::DirPath), short, long)]
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        output: Option<PathBuf>,
    },
    GenCompletion {
        shell: Shell,
    },
}

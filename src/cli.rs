use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // Subcommand to check file
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Check {
        #[clap(value_hint(ValueHint::DirPath))]
        file: PathBuf,

        #[clap(value_hint(ValueHint::DirPath), short, long)]
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        input: Option<PathBuf>,

        #[clap(value_hint(ValueHint::DirPath), short, long)]
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        output: Option<PathBuf>,
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
    Compile {
        #[clap(value_hint(ValueHint::DirPath))]
        file: PathBuf,

        #[arg(default_value_t = false)]
        #[clap(long)]
        turing_machine: bool,
    },
    Debug {
        #[clap(value_hint(ValueHint::DirPath))]
        file: PathBuf,
    },
}

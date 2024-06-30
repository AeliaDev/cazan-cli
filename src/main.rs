use cprint::ceprintln;
use std::process::ExitCode;

mod cli;
mod config;
mod terminal;

fn main() -> ExitCode {
    let cli: cli::Cli = argh::from_env();

    if cli.version {
        println!(
            "{} version {}",
            env!("CARGO_BIN_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        return ExitCode::SUCCESS;
    }

    match cli.subcommand {
        Some(subcommand) => subcommand.run(),
        None => {
            ceprintln!(
                "Error no subcommand was given, use --help to see the available subcommands"
            );
            ExitCode::FAILURE
        }
    }
}

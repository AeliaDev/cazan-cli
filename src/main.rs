use crate::cli::Subcommand;

mod cli;

fn main() {
    let cli: cli::CLI = argh::from_env();

    match cli.subcommand {
        _ => {}
    }
}

mod cli;

fn main() {
    let cli: cli::Cli = argh::from_env();

    if cli.version {
        println!("cazan version {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    match cli.subcommand {
        None => {}
        Some(_) => {}
    }
}

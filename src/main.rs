mod cli;

fn main() {
    let cli: cli::Cli = argh::from_env();

    if cli.version {
        println!("cazan-cli version {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    match cli.subcommand {
        Some(subcommand) => {
            subcommand.run();
        }
        None => {
            println!("Error: No subcommand was used");
        }
    }
}

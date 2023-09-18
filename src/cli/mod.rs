mod init;
mod subcommands;

use argh::FromArgs;
pub use subcommands::{SubCommandEnum, SubCommandTrait};

#[derive(FromArgs, Debug)]
#[argh(
    name = "cazan",
    description = "The Command Line Tool to install to build your Cazan project"
)]
pub(crate) struct Cli {
    #[argh(switch, short = 'v', description = "print version info")]
    pub(crate) version: bool,

    #[argh(subcommand)]
    pub subcommand: Option<SubCommandEnum>,
}

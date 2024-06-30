mod init;
mod prebuild;
mod subcommands;
mod lock;

use argh::FromArgs;
pub use subcommands::{SubCommandEnum, SubCommandTrait};

#[derive(FromArgs, Debug)]
#[argh(description = "Cazan CLI")]
pub(crate) struct Cli {
    #[argh(switch, short = 'v', description = "print version info")]
    pub(crate) version: bool,

    #[argh(subcommand)]
    pub subcommand: Option<SubCommandEnum>,
}

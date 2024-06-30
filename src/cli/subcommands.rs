use std::process::ExitCode;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommandEnum {
    PreBuild(super::prebuild::PreBuild),
    Init(super::init::Init),
}

pub trait SubCommandTrait {
    fn run(&self) -> ExitCode;
}

impl SubCommandEnum {
    pub fn run(&self) -> ExitCode {
        match self {
            SubCommandEnum::PreBuild(prebuild) => prebuild.run(),
            SubCommandEnum::Init(init) => init.run(),
        }
    }
}

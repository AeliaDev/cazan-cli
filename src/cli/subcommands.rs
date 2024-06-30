use argh::FromArgs;
use std::process::ExitCode;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommandEnum {
    PreBuild(super::prebuild::PreBuild),
    Init(super::init::Init),
    Lock(super::lock::Lock),
}

pub trait SubCommandTrait {
    fn run(&self) -> ExitCode;
}

impl SubCommandEnum {
    pub fn run(&self) -> ExitCode {
        match self {
            SubCommandEnum::PreBuild(prebuild) => prebuild.run(),
            SubCommandEnum::Init(init) => init.run(),
            SubCommandEnum::Lock(lock) => lock.run(),
        }
    }
}

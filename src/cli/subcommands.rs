use super::init::Init;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommandEnum {
    Init(Init),
}

pub trait SubCommandTrait {
    fn run(&self);
}

impl SubCommandEnum {
    pub fn run(&self) {
        match self {
            SubCommandEnum::Init(init) => {
                init.run();
            }
        }
    }
}

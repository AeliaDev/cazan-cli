use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommandEnum {
    PreBuild(super::prebuild::PreBuild),
}

pub trait SubCommandTrait {
    fn run(&self);
}

impl SubCommandEnum {
    pub fn run(&self) {
        match self {
            SubCommandEnum::PreBuild(prebuild) => {
                prebuild.run();
            }
        }
    }
}

//! The `init` subcommand
//! This command initializes a new Cazan project
//! It creates a new directory named `.cazan` in the current working directory with some default files
//!
//! Optionally, if the `--re-init` flag is used, it will re-initialize an existing Cazan project.
//! This will overwrite the existing `.cazan` directory and all of its contents.

use super::SubCommandTrait;
use argh::FromArgs;
use std::fs;

#[derive(PartialEq, Debug, FromArgs)]
#[argh(
    subcommand,
    name = "init",
    description = "initialize a new Cazan project"
)]
pub struct Init {
    #[argh(
        switch,
        short = 'r',
        description = "re-initialize an existing Cazan project"
    )]
    pub re_init: bool,
}

impl SubCommandTrait for Init {
    fn run(&self) {
        if self.re_init {
            println!("Re-initializing Cazan project");
        } else {
            println!("Initializing Cazan project");
        }

        match fs::create_dir(".cazan") {
            Ok(_) => {}
            Err(_) => {
                if self.re_init {
                    fs::remove_dir_all(".cazan").expect("Failed to remove .cazan directory");
                    fs::create_dir(".cazan").expect("Failed to create .cazan directory");
                } else {
                    println!("A Cazan project already exists in this directory");
                    return;
                }
            }
        }
        fs::write(
            ".cazan/cazan.json",
            r#"{
  "assets-dir": {
      "input": "assets",
      "output": ".cazan/assets"
  }
}"#,
        )
        .expect("Failed to create .cazan/cazan.json file");
        fs::create_dir(".cazan/assets").expect("Failed to create .cazan/assets directory");
    }
}

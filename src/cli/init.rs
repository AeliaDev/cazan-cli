use crate::cli::SubCommandTrait;
use crate::config::Config;
use argh::FromArgs;
use cprint::{ceprintln, cprintln};
use semver::Version;
use std::process::ExitCode;
use std::{env, fs};

#[derive(PartialEq, Debug, FromArgs)]
#[argh(subcommand, name = "init", description = "Init your Cazan project")]
pub struct Init {
    #[argh(
        switch,
        short = 'f',
        description = "force re-initialization of your project"
    )]
    pub force: bool,
}

impl SubCommandTrait for Init {
    fn run(&self) -> ExitCode {
        let current_dir = env::current_dir().unwrap();
        let dir_name = &current_dir.file_name().unwrap().to_str().unwrap();
        let config = Config {
            name: dir_name,
            version: Version::new(0, 0, 1),
            authors: Vec::new(),
            use_autoplay_for_multimedia: None,
            rdp_epsilon: None,
            assets: Some(vec![]),
            plugins: Some(vec![]),
        };

        let serialized_config = serde_json::to_string_pretty(&config).unwrap();

        // Create .cazan dir
        let dot_cazan_dir = current_dir.join(".cazan");

        if self.force
            && dot_cazan_dir.exists()
            && fs::remove_dir_all(dot_cazan_dir.clone()).is_err()
        {
            ceprintln!("Error recreating .cazan directory");
            return ExitCode::FAILURE;
        }

        if fs::create_dir(dot_cazan_dir.clone()).is_err() {
            let message = if dot_cazan_dir.exists() {
                "Error creating .cazan directory: Already existing (try using --force)"
            } else {
                &*format!(
                    "Error {}creating .cazan directory",
                    if self.force && dot_cazan_dir.exists() {
                        "-re"
                    } else {
                        ""
                    }
                )
            };
            ceprintln!(message);
            return ExitCode::FAILURE;
        }

        // Create cazan.json file
        let config_file = current_dir.join("cazan.json");

        // If there's already a config file but the "--force" is not used
        if config_file.exists() && !self.force {
            return ExitCode::SUCCESS;
        }

        if fs::write(config_file, serialized_config).is_err() {
            let message = format!(
                "Error {}creating cazan.json with default config",
                if self.force { "re-" } else { "" }
            );
            ceprintln!(message);
            return ExitCode::FAILURE;
        }

        cprintln!("Initialized cazan project");
        ExitCode::SUCCESS
    }
}

use std::process::ExitCode;
use std::fs;
use std::env;
use argh::FromArgs;
use cprint::{ceprintln, cprintln};
use crate::cli::SubCommandTrait;
use crate::config::Config;

#[derive(PartialEq, Debug, FromArgs)]
#[argh(subcommand, name = "lock", description = "Init your Cazan project")]
pub struct Lock {
    #[argh(
        switch,
        short = 'f',
        description = "force locking the cazan.json file, without checking if the JSON is valid"
    )]
    pub force: bool,

    #[argh(
        switch,
        short = 'u',
        description = "allow unknown field to locked file"
    )]
    pub allow_unknown: bool,
}

impl SubCommandTrait for Lock {
    fn run(&self) -> ExitCode {
        let cazan_json = env::current_dir().unwrap().join("cazan.json");
        let cazan_directory = env::current_dir().unwrap().join(".cazan");
        let locked_config_json = cazan_directory.join("config.json");

        if !cazan_json.exists() || !cazan_directory.exists() {
            ceprintln!("Error current directory is not initialized for Cazan (use `cazan init`)");
            return ExitCode::FAILURE;
        }


        if self.force {
            if fs::copy(cazan_json.clone(), locked_config_json).is_err() {
                ceprintln!("Error copying cazan.json file to .cazan/config.json");
                return ExitCode::FAILURE;
            }
            cprintln!("Locked config");
            return ExitCode::SUCCESS;
        }

        let config = match fs::read_to_string(cazan_json.clone()) {
            Ok(config) => config,
            Err(_) => {
                ceprintln!("Error reading cazan.json file");
                return ExitCode::FAILURE;
            }
        };

        let config = config.as_str();
        let deserializer = &mut serde_json::Deserializer::from_str(config);
        let mut unused: Vec<String> = vec![];

        let config: Config = match serde_ignored::deserialize(deserializer, |field| { unused.push(field.to_string()) }) {
            Ok(config) => config,
            Err(_) => { ceprintln!("Error cazan.json is invalid"); return ExitCode::FAILURE; }
        };

        if self.allow_unknown {
            if fs::copy(cazan_json.clone(), locked_config_json).is_err() {
                ceprintln!("Error copying cazan.json file to .cazan/config.json");
                return ExitCode::FAILURE;
            }
            cprintln!("Locked config");
            return ExitCode::SUCCESS;
        }

        if !unused.is_empty() {
            let warning = format!("Warning unknown fields ({}) are ignored. To force using them, use --allow-unknown", unused.iter().map(|field| format!("`{}`", field)).collect::<Vec<_>>().join(", "));
            cprintln!(warning => Yellow)
        }

        let config = serde_json::to_string_pretty(&config).unwrap();

        if fs::write(locked_config_json, config).is_err() {
            ceprintln!("Error copying cazan.json to .cazan/config.json");
            return ExitCode::FAILURE;
        }

        cprintln!("Locked config");

        ExitCode::SUCCESS
    }
}
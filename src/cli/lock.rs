use crate::cli::SubCommandTrait;
use crate::config::{checksum, Config};
use argh::FromArgs;
use cprint::{ceprintln, cprintln};
use std::env;
use std::fs;
use std::process::ExitCode;

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
            ceprintln!("Error cazan is not initialized for this directory");
            return ExitCode::FAILURE;
        }

        let config = match fs::read_to_string(cazan_json.clone()) {
            Ok(config) => config,
            Err(_) => {
                ceprintln!("Error reading cazan.json file");
                return ExitCode::FAILURE;
            }
        };

        let config_string = config.as_str();

        let new_checksum = match checksum(&cazan_json) {
            Ok(checksum) => checksum,
            Err(_) => {
                ceprintln!("Error calculating checksum of cazan.json");
                return ExitCode::FAILURE;
            }
        };

        let checksum_file = cazan_directory.join("checksum.txt");

        let old_checksum = fs::read_to_string(checksum_file.clone()).unwrap_or_default();

        if old_checksum.is_empty() && checksum_file.exists() {
            ceprintln!("Error reading checksum file");
            return ExitCode::FAILURE;
        }

        if self.force {
            if fs::copy(cazan_json.clone(), locked_config_json).is_err() {
                ceprintln!("Error copying cazan.json file to .cazan/config.json");
                return ExitCode::FAILURE;
            }

            if fs::write(checksum_file, new_checksum).is_err() {
                ceprintln!("Error saving checksum");
                return ExitCode::FAILURE;
            }

            cprintln!("Locked config");
            return ExitCode::SUCCESS;
        }

        if old_checksum == new_checksum {
            cprintln!("Already up-to-date");
            return ExitCode::SUCCESS;
        }

        let deserializer = &mut serde_json::Deserializer::from_str(config_string);
        let mut unused: Vec<String> = vec![];

        let config: Config = match serde_ignored::deserialize(deserializer, |field| {
            unused.push(field.to_string())
        }) {
            Ok(config) => config,
            Err(_) => {
                ceprintln!("Error cazan.json is invalid");
                return ExitCode::FAILURE;
            }
        };

        if self.allow_unknown {
            if fs::copy(cazan_json.clone(), locked_config_json).is_err() {
                ceprintln!("Error copying cazan.json file to .cazan/config.json");
                return ExitCode::FAILURE;
            }

            if fs::write(checksum_file, new_checksum).is_err() {
                ceprintln!("Error saving checksum");
                return ExitCode::FAILURE;
            }

            cprintln!("Locked config");
            return ExitCode::SUCCESS;
        }

        if !unused.is_empty() {
            let warning = format!(
                "Warning unknown fields ({}) are ignored. To force using them, use --allow-unknown",
                unused
                    .iter()
                    .map(|field| format!("`{}`", field))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            cprintln!(warning => Yellow);
        }

        let config = serde_json::to_string_pretty(&config).unwrap();

        if fs::write(locked_config_json, config).is_err() {
            ceprintln!("Error copying cazan.json to .cazan/config.json");
            return ExitCode::FAILURE;
        }

        if fs::write(checksum_file, new_checksum).is_err() {
            ceprintln!("Error saving checksum");
            return ExitCode::FAILURE;
        }

        cprintln!("Locked config");

        ExitCode::SUCCESS
    }
}

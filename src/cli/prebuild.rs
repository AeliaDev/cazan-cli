//! The `prebuild` subcommand
//! This command is used to prebuild the assets of your project
//! It builds the PNG assets by reading PNG files, extracting the edges, simplifying the edges, and writing the edges to a JSON file

use std::fs;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use super::SubCommandTrait;
use crate::config::checksum;
use crate::terminal::SubTerminal;
use argh::FromArgs;
use cazan_common::rdp::rdp;
use cazan_common::{image::ImageEdgesParser, triangulation::triangulate};
use cprint::{ceprintln, cformat, cprintln};
use serde_json::{json, Value};

#[derive(PartialEq, Debug, FromArgs)]
#[argh(
    subcommand,
    name = "prebuild",
    description = "pre-build the assets of your project"
)]
pub struct PreBuild {
    #[argh(option, short = 'a', description = "asset directories")]
    pub asset_dirs: Vec<String>,

    #[argh(
        option,
        short = 'e',
        description = "epsilon value for the Ramer-Douglas-Peucker algorithm (Image simplification)",
        default = "3.0"
    )]
    pub epsilon: f64,
}

fn read_dir_recursive(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files.extend(read_dir_recursive(&path));
        } else {
            files.push(path);
        }
    }
    files
}

impl SubCommandTrait for PreBuild {
    fn run(&self) -> ExitCode {
        let current_dir = std::env::current_dir().unwrap();
        let cazan_directory = current_dir.join(".cazan");
        if !cazan_directory.exists() {
            ceprintln!("Error cazan is not initialized for this directory");
            return ExitCode::FAILURE;
        }

        let cazan_config = current_dir.join("cazan.json");
        let checksum_file = current_dir.join(".cazan/checksum.txt");

        if checksum(&cazan_config).unwrap() != fs::read_to_string(checksum_file).unwrap_or_default()
        {
            cprintln!("Warning lock file is not up-to-date with cazan.json. To update it use `cazan lock`" => Yellow);
        }

        let files = self
            .asset_dirs
            .iter()
            .flat_map(|dir| read_dir_recursive(dir.as_ref()))
            .collect::<Vec<std::path::PathBuf>>();

        let files: Vec<std::path::PathBuf> = files
            .iter()
            .filter(|file| {
                file.extension()
                    .map_or(false, |ext| ext == "png" || ext == "jpg" || ext == "jpeg")
            })
            .cloned()
            .collect();

        let mut map = serde_json::Map::<String, Value>::new();
        let terminal: Arc<Mutex<SubTerminal>> =
            Arc::new(Mutex::new(SubTerminal::new(files.len() as u16)));

        let handles: Vec<_> = files
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let epsilon = self.epsilon;
                let file = file.clone();
                let terminal = terminal.clone();
                std::thread::spawn(move || {
                    terminal.lock().unwrap().write_to(
                        cformat!("Parsing", file.to_str().unwrap() => Cyan).as_str(),
                        i,
                    );

                    let image = image::open(&file).unwrap();
                    let edges_parser = ImageEdgesParser::new(image);
                    let polygon = edges_parser.as_polygon();
                    let rdp_polygon = rdp(&polygon, epsilon);
                    let triangles = triangulate(&rdp_polygon).expect("Error triangulating");

                    terminal.lock().unwrap().rewrite_to(
                        cformat!(
                            "Parsed",
                            format!(
                                "`{}` to {} triangles",
                                file.file_name().unwrap().to_str().unwrap().to_string(),
                                triangles.len()
                            )
                        )
                        .as_ref(),
                        i,
                    );

                    (file.to_str().unwrap().to_string(), json!(triangles))
                })
            })
            .collect();

        for handle in handles {
            let (file, triangles) = handle.join().unwrap();
            map.insert(file, triangles);
        }

        let cazan_build_directory = cazan_directory.join("build");

        if !cazan_build_directory.exists() && fs::create_dir(cazan_build_directory.clone()).is_err()
        {
            ceprintln!("Error creating `.cazan/build` directory")
        }
        terminal.lock().unwrap().move_to_last_line_and_new_line();

        let mut writer = fs::File::create(cazan_build_directory.join("assets.json")).unwrap();
        serde_json::to_writer(&mut writer, &map).unwrap();

        ExitCode::SUCCESS
    }
}

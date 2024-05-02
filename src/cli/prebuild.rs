//! The `prebuild` subcommand
//! This command is used to prebuild the assets of your project
//! It builds the PNG assets by reading PNG files, extracting the edges, simplifying the edges, and writing the edges to a JSON file

use std::fs;
use std::io::Write;

use argh::FromArgs;
use cprint::{Color, cprint, cprintln};
use serde_json::{json, Value};

use super::SubCommandTrait;

use cazan_common::rdp::rdp;
use cazan_common::{image::ImageEdgesParser, triangulation::triangulate};

#[derive(PartialEq, Debug, FromArgs)]
#[argh(
subcommand,
name = "prebuild",
description = "pre-build the assets of your project"
)]
pub struct PreBuild {
    #[argh(
    option,
    short = 'o',
    description = "output file",
    default = "String::from(\"cazan-assets.json\")"
    )]
    pub output: String,

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
    fn run(&self) {
        let cwd = std::env::current_dir().unwrap();

        let files = read_dir_recursive(&cwd);

        // Filter PNG and JPEG files
        let png_files: Vec<std::path::PathBuf> = files
            .iter()
            .filter(|file| {
                file.extension()
                    .map_or(false, |ext| ext == "png" || ext == "jpg" || ext == "jpeg")
            })
            .cloned()
            .collect();

        let mut map = serde_json::Map::<String, Value>::new();

        for file in png_files {
            cprint!("Parsing", format!("`{}`...\r",file.file_name().unwrap().to_str().unwrap().to_string()), Color::Cyan);
            std::io::stdout().flush().unwrap();

            let image = image::open(&file).unwrap();
            let edges_parser = ImageEdgesParser::new(image);
            let polygon = edges_parser.as_polygon();
            let rdp_polygon = rdp(&polygon, self.epsilon);
            let triangles = triangulate(&rdp_polygon).expect("Error triangulating");


            map.insert(file.to_str().unwrap().to_string(), json!(triangles));
            cprintln!("Parsed", format!("`{}` to {} triangles", file.file_name().unwrap().to_str().unwrap().to_string(), triangles.len()), Color::Green);
        }

        let mut writer = fs::File::create(&self.output).unwrap();
        serde_json::to_writer(&mut writer, &map).unwrap();
    }
}

//! The `prebuild` subcommand
//! This command is used to prebuild the assets of your project
//! It builds the PNG assets by reading PNG files, extracting the edges, simplifying the edges, and writing the edges to a JSON file

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use super::SubCommandTrait;
use crate::config::{checksum, Config};
use crate::terminal::SubTerminal;

use cazan_common::geometry::Triangle;
use cazan_common::rdp::rdp;
use cazan_common::{image::ImageEdgesParser, triangulation::triangulate};

use argh::FromArgs;
use cprint::{ceprintln, cformat, cprintln};
use glob::glob;
use image::GenericImageView;
use plotters::prelude::*;
use serde_json::{json, Value};

const DEFAULT_EPSILON: f64 = 3.0;

#[derive(PartialEq, Debug, FromArgs)]
#[argh(
    subcommand,
    name = "prebuild",
    description = "pre-build the assets of your project"
)]
pub struct PreBuild {
    #[argh(
        option,
        short = 'a',
        description = "the assets of your project (ex: assets/sprite-*.png"
    )]
    pub assets: Vec<String>,

    #[argh(
        option,
        short = 'e',
        description = "epsilon value for the Ramer-Douglas-Peucker algorithm (Image simplification)"
    )]
    pub epsilon: Option<f64>,

    #[argh(
        switch,
        short = 'p',
        description = "create preview files to compare hit-boxes with the original images"
    )]
    pub preview: bool,

    #[argh(
        switch,
        description = "open the folder to the preview files (do not use without --prebuild)"
    )]
    pub open: bool,
}

impl SubCommandTrait for PreBuild {
    fn run(&self) -> ExitCode {
        if self.open && !self.preview {
            cprintln!("Warning use of `--open` without `--preview` is useless" => Yellow);
        }

        let current_dir = std::env::current_dir().unwrap();
        let cazan_directory = current_dir.join(".cazan");
        if !cazan_directory.exists() {
            ceprintln!("Error cazan is not initialized for this directory");
            return ExitCode::FAILURE;
        }

        let cazan_config = current_dir.join("cazan.json");
        let checksum_file = current_dir.join(".cazan/checksum.txt");
        let config = fs::read_to_string(current_dir.join(".cazan/config.json")).unwrap();
        let config: Config = serde_json::from_str(config.as_str()).unwrap();

        if checksum(&cazan_config).unwrap() != fs::read_to_string(checksum_file).unwrap_or_default()
        {
            cprintln!("Warning lock file is not up-to-date with cazan.json. To update it use `cazan lock`" => Yellow);
        }

        let assets = if self.assets.is_empty() {
            config.assets.unwrap_or_default()
        } else {
            self.assets.iter().map(|s| s.as_str()).collect()
        };

        let files: Vec<PathBuf> = assets
            .iter()
            .flat_map(|pattern| glob(pattern).expect("Failed to read pattern"))
            .map(|entry| entry.unwrap_or_else(|_| PathBuf::new()))
            .filter(|file| file.extension().map_or(false, |ext| ext == "png"))
            .collect();

        if files.is_empty() {
            return ExitCode::SUCCESS;
        }

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
                    let rdp_polygon = rdp(
                        &polygon,
                        epsilon.unwrap_or(config.rdp_epsilon.unwrap_or(DEFAULT_EPSILON)),
                    );
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

                    (file, triangles)
                })
            })
            .collect();

        let cazan_tmp: PathBuf = if self.preview {
            let c = std::env::current_dir().unwrap().join(".cazan-tmp");
            if !c.exists() {
                fs::create_dir(&c).expect("Error creating the cazan temp directory");
            }
            c
        } else {
            PathBuf::new()
        };

        let mut warnings: Vec<String> = vec![];

        for handle in handles {
            let (file, triangles) = handle.join().unwrap();

            if self.preview && file.extension() == Some("png".as_ref()) {
                match preview(&file, &triangles, &cazan_tmp) {
                    Ok(_) => {}
                    Err(e) => warnings.push(format!(
                        "Warning `{}` preview couldn't have been created: {e}",
                        file.to_str().unwrap()
                    )),
                }
            }

            map.insert(checksum(&file).unwrap().to_string(), json!(triangles));
        }

        terminal.lock().unwrap().move_to_last_line_and_new_line();

        for warning in warnings {
            cprintln!(warning => Yellow);
        }

        let cazan_build_directory = cazan_directory.join("build");

        if !cazan_build_directory.exists() && fs::create_dir(cazan_build_directory.clone()).is_err()
        {
            ceprintln!("Error creating `.cazan/build` directory")
        }

        let mut writer = fs::File::create(cazan_build_directory.join("assets.json")).unwrap();
        serde_json::to_writer(&mut writer, &map).unwrap();

        if self.preview && self.open {
            open::that(cazan_tmp).expect("Couldn't open file explorer");
        }
        ExitCode::SUCCESS
    }
}

fn preview(
    file: &PathBuf,
    triangles: &Vec<Triangle>,
    cazan_tmp: &Path,
) -> Result<(), Box<dyn Error>> {
    let image = image::open(file)?;
    let save_path = cazan_tmp.join(
        file.file_stem().unwrap().to_str().unwrap().to_owned()
            + "-"
            + &checksum(file)?[0..5]
            + "."
            + file.extension().unwrap().to_str().unwrap(),
    );

    let root = BitMapBackend::new(&save_path, image.dimensions()).into_drawing_area();
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0..image.dimensions().0, image.dimensions().1..0)?;

    let rgb = image
        .to_rgba8()
        .chunks(4)
        .flat_map(|rgba| {
            let (r, g, b, a) = (
                rgba[0] as i32,
                rgba[1] as i32,
                rgba[2] as i32,
                rgba[3] as f64 / 255.,
            );

            vec![
                ((1. - a) * 255. + a * r as f64) as u8,
                ((1. - a) * 255. + a * g as f64) as u8,
                ((1. - a) * 255. + a * b as f64) as u8,
            ]
        })
        .collect::<Vec<u8>>();

    let elem = BitMapElement::with_owned_buffer((0, 0), image.dimensions(), rgb).unwrap();

    root.draw(&elem)?;

    let line_style = ShapeStyle {
        color: RED.mix(0.6),
        filled: true,
        stroke_width: 2,
    };

    for triangle in triangles {
        let triangle = &vec![
            (triangle.0.x as i32, triangle.0.y as i32),
            (triangle.1.x as i32, triangle.1.y as i32),
            (triangle.2.x as i32, triangle.2.y as i32),
        ]; // TODO: Simplify this by implementing Into<Vec<(T,T)>> for Triangle in cazan common

        root.draw(&Polygon::new(triangle.clone(), RED.mix(0.3)))?;
        chart
            .draw_series(LineSeries::new(
                triangle
                    .iter()
                    .chain(std::iter::once(&triangle[0]))
                    .map(|&(x, y)| (x as u32, y as u32)),
                line_style,
            ))
            .unwrap();
    }

    root.present()?;

    Ok(())
}

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::{arg, value_parser, Parser};
use image::DynamicImage;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn, LevelFilter};
use rayon::prelude::*;

use jmfm::palette::MapPalette;
use jmfm::structs::{IdCounts, Map, MapData};
use jmfm::{cut_into_maps, index_colors, read_nbt, write_nbt};

use crate::config::Config;

mod config;

#[derive(Parser)]
#[command(disable_help_flag(true))]
#[command(arg_required_else_help = true)]
#[command(about = "Blazingly fast conversion of images into Minecraft maps", long_about = None)]
struct Args {
    /// By default, the width is chosen automatically
    #[arg(long, short, num_args(0..=1), value_parser(value_parser!(u32).range(1..)))]
    width: Option<u32>,
    /// By default, the height is chosen automatically
    #[arg(long, short, num_args(0..=1), value_parser(value_parser!(u32).range(1..)))]
    height: Option<u32>,
    #[arg(long("first-map-id"), short('i'), value_name = "FIRST-MAP-ID")]
    /// ID of the first map in the order
    first_map_id: Option<i32>,
    /// Root directory of the target world
    #[arg(required = true)]
    output: PathBuf,
    /// Paths to image files
    #[arg(required = true)]
    images: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter(None, LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let config = confy::load::<Config>("jmfm", None)?;

    let arguments = Args::parse();
    let output = arguments.output.join("data");
    if !output.exists() {
        warn!(
            "{:?} - No such directory. It will be created automatically",
            output
        );
        fs::create_dir(&output)?
    };

    let image_paths = arguments.images;
    let width = arguments.width.unwrap_or(1);
    let height = arguments.height.unwrap_or(1);

    info!("Reading image files");
    let images: Vec<DynamicImage> = image_paths
        .par_iter()
        .map(|path| (path, image::open(path)))
        .filter_map(|(p, i)| match i {
            Ok(image) => Some(image),
            Err(err) => {
                error!("{p:?} - {err}");
                None
            }
        })
        .collect();

    let total_count = width * height * images.len() as u32;

    let id_counts = match read_nbt::<IdCounts, _>(&output) {
        Ok(dat) => dat,
        Err(err) => {
            warn!("Unable to read 'idcounts.dat': {err}");
            IdCounts::new(
                config.data_version,
                arguments.first_map_id.unwrap_or(0) + total_count as i32,
            )
        }
    };

    let data_version = id_counts.data_version;
    let first_map_id = id_counts.data.map;

    let palette = MapPalette::new(&config.colors, &config.multipliers);
    info!("{} image(s) -> {} map(s)", images.len(), total_count);
    info!("The first map id will be {first_map_id}");
    info!("Processing...");
    let maps: Vec<(String, Map)> = images
        .into_par_iter()
        .map(|image| cut_into_maps(image, width, height))
        .flat_map(|sheet| sheet.into_par_iter().enumerate())
        .map(|(idx, piece)| (first_map_id + idx as i32, piece.to_rgb8()))
        .map(|(idx, piece)| (format!("map_{idx}.dat"), index_colors(piece, &palette)))
        .map(|(name, colors)| (name, MapData::new(&colors)))
        .map(|(name, data)| (name, Map::new(data_version, data)))
        .collect();

    let pb = ProgressBar::new(image_paths.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );

    info!("Saving maps to {output:?}");
    maps.par_iter().for_each(|(name, map)| {
        pb.inc(1);
        write_nbt(output.join(name), map).unwrap_or_else(|err| {
            error!("Unable to write {name}: {err}");
        });
    });

    write_nbt(output.join("idcounts.dat"), &id_counts).unwrap_or_else(|err| {
        error!("Unable to write \"idcounts.dat\": {err}");
    });

    pb.finish_and_clear();

    info!(
        "Total: {:.1} secs | AVG Speed: {:.1} maps/sec",
        pb.elapsed().as_secs_f32(),
        total_count as f32 / pb.elapsed().as_secs_f32()
    );
    Ok(())
}

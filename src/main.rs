use jmfm::id_count::{self, IdCount};
use log::{error, info, warn, LevelFilter};

use std::fs::{File, OpenOptions};
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process;

use clap::{arg, value_parser, Parser};
use image::DynamicImage;
use indicatif::{ProgressBar, ProgressStyle};
use jmfm::map::{Data, MapEntry};
use rayon::prelude::*;

use jmfm::palette::MapPalette;
use jmfm::DynamicImageMethods;

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
    first_map_id: Option<usize>,
    /// Root directory of the target world
    #[arg(required = true)]
    world: PathBuf,
    /// Paths to image files
    #[arg(required = true)]
    images: Vec<PathBuf>,
}

fn main() {
    env_logger::builder()
        .filter(None, LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let config = confy::load::<Config>("jmfm", None).unwrap_or_else(|err| {
        error!("Unable to load config: {}", err);
        process::exit(1);
    });

    let arguments = Args::parse();
    let world = Path::new(&arguments.world);
    let path_iter = arguments.images.into_par_iter();
    let width = arguments.width.unwrap_or(1) as usize;
    let height = arguments.height.unwrap_or(1) as usize;
    let first_map_id = arguments.first_map_id.unwrap_or_else(|| {
        last_map_id(world).unwrap_or_else(|err| {
            warn!("Unable to read idcounts.dat - {}", err);
            0
        })
    });

    let output = if world.join("data").exists() {
        world.join("data")
    } else {
        world.to_path_buf()
    };

    if !world.exists() {
        error!("{:?} - No such directory", world);
        process::exit(1);
    }

    info!("Reading image files");

    let images = path_iter
        .enumerate()
        .map(|(i, path)| {
            image::open(&path)
                .map(|image| (i, image))
                .unwrap_or_else(|err| {
                    error!("{} - {}", path.to_str().unwrap(), err);
                    process::exit(1);
                })
        })
        .collect::<Vec<(usize, DynamicImage)>>();

    let total_count = width * height * images.len();
    let palette = MapPalette {
        multipliers: config.multipliers.clone(),
        colors: config.colors(),
    };

    info!("The first map id will be {}", first_map_id);
    info!("{} image(s) -> {} map(s)", images.len(), total_count);

    let pb = ProgressBar::new(total_count as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len}")
            .unwrap()
            .progress_chars("#-"),
    );

    images
        .into_par_iter()
        .map(|(i, image)| (i, image.into_map_sheet(width as u32, height as u32)))
        .flat_map(|(i, s)| s.into_par_iter().enumerate().map(move |(j, m)| (i, j, m)))
        .map(|(i, j, s)| (first_map_id + i * width * height + j, s))
        .map(|(idx, s)| (format!("map_{}.dat", idx), s.into_map_colors(&palette)))
        .map(|(name, colors)| (name, Data::from(colors)))
        .map(|(name, data)| (name, MapEntry::new(3337, data)))
        .for_each(|(name, map)| {
            let mut l = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(output.join(name))
                .unwrap();
            nbt::to_gzip_writer(&mut l, &map, None).unwrap();
            pb.inc(1)
        });

    pb.finish_and_clear();

    info!(
        "AVG Speed: {:.1} maps/sec",
        total_count as f32 / pb.elapsed().as_secs_f32()
    );

    let id_count = IdCount {
        data_version: 3337,
        data: id_count::Data {
            map: (first_map_id + total_count) as i32,
        },
    };

    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .open(world.join("data").join("idcounts.dat"))
    {
        nbt::to_gzip_writer(&mut file, &id_count, None).unwrap();
    }
}

fn last_map_id<P>(path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().join("data").join("idcounts.dat");
    let file = File::open(path)?;

    let blob = nbt::from_gzip_reader::<File, IdCount>(file)?;
    Ok(blob.data.map as usize)
}

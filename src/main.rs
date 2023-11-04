use std::path::PathBuf;
use std::process;

use clap::{arg, value_parser, Parser};
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info, warn, LevelFilter};
use rayon::prelude::*;

use jmfm::id_counts::{self, IdCounts};
use jmfm::map::{Data, MapEntry};
use jmfm::palette::MapPalette;
use jmfm::{read_nbt, write_nbt, DynamicImageMethods};

use crate::config::Config;

mod config;

#[derive(Parser)]
#[command(disable_help_flag(true))]
#[command(arg_required_else_help = true)]
#[command(about = "Blazingly fast conversion of images into Minecraft maps", long_about = None)]
struct Args {
    /// By default, the width is chosen automatically
    #[arg(long, short, num_args(0..=1), value_parser(value_parser!(i32).range(1..)))]
    width: Option<i32>,
    /// By default, the height is chosen automatically
    #[arg(long, short, num_args(0..=1), value_parser(value_parser!(i32).range(1..)))]
    height: Option<i32>,
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
    let output = arguments.output;
    let output = if output.join("data").exists() {
        output.join("data")
    } else {
        warn!("{:?} - No such directory", output.join("data"));
        output
    };

    let image_paths = arguments.images;
    let width = arguments.width.unwrap_or(1);
    let height = arguments.height.unwrap_or(1);

    info!("Reading image files");
    let pb = ProgressBar::new(image_paths.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar}] {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );

    let image_iter = image_paths.par_iter().enumerate().map(|(i, path)| {
        let result = image::open(path)
            .map(|image| (i, image))
            .unwrap_or_else(|err| {
                error!("{} - {}", path.to_str().unwrap(), err);
                process::exit(1);
            });
        pb.inc(1);
        result
    });

    pb.finish_and_clear();
    let total_count = width * height * image_iter.len() as i32;
    let id_counts = read_nbt::<IdCounts, &PathBuf>(&output).ok();
    let id_counts = IdCounts {
        data_version: id_counts
            .as_ref()
            .map(|d| d.data_version)
            .unwrap_or_else(|| config.data_version()),
        data: id_counts::Data {
            map: arguments
                .first_map_id
                .map(|f| f + total_count)
                .unwrap_or_else(|| {
                    id_counts.map(|d| d.data.map).unwrap_or_else(|| {
                        warn!("Unable to read 'idcounts.dat'");
                        0
                    })
                }),
        },
    };

    let first_map_id = id_counts.data.map;

    let palette = MapPalette::new(&config.colors(), &config.multipliers());
    info!("The first map id will be {}", first_map_id);
    info!("{} image(s) -> {} map(s)", image_iter.len(), total_count);

    image_iter
        .map(|(i, image)| (i, image.into_map_sheet(width, height)))
        .flat_map(|(i, s)| s.into_par_iter().enumerate().map(move |(j, m)| (i, j, m)))
        .map(|(i, j, s)| (first_map_id + i as i32 * width * height + j as i32, s))
        .map(|(idx, s)| (format!("map_{}.dat", idx), s.into_map_colors(&palette)))
        .map(|(name, colors)| (name, Data::from(colors)))
        .map(|(name, data)| (name, MapEntry::new(config.data_version(), data)))
        .for_each(|(name, map)| {
            pb.inc(1);
            write_nbt(output.join(&name), &map).unwrap_or_else(|e| {
                error!("{} - {}", name, e);
            });
        });

    write_nbt(output.join("idcounts.dat"), &id_counts).unwrap_or_else(|e| {
        error!("Unable to write 'idcounts.dat' - {}", e.to_string());
    });

    pb.finish_and_clear();

    info!(
        "Total: {:.1} secs | AVG Speed: {:.1} maps/sec",
        pb.elapsed().as_secs_f32(),
        total_count as f32 / pb.elapsed().as_secs_f32()
    );
}

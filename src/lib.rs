use std::fs::{File, OpenOptions};
use std::path::Path;

use image::{DynamicImage, GenericImage, imageops::FilterType::Lanczos3, Rgb};
use serde::{de, ser};

use crate::palette::MapPalette;

pub mod id_counts;
pub mod map;
pub mod palette;

pub trait ColorDistance {
    fn dist(&self, other: &Self) -> usize;
}

impl ColorDistance for Rgb<u8> {
    fn dist(&self, other: &Self) -> usize {
        let m = self.0[0] as isize + other.0[0] as isize;
        let r = self.0[0] as isize - other.0[0] as isize;
        let g = self.0[1] as isize - other.0[1] as isize;
        let b = self.0[2] as isize - other.0[2] as isize;
        ((((1024 + m) * r * r + (1534 - m) * b * b) >> 9) + 4 * g * g).unsigned_abs()
    }
}

pub trait DynamicImageMethods {
    /// allows you to cut the image into equal square parts of 128x128 size
    fn into_map_sheet(self, width: i32, height: i32) -> Vec<DynamicImage>;
    /// Quantize the image according to the specified [MapPalette](MapPalette)
    fn into_map_colors(self, palette: &MapPalette) -> Vec<i8>;
}

impl DynamicImageMethods for DynamicImage {
    fn into_map_sheet(self, width: i32, height: i32) -> Vec<DynamicImage> {
        let (width, height) = (width * 128, height * 128);
        let mut image = self.resize_exact(width as u32, height as u32, Lanczos3);
        let i = (0..height)
            .step_by(128)
            .flat_map(|y| (0..width).step_by(128).map(move |x| (x, y)))
            .map(|(x, y)| image.sub_image(x as u32, y as u32, 128, 128).to_image())
            .map(DynamicImage::from)
            .collect();
        i
    }

    fn into_map_colors(self, palette: &MapPalette) -> Vec<i8> {
        let mut target = self.to_rgb8();
        image::imageops::dither(&mut target, palette);
        image::imageops::index_colors(&target, palette)
            .iter()
            .map(|&b| b as i8)
            .collect()
    }
}

pub fn read_nbt<T, P>(path: P) -> nbt::Result<T>
where
    T: de::DeserializeOwned,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    nbt::from_gzip_reader::<File, T>(file)
}

pub fn write_nbt<T, P>(path: P, nbt: &T) -> nbt::Result<()>
where
    T: ?Sized + ser::Serialize,
    P: AsRef<Path>,
{
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    nbt::to_gzip_writer(&mut file, &nbt, None)
}

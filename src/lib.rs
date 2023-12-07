use std::fs::{File, OpenOptions};
use std::path::Path;

use image::imageops::ColorMap;
use image::{imageops::FilterType::Lanczos3, DynamicImage, GenericImage, ImageBuffer, Pixel};
use serde::{de, ser};

pub mod palette;
pub mod structs;

/// A trait for calculating distance between colors
pub trait ColorDistance {
    fn dist(&self, other: &Self) -> isize;
}

impl ColorDistance for [u8] {
    /// Calculates the distance between two colors
    ///
    /// # Arguments
    ///
    /// * `self` - The first color
    /// * `other` - The second color
    ///
    /// # Examples
    ///
    /// ```
    /// use jmfm::ColorDistance;
    /// let color1: [u8; 3] = [255, 0, 0];
    /// let color2: [u8; 3] = [0, 255, 0];
    /// assert_eq!(color1.dist(&color2), 195075);
    /// ```
    fn dist(&self, other: &Self) -> isize {
        let m = self[0] as isize + other[0] as isize;
        let r = (self[0] as isize - other[0] as isize).pow(2);
        let g = (self[1] as isize - other[1] as isize).pow(2);
        let b = (self[2] as isize - other[2] as isize).pow(2);
        (((512 + m) * r) / 256) + 4 * g + (((767 - m) * b) / 256)
    }
}

pub fn cut_into_maps(image: DynamicImage, width: u32, height: u32) -> Vec<DynamicImage> {
    let (width, height) = (width * 128, height * 128);
    let mut image = image.resize_exact(width, height, Lanczos3);
    (0..height)
        .step_by(128)
        .flat_map(|y| (0..width).step_by(128).map(move |x| (x, y)))
        .map(|(x, y)| image.sub_image(x, y, 128, 128).to_image())
        .map(DynamicImage::from)
        .collect()
}

pub fn index_colors<Pix, Map>(mut image: ImageBuffer<Pix, Vec<u8>>, palette: &Map) -> Vec<i8>
where
    Map: ColorMap<Color = Pix> + ?Sized,
    Pix: Pixel<Subpixel = u8> + 'static,
{
    image::imageops::dither(&mut image, palette);
    image::imageops::index_colors(&image, palette)
        .iter()
        .map(|&b| b as i8)
        .collect()
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

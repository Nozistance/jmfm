use crate::palette::MapPalette;
use image::{imageops::FilterType::Lanczos3, DynamicImage, GenericImage};

pub mod map;
pub mod id_count;
pub mod palette;

pub trait DynamicImageMethods {
    /// allows you to cut the image into equal square parts of 128x128 size
    fn into_map_sheet(self, width: u32, height: u32) -> Vec<DynamicImage>;
    /// allows you to quantize the image according to the given `MapPalette`
    fn into_map_colors(self, palette: &MapPalette) -> Vec<i8>;
}

impl DynamicImageMethods for DynamicImage {
    fn into_map_sheet(self, width: u32, height: u32) -> Vec<DynamicImage> {
        let (width, height) = (width * 128, height * 128);
        let mut image = self.resize_exact(width, height, Lanczos3);
        (0..height)
            .step_by(128)
            .flat_map(|y| (0..width).step_by(128).map(move |x| (x, y)))
            .map(|(x, y)| image.sub_image(x, y, 128, 128).to_image())
            .map(DynamicImage::from)
            .collect()
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

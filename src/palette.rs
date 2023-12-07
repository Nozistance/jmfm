use image::imageops::ColorMap;
use image::{Pixel, Rgb};

use crate::ColorDistance;

/// This represents Java Minecraft map colors palette that can be used in [image dithering](image::imageops::dither)
#[derive(Debug)]
pub struct MapPalette {
    margin: usize,
    colors: Vec<[u8; 3]>,
}

#[rustfmt::skip]
impl MapPalette {
    pub fn new(colors: &[[u8; 3]], multipliers: &[u8]) -> Self {
        Self {
            margin: multipliers.len(),
            colors: colors.iter()
                .flat_map(|c| multipliers.iter().map(move |&m| (c, m)))
                .flat_map(|(c, m)| c.iter().map(move |&c| (c, m)))
                .map(|(c, m)| (c as u16, m as u16))
                .map(|(c, m)| (c * m) as f32 / 255.0)
                .map(|r| r as u8).collect::<Vec<u8>>()
                .chunks(3).map(|a| [a[0], a[1], a[2]])
                .collect()
        }
    }
}

impl ColorMap for MapPalette {
    type Color = Rgb<u8>;

    fn index_of(&self, color: &Rgb<u8>) -> usize {
        self.colors
            .iter()
            .position(|r| r == color.channels())
            .map(|p| p + self.margin)
            .unwrap_or(0)
    }

    fn lookup(&self, idx: usize) -> Option<Self::Color> {
        self.colors.get(idx).copied().map(|p| p.into())
    }

    fn has_lookup(&self) -> bool {
        true
    }

    #[rustfmt::skip]
    fn map_color(&self, color: &mut Rgb<u8>) {
        let original = *color;
        if let Some(closest_color) = self.colors.iter()
            .min_by_key(|&c| original.channels().dist(c)) {
            color.0 = *closest_color;
        }
    }
}

use image::imageops::ColorMap;
use image::Rgb;

use crate::ColorDistance;

/// This represents Java Minecraft map colors palette that can be used in [image dithering](image::imageops::dither)
#[derive(Debug)]
pub struct MapPalette {
    margin: usize,
    colors: Vec<Rgb<u8>>,
}

impl MapPalette {
    pub fn new(colors: &[Rgb<u8>], multipliers: &[u8]) -> Self {
        Self {
            margin: multipliers.len(),
            colors: colors
                .iter()
                .flat_map(|c| multipliers.iter().map(move |m| (c, m)))
                .map(|(c, &m)| (c.0.map(|c| c as f32), m as f32))
                .map(|(c, m)| c.map(|c| c * m / 255.0).map(|c| c as u8))
                .map(Rgb::from)
                .collect(),
        }
    }
}

impl ColorMap for MapPalette {
    type Color = Rgb<u8>;

    fn index_of(&self, color: &Rgb<u8>) -> usize {
        self.colors.iter().position(|r| r == color).unwrap() + self.margin
    }

    fn lookup(&self, index: usize) -> Option<Rgb<u8>> {
        self.colors.get(index).copied()
    }

    fn has_lookup(&self) -> bool {
        true
    }

    fn map_color(&self, color: &mut Rgb<u8>) {
        let mut least_diff = usize::MAX;
        let original = *color;
        for candidate in self.colors.iter() {
            let diff = original.dist(candidate);
            if diff == 0 {
                color.0 = candidate.0;
                return;
            }
            if diff < least_diff {
                least_diff = diff;
                color.0 = candidate.0;
            }
        }
    }
}

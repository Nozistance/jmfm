use image::imageops::ColorMap;
use image::Rgb;

/// This represents Java Minecraft map colors palette that can be used in image dithering
/// # Usage
/// ```
/// image::imageops::dither(&mut image, palette);
/// ```
#[derive(Debug)]
pub struct MapPalette {
    pub multipliers: Vec<u8>,
    pub colors: Vec<Rgb<u8>>,
}

impl ColorMap for MapPalette {
    type Color = Rgb<u8>;

    #[inline(always)]
    fn index_of(&self, color: &Rgb<u8>) -> usize {
        self.colors.iter().position(|r| r == color).unwrap() + self.multipliers.len()
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Rgb<u8>) {
        let mut least_distance = i32::max_value();
        let original = color.0;
        for candidate in self.colors.iter() {
            let distance = color_distance(&original, &candidate.0);
            if distance == 0 {
                color.0 = candidate.0;
                return;
            }
            if distance < least_distance {
                least_distance = distance;
                color.0 = candidate.0;
            }
        }
    }

    fn lookup(&self, index: usize) -> Option<Self::Color> {
        self.colors.get(index).copied()
    }

    fn has_lookup(&self) -> bool {
        true
    }
}

fn color_distance(a: &[u8], b: &[u8]) -> i32 {
    let m = (a[0] as i32 + b[0] as i32) >> 1;
    let r = a[0] as i32 - b[0] as i32;
    let g = a[1] as i32 - b[1] as i32;
    let b = a[2] as i32 - b[2] as i32;
    (((512 + m) * r * r) >> 8) + 4 * g * g + (((767 - m) * b * b) >> 8)
}

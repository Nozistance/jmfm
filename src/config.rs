use image::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub multipliers: Vec<u8>,
    pub colors: Vec<[u8; 3]>,
}

impl Config {
    pub fn colors(&self) -> Vec<Rgb<u8>> {
        let mut colors = Vec::new();
        for color in self.colors.as_slice() {
            for m in self.multipliers.as_slice() {
                colors.push(Rgb::from(
                    color
                        .map(|c| c as f32)
                        .map(|c| c * *m as f32)
                        .map(|c| c / 255.0)
                        .map(|c| c as u8),
                ));
            }
        }
        colors
    }
}

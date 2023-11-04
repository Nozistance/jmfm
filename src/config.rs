use image::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    data_version: i32,
    multipliers: Vec<u8>,
    colors: Vec<[u8; 3]>,
}

impl Config {
    pub fn data_version(&self) -> i32 {
        self.data_version
    }

    pub fn multipliers(&self) -> Vec<u8> {
        self.multipliers.clone()
    }

    pub fn colors(&self) -> Vec<Rgb<u8>> {
        self.colors.iter().map(|&c| Rgb::from(c)).collect()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub data_version: i32,
    pub multipliers: Box<[u8]>,
    pub colors: Box<[[u8; 3]]>,
}

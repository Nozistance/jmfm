use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapEntry {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    data: Data,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    scale: i8,
    dimension: String,
    tracking_position: i8,
    unlimited_tracking: i8,
    locked: i8,
    x_center: i32,
    z_center: i32,
    banners: Vec<Banner>,
    colors: Vec<i8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Banner {
    color: String,
    name: String,
    pos: Pos,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Frame {
    entity_id: i32,
    rotation: i32,
    pos: Pos,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl MapEntry {
    pub fn new(data_version: i32, data: Data) -> Self {
        MapEntry { data_version, data }
    }
}

impl From<Vec<i8>> for Data {
    fn from(value: Vec<i8>) -> Self {
        Data {
            dimension: String::from("minecraft:overworld"),
            locked: 1,
            colors: value,
            ..Default::default()
        }
    }
}

use serde::{Deserialize, Serialize};

/// Structure for representing `idcounts.dat` file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdCounts {
    pub data: IdCountsData,
    pub data_version: i32,
}

impl IdCounts {
    pub fn new(data_version: i32, map: i32) -> Self {
        IdCounts {
            data_version,
            data: IdCountsData { map },
        }
    }
}

/// Structure for the data within IdCounts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdCountsData {
    pub map: i32,
}

/// Structure for representing a `map_*.dat` file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "data")]
    pub data: MapData,
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
}

impl Map {
    pub fn new(data_version: i32, data: MapData) -> Self {
        Map { data_version, data }
    }
}

/// Structure for the data within a Map
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    pub scale: i8,
    pub dimension: String,
    pub tracking_position: i8,
    pub unlimited_tracking: i8,
    pub locked: i8,
    pub x_center: i32,
    pub z_center: i32,
    pub banners: Vec<Banner>,
    pub frames: Vec<Frame>,
    #[serde(serialize_with = "nbt::i8_array")]
    pub colors: Vec<i8>,
}

impl MapData {
    pub fn from(colors: Vec<i8>) -> Self {
        MapData {
            dimension: String::from("minecraft:overworld"),
            locked: 1,
            colors,
            ..Default::default()
        }
    }
}

/// Structure for representing a banner
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Banner {
    pub color: String,
    pub name: String,
    pub pos: Pos,
}

/// Structure for representing a frame
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Frame {
    pub entity_id: i32,
    pub rotation: i32,
    pub pos: Pos,
}

/// Structure representing a position on a map
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

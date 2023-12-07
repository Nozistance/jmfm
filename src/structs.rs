use serde::{Deserialize, Serialize};

/// Structure for representing `idcounts.dat` file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdCounts {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub data: IdCountsData,
}

/// Structure for the data within IdCounts
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdCountsData {
    pub map: i32,
}

/// Structure for representing a `map_*.dat` file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Map {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub data: MapData,
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
    pub banners: Box<[Banner]>,
    pub colors: Box<[i8]>,
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

impl IdCounts {
    /// Creates a new IdCounts instance with the provided data version and map data
    pub fn new(data_version: i32, map: i32) -> Self {
        IdCounts {
            data_version,
            data: IdCountsData { map },
        }
    }
}

impl Map {
    /// Creates a new Map instance with the provided data version and map data
    pub fn new(data_version: i32, data: MapData) -> Self {
        Map { data_version, data }
    }
}

impl MapData {
    /// Creates a new MapData instance with default values and the provided array of colors
    pub fn new(value: &[i8]) -> Self {
        MapData {
            dimension: String::from("minecraft:overworld"),
            locked: 1,
            colors: value.into(),
            ..Default::default()
        }
    }
}

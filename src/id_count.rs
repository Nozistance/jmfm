use serde::{Serialize, Deserialize};

/// This represents the `idcount.dat` file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdCount {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub data: Data,
}

/// This represents the root compound of `IdCount`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub map: i32,
}

impl From<i32> for Data {
    fn from(map: i32) -> Self {
        Data { map }
    }
}

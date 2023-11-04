use serde::{Deserialize, Serialize};

/// This represents the `idcounts.dat` file
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdCounts {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    pub data: Data,
}

/// This represents the root compound of `IdCounts`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub map: i32,
}

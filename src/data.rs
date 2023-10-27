use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pak {
    pub name: String,
    pub version: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Data {
    pub pak: Vec<Pak>,
}

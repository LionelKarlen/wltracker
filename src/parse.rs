use serde::Deserialize;

#[derive(Deserialize)]
pub struct APIResult {
    pub data: APIBody,
}

#[derive(Deserialize)]
pub struct APIBody {
    pub seasonal: Vec<Seasonal>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Seasonal {
    pub season: Season,
    pub wins: u32,
    pub games: u32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Season {
    pub id: String,
    pub short: String,
}

impl APIResult {
    pub fn from_raw_string(raw_string: &str) -> Self {
        // FIXME: don't panic on errors, just keep old value
        return serde_json::from_str(raw_string).expect("failed to serialise to APIResult");
    }
}

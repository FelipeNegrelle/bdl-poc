use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeeResponse {
    #[serde(rename = "fastestFee")]
    pub fastest_fee: f32,
    #[serde(rename = "halfHourFee")]
    pub half_hour_fee: f32,
    #[serde(rename = "hourFee")]
    pub hour_fee: f32,
    #[serde(rename = "economyFee")]
    pub economy_fee: f32,
    #[serde(rename = "minimumFee")]
    pub minimum_fee: f32,
}

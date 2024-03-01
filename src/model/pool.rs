use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct Pool {
    pub name: Option<String>,

    #[serde(rename = "lanemin")]
    pub lane_min: Option<u32>,

    #[serde(rename = "lanemax")]
    pub lane_max: Option<u32>,
}

impl Pool {
    pub fn number_of_lane(&self) -> Option<u32> {
        if let (Some(min), Some(max)) = (self.lane_min, self.lane_max) {
            return Some(max - min + 1);
        }

        None
    }
}

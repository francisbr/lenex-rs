use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub mod age_date;
pub mod age_group;
pub mod athlete;
pub mod club;
pub mod course;
pub mod entry;
pub mod event;
pub mod fee;
pub mod gender;
pub mod lenex;
pub mod meet;
pub mod pool;
pub mod round;
pub mod session;
pub mod stroke;
pub mod swimstyle;
pub mod timing;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Facility {
    pub city: String,
    pub name: String,
    pub nation: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct PointTable {
    #[serde(rename = "pointtableid")]
    pub id: u32,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Qualify {
    pub from: NaiveDate,
}

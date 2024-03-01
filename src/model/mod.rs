use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

mod age_date;
mod age_group;
mod athlete;
mod club;
mod course;
mod entry;
mod event;
mod fee;
mod gender;
mod lenex;
mod meet;
mod pool;
mod round;
mod session;
mod stroke;
mod swimstyle;
mod timing;

pub use age_date::*;
pub use age_group::*;
pub use athlete::*;
pub use club::*;
pub use course::*;
pub use entry::*;
pub use event::*;
pub use fee::*;
pub use gender::*;
pub use lenex::*;
pub use meet::*;
pub use pool::*;
pub use round::*;
pub use session::*;
pub use stroke::*;
pub use swimstyle::*;
pub use timing::*;

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct Facility {
    pub city: String,
    pub name: String,
    pub nation: String,
}

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct PointTable {
    #[serde(rename = "pointtableid")]
    pub id: u32,
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct Qualify {
    pub from: NaiveDate,
}

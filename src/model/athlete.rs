use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::collection::Collection;

use super::{entry::Entry, gender::Gender};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "ATHLETE")]
pub struct Athlete {
    #[serde(rename = "athleteid")]
    id: u32,

    #[serde(rename = "firstname")]
    first_name: String,

    #[serde(rename = "lastname")]
    last_name: String,

    gender: Gender,

    license: Option<String>,

    #[serde(rename = "birthdate")]
    birth_date: NaiveDate,

    #[serde(rename = "ENTRIES")]
    entries: Collection<Entry>,
}

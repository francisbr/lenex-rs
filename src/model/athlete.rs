use chrono::NaiveDate;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::collection::Collection;

use super::{entry::Entry, gender::Gender};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Builder)]
#[serde(rename = "ATHLETE")]
#[builder(setter(strip_option))]
pub struct Athlete {
    #[serde(rename = "athleteid")]
    id: u32,

    #[serde(rename = "firstname")]
    first_name: String,

    #[serde(rename = "lastname")]
    last_name: String,

    gender: Gender,

    #[builder(default)]
    license: Option<String>,

    #[serde(rename = "birthdate")]
    birth_date: NaiveDate,

    #[serde(rename = "ENTRIES")]
    #[builder(setter(skip))]
    entries: Collection<Entry>,
}

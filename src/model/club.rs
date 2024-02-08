use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::collection::Collection;

use super::athlete::Athlete;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Builder)]
#[serde(rename = "CLUB")]
#[builder(setter(strip_option))]
pub struct Club {
    #[serde(rename = "clubid")]
    id: u32,

    name: String,

    #[builder(default)]
    code: Option<String>,

    #[builder(default)]
    nation: Option<String>,

    #[builder(default)]
    region: Option<String>,

    #[serde(rename = "ATHLETES")]
    #[builder(setter(skip))]
    athletes: Collection<Athlete>,
}

use serde::{Deserialize, Serialize};

use crate::collection::Collection;

use super::athlete::Athlete;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "CLUB")]
pub struct Club {
    #[serde(rename = "clubid")]
    id: u32,

    name: String,

    code: Option<String>,

    nation: Option<String>,

    region: Option<String>,

    #[serde(rename = "ATHLETES")]
    athletes: Collection<Athlete>,
}

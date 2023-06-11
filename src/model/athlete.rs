use chrono::NaiveDate;
use derive_builder::Builder;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

use super::{
    entry::{Entries, Entry},
    gender::Gender,
};

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
    entries: Entries,
}

impl Athlete {
    pub fn entries(&self) -> &Vec<Entry> {
        self.entries.items()
    }

    pub fn entries_mut(&mut self) -> &mut Vec<Entry> {
        self.entries.items_mut()
    }
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "ATHLETES")]
pub(crate) struct Athletes {
    #[serde(rename = "ATHLETE")]
    items: Vec<Athlete>,
}

impl From<Vec<Athlete>> for Athletes {
    fn from(items: Vec<Athlete>) -> Self {
        Self { items }
    }
}

impl Athletes {
    pub fn items(&self) -> &Vec<Athlete> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<Athlete> {
        &mut self.items
    }
}

struct AthletesVisitor;

impl<'de> Deserialize<'de> for Athletes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(AthletesVisitor)
    }
}

impl<'de> Visitor<'de> for AthletesVisitor {
    type Value = Athletes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("the athletes")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut athletes: Vec<Athlete> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, Athlete>()? {
            if key.eq("ATHLETE") {
                athletes.push(value);
            }
        }

        return Ok(Athletes::from(athletes));
    }
}

use core::fmt;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use super::{
    age_date::AgeDate, course::Course, fee::Fees, pool::Pool, session::Sessions, timing::Timing,
    Facility, PointTable, Qualify,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "MEET")]
pub struct Meet {
    pub name: String,

    #[serde(rename = "name.en")]
    pub name_en: Option<String>,

    pub city: String,

    #[serde(rename = "city.en")]
    pub city_en: Option<String>,

    pub nation: String,

    pub course: Option<Course>,

    #[serde(rename = "reservecount")]
    pub reserve_count: Option<u32>,

    #[serde(rename = "startmethod")]
    pub start_method: Option<u32>,

    pub timing: Option<Timing>,

    #[serde(rename = "AGEDATE")]
    pub age_date: Option<AgeDate>,

    #[serde(rename = "POOL")]
    pub pool: Option<Pool>,

    #[serde(rename = "FACILITY")]
    pub facility: Option<Facility>,

    #[serde(rename = "POINTTABLE")]
    pub point_table: Option<PointTable>,

    #[serde(rename = "FEES", default)]
    pub fees: Fees,

    #[serde(rename = "QUALIFY")]
    pub qualify: Option<Qualify>,

    #[serde(rename = "SESSIONS")]
    pub sessions: Sessions,
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "MEETS")]
pub struct Meets {
    #[serde(rename = "MEET")]
    pub items: Vec<Meet>,
}

impl Into<Vec<Meet>> for Meets {
    fn into(self) -> Vec<Meet> {
        self.items
    }
}

impl From<Vec<Meet>> for Meets {
    fn from(meets: Vec<Meet>) -> Self {
        Meets { items: meets }
    }
}

impl<'de> Deserialize<'de> for Meets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MeetsVisitor)
    }
}

struct MeetsVisitor;

impl<'de> Visitor<'de> for MeetsVisitor {
    type Value = Meets;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the meets")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut meets: Vec<Meet> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, Meet>()? {
            if key.eq("MEET") {
                meets.push(value);
            }
        }

        return Ok(meets.into());
    }
}

use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

use super::athlete::{Athlete, Athletes};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "CLUB")]
pub struct Club {
    #[serde(rename = "clubid")]
    pub id: u32,

    pub name: String,

    pub code: Option<String>,

    pub nation: Option<String>,

    pub region: Option<String>,

    #[serde(rename = "ATHLETES")]
    athletes: Athletes,
}

impl Club {
    pub fn athletes(&self) -> &Vec<Athlete> {
        self.athletes.items()
    }

    pub fn athletes_mut(&mut self) -> &mut Vec<Athlete> {
        self.athletes.items_mut()
    }
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "CLUBS")]
pub(crate) struct Clubs {
    #[serde(rename = "CLUB")]
    items: Vec<Club>,
}

impl From<Vec<Club>> for Clubs {
    fn from(items: Vec<Club>) -> Self {
        Self { items }
    }
}

impl Clubs {
    pub fn items(&self) -> &Vec<Club> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<Club> {
        &mut self.items
    }
}

struct ClubsVisitor;

impl<'de> Deserialize<'de> for Clubs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ClubsVisitor)
    }
}

impl<'de> Visitor<'de> for ClubsVisitor {
    type Value = Clubs;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("the clubs")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut clubs: Vec<Club> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, Club>()? {
            if key.eq("CLUB") {
                clubs.push(value);
            }
        }

        return Ok(Clubs::from(clubs));
    }
}

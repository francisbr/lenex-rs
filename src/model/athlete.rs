use chrono::NaiveDate;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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

pub(super) mod vec_serializer {
    use std::fmt::{self, Formatter};

    use serde::{
        de::{MapAccess, Visitor},
        Serialize, Serializer,
    };

    use super::Athlete;

    pub fn serialize<S>(value: &Vec<Athlete>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Collection<'a> {
            #[serde(rename = "ATHLETE")]
            items: &'a Vec<Athlete>,
        }

        Collection::serialize(&Collection { items: value }, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Athlete>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Vec<Athlete>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("the athletes")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut items = map.size_hint().map_or(Vec::new(), Vec::with_capacity);

                while let Some((_, value)) = map.next_entry::<String, Athlete>()? {
                    items.push(value);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_map(MyVisitor)
    }
}

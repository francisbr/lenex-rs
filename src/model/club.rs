use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::athlete::{self, Athlete};

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

    #[serde(rename = "ATHLETES", with = "athlete::vec_serializer")]
    #[builder(setter(skip))]
    athletes: Vec<Athlete>,
}

pub(super) mod vec_serializer {
    use std::fmt::{self, Formatter};

    use serde::{
        de::{MapAccess, Visitor},
        Serialize, Serializer,
    };

    use super::Club;

    pub fn serialize<S>(value: &Vec<Club>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Collection<'a> {
            #[serde(rename = "CLUB")]
            items: &'a Vec<Club>,
        }

        Collection::serialize(&Collection { items: value }, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Club>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Vec<Club>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("the clubs")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut items = map.size_hint().map_or(Vec::new(), Vec::with_capacity);

                while let Some((_, value)) = map.next_entry::<String, Club>()? {
                    items.push(value);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_map(MyVisitor)
    }
}

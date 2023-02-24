use core::fmt;

use fast_xml::{de, se, DeError};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::meet::{Meet, Meets};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "LENEX")]
pub struct Lenex {
    #[serde(serialize_with = "serialize_version")]
    pub version: f32,

    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Constructor,

    #[serde(rename = "MEETS")]
    pub meets: Meets,
}

impl Lenex {
    pub fn new() -> Self {
        Lenex {
            version: 3.0,
            constructor: Constructor {
                name: "lenex-rs".into(),
                registration: "".into(),
                version: env!("CARGO_PKG_VERSION").into(),
            },
            meets: Vec::new().into(),
        }
    }

    pub fn xml(&self) -> Result<String, DeError> {
        Ok(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}",
            se::to_string(&self)?
        ))
    }
}

impl TryFrom<String> for Lenex {
    type Error = DeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        de::from_str(&value)
    }
}

impl TryInto<String> for Lenex {
    type Error = DeError;

    fn try_into(self) -> Result<String, Self::Error> {
        Ok(self.xml()?)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Constructor {
    pub name: String,
    pub registration: String,
    pub version: String,
}

pub fn deserialize_meets<'de, D>(deserializer: D) -> Result<Vec<Meet>, D::Error>
where
    D: Deserializer<'de>,
{
    struct MeetVisitor;

    impl<'de> Visitor<'de> for MeetVisitor {
        type Value = Vec<Meet>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map")
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

            return Ok(meets);
        }
    }

    deserializer.deserialize_any(MeetVisitor)
}

fn serialize_version<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{x:.1}"))
}

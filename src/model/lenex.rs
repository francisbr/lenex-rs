use fast_xml::{de, se, DeError};
use serde::{Deserialize, Serialize, Serializer};

use super::meet::{Meet, Meets};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "LENEX")]
pub struct Lenex {
    #[serde(serialize_with = "serialize_version")]
    pub version: f32,

    #[serde(rename = "CONSTRUCTOR")]
    pub constructor: Constructor,

    #[serde(rename = "MEETS")]
    meets: Meets,
}

impl Lenex {
    pub fn new() -> Self {
        Lenex {
            version: 3.0,
            constructor: Constructor {
                name: "lenex-rs".into(),
                registration: "".into(),
                contact: Contact {
                    name: "Francis Boulet-Rouleau".into(),
                    country: "CA".into(),
                    email: "francisbouletrouleau@gmail.com".into(),
                    internet: "https://github.com/francisbr/lenex-rs".into(),
                },
                version: env!("CARGO_PKG_VERSION").into(),
            },
            meets: Meets::default(),
        }
    }

    pub fn meets(&self) -> &Vec<Meet> {
        self.meets.items()
    }

    pub fn meets_mut(&mut self) -> &mut Vec<Meet> {
        self.meets.items_mut()
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

    #[serde(rename = "CONTACT")]
    pub contact: Contact,
}

fn serialize_version<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{x:.1}"))
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub name: String,
    pub country: String,
    pub email: String,
    pub internet: String,
}

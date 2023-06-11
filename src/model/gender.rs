use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Gender {
    #[serde(rename = "M")]
    Male,

    #[serde(rename = "F")]
    Female,

    #[serde(rename = "X")]
    Mixed,

    #[serde(rename = "A")]
    All,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::All
    }
}

impl Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Gender::Male => serializer.serialize_char('M'),
            Gender::Female => serializer.serialize_char('F'),
            Gender::Mixed => serializer.serialize_char('X'),
            Gender::All => serializer.serialize_none(),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, PartialEq)]
pub enum Gender {
    #[serde(rename = "M")]
    Male,

    #[serde(rename = "F")]
    Female,

    #[serde(rename = "X")]
    Mixed,

    #[default]
    #[serde(rename = "A")]
    All,
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub enum Timing {
    #[serde(rename = "AUTOMATIC")]
    Automatic,

    #[serde(rename = "SEMIAUTOMATIC")]
    SemiAutomatic,

    #[serde(rename = "MANUAL1")]
    Manual1,

    #[serde(rename = "MANUAL2")]
    Manual2,

    #[serde(rename = "MANUAL3")]
    Manual3,
}

impl Serialize for Timing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Timing::Automatic => serializer.serialize_str("AUTOMATIC"),
            Timing::SemiAutomatic => serializer.serialize_str("SEMIAUTOMATIC"),
            Timing::Manual1 => serializer.serialize_str("MANUAL1"),
            Timing::Manual2 => serializer.serialize_str("MANUAL2"),
            Timing::Manual3 => serializer.serialize_str("MANUAL3"),
        }
    }
}

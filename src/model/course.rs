use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Debug)]
pub enum Course {
    LCM,
    SCM,
    SCY,
    SCM16,
    SCM20,
    SCM33,
    SCY20,
    SCY27,
    SCY33,
    SCY36,

    #[serde(rename = "OPEN")]
    Open,
}

impl Serialize for Course {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Course::LCM => serializer.serialize_str("LCM"),
            Course::SCM => serializer.serialize_str("SCM"),
            Course::SCY => serializer.serialize_str("SCY"),
            Course::SCM16 => serializer.serialize_str("SCM16"),
            Course::SCM20 => serializer.serialize_str("SCM20"),
            Course::SCM33 => serializer.serialize_str("SCM33"),
            Course::SCY20 => serializer.serialize_str("SCY20"),
            Course::SCY27 => serializer.serialize_str("SCY27"),
            Course::SCY33 => serializer.serialize_str("SCY33"),
            Course::SCY36 => serializer.serialize_str("SCY36"),
            Course::Open => serializer.serialize_str("OPEN"),
        }
    }
}

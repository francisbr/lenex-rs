use std::fmt;

use serde::{
    de::{Deserializer, Error, Visitor},
    Deserialize, Serialize, Serializer,
};

#[derive(Default, Debug, PartialEq)]
pub enum Stroke {
    Surface,

    #[default]
    Unknown,
}

impl<'de> Deserialize<'de> for Stroke {
    fn deserialize<D>(deserializer: D) -> Result<Stroke, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrokeVisitor;

        impl<'de> Visitor<'de> for StrokeVisitor {
            type Value = Stroke;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with the stroke")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(match v {
                    "SURFACE" => Stroke::Surface,
                    _ => Stroke::Unknown,
                })
            }
        }

        deserializer.deserialize_str(StrokeVisitor)
    }
}
impl Serialize for Stroke {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            Stroke::Surface => serializer.serialize_str("SURFACE"),
            Stroke::Unknown => serializer.serialize_str("UNKNOWN"),
        }
    }
}

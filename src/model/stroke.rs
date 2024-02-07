use std::fmt;

use serde::{de, Deserialize, Serialize};

#[derive(Default, Debug, PartialEq)]
pub enum Stroke {
    #[default]
    Unknown,
}

impl<'de> Deserialize<'de> for Stroke {
    fn deserialize<D>(deserializer: D) -> Result<Stroke, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct StrokeVisitor;

        impl<'de> de::Visitor<'de> for StrokeVisitor {
            type Value = Stroke;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a char with the gender")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(match v {
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
        S: serde::Serializer,
    {
        match &self {
            Stroke::Unknown => serializer.serialize_str("UNKNOWN"),
        }
    }
}

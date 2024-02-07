use core::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Default, Debug, PartialEq)]
pub enum Calculate {
    #[default]
    Single,
    Total,
}

impl<'de> Deserialize<'de> for Calculate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CalculateVisitor)
    }
}

impl Serialize for Calculate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Calculate::Single => serializer.serialize_none(),
            Calculate::Total => serializer.serialize_str("TOTAL"),
        }
    }
}

struct CalculateVisitor;

impl<'de> Visitor<'de> for CalculateVisitor {
    type Value = Calculate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the information for relay events about how the age is calculated")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(match v {
            "SINGLE" => Calculate::Single,
            "TOTAL" => Calculate::Total,
            _ => Calculate::Single,
        })
    }
}

use std::fmt;

use chrono::NaiveDate;
use serde::{de, Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct AgeDate {
    pub value: NaiveDate,
    pub r#type: AgeDateType,
}

#[derive(PartialEq, Default, Debug)]
pub enum AgeDateType {
    Year,
    #[default]
    Date,
    Por,
    CanFnq,
    Lux,
}

impl<'de> Deserialize<'de> for AgeDateType {
    fn deserialize<D>(deserializer: D) -> Result<AgeDateType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct AgeDateTypeVisitor;

        impl<'de> de::Visitor<'de> for AgeDateTypeVisitor {
            type Value = AgeDateType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("the type that describes how the age is calculated")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(match v {
                    "YEAR" => AgeDateType::Year,
                    "DATE" => AgeDateType::Date,
                    "POR" => AgeDateType::Por,
                    "CAN.FNQ" => AgeDateType::CanFnq,
                    "LUX" => AgeDateType::Lux,
                    _ => AgeDateType::default(),
                })
            }
        }

        deserializer.deserialize_str(AgeDateTypeVisitor)
    }
}

impl Serialize for AgeDateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            AgeDateType::Year => serializer.serialize_str("YEAR"),
            AgeDateType::Date => serializer.serialize_str("DATE"),
            AgeDateType::Por => serializer.serialize_str("POR"),
            AgeDateType::CanFnq => serializer.serialize_str("CAN.FNQ"),
            AgeDateType::Lux => serializer.serialize_str("LUX"),
        }
    }
}

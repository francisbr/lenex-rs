use core::fmt;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Default, Debug, PartialEq)]
pub enum Round {
    #[default]
    TimedFinals,

    FastestHeat,

    Finals,

    SemiFinals,

    QuarterFinals,

    Prelims,

    SwimOffPrelims,

    SwimOffSemiFinals,

    SwimOffQuarterFinals,
}

struct RoundVisitor;

impl<'de> Visitor<'de> for RoundVisitor {
    type Value = Round;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the information of the round")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(match v {
            "TIM" => Round::TimedFinals,
            "FHT" => Round::FastestHeat,
            "FIN" => Round::Finals,
            "SEM" => Round::SemiFinals,
            "QUA" => Round::QuarterFinals,
            "PRE" => Round::Prelims,
            "SOP" => Round::SwimOffPrelims,
            "SOS" => Round::SwimOffSemiFinals,
            "SOQ" => Round::SwimOffQuarterFinals,
            _ => Round::default(),
        })
    }
}

impl<'de> Deserialize<'de> for Round {
    fn deserialize<D>(deserializer: D) -> Result<Round, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RoundVisitor)
    }
}

impl Serialize for Round {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Round::TimedFinals => serializer.serialize_none(),
            Round::FastestHeat => serializer.serialize_str("FHT"),
            Round::Finals => serializer.serialize_str("FIN"),
            Round::SemiFinals => serializer.serialize_str("SEM"),
            Round::QuarterFinals => serializer.serialize_str("QUA"),
            Round::Prelims => serializer.serialize_str("PRE"),
            Round::SwimOffPrelims => serializer.serialize_str("SOP"),
            Round::SwimOffSemiFinals => serializer.serialize_str("SOS"),
            Round::SwimOffQuarterFinals => serializer.serialize_str("SOQ"),
        }
    }
}

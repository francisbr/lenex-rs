use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::{collection::Collection, serialization::serde_time};

use super::{age_group::AgeGroup, gender::Gender, round::Round, swimstyle::SwimStyle};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "EVENT")]
pub struct Event {
    #[serde(rename = "eventid")]
    pub id: u32,

    #[serde(
        rename = "preveventid",
        default,
        with = "crate::serialization::serde_number"
    )]
    pub prev_event_id: Option<u32>,

    #[serde(rename = "daytime", default, with = "serde_time::optional")]
    day_time: Option<NaiveTime>,

    #[serde(default)]
    gender: Option<Gender>,

    number: u32,

    #[serde(default, with = "crate::serialization::serde_number")]
    pub order: Option<u32>,

    round: Option<Round>,

    #[serde(rename = "SWIMSTYLE")]
    swim_style: SwimStyle,

    #[serde(rename = "AGEGROUPS", default, skip_serializing_if = "Vec::is_empty")]
    age_groups: Collection<AgeGroup>,
}

impl Event {
    pub fn new(id: u32, number: u32, swim_style: SwimStyle) -> Self {
        Self {
            id,
            number,
            swim_style,
            ..Default::default()
        }
    }
}
#[cfg(test)]
mod tests {
    use fast_xml::{de, se};

    use super::*;

    #[test]
    fn deserialize_empty() {
        let result = de::from_str::<Event>(r#"<EVENT/>"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_basic() {
        let result = de::from_str::<Event>(
            r#"<EVENT eventid="123" number="456"><SWIMSTYLE distance="50" relaycount="1" swimstyleid="5840" stroke="UNKNOWN"/></EVENT>"#,
        );
        assert!(result.is_ok());

        let event = result.unwrap();
        assert_eq!(123, event.id);
        assert_eq!(456, event.number);
    }

    #[test]
    fn serialize_basic_collection() {
        let events = vec![Event {
            id: 123,
            ..Default::default()
        }];

        let result = se::to_string(&events);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<EVENT eventid="123" number="0"><SWIMSTYLE swimstyleid="0" distance="0" relaycount="0" stroke="UNKNOWN"/></EVENT>"#,
            xml
        );
    }
}

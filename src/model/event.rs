use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::serialization::serde_time;

use super::{
    age_group::{self, AgeGroup},
    gender::Gender,
    round::Round,
    swimstyle::SwimStyle,
};

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

    #[serde(rename = "AGEGROUPS", with = "age_group::vec_serializer", default)]
    age_groups: Vec<AgeGroup>,
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

pub(super) mod vec_serializer {
    use std::fmt::{self, Formatter};

    use serde::{
        de::{MapAccess, Visitor},
        Serialize, Serializer,
    };

    use super::Event;

    pub fn serialize<S>(value: &Vec<Event>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Collection<'a> {
            #[serde(rename = "EVENT")]
            items: &'a Vec<Event>,
        }

        Collection::serialize(&Collection { items: value }, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Event>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Vec<Event>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("the events")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut items = map.size_hint().map_or(Vec::new(), Vec::with_capacity);

                while let Some((_, value)) = map.next_entry::<String, Event>()? {
                    items.push(value);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_map(MyVisitor)
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

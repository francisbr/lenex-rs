use core::fmt;

use chrono::NaiveTime;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::util::{self, serde_time};

use super::{age_group::AgeGroups, gender::Gender, round::Round, swimstyle::SwimStyle};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "EVENT")]
pub struct Event {
    #[serde(rename = "eventid")]
    pub id: u64,

    #[serde(rename = "preveventid", default, with = "util::serde_number")]
    pub prev_event_id: Option<u32>,

    #[serde(rename = "daytime", default, with = "serde_time::optional")]
    pub day_time: Option<NaiveTime>,

    #[serde(default)]
    pub gender: Gender,

    pub number: u32,

    #[serde(default, with = "util::serde_number")]
    pub order: Option<u32>,

    pub round: Option<Round>,

    #[serde(rename = "SWIMSTYLE")]
    pub swim_style: SwimStyle,

    #[serde(rename = "AGEGROUPS")]
    pub age_groups: Option<AgeGroups>,
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "EVENTS")]
pub struct Events {
    #[serde(rename = "EVENT")]
    items: Vec<Event>,
}

impl From<Vec<Event>> for Events {
    fn from(value: Vec<Event>) -> Self {
        Self { items: value }
    }
}

impl Events {
    pub fn items_owned(self) -> Vec<Event> {
        self.items
    }

    pub fn items(&self) -> &Vec<Event> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<Event> {
        &mut self.items
    }
}

impl<'de> Deserialize<'de> for Events {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(EventsVisitor)
    }
}

struct EventsVisitor;

impl<'de> Visitor<'de> for EventsVisitor {
    type Value = Events;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the events")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut events: Vec<Event> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, Event>()? {
            if key.eq("EVENT") {
                events.push(value);
            }
        }

        return Ok(events.into());
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
    fn serialize_empty_collection() {
        let events = Events::from(Vec::new());

        let result = se::to_string(&events);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(r#"<EVENTS/>"#, xml);
    }

    #[test]
    fn serialize_basic_collection() {
        let events = Events::from(vec![Event {
            id: 123,
            ..Default::default()
        }]);

        let result = se::to_string(&events);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<EVENTS><EVENT eventid="123" number="0"><SWIMSTYLE swimstyleid="0" distance="0" relaycount="0" stroke="UNKNOWN"/></EVENT></EVENTS>"#,
            xml
        );
    }
}

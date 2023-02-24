use core::fmt;

use chrono::{NaiveDate, NaiveTime};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::util::serde_time;

use super::event::Events;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "SESSION")]
pub struct Session {
    pub date: NaiveDate,

    #[serde(rename = "daytime", default, with = "serde_time::optional")]
    pub day_time: Option<NaiveTime>,

    #[serde(rename = "endtime", default, with = "serde_time::optional")]
    pub end_time: Option<NaiveTime>,

    pub name: Option<String>,

    pub number: u32,

    #[serde(rename = "teamleadermeeting", default, with = "serde_time::optional")]
    pub team_leader_meeting: Option<NaiveTime>,

    #[serde(rename = "warmupfrom", default, with = "serde_time::optional")]
    pub warmup_from: Option<NaiveTime>,

    #[serde(rename = "warmupuntil", default, with = "serde_time::optional")]
    pub warmup_until: Option<NaiveTime>,

    pub timing: Option<String>,

    #[serde(rename = "EVENTS")]
    pub events: Events,
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "SESSIONS")]
pub struct Sessions {
    #[serde(rename = "SESSION")]
    pub items: Vec<Session>,
}

impl Into<Vec<Session>> for Sessions {
    fn into(self) -> Vec<Session> {
        self.items
    }
}

impl From<Vec<Session>> for Sessions {
    fn from(sessions: Vec<Session>) -> Self {
        Sessions { items: sessions }
    }
}

impl<'de> Deserialize<'de> for Sessions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(SessionsVisitor)
    }
}

struct SessionsVisitor;

impl<'de> Visitor<'de> for SessionsVisitor {
    type Value = Sessions;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the sessions")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut sessions: Vec<Session> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, Session>()? {
            if key.eq("SESSION") {
                sessions.push(value);
            }
        }

        return Ok(sessions.into());
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};
    use fast_xml::{de, se};

    use crate::model::{event::Event, gender::Gender, swimstyle::SwimStyle};

    use super::*;

    #[test]
    fn deserialize_empty() {
        let result = de::from_str::<Session>(r#"<SESSION/>"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_missing_date() {
        let result = de::from_str::<Session>(r#"<SESSION number="123"><EVENTS/></SESSION>"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_missing_number() {
        let result = de::from_str::<Session>(r#"<SESSION date="2023-02-11"><EVENTS/></SESSION>"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_missing_events() {
        let result =
            de::from_str::<Session>(r#"<SESSION date="2023-02-11" number="123"></SESSION>"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_basic() {
        let result = de::from_str::<Session>(
            r#"<SESSION date="2023-02-11" number="123"><EVENTS/></SESSION>"#,
        );
        assert!(result.is_ok());

        let session = result.unwrap();
        assert_eq!(123, session.number);
        assert_eq!(11, session.date.day());
        assert_eq!(02, session.date.month());
        assert_eq!(2023, session.date.year());
        assert_eq!(0, session.events.items.len());
    }

    #[test]
    fn deserialize_with_times() {
        let result = de::from_str::<Session>(
            r#"<SESSION date="2023-02-11" number="123" daytime="18:00" endtime="22:00" warmupfrom="16:45" warmupuntil="17:30" teamleadermeeting="07:00"><EVENTS/></SESSION>"#,
        );
        assert!(result.is_ok());

        let session = result.unwrap();
        assert_eq!(123, session.number);
        assert_eq!(11, session.date.day());
        assert_eq!(02, session.date.month());
        assert_eq!(2023, session.date.year());
        assert_eq!(0, session.events.items.len());

        assert!(session.day_time.is_some());
        let day_time = session.day_time.unwrap();
        assert_eq!(18, day_time.hour());
        assert_eq!(0, day_time.minute());
        assert!(session.end_time.is_some());
        assert!(session.warmup_from.is_some());
        assert!(session.warmup_until.is_some());
        assert!(session.team_leader_meeting.is_some());
    }

    #[test]
    fn deserialize_name() {
        let result = de::from_str::<Session>(
            r#"<SESSION date="2023-02-11" number="123" name="test session"><EVENTS/></SESSION>"#,
        );
        assert!(result.is_ok());

        let session = result.unwrap();
        assert!(session.name.is_some());
        assert_eq!("test session", session.name.unwrap());
    }

    #[test]
    fn deserialize_event() {
        let result = de::from_str::<Session>(
            r#"<SESSION date="2023-02-11" number="123" name="test session"><EVENTS><EVENT eventid="1176" daytime="08:30" number="1" order="1" round="TIM" preveventid="-1"><SWIMSTYLE distance="4" relaycount="1" swimstyleid="511" name="4 m  Lancer de précision 10ans et -" stroke="UNKNOWN"/><AGEGROUPS/></EVENT></EVENTS></SESSION>"#,
        );
        println!("{result:#?}");
        assert!(result.is_ok());

        let session = result.unwrap();
        assert!(session.name.is_some());
        assert_eq!("test session", session.name.unwrap());
        assert_eq!(1, session.events.items.len());
        assert_eq!(1176, session.events.items.get(0).unwrap().id)
    }

    #[test]
    fn serialize_basic() {
        let session = Session {
            date: NaiveDate::default(),
            day_time: None,
            end_time: None,
            name: None,
            number: 123,
            team_leader_meeting: None,
            warmup_from: None,
            warmup_until: None,
            timing: None,
            events: Events::from(Vec::new()),
        };

        let result = se::to_string(&session);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<SESSION date="1970-01-01" number="123"><EVENTS/></SESSION>"#,
            xml
        );
    }

    #[test]
    fn serialize_two_events() {
        let session = Session {
            date: NaiveDate::default(),
            day_time: None,
            end_time: None,
            name: None,
            number: 123,
            team_leader_meeting: None,
            warmup_from: None,
            warmup_until: None,
            timing: None,
            events: Events::from(vec![
                Event {
                    id: 123,
                    prev_event_id: None,
                    day_time: None,
                    gender: Gender::default(),
                    number: 123,
                    order: Some(1),
                    round: None,
                    swim_style: SwimStyle::default(),
                    age_groups: None,
                },
                Event {
                    id: 456,
                    prev_event_id: Some(123),
                    day_time: None,
                    gender: Gender::default(),
                    number: 456,
                    order: Some(2),
                    round: None,
                    swim_style: SwimStyle::default(),
                    age_groups: None,
                },
            ]),
        };

        let result = se::to_string(&session);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<SESSION date="1970-01-01" number="123"><EVENTS><EVENT eventid="123" number="123" order="1"><SWIMSTYLE swimstyleid="0" distance="0" relaycount="0" stroke="UNKNOWN"/></EVENT><EVENT eventid="456" preveventid="123" number="456" order="2"><SWIMSTYLE swimstyleid="0" distance="0" relaycount="0" stroke="UNKNOWN"/></EVENT></EVENTS></SESSION>"#,
            xml
        );
    }
}

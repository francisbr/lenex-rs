use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::{collection::Collection, serialization::serde_time};

use super::event::Event;

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
#[serde(rename = "SESSION")]
pub struct Session {
    date: NaiveDate,

    #[serde(rename = "daytime", default, with = "serde_time::optional")]
    day_time: Option<NaiveTime>,

    #[serde(rename = "endtime", default, with = "serde_time::optional")]
    end_time: Option<NaiveTime>,

    name: Option<String>,

    number: u32,

    #[serde(rename = "teamleadermeeting", default, with = "serde_time::optional")]
    team_leader_meeting: Option<NaiveTime>,

    #[serde(rename = "warmupfrom", default, with = "serde_time::optional")]
    warmup_from: Option<NaiveTime>,

    #[serde(rename = "warmupuntil", default, with = "serde_time::optional")]
    warmup_until: Option<NaiveTime>,

    timing: Option<String>,

    #[serde(rename = "EVENTS")]
    events: Collection<Event>,
}

impl Session {
    pub fn new(number: u32, date: NaiveDate, events: Vec<Event>) -> Self {
        Session {
            number,
            date,
            events: events.into(),
            ..Default::default()
        }
    }

    pub fn with_day_time(&mut self, time: NaiveTime) -> &mut Self {
        self.day_time = Some(time);

        self
    }

    pub fn with_end_time(&mut self, time: NaiveTime) -> &mut Self {
        self.end_time = Some(time);

        self
    }

    pub fn with_warmup_from(&mut self, time: NaiveTime) -> &mut Self {
        self.warmup_from = Some(time);

        self
    }

    pub fn with_warmup_until(&mut self, time: NaiveTime) -> &mut Self {
        self.warmup_until = Some(time);

        self
    }
}
#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike};
    use fast_xml::{de, se};

    use crate::model::{event::Event, swimstyle::SwimStyle};

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
        assert_eq!(0, session.events.len());
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
        assert_eq!(0, session.events.len());

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
        assert!(result.is_ok());

        let session = result.unwrap();
        assert!(session.name.is_some());
        assert_eq!("test session", session.name.unwrap());
        assert_eq!(1, session.events.len());
        assert_eq!(1176, session.events.get(0).unwrap().id)
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
            events: Vec::new().into(),
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
    fn serialize_basic_collection() {
        let sessions = vec![
            Session {
                date: NaiveDate::default(),
                day_time: None,
                end_time: None,
                name: None,
                number: 123,
                team_leader_meeting: None,
                warmup_from: None,
                warmup_until: None,
                timing: None,
                events: Vec::new().into(),
            },
            Session {
                date: NaiveDate::default(),
                day_time: None,
                end_time: None,
                name: None,
                number: 456,
                team_leader_meeting: None,
                warmup_from: None,
                warmup_until: None,
                timing: None,
                events: Vec::new().into(),
            },
        ];

        let result = se::to_string(&sessions);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<SESSION date="1970-01-01" number="123"><EVENTS/></SESSION><SESSION date="1970-01-01" number="456"><EVENTS/></SESSION>"#,
            xml
        );
    }

    #[test]
    fn serialize_two_events() {
        let mut events = Vec::new();

        let mut event = Event::new(123, 123, SwimStyle::default());
        event.order = 1.into();
        events.push(event);

        let mut event = Event::new(456, 456, SwimStyle::default());
        event.prev_event_id = 123.into();
        event.order = 2.into();
        events.push(event);

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
            events: events.into(),
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

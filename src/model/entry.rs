use chrono::Duration;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Builder)]
#[serde(rename = "ENTRY")]
#[builder(setter(strip_option))]
pub struct Entry {
    #[serde(rename = "eventid")]
    event_id: u32,

    #[serde(
        rename = "entrytime",
        default,
        with = "crate::serialization::serde_time::swim_time"
    )]
    #[builder(default)]
    entry_time: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use fast_xml::{de, se};

    use crate::collection::Collection;

    use super::*;

    #[test]
    fn deserialize_entries() {
        let result = de::from_str::<Collection<Entry>>(
            r#"<ENTRIES><ENTRY eventid="150" entrytime="00:00:01.25"/><ENTRY eventid="280"/></ENTRIES>"#,
        );
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(2, entries.len());

        let first = entries.first().unwrap();
        assert_eq!(150, first.event_id);
        assert_eq!(1, first.entry_time.unwrap().num_seconds());
        assert_eq!(1250, first.entry_time.unwrap().num_milliseconds());
    }

    #[test]
    fn serialize_entries() {
        let entries = Collection::<Entry>::from(vec![
            Entry {
                event_id: 64,
                ..Default::default()
            },
            Entry {
                event_id: 48,
                entry_time: Some(Duration::seconds(5) + Duration::milliseconds(500)),
                ..Default::default()
            },
        ]);

        let result = se::to_string(&entries);
        assert!(result.is_ok());
        assert_eq!("<ENTRIES><ENTRY eventid=\"64\"/><ENTRY eventid=\"48\" entrytime=\"00:00:05.50\"/></ENTRIES>", result.unwrap());
    }
}

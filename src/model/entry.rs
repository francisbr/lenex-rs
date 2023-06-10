use chrono::Duration;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "ENTRY")]
pub struct Entry {
    #[serde(rename = "eventid")]
    event_id: u32,

    #[serde(
        rename = "entrytime",
        default,
        with = "crate::serialization::serde_time::swim_time"
    )]
    entry_time: Option<Duration>,
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "ENTRIES")]
pub struct Entries {
    #[serde(rename = "ENTRY")]
    items: Vec<Entry>,
}

impl From<Vec<Entry>> for Entries {
    fn from(items: Vec<Entry>) -> Self {
        Self { items }
    }
}

impl Entries {
    pub fn items_owned(self) -> Vec<Entry> {
        self.items
    }

    pub fn items(&self) -> &Vec<Entry> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<Entry> {
        &mut self.items
    }
}

struct EntriesVisitor;

impl<'de> Deserialize<'de> for Entries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(EntriesVisitor)
    }
}

impl<'de> Visitor<'de> for EntriesVisitor {
    type Value = Entries;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("the entries")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut entries: Vec<Entry> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, Entry>()? {
            if key.eq("ENTRY") {
                entries.push(value);
            }
        }

        return Ok(Entries::from(entries));
    }
}

#[cfg(test)]
mod tests {
    use fast_xml::{de, se};

    use super::*;

    #[test]
    fn deserialize_entries() {
        let result = de::from_str::<Entries>(
            r#"<ENTRIES><ENTRY eventid="150" entrytime="00:00:01.25"/><ENTRY eventid="280"/></ENTRIES>"#,
        );
        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(2, entries.items().len());

        let first = entries.items().first().unwrap();
        assert_eq!(150, first.event_id);
        assert_eq!(1, first.entry_time.unwrap().num_seconds());
        assert_eq!(1250, first.entry_time.unwrap().num_milliseconds());
    }

    #[test]
    fn serialize_entries() {
        let entries = Entries::from(vec![
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

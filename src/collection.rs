use std::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize,
};

use crate::model::{AgeGroup, Athlete, Club, Entry, Event, Fee, Meet, Session};

#[derive(Debug, PartialEq)]
pub struct Collection<I> {
    collection_key: &'static str,
    item_key: &'static str,
    items: Vec<I>,
}

impl<I> Default for Collection<I> {
    fn default() -> Self {
        Self {
            collection_key: "UNKNOWN",
            item_key: "UNKNOWN",
            items: Vec::new(),
        }
    }
}

impl From<Vec<AgeGroup>> for Collection<AgeGroup> {
    fn from(value: Vec<AgeGroup>) -> Self {
        Collection {
            collection_key: "AGEGROUPS",
            item_key: "AGEGROUP",
            items: value,
        }
    }
}

impl From<Vec<Athlete>> for Collection<Athlete> {
    fn from(value: Vec<Athlete>) -> Self {
        Collection {
            collection_key: "ATHLETES",
            item_key: "ATHLETE",
            items: value,
        }
    }
}

impl From<Vec<Club>> for Collection<Club> {
    fn from(value: Vec<Club>) -> Self {
        Collection {
            collection_key: "CLUBS",
            item_key: "CLUB",
            items: value,
        }
    }
}

impl From<Vec<Entry>> for Collection<Entry> {
    fn from(value: Vec<Entry>) -> Self {
        Collection {
            collection_key: "ENTRIES",
            item_key: "ENTRY",
            items: value,
        }
    }
}

impl From<Vec<Event>> for Collection<Event> {
    fn from(value: Vec<Event>) -> Self {
        Collection {
            collection_key: "EVENTS",
            item_key: "EVENT",
            items: value,
        }
    }
}

impl From<Vec<Fee>> for Collection<Fee> {
    fn from(value: Vec<Fee>) -> Self {
        Collection {
            collection_key: "FEES",
            item_key: "FEE",
            items: value,
        }
    }
}

impl From<Vec<Meet>> for Collection<Meet> {
    fn from(value: Vec<Meet>) -> Self {
        Collection {
            collection_key: "MEETS",
            item_key: "MEET",
            items: value,
        }
    }
}

impl From<Vec<Session>> for Collection<Session> {
    fn from(value: Vec<Session>) -> Self {
        Collection {
            collection_key: "SESSIONS",
            item_key: "SESSION",
            items: value,
        }
    }
}

impl<I> AsRef<Vec<I>> for Collection<I> {
    fn as_ref(&self) -> &Vec<I> {
        &self.items
    }
}

impl<I> Deref for Collection<I> {
    type Target = Vec<I>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<I> DerefMut for Collection<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<I> Serialize for Collection<I>
where
    I: Serialize,
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut t = s.serialize_struct(self.collection_key, 1)?;
        t.serialize_field(self.item_key, &self.items)?;
        t.end()
    }
}

impl<'de, I> Deserialize<'de> for Collection<I>
where
    I: Deserialize<'de>,
    Collection<I>: From<Vec<I>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = CollectionVisitor::default();

        deserializer.deserialize_map(v)
    }
}

struct CollectionVisitor<I> {
    p: PhantomData<I>,
}

impl<I> Default for CollectionVisitor<I> {
    fn default() -> Self {
        Self {
            p: PhantomData::default(),
        }
    }
}

impl<'de, I> Visitor<'de> for CollectionVisitor<I>
where
    I: Deserialize<'de>,
    Collection<I>: From<Vec<I>>,
{
    type Value = Collection<I>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a collection of items")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut items = map.size_hint().map_or(Vec::new(), Vec::with_capacity);

        while let Some((_, value)) = map.next_entry::<String, I>()? {
            items.push(value);
        }

        Ok(items.into())
    }
}

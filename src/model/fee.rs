use core::fmt;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "FEE")]
pub struct Fee {
    r#type: String,
    value: u64,
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "FEES")]
pub struct Fees {
    #[serde(rename = "FEE")]
    pub items: Vec<Fee>,
}

impl Into<Vec<Fee>> for Fees {
    fn into(self) -> Vec<Fee> {
        self.items
    }
}

impl From<Vec<Fee>> for Fees {
    fn from(fees: Vec<Fee>) -> Self {
        Fees { items: fees }
    }
}

impl<'de> Deserialize<'de> for Fees {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(FeesVisitor)
    }
}

struct FeesVisitor;

impl<'de> Visitor<'de> for FeesVisitor {
    type Value = Fees;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the events")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut fees: Vec<Fee> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, Fee>()? {
            if key.eq("FEE") {
                fees.push(value);
            }
        }

        return Ok(fees.into());
    }
}

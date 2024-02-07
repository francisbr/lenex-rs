use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Fee {
    r#type: String,
    value: u64,
}

pub(super) mod vec_serializer {
    use std::fmt::{self, Formatter};

    use serde::{
        de::{MapAccess, Visitor},
        Serialize, Serializer,
    };

    use super::Fee;

    pub fn serialize<S>(value: &Vec<Fee>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Collection<'a> {
            #[serde(rename = "FEE")]
            items: &'a Vec<Fee>,
        }

        Collection::serialize(&Collection { items: value }, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Fee>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Vec<Fee>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("the fees")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut items = map.size_hint().map_or(Vec::new(), Vec::with_capacity);

                while let Some((_, value)) = map.next_entry::<String, Fee>()? {
                    items.push(value);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_map(MyVisitor)
    }
}

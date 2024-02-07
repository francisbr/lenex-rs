use serde::{Deserialize, Serialize};

use super::{
    age_date::AgeDate,
    club::{self, Club},
    course::Course,
    fee::{self, Fee},
    pool::Pool,
    session::{self, Session},
    timing::Timing,
    Facility, PointTable, Qualify,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "MEET")]
pub struct Meet {
    name: String,

    #[serde(rename = "name.en")]
    name_en: Option<String>,

    city: String,

    #[serde(rename = "city.en")]
    city_en: Option<String>,

    nation: String,

    course: Option<Course>,

    #[serde(rename = "reservecount")]
    reserve_count: Option<u32>,

    #[serde(rename = "startmethod")]
    start_method: Option<u32>,

    timing: Option<Timing>,

    #[serde(rename = "AGEDATE")]
    age_date: Option<AgeDate>,

    #[serde(rename = "POOL")]
    pool: Option<Pool>,

    #[serde(rename = "FACILITY")]
    facility: Option<Facility>,

    #[serde(rename = "POINTTABLE")]
    point_table: Option<PointTable>,

    #[serde(rename = "FEES", with = "fee::vec_serializer", default)]
    fees: Vec<Fee>,

    #[serde(rename = "QUALIFY")]
    qualify: Option<Qualify>,

    #[serde(rename = "SESSIONS", with = "session::vec_serializer")]
    sessions: Vec<Session>,

    #[serde(rename = "CLUBS", with = "club::vec_serializer")]
    clubs: Vec<Club>,
}

impl Meet {
    pub fn new(name: String, nation: String, city: String, sessions: Vec<Session>) -> Self {
        Self {
            name,
            nation,
            city,
            sessions,
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

    use super::Meet;

    pub fn serialize<S>(value: &Vec<Meet>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Collection<'a> {
            #[serde(rename = "MEET")]
            items: &'a Vec<Meet>,
        }

        Collection::serialize(&Collection { items: value }, serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Meet>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Vec<Meet>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("the meets")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut items = map.size_hint().map_or(Vec::new(), Vec::with_capacity);

                while let Some((_, value)) = map.next_entry::<String, Meet>()? {
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
    use super::*;

    #[test]
    fn test_deserialize_fees() {
        let result = fast_xml::de::from_str::<Meet>(
            "<MEET name=\"\" city=\"\" nation=\"\"><FEES><FEE type=\"my_type\" value=\"0\"/></FEES><SESSIONS/><CLUBS/></MEET>",
        );
        assert!(result.is_ok());

        let meet = result.unwrap();
        assert_eq!(1, meet.fees.len());
    }

    #[test]
    fn test_deserialize_sessions() {
        let result = fast_xml::de::from_str::<Meet>(
            r#"<MEET name="" city="" nation=""><SESSIONS><SESSION date="2023-02-11" number="123"><EVENTS/></SESSION></SESSIONS><CLUBS/></MEET>"#,
        );
        assert!(result.is_ok());

        let meet = result.unwrap();
        assert_eq!(1, meet.sessions.len());
    }

    #[test]
    fn test_serialize_fees() {
        let meet = Meet {
            fees: vec![Fee::default()],
            ..Default::default()
        };

        let result = fast_xml::se::to_string(&meet);

        assert!(result.is_ok());
        assert_eq!(
            "<MEET><FEES><FEE value=\"0\"/></FEES><SESSIONS/><CLUBS/></MEET>",
            &result.unwrap()
        );
    }

    #[test]
    fn test_serialize_sessions() {
        let meet = Meet {
            sessions: vec![Session::default()],
            ..Default::default()
        };

        let result = fast_xml::se::to_string(&meet);

        assert!(result.is_ok());
        assert_eq!(
            r#"<MEET><FEES/><SESSIONS><SESSION date="1970-01-01" number="0"><EVENTS/></SESSION></SESSIONS><CLUBS/></MEET>"#,
            &result.unwrap()
        );
    }
}

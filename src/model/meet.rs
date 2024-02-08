use serde::{Deserialize, Serialize};

use crate::collection::Collection;

use super::{
    age_date::AgeDate, club::Club, course::Course, fee::Fee, pool::Pool, session::Session,
    timing::Timing, Facility, PointTable, Qualify,
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

    #[serde(rename = "FEES", default)]
    fees: Collection<Fee>,

    #[serde(rename = "QUALIFY")]
    qualify: Option<Qualify>,

    #[serde(rename = "SESSIONS")]
    sessions: Collection<Session>,

    #[serde(rename = "CLUBS")]
    clubs: Collection<Club>,
}

impl Meet {
    pub fn new(name: String, nation: String, city: String, sessions: Vec<Session>) -> Self {
        Self {
            name,
            nation,
            city,
            sessions: sessions.into(),
            ..Default::default()
        }
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
            fees: vec![Fee::default()].into(),
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
            sessions: vec![Session::default()].into(),
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

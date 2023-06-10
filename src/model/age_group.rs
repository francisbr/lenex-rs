use std::fmt;

use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Serialize,
};

use self::calculate::Calculate;

use super::gender::Gender;

pub mod calculate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename = "AGEGROUP")]
pub struct AgeGroup {
    #[serde(rename = "agegroupid")]
    pub id: u32,

    #[serde(rename = "agemin", default, with = "crate::serialization::serde_age")]
    pub age_min: Option<u8>,

    #[serde(rename = "agemax", default, with = "crate::serialization::serde_age")]
    pub age_max: Option<u8>,

    #[serde(default)]
    pub gender: Gender,

    #[serde(default)]
    pub calculate: Calculate,

    pub name: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Default)]
#[serde(rename = "AGEGROUPS")]
pub(crate) struct AgeGroups {
    #[serde(rename = "AGEGROUP")]
    items: Vec<AgeGroup>,
}

impl From<Vec<AgeGroup>> for AgeGroups {
    fn from(value: Vec<AgeGroup>) -> Self {
        Self { items: value }
    }
}

impl AgeGroups {
    pub fn items(&self) -> &Vec<AgeGroup> {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<AgeGroup> {
        &mut self.items
    }
}

impl<'de> Deserialize<'de> for AgeGroups {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(AgeGroupsVisitor)
    }
}

struct AgeGroupsVisitor;

impl<'de> Visitor<'de> for AgeGroupsVisitor {
    type Value = AgeGroups;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the age groups")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut age_groups: Vec<AgeGroup> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry::<String, AgeGroup>()? {
            if key.eq("AGEGROUP") {
                age_groups.push(value);
            }
        }

        return Ok(age_groups.into());
    }
}

#[cfg(test)]
mod tests {
    use fast_xml::{de, se};

    use super::*;

    #[test]
    fn deserialize_empty_age_group() {
        let result = de::from_str::<AgeGroup>(r#"<AGEGROUP/>"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_basic_age_group() {
        let result = de::from_str::<AgeGroup>(r#"<AGEGROUP agegroupid="123"/>"#);
        assert!(result.is_ok());

        let age_group = result.unwrap();

        assert_eq!(123, age_group.id);
        assert!(age_group.age_min.is_none());
        assert!(age_group.age_max.is_none());
        assert_eq!(Gender::default(), age_group.gender);
        assert_eq!(Calculate::Single, age_group.calculate);
    }

    #[test]
    fn deserialize_vetor() {
        let result = de::from_str::<AgeGroups>(
            r#"<AGEGROUPS><AGEGROUP agegroupid="123"/><AGEGROUP agegroupid="456"/></AGEGROUPS"#,
        );
        assert!(result.is_ok());
        let age_groups = result.unwrap();

        assert_eq!(2, age_groups.items.len());
        assert_eq!(123, age_groups.items.get(0).unwrap().id);
    }

    #[test]
    fn deserialize_mixed_age_group() {
        let result = de::from_str::<AgeGroup>(
            r#"<AGEGROUP agegroupid="123" agemin="13" agemax="14" name="13-14 mixed"/>"#,
        );
        assert!(result.is_ok());

        let age_group = result.unwrap();

        assert_eq!(123, age_group.id);

        assert!(age_group.name.is_some());
        assert!(age_group.name.unwrap().eq("13-14 mixed".into()));

        assert!(age_group.age_min.is_some());
        assert_eq!(13, age_group.age_min.unwrap());
        assert!(age_group.age_max.is_some());
        assert_eq!(14, age_group.age_max.unwrap());

        assert_eq!(Gender::default(), age_group.gender);
        assert_eq!(Calculate::Single, age_group.calculate);
    }

    #[test]
    fn deserialize_male_age_group() {
        let result = de::from_str::<AgeGroup>(
            r#"<AGEGROUP agegroupid="123" agemin="-1" agemax="-1" gender="M" />"#,
        );
        assert!(result.is_ok());

        let age_group = result.unwrap();

        assert_eq!(123, age_group.id);
        assert!(age_group.age_min.is_none());
        assert!(age_group.age_max.is_none());
        assert_eq!(Gender::Male, age_group.gender);
        assert_eq!(Calculate::Single, age_group.calculate);
    }

    #[test]
    fn serialize_age_group() {
        let age_group = AgeGroup {
            id: 123,
            age_min: Some(13),
            age_max: None,
            gender: Gender::Female,
            name: Some("13+ female".into()),
            ..Default::default()
        };

        let result = se::to_string(&age_group);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<AGEGROUP agegroupid="123" agemin="13" agemax="-1" gender="F" name="13+ female"/>"#,
            xml
        );
    }

    #[test]
    fn serialize_mixed_age_group() {
        let age_group = AgeGroup {
            id: 123,
            age_min: Some(13),
            age_max: None,
            gender: Gender::default(),
            name: Some("13+ female".into()),
            ..Default::default()
        };

        let result = se::to_string(&age_group);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<AGEGROUP agegroupid="123" agemin="13" agemax="-1" name="13+ female"/>"#,
            xml
        );
    }

    #[test]
    fn serialize_vetor() {
        let age_groups = AgeGroups {
            items: vec![
                AgeGroup {
                    id: 123,
                    age_min: None,
                    age_max: None,
                    gender: Gender::default(),
                    calculate: Calculate::default(),
                    name: Some("age group 1 name".into()),
                },
                AgeGroup {
                    id: 456,
                    age_min: None,
                    age_max: None,
                    gender: Gender::default(),
                    calculate: Calculate::default(),
                    name: Some("age group 2 name".into()),
                },
            ],
        };

        let result = se::to_string(&age_groups);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<AGEGROUPS><AGEGROUP agegroupid="123" agemin="-1" agemax="-1" name="age group 1 name"/><AGEGROUP agegroupid="456" agemin="-1" agemax="-1" name="age group 2 name"/></AGEGROUPS>"#,
            xml
        );
    }
}

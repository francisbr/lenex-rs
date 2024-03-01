use serde::{Deserialize, Serialize};

use self::calculate::Calculate;

use super::gender::Gender;

pub mod calculate;

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
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
        let result = de::from_str::<Vec<AgeGroup>>(
            r#"<AGEGROUP agegroupid="123"/><AGEGROUP agegroupid="456"/>"#,
        );
        assert!(result.is_ok());
        let age_groups = result.unwrap();

        assert_eq!(2, age_groups.len());
        assert_eq!(123, age_groups.get(0).unwrap().id);
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
        let age_groups = vec![
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
        ];

        let result = se::to_string(&age_groups);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<AGEGROUP agegroupid="123" agemin="-1" agemax="-1" name="age group 1 name"/><AGEGROUP agegroupid="456" agemin="-1" agemax="-1" name="age group 2 name"/>"#,
            xml
        );
    }
}

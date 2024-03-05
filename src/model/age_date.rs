use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct AgeDate {
    pub value: NaiveDate,
    pub r#type: AgeDateType,
}

#[derive(Serialize, Deserialize, IntoStaticStr, PartialEq, Default, Debug, Clone)]
#[serde(rename_all = "UPPERCASE", into = "&str")]
#[strum(serialize_all = "UPPERCASE")]
pub enum AgeDateType {
    Year,
    #[default]
    Date,
    Por,
    #[serde(rename = "CAN.FNQ")]
    #[strum(serialize = "CAN.FNQ")]
    CanFnq,
    Lux,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_xml::{de, se};

    #[test]
    fn serialize() {
        let value = AgeDateType::CanFnq;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("CAN.FNQ", result.unwrap());
    }

    #[test]
    fn deserialize() {
        let result = de::from_str::<AgeDateType>("CAN.FNQ");
        assert!(result.is_ok());

        assert_eq!(AgeDateType::CanFnq, result.unwrap());
    }
}

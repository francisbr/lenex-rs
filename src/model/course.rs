use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, IntoStaticStr, PartialEq, Debug, Clone)]
#[serde(rename_all = "UPPERCASE", into = "&str")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Course {
    LCM,
    SCM,
    SCY,
    SCM16,
    SCM20,
    SCM33,
    SCY20,
    SCY27,
    SCY33,
    SCY36,
    Open,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_xml::{de, se};

    #[test]
    fn serialize() {
        let value = Course::SCM16;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("SCM16", result.unwrap());

        let value = Course::Open;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("OPEN", result.unwrap());
    }

    #[test]
    fn deserialize() {
        let result = de::from_str::<Course>("SCY27");
        assert!(result.is_ok());

        assert_eq!(Course::SCY27, result.unwrap());

        let result = de::from_str::<Course>("OPEN");
        assert!(result.is_ok());

        assert_eq!(Course::Open, result.unwrap());
    }
}

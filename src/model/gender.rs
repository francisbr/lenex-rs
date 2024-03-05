use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, IntoStaticStr, PartialEq, Default, Debug, Clone)]
#[serde(into = "&str")]
#[strum()]
pub enum Gender {
    #[serde(rename(deserialize = "M"))]
    #[strum(serialize = "M")]
    Male,

    #[serde(rename(deserialize = "F"))]
    #[strum(serialize = "F")]
    Female,

    #[serde(rename(deserialize = "X"))]
    #[strum(serialize = "X")]
    Mixed,

    #[default]
    #[serde(rename(deserialize = "A"))]
    #[strum(serialize = "")]
    All,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_xml::{de, se};

    #[test]
    fn serialize() {
        let value = Gender::Male;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("M", result.unwrap());

        let value = Gender::All;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("", result.unwrap());
    }

    #[test]
    fn deserialize() {
        let result = de::from_str::<Gender>("X");
        assert!(result.is_ok());

        assert_eq!(Gender::Mixed, result.unwrap());
    }
}

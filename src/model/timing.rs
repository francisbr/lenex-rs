use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, IntoStaticStr, PartialEq, Debug, Clone)]
#[serde(rename_all = "UPPERCASE", into = "&str")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Timing {
    Automatic,
    SemiAutomatic,
    Manual1,
    Manual2,
    Manual3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_xml::{de, se};

    #[test]
    fn serialize() {
        let value = Timing::Automatic;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("AUTOMATIC", result.unwrap());
    }

    #[test]
    fn deserialize() {
        let result = de::from_str::<Timing>("MANUAL3");
        assert!(result.is_ok());

        assert_eq!(Timing::Manual3, result.unwrap());
    }
}

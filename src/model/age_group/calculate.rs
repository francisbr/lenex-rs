use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, IntoStaticStr, PartialEq, Default, Debug, Clone)]
#[serde(rename_all = "UPPERCASE", into = "&str")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Calculate {
    #[default]
    #[strum(serialize = "")]
    Single,
    Total,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_xml::{de, se};

    #[test]
    fn serialize() {
        let value = Calculate::Single;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("", result.unwrap());

        let value = Calculate::Total;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("TOTAL", result.unwrap());
    }

    #[test]
    fn deserialize() {
        let result = de::from_str::<Calculate>("TOTAL");
        assert!(result.is_ok());

        assert_eq!(Calculate::Total, result.unwrap());
    }
}

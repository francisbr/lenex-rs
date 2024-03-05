use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, IntoStaticStr, PartialEq, Default, Debug, Clone)]
#[serde(rename_all = "UPPERCASE", into = "&str")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Stroke {
    Apnea,
    Back,
    Bifins,
    Breast,
    Fly,
    Free,
    Immersion,
    Imrelay,
    Medley,
    Surface,
    #[default]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_xml::{de, se};

    #[test]
    fn serialize() {
        let value = Stroke::Surface;

        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("SURFACE", result.unwrap());
    }

    #[test]
    fn deserialize() {
        let result = de::from_str::<Stroke>("SURFACE");
        assert!(result.is_ok());

        assert_eq!(Stroke::Surface, result.unwrap());
    }
}

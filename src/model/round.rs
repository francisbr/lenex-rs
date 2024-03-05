use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, IntoStaticStr, PartialEq, Default, Debug, Clone)]
#[serde(into = "&str")]
#[strum()]
pub enum Round {
    #[default]
    #[serde(rename(deserialize = "TIM"))]
    #[strum(serialize = "")]
    TimedFinals,

    #[serde(rename(deserialize = "FHT"))]
    #[strum(serialize = "FHT")]
    FastestHeat,

    #[serde(rename(deserialize = "FIN"))]
    #[strum(serialize = "FIN")]
    Finals,

    #[serde(rename(deserialize = "SEM"))]
    #[strum(serialize = "SEM")]
    SemiFinals,

    #[serde(rename(deserialize = "QUA"))]
    #[strum(serialize = "QUA")]
    QuarterFinals,

    #[serde(rename(deserialize = "PRE"))]
    #[strum(serialize = "PRE")]
    Prelims,

    #[serde(rename(deserialize = "SOP"))]
    #[strum(serialize = "SOP")]
    SwimOffPrelims,

    #[serde(rename(deserialize = "SOS"))]
    #[strum(serialize = "SOS")]
    SwimOffSemiFinals,

    #[serde(rename(deserialize = "SOQ"))]
    #[strum(serialize = "SOQ")]
    SwimOffQuarterFinals,
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_xml::{de, se};

    #[test]
    fn serialize() {
        let value = Round::TimedFinals;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("", result.unwrap());

        let value = Round::QuarterFinals;
        let result = se::to_string(&value);
        assert!(result.is_ok());

        assert_eq!("QUA", result.unwrap());
    }

    #[test]
    fn deserialize() {
        let result = de::from_str::<Round>("SOQ");
        assert!(result.is_ok());

        assert_eq!(Round::SwimOffQuarterFinals, result.unwrap());
    }
}

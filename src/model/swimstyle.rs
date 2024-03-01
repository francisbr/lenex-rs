use serde::{Deserialize, Serialize};

use super::stroke::Stroke;

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
#[serde(rename = "SWIMSTYLE")]
pub struct SwimStyle {
    #[serde(rename = "swimstyleid")]
    pub id: u64,

    pub distance: u64,

    #[serde(rename = "relaycount")]
    pub relay_count: u64,

    pub name: Option<String>,

    pub stroke: Stroke,
}

#[cfg(test)]
mod tests {
    use fast_xml::{de, se};

    use super::*;

    #[test]
    fn deserialize_empty() {
        let result = de::from_str::<SwimStyle>(r#"<SWIMSTYLE/>"#);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_basic() {
        let result = de::from_str::<SwimStyle>(
            r#"<SWIMSTYLE swimstyleid="123" distance="50" relaycount="1" name="50m" stroke="UNKNOWN"/>"#,
        );
        assert!(result.is_ok());

        let swim_style = result.unwrap();

        assert_eq!(123, swim_style.id);
        assert_eq!(50, swim_style.distance);
        assert_eq!(1, swim_style.relay_count);
        assert!(swim_style.name.unwrap().eq("50m"));
        assert_eq!(Stroke::Unknown, swim_style.stroke);
    }

    #[test]
    fn serialize() {
        let swim_style = SwimStyle {
            id: 123,
            distance: 50,
            relay_count: 1,
            name: Some("50m swim".into()),
            stroke: Stroke::Unknown,
        };

        let result = se::to_string(&swim_style);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert_eq!(
            r#"<SWIMSTYLE swimstyleid="123" distance="50" relaycount="1" name="50m swim" stroke="UNKNOWN"/>"#,
            xml
        );
    }
}

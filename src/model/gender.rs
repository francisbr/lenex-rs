use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub enum Gender {
    #[serde(rename = "M")]
    Male,

    #[serde(rename = "F")]
    Female,

    #[serde(rename = "X")]
    Mixed,

    #[serde(rename = "A")]
    All,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::All
    }
}

// impl<'de> Deserialize<'de> for Gender {
//     fn deserialize<D>(deserializer: D) -> Result<Gender, D::Error>
//     where
//         D: de::Deserializer<'de>,
//     {
//         struct GenderVisitor;

//         impl<'de> de::Visitor<'de> for GenderVisitor {
//             type Value = Gender;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("a char with the gender")
//             }

//             fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
//             where
//                 E: de::Error,
//             {
//                 Ok(match v {
//                     "F" | "f" => Gender::Female,
//                     "M" | "m" => Gender::Male,
//                     _ => Gender::Mixed,
//                 })
//             }
//         }

//         deserializer.deserialize_str(GenderVisitor)
//     }
// }

impl Serialize for Gender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Gender::Male => serializer.serialize_char('M'),
            Gender::Female => serializer.serialize_char('F'),
            Gender::Mixed => serializer.serialize_char('X'),
            Gender::All => serializer.serialize_none(),
        }
    }
}

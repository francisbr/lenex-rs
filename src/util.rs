pub mod serde_time {
    const TIME_FORMAT: &str = "%H:%M";

    pub mod optional {
        use chrono::NaiveTime;
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;

            if let Some(s) = s {
                return Ok(Some(
                    NaiveTime::parse_from_str(&s, super::TIME_FORMAT)
                        .map_err(serde::de::Error::custom)?,
                ));
            }

            Ok(None)
        }

        pub fn serialize<S>(x: &Option<NaiveTime>, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match x {
                Some(time) => s.serialize_str(&time.format(super::TIME_FORMAT).to_string()),
                None => s.serialize_none(),
            }
        }
    }
}

pub mod serde_age {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let x: Option<i64> = Option::deserialize(deserializer)?;

        if let None = x {
            return Ok(None);
        }

        let x = x.unwrap_or(-1);

        if x < 0 {
            return Ok(None);
        }

        return Ok(Some(x as u8));
    }

    pub fn serialize<S>(x: &Option<u8>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match x {
            Some(x) => s.serialize_u8(x.to_owned()),
            None => s.serialize_i8(-1),
        }
    }
}

pub mod serde_number {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let x: Option<i32> = Option::deserialize(deserializer)?;

        if x.is_none() {
            return Ok(None);
        }

        let x = x.unwrap_or(-1);

        if x < 0 {
            return Ok(None);
        }

        return Ok(Some(x as u32));
    }

    pub fn serialize<S>(x: &Option<u32>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match x {
            Some(x) => s.serialize_u32(x.to_owned()),
            None => s.serialize_none(),
        }
    }
}

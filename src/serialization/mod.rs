pub mod serde_time {

    pub mod optional {
        use chrono::NaiveTime;
        use serde::{Deserialize, Deserializer, Serializer};

        const TIME_FORMAT: &str = "%H:%M";

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;

            if let Some(s) = s {
                return Ok(Some(
                    NaiveTime::parse_from_str(&s, TIME_FORMAT).map_err(serde::de::Error::custom)?,
                ));
            }

            Ok(None)
        }

        pub fn serialize<S>(x: &Option<NaiveTime>, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match x {
                Some(time) => s.serialize_str(&time.format(TIME_FORMAT).to_string()),
                None => s.serialize_none(),
            }
        }
    }

    pub mod swim_time {
        use chrono::{Duration, NaiveTime};
        use serde::{Deserialize, Deserializer, Serializer};

        const TIME_FORMAT: &str = "%H:%M:%S.%3f";

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Option<String> = Option::deserialize(deserializer)?;

            match s {
                Some(mut value) => {
                    let mut parts = Vec::new();
                    for (index, part) in value.split(".").enumerate() {
                        let mut s = part.to_string();

                        if index == 1 {
                            s = format!("{s:0<3}");
                        }

                        parts.push(s)
                    }
                    value = parts.join(".");

                    let time = NaiveTime::parse_from_str(&value, TIME_FORMAT)
                        .map_err(serde::de::Error::custom)?;
                    let duration = time.signed_duration_since(
                        NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default(),
                    );

                    Ok(Some(duration))
                }
                None => Ok(None),
            }
        }

        pub fn serialize<S>(x: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let duration = match x {
                Some(duration) => duration,
                None => {
                    return s.serialize_none();
                }
            };

            let seconds = format!("{:.2}", duration.num_milliseconds() as f32 / 1000.);
            s.serialize_str(&format!(
                "{:0>2}:{:0>2}:{:0>5}",
                duration.num_hours(),
                duration.num_minutes(),
                seconds
            ))
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

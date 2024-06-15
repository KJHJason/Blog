use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};
use std::str::FromStr;

pub fn get_dtnow_str() -> String {
    Utc::now().to_rfc3339()
}

pub fn get_readable_date_diff(timestamp: DateTime<Utc>) -> String {
    let time_diff = Utc::now().signed_duration_since(timestamp);
    if time_diff.num_days() >= 365 {
        format!("{}y ago", time_diff.num_days() / 365)
    } else if time_diff.num_days() >= 30 {
        format!("{}mo ago", time_diff.num_days() / 30)
    } else if time_diff.num_days() >= 1 {
        format!("{}d ago", time_diff.num_days())
    } else if time_diff.num_hours() >= 1 {
        format!("{}h ago", time_diff.num_hours())
    } else if time_diff.num_minutes() >= 1 {
        format!("{}m ago", time_diff.num_minutes())
    } else if time_diff.num_seconds() > 0 {
        format!("{}s ago", time_diff.num_seconds())
    } else {
        "just now".to_string()
    }
}

pub mod rfc3339 {
    use super::*;

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::from_str(&s).map_err(serde::de::Error::custom)
    }

    pub mod option {
        use super::*;

        pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match date {
                Some(d) => serializer.serialize_some(&d.to_rfc3339()),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let opt = Option::<String>::deserialize(deserializer)?;
            match opt {
                Some(s) => DateTime::from_str(&s)
                    .map(Some)
                    .map_err(serde::de::Error::custom),
                None => Ok(None),
            }
        }
    }
}

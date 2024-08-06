use std::{fmt::Display, str::FromStr};

#[cfg(feature = "condition")]
use crate::condition::Condition;

use cfg_if::cfg_if;
use chrono::NaiveDateTime;
use serde::{de, Deserialize, Deserializer};
const FORMAT_DATETIME: &str = "%Y-%m-%d %H:%M:%S";
pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn from_str_tof32<'de, D>(d: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(d)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0f32))
}

#[cfg(feature = "des_opt_timestamp")]
pub fn from_optional_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Ok(s) = i64::deserialize(deserializer) {
        match chrono::DateTime::from_timestamp(s, 0) {
            None => {
                panic!("{s}")
            }
            Some(a) => {
                let date = a.checked_add_days(chrono::Days::new(1)).unwrap();
                Ok(Some(date.date_naive()))
            }
        }
    } else {
        Ok(None)
    }
}

pub fn from_optional_naivedatetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Ok(s) = String::deserialize(deserializer) {
        match NaiveDateTime::parse_from_str(&s, FORMAT_DATETIME) {
            Ok(a) => Ok(Some(a)),
            Err(err) => Err(de::Error::custom(err)),
        }
    } else {
        Ok(None)
    }
}

pub fn from_str_optional<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    if let Ok(s) = String::deserialize(deserializer) {
        if s.is_empty() || s == "null" {
            return Ok(None);
        };
        match T::from_str(&s) {
            Ok(a) => Ok(Some(a)),
            Err(err) => Err(de::Error::custom(err)),
        }
    } else {
        Ok(None)
    }
}
#[cfg(feature = "condition")]
pub fn deserialize_state<'de, D>(deserializer: D) -> Result<Option<Condition>, D::Error>
where
    D: de::Deserializer<'de>,
{
    if let Ok(s) = de::Deserialize::deserialize(deserializer) {
        Ok(Some(Condition::from(s)))
    } else {
        Ok(Some(Condition::New))
    }
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    if let Ok(s) = de::Deserialize::deserialize(deserializer) {
        match s {
            "1" | "true" => Ok(true),
            "0" | "false" => Ok(false),
            _ => Err(de::Error::unknown_variant(s, &["1", "0", "true", "false"])),
        }
    } else {
        Ok(false)
    }
}
#[cfg(feature = "des_opt_bool")]
pub fn deserialize_optional_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    if let Ok(s) = String::deserialize(deserializer) {
        match s.as_str() {
            "" | "none" => Ok(None),
            "1" | "true" => Ok(Some(true)),
            "0" | "false" => Ok(None),
            _ => Err(de::Error::unknown_variant(
                s.as_str(),
                &["1", "0", "true", "false"],
            )),
        }
    } else {
        Ok(None)
    }
}

cfg_if! {
if #[cfg(feature = "ser_opt_bool_to_int")] {
use serde::Serializer;
pub fn serialize_optional_bool_to_integer<S>(
    maybe: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match maybe {
        None => serializer.serialize_i8(0),
        Some(b) => match b {
            true => serializer.serialize_i8(1),
            false => serializer.serialize_i8(0),
        },
    }
}
}

}

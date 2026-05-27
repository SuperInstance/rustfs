// Copyright 2024 RustFS Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Serde helpers for expiration timestamp: serialize as RFC3339 (MinIO-compatible),
//! deserialize from RFC3339 or legacy RustFS human-readable format.

use time::OffsetDateTime;
use time::format_description;
use time::format_description::well_known::Rfc3339;

static LEGACY_FORMAT: std::sync::OnceLock<time::format_description::OwnedFormatItem> = std::sync::OnceLock::new();

fn legacy_format() -> &'static time::format_description::OwnedFormatItem {
    LEGACY_FORMAT.get_or_init(|| {
        format_description::parse_owned::<2>(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
        )
        .expect("legacy format description is valid")
    });
    LEGACY_FORMAT.get().expect("initialized above")
}

fn parse_rfc3339_or_legacy(s: &str) -> Result<OffsetDateTime, time::Error> {
    OffsetDateTime::parse(s, &Rfc3339).or_else(|_| OffsetDateTime::parse(s, legacy_format()).map_err(Into::into))
}

/// Option<OffsetDateTime>: serialize as RFC3339; deserialize from RFC3339 or legacy.
pub mod option {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    use super::{Rfc3339, parse_rfc3339_or_legacy};

    pub fn serialize<S>(opt: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(dt) => {
                let s = dt.format(&Rfc3339).map_err(serde::ser::Error::custom)?;
                serializer.serialize_some(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<&str> = Option::deserialize(deserializer)?;
        match opt {
            None => Ok(None),
            Some(s) => parse_rfc3339_or_legacy(s).map(Some).map_err(serde::de::Error::custom),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use time::OffsetDateTime;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Wrapper {
        #[serde(default, with = "crate::serde_datetime::option")]
        expiry: Option<OffsetDateTime>,
    }

    #[test]
    fn test_serialize_none() {
        let w = Wrapper { expiry: None };
        let json = serde_json::to_string(&w).unwrap();
        assert_eq!(json, r#"{"expiry":null}"#);
    }

    #[test]
    fn test_serialize_some_roundtrip_rfc3339() {
        let dt = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let w = Wrapper { expiry: Some(dt) };
        let json = serde_json::to_string(&w).unwrap();
        let back: Wrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(back.expiry, Some(dt));
    }

    #[test]
    fn test_deserialize_null() {
        let json = r#"{"expiry":null}"#;
        let w: Wrapper = serde_json::from_str(json).unwrap();
        assert!(w.expiry.is_none());
    }

    #[test]
    fn test_deserialize_rfc3339() {
        let json = r#"{"expiry":"2025-01-01T00:00:00Z"}"#;
        let w: Wrapper = serde_json::from_str(json).unwrap();
        assert!(w.expiry.is_some());
        let dt = w.expiry.unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), time::Month::January);
        assert_eq!(dt.day(), 1);
    }

    #[test]
    fn test_deserialize_legacy_format() {
        let json = r#"{"expiry":"2025-03-07 12:30:00.000000000 +00:00:00"}"#;
        let w: Wrapper = serde_json::from_str(json).unwrap();
        assert!(w.expiry.is_some());
        let dt = w.expiry.unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), time::Month::March);
        assert_eq!(dt.day(), 7);
        assert_eq!(dt.hour(), 12);
        assert_eq!(dt.minute(), 30);
    }
}

//! 🕰️ Owned host-temporal presentation contract for renderer surfaces.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const HOST_TEMPORAL_FORMAT_MAX_VALUES: usize = 64;
pub const HOST_TEMPORAL_FORMAT_MIN_TIMESTAMP_MS: i64 = -9_007_199_254_740_991;
pub const HOST_TEMPORAL_FORMAT_MAX_TIMESTAMP_MS: i64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostHourCycleV1 {
    #[serde(rename = "h11")]
    H11,
    #[serde(rename = "h12")]
    H12,
    #[serde(rename = "h23")]
    H23,
    #[serde(rename = "h24")]
    H24,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HostTemporalFormatV1 {
    Time,
    Date,
    DateTime,
    Relative,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum HostTemporalSourceV1 {
    EpochMs {
        #[serde(rename = "timestampMs")]
        timestamp_ms: i64,
    },
    Iso {
        iso: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostTemporalValueV1 {
    pub id: String,
    pub source: HostTemporalSourceV1,
    pub format: HostTemporalFormatV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostTemporalFormatRequestV1 {
    pub now_ms: i64,
    pub values: Vec<HostTemporalValueV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostTemporalProfileV1 {
    pub locale: String,
    pub time_zone: String,
    pub hour_cycle: HostHourCycleV1,
    pub profile_revision: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostTemporalLabelV1 {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostTemporalFormatReplyV1 {
    pub profile: HostTemporalProfileV1,
    pub labels: Vec<HostTemporalLabelV1>,
}

fn temporal_iso_digits(value: &str, start: usize, length: usize) -> Option<u32> {
    value.get(start..start + length)?.parse().ok()
}

/// 🧭 Validates the deterministic ISO subset shared by browser and native host formatters.
pub fn is_host_temporal_iso_v1(value: &str) -> bool {
    let date_only = value.len() == 10;
    if value.len() < 10 || value.as_bytes().get(4) != Some(&b'-') || value.as_bytes().get(7) != Some(&b'-') {
        return false;
    }
    if !date_only {
        if value.len() < 17 || value.as_bytes().get(10) != Some(&b'T') || value.as_bytes().get(13) != Some(&b':') {
            return false;
        }
        let has_seconds = value.as_bytes().get(16) == Some(&b':');
        let mut cursor = if has_seconds { 19 } else { 16 };
        if has_seconds && value.as_bytes().get(cursor) == Some(&b'.') {
            cursor += 1;
            let start = cursor;
            while value.as_bytes().get(cursor).is_some_and(u8::is_ascii_digit) {
                cursor += 1;
            }
            if cursor == start {
                return false;
            }
        }
        match value.get(cursor..) {
            Some("Z") => {}
            Some(offset) if offset.len() == 6 && matches!(offset.as_bytes()[0], b'+' | b'-') && offset.as_bytes()[3] == b':' => {
                let Some(hours) = offset.get(1..3).and_then(|value| value.parse::<u32>().ok()) else { return false };
                let Some(minutes) = offset.get(4..6).and_then(|value| value.parse::<u32>().ok()) else { return false };
                if hours > 23 || minutes > 59 {
                    return false;
                }
            }
            _ => return false,
        }
    }
    let Some(year) = temporal_iso_digits(value, 0, 4) else { return false };
    let Some(month) = temporal_iso_digits(value, 5, 2) else { return false };
    let Some(day) = temporal_iso_digits(value, 8, 2) else { return false };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if !(1..=12).contains(&month) || !(1..=month_days[(month - 1) as usize]).contains(&day) {
        return false;
    }
    if date_only {
        return true;
    }
    let Some(hour) = temporal_iso_digits(value, 11, 2) else { return false };
    let Some(minute) = temporal_iso_digits(value, 14, 2) else { return false };
    let second = if value.as_bytes().get(16) == Some(&b':') { temporal_iso_digits(value, 17, 2) } else { Some(0) };
    hour <= 23 && minute <= 59 && second.is_some_and(|second| second <= 59)
}

impl HostTemporalFormatRequestV1 {
    pub fn validate(&self) -> bool {
        if !(HOST_TEMPORAL_FORMAT_MIN_TIMESTAMP_MS..=HOST_TEMPORAL_FORMAT_MAX_TIMESTAMP_MS).contains(&self.now_ms) || self.values.is_empty() || self.values.len() > HOST_TEMPORAL_FORMAT_MAX_VALUES {
            return false;
        }
        let mut ids = BTreeSet::new();
        self.values.iter().all(|value| {
            !value.id.is_empty()
                && value.id.len() <= 256
                && ids.insert(value.id.as_str())
                && match &value.source {
                    HostTemporalSourceV1::EpochMs { timestamp_ms } => (HOST_TEMPORAL_FORMAT_MIN_TIMESTAMP_MS..=HOST_TEMPORAL_FORMAT_MAX_TIMESTAMP_MS).contains(timestamp_ms),
                    HostTemporalSourceV1::Iso { iso } => !iso.is_empty() && iso.len() <= 256,
                }
        })
    }
}

impl HostTemporalFormatReplyV1 {
    pub fn matches(&self, request: &HostTemporalFormatRequestV1) -> bool {
        if !request.validate()
            || self.profile.profile_revision == 0
            || self.profile.profile_revision > HOST_TEMPORAL_FORMAT_MAX_TIMESTAMP_MS as u64
            || self.profile.locale.is_empty()
            || self.profile.locale.len() > 64
            || self.profile.time_zone.is_empty()
            || self.profile.time_zone.len() > 64
            || self.labels.len() != request.values.len()
        {
            return false;
        }
        let requested = request.values.iter().map(|value| value.id.as_str()).collect::<BTreeSet<_>>();
        let labels = self.labels.iter().map(|label| label.id.as_str()).collect::<BTreeSet<_>>();
        labels.len() == self.labels.len() && labels == requested && self.labels.iter().all(|label| !label.text.is_empty() && label.text.len() <= 256)
    }

    pub fn label(&self, id: &str) -> Option<&str> {
        self.labels.iter().find(|label| label.id == id).map(|label| label.text.as_str())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🕰️host-temporal-format/🦀️.rs"]
mod tests;

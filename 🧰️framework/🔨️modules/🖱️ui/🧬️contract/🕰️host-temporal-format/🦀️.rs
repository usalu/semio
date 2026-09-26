//! @emoji 🕰️ Owned host-temporal presentation contract for renderer surfaces.

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
    Iso { iso: String },
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

impl HostTemporalFormatRequestV1 {
    pub fn validate(&self) -> bool {
        if !(HOST_TEMPORAL_FORMAT_MIN_TIMESTAMP_MS..=HOST_TEMPORAL_FORMAT_MAX_TIMESTAMP_MS).contains(&self.now_ms)
            || self.values.is_empty()
            || self.values.len() > HOST_TEMPORAL_FORMAT_MAX_VALUES
        {
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

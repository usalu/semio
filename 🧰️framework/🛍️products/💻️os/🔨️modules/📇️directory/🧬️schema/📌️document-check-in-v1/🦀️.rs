//! 📌️ Check In: a document author names a committed ledger head; the hub materializes the checkpoint
//! from its own document log and makes it the document's active checkpoint. The JSON Schema is the
//! directory schema's `DocumentCheckInV1`/`DocumentCheckInStatusV1`; the TypeScript twin is adjacent.
//!
//! See `🌎️hub/🗿️artifact-authority/📌️check-in` for the hub job.

use super::{valid_document_open_hash, EditedArtifactFrontierV1, DOCUMENT_OPEN_MAX_SAFE_INTEGER};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧯️ Maximum canonical Check In request or status bytes.
pub const DOCUMENT_CHECK_IN_MAX_BYTES: usize = 4096;
/// 📛️ Request schema identifier.
pub const DOCUMENT_CHECK_IN_SCHEMA_V1: &str = "semio.hub.document-check-in/v1";
/// 📛️ Status schema identifier.
pub const DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1: &str = "semio.hub.document-check-in-status/v1";

fn request_id(value: &str) -> bool {
    value.len() == 32 && value.bytes().any(|byte| byte != b'0') && value.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

/// 📥️ One Check In; scope comes from the route and author from the session, and no pair, pair hash,
/// or parent checkpoint is ever accepted from a client.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInV1 {
    pub schema: String,
    pub request_id: String,
    pub head: EditedArtifactFrontierV1,
}

impl DocumentCheckInV1 {
    /// 🛡️ Validates the schema-owned bounds before any ledger or catalog access.
    pub fn validate(&self) -> bool {
        self.schema == DOCUMENT_CHECK_IN_SCHEMA_V1 && request_id(&self.request_id) && self.head.validate()
    }

    /// 📦️ Rejects duplicate fields, unknown inputs, padding, and noncanonical JSON.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > DOCUMENT_CHECK_IN_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }

    /// 📤️ Emits only a canonical bounded request.
    pub fn canonical_json(&self) -> Option<String> {
        if !self.validate() {
            return None;
        }
        let source = crate::os_pack::json::to_json_string(self);
        (source.len() <= DOCUMENT_CHECK_IN_MAX_BYTES).then_some(source)
    }
}

/// 🚦️ A Check In is ready only once its checkpoint is the document's active checkpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "lowercase")]
#[value(rename_all = "lowercase")]
pub enum DocumentCheckInPhaseV1 {
    Accepted,
    Materializing,
    Publishing,
    Ready,
    Failed,
    Cancelled,
}

impl DocumentCheckInPhaseV1 {
    /// 🏁️ Ready, Failed and Cancelled never change again.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Ready | Self::Failed | Self::Cancelled)
    }
}

/// ⛔️ Why a Check In ended without advancing the active checkpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum DocumentCheckInRefusalV1 {
    UnknownHead,
    StaleHead,
    ActiveCheckpointChanged,
    LedgerNotReplayable,
    CodecRefused,
    AuthorityChanged,
    Unavailable,
}

/// 📈️ Monotonic bounded materialization progress.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInProgressV1 {
    pub completed_units: u64,
    pub total_units: u64,
}

/// 🚩️ The checkpoint a ready Check In made active.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInReadyV1 {
    pub checkpoint_id: String,
    pub parent_checkpoint_id: String,
    pub baseline: EditedArtifactFrontierV1,
}

/// 📣️ One Check In's observable state; only Ready names a checkpoint and only Failed a refusal.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentCheckInStatusV1 {
    pub schema: String,
    pub request_id: String,
    pub phase: DocumentCheckInPhaseV1,
    pub progress: DocumentCheckInProgressV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<DocumentCheckInReadyV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<DocumentCheckInRefusalV1>,
}

impl DocumentCheckInStatusV1 {
    /// 🔐️ Phase, checkpoint and refusal agree; progress never runs past its total.
    pub fn validate(&self) -> bool {
        self.schema == DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1
            && request_id(&self.request_id)
            && self.progress.completed_units <= self.progress.total_units
            && self.progress.total_units <= DOCUMENT_OPEN_MAX_SAFE_INTEGER
            && match (self.phase, &self.ready, &self.refusal) {
                (DocumentCheckInPhaseV1::Ready, Some(ready), None) => valid_document_open_hash(&ready.checkpoint_id) && valid_document_open_hash(&ready.parent_checkpoint_id) && ready.baseline.validate(),
                (DocumentCheckInPhaseV1::Failed, None, Some(_)) => true,
                (DocumentCheckInPhaseV1::Ready | DocumentCheckInPhaseV1::Failed, _, _) => false,
                (_, None, None) => true,
                _ => false,
            }
    }

    /// 📤️ Emits only a canonical bounded status.
    pub fn canonical_json(&self) -> Option<String> {
        if !self.validate() {
            return None;
        }
        let source = crate::os_pack::json::to_json_string(self);
        (source.len() <= DOCUMENT_CHECK_IN_MAX_BYTES).then_some(source)
    }

    /// 🧾️ Reads one exact status, withholding malformed or overposted results.
    pub fn parse_canonical_json(source: &str) -> Option<Self> {
        if source.len() > DOCUMENT_CHECK_IN_MAX_BYTES {
            return None;
        }
        let value: Self = crate::os_pack::json::from_json_str(source).ok()?;
        (value.validate() && crate::os_pack::json::to_json_string(&value) == source).then_some(value)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

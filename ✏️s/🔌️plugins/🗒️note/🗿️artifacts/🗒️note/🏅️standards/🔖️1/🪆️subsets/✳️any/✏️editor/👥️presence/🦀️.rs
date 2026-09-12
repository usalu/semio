//! 👥️ Note play presence — shareable live ephemeral state + mutations.

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live canvas camera state. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: peer selection/hover no longer live here —
/// they broadcast automatically via the framework's typed `PresenceInteraction` (assembled from the
/// "blocks" domain's `InteractionState`, zero app code).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "note.presence")]
#[dsl(layout = "lines")]
pub struct NotePresence {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
}

impl Default for NotePresence {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0 }
    }
}

impl protocol::MutationDiff<NotePresence> for NotePresence {
    fn apply(&self, _base: &NotePresence) -> protocol::MutationApplyResult<NotePresence> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for NotePresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for NotePresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧹️Retirement
const _: () = assert!(!std::mem::needs_drop::<NotePresence>());

fn note_presence_is_terminal_empty(presence: &NotePresence) -> bool {
    presence == &NotePresence::default()
}

/// 👥️ Returns the exact local or peer Note presence root in one bounded item.
pub struct NotePresenceRetirementFactory;

impl store::SnapshotRetirementFactory<NotePresence> for NotePresenceRetirementFactory {
    fn retire(&self, root: std::sync::Arc<NotePresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(NotePresenceRetirement(std::mem::ManuallyDrop::new(Some(root))))
    }
}

struct NotePresenceRetirement(std::mem::ManuallyDrop<Option<std::sync::Arc<NotePresence>>>);

impl store::ErasedSnapshotRetirement for NotePresenceRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if self.0.is_none() {
            return Ok(store::SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        drop(self.0.take());
        Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl Drop for NotePresenceRetirement {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.0.is_none(), "Note presence must return its exact root before drop");
        }
    }
}

/// 🧹️ Replaces the live local root with Note's exact empty presence and retires its peer roster.
pub fn note_presence_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<NotePresence, NotePresenceMutation>>> {
    Box::new(
        semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(NotePresence::default()), note_presence_is_terminal_empty)
            .expect("default Note presence is terminal-empty"),
    )
}
//#endregion 🧹️Retirement

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;

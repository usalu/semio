//! 👥️ Note play presence — shareable live ephemeral state + mutations.

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live canvas camera state. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: peer selection/hover no longer live here —
/// they broadcast automatically via the framework's typed `PresenceInteraction` (assembled from the
/// "blocks" domain's `InteractionState`, zero app code).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "note.presence")]
#[dsl(layout = "lines")]
pub struct NotePresence {
    pub camera_x: f64,
    pub camera_y: f64,
    pub camera_zoom: f64,
}

impl store::ArtifactPresenceSnapshot for NotePresence {}

impl Default for NotePresence {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0 }
    }
}

impl store::ArtifactDsl for NotePresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for NotePresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod diff;
pub use diff::NotePresenceDiff;
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧹️Retirement
const _: () = assert!(!std::mem::needs_drop::<NotePresence>());

fn note_presence_is_terminal_empty(presence: &NotePresence) -> bool {
    presence == &NotePresence::default()
}

semio_framework_value::artifact_retire_struct!(NotePresence { camera_x, camera_y, camera_zoom });

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

#[path = "🚪️io/🦀️.rs"]
pub mod io;

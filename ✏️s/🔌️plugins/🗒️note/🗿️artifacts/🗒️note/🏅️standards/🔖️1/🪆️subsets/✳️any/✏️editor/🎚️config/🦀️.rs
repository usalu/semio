//! 🧮️ Note play app — view state (`NoteConfig`) and its operation enum (`NoteConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.note` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so camera/utility edits are VCS'd exactly like document
//! content.

use crate::NoteCamera;
#[cfg(test)]
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🧮️ Note's real `ArtifactEditor::Config` — mirrors `shooting_engine::ShootingConfig`'s pilot shape.
/// Absorbs every field that used to live on the old ui crate's `NotePlayRuntime` (the in-progress
/// engagement-rename input, and the free/live canvas camera) plus the two `ViewModel` fields the note
/// UI actually reads (`locale`/`active_utility_id`) — see `crate::editor::note::NotePlayApp::render`.
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_block_ids`/`hovered_block_id`
/// moved OUT of here into the framework-owned `InteractionState` (the "blocks" domain declared on
/// `create_note_app`) — see `crate::editor::note::NoteDispatchCtx`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
#[dsl(id = "note.config", layout = "lines")]
pub struct NoteConfig {
    /// ✏️ In-progress engagement-rename input — was `NotePlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 📷️ The free/live canvas camera — session-only, never a document field. Was
    /// `NotePlayRuntime::camera`.
    #[dsl(block)]
    pub camera: NoteCamera,
    /// 🧰️ The active canvas utility (select/pencil/eraser/…) — was read off
    /// `view_state.active_utility_id` (host-pushed `ViewModel`, deleted by the pure-trait migration).
    pub active_utility_id: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for NoteConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for NoteConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

impl Default for NoteConfig {
    fn default() -> Self {
        Self { engagement_input: String::new(), camera: NoteCamera::default(), active_utility_id: "selectDirect".into(), }
    }
}

store::impl_whole_record_config!(NoteConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;

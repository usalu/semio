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
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
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
        Self { engagement_input: String::new(), camera: NoteCamera::default(), active_utility_id: "selectDirect".into(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(NoteConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;


//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn note_config_default_matches_the_pre_migration_runtime_defaults() {
        let config = NoteConfig::default();
        assert_eq!(config.active_utility_id, "selectDirect");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.camera, NoteCamera::default());
    }

    /// 🧮️ B1 Config dsl/pack round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE).
    #[semio_framework_async_macros::async_test]
    async fn note_config_dsl_pack_round_trips() {
        let config = NoteConfig { engagement_input: "Renaming…".into(), camera: NoteCamera { x: 12.5, y: -4.0, zoom: 2.5 }, active_utility_id: "pencil".into(), locale: "de-DE".into() };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn note_config_operation_text_and_binary_round_trip_every_variant() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetEngagementInput(SetEngagementInput { value: "Renaming…".into() }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetCamera(SetCamera { camera: NoteCamera { x: 4.0, y: 5.0, zoom: 2.0 } }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: "eraserStroke".into() }));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    }

    /// ↩️ Inversion restores the utility captured before the operation.
    #[semio_framework_async_macros::async_test]
    async fn note_config_operation_inverse_restores_the_prior_utility() {
        let base = NoteConfig::default();
        let operation = NoteConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: "pencil".into() });
        assert_eq!(operation.inverse(&base), vec![NoteConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: base.active_utility_id.clone() })]);
        let next = operation.diff(&base).into_parts().0;
        assert_eq!(next.active_utility_id, "pencil");
        let restored = operation.inverse(&base)[0].diff(&next).into_parts().0;
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn note_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: NoteConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<NoteConfigMutation as Mutation<NoteConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: NoteConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(NoteConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(NoteConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
        }
    }
}

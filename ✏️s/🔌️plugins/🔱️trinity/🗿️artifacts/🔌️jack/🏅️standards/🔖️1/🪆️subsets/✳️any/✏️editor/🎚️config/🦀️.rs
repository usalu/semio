//! 🧮️ Trinity Jack app — view-state config + config operations.

use crate::artifacts::jack::Camera;
use std::collections::BTreeMap;

/// 🎯️ Ephemeral editor selection range (offsets into the jack query text).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct JackEditorSelection {
    pub start: u64,
    pub end: u64,
}

/// 🧮️ Jack's `ArtifactApp::Config` — node selection, the live node-graph viewport camera (seeded once
/// from the initial fixture's seed-only `camera` field, then only ever written by
/// `nodeGraphViewport`), the active fixture/example id, the jack query draft + its last result, the
/// three engagement-input drafts, the reorganize epoch, the editor's text selection, the per-window
/// LOD mode, a completion-request revision counter, and the BCP-47 locale tag.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "trinity.jackcfg")]
#[dsl(layout = "lines")]
pub struct JackConfig {
    #[dsl(block)]
    pub camera: Camera,
    pub active_fixture_id: String,
    pub jack_query: String,
    pub jack_result_json: String,
    pub editor_engagement_input: String,
    pub graph_engagement_input: String,
    pub results_engagement_input: String,
    pub reorganize_epoch: u64,
    #[dsl(block)]
    pub editor_selection: Option<JackEditorSelection>,
    pub lod_mode_by_window: BTreeMap<String, String>,
    pub revision: u64,
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for JackConfig {
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
impl store::ArtifactPack for JackConfig {
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

impl Default for JackConfig {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            active_fixture_id: String::new(),
            jack_query: String::new(),
            jack_result_json: String::new(),
            editor_engagement_input: String::new(),
            graph_engagement_input: String::new(),
            results_engagement_input: String::new(),
            reorganize_epoch: 0,
            editor_selection: None,
            lod_mode_by_window: BTreeMap::new(),
            revision: 0,
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(JackConfig);

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    #[semio_framework_async_macros::async_test]
    async fn jack_config_default_has_default_locale() {
        let config = JackConfig::default();
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.camera, Camera::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_config_dsl_round_trips() {
        let mut config = JackConfig { jack_query: "MATCH (a:Piece) RETURN a".into(), editor_selection: Some(JackEditorSelection { start: 3, end: 9 }), ..JackConfig::default() };
        config.lod_mode_by_window.insert("trinity-jack-graph".into(), "compact".into());
        ::store::os_store::test_support::assert_dsl_round_trip(&config);
        ::store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_config_operation_backwards_restores_prior_snapshot() {
        let base = JackConfig::default();
        let operation = JackConfigMutation::SetActiveFixture(SetActiveFixture { value: "nakagin".into() });
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.active_fixture_id, "nakagin".to_string());
        let backwards = operation.inverse(&base);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_config_operation_text_round_trips() {
        ::store::os_store::test_support::assert_op_line_round_trip(&JackConfigMutation::SetLodMode(SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() }));
        ::store::os_store::test_support::assert_op_line_round_trip(&JackConfigMutation::SetActiveFixture(SetActiveFixture { value: "nakagin".into() }));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn configuration_and_presence_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).unwrap();
        let base: JackConfig = pack::from_json_str(&vectors["base"].to_string()).unwrap();
        assert_eq!(<JackConfigMutation as Mutation<JackConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().unwrap().len());
        for vector in vectors["cases"].as_array().unwrap() {
            let mutation: JackConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).unwrap();
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).unwrap(), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
            let next = mutation.diff(&base).diff().apply(&base).unwrap();
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).unwrap(), vector["expected"]);
            assert_eq!(JackConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            assert_eq!(JackConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
            assert_eq!(restored, base);
        }
    }
}

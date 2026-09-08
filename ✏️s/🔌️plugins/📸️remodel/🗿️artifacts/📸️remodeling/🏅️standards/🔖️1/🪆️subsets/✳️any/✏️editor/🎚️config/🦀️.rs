//! 🧮️ Remodeling play app — the `ArtifactEditor::Config` view state and its operation vocabulary.
//!
//! Every former `RemodelingPlayRuntime` field (camera/selection/layers/frame cursor/report table) lives
//! here, written through `RemodelingConfigMutation`s with a real `backwards`, never ad hoc runtime
//! mutation. This is app-level, not artifact-level, precisely because it is view state: the artifact
//! must never depend on the app, so nothing under `🗿️artifacts/` may reference these types.

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎥️ Ephemeral viewport orbit camera — never persisted as document content, mirrors the pre-B1
/// `RemodelingPlayRuntime::camera`'s shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingWorldCamera {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(coord)]
    pub target: [f64; 3],
    pub fov: f64,
}

impl Default for RemodelingWorldCamera {
    fn default() -> Self {
        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }
    }
}

/// 👁️ Which `remodeling-main` point-cloud/mesh layers are visible — was `RemodelingPlayRuntime::layers`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingLayerVisibility {
    pub mesh: bool,
    pub dense: bool,
    pub sparse: bool,
    pub cameras: bool,
    pub gcps: bool,
}

impl Default for RemodelingLayerVisibility {
    fn default() -> Self {
        Self { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }
    }
}

/// 🎞️ Which frame `remodeling-frames` currently shows — was `RemodelingPlayRuntime::frame_cursor`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingFrameCursor {
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

/// 🧮️ Remodeling's `ArtifactEditor::Config` — absorbs every former `RemodelingPlayRuntime` view/session field
/// (camera/selection/layers/frame cursor/report table selection) plus the two `ViewModel`-sourced
/// fields the UI actually reads (`active_utility_id`/`locale`).
/// The live `engine::reconstruction::ReconstructionEngine` (now `crate::editor::remodeling::engine::reconstruction::ReconstructionEngine`) and the video-import blur-gate rolling
/// window are deliberately NOT here: neither is `Clone + ToValue + FromValue` in a way that
/// round-trips through a pure `&self` `handle()`. Both are rebuilt from already-persisted document
/// state instead of carried as hidden interior-mutable scratch — see `🎮️commands/🏗️run-reconstruction`
/// and `🎮️commands/🖼️import-frame-payload` for how.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
#[dsl(id = "remodeling.config", extension = "remodelingcfg")]
#[dsl(layout = "lines")]
pub struct RemodelingConfig {
    #[dsl(block)]
    pub camera: RemodelingWorldCamera,
    #[dsl(block)]
    pub layers: RemodelingLayerVisibility,
    #[dsl(block)]
    pub frame_cursor: RemodelingFrameCursor,
    /// 📊️ Which `remodeling-report` dataset is selected (`"frames"`/`"cameras"`/`"tracks"`/`"gcps"`/…).
    pub report_table: String,
    /// 🧰️ The active utility for `remodeling-main`/`remodeling-frames` — was read off `view_state.active_utility_id`.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for RemodelingConfig {
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
impl store::ArtifactPack for RemodelingConfig {
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

impl Default for RemodelingConfig {
    fn default() -> Self {
        Self { camera: RemodelingWorldCamera::default(), layers: RemodelingLayerVisibility::default(), frame_cursor: RemodelingFrameCursor::default(), report_table: "frames".into(), active_utility_id: "select".into(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(RemodelingConfig);

//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::{RemodelingConfigMutation, ReplaceConfig, SetCamera, SetLayerVisibility, SetFrameCursor, SetReportTable, SetActiveUtility, SetLocale};

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;

    #[semio_framework_async_macros::async_test]
    async fn remodeling_config_default_matches_the_former_runtime_defaults() {
        let config = RemodelingConfig::default();
        assert_eq!(config.camera, RemodelingWorldCamera { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
        assert!(config.layers.mesh && config.layers.dense && config.layers.sparse && config.layers.cameras && config.layers.gcps);
        assert_eq!(config.frame_cursor, RemodelingFrameCursor::default());
        assert_eq!(config.report_table, "frames");
        assert_eq!(config.active_utility_id, "select");
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn remodeling_config_operation_diff_is_whole_record_replace() {
        let base = RemodelingConfig::default();
        let mut next = base.clone();
        next.report_table = "gcps".into();
        assert_eq!(protocol::MutationDiff::apply(&next, &base).expect("valid config mutation diff"), next, "apply ignores base entirely, like ShootingConfig");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_mutations_apply_and_backwards_restore_the_pre_edit_snapshot() {
        let base = RemodelingConfig::default();

        let camera = RemodelingWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
        let op = RemodelingConfigMutation::SetCamera(SetCamera { camera: camera.clone() });
        let next = op.diff(&base).into_parts().0;
        assert_eq!(next.camera, camera);
        assert_eq!(op.inverse(&base), vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]);
        assert_eq!(op.inverse(&base)[0].diff(&next).into_parts().0, base, "backwards restores the exact pre-edit config");

        let op = RemodelingConfigMutation::SetLayerVisibility(SetLayerVisibility { layer: "dense".into(), visible: false });
        let next = op.diff(&base).into_parts().0;
        assert!(!next.layers.dense);
        assert!(next.layers.mesh, "only the named layer flips");

        let op = RemodelingConfigMutation::SetFrameCursor(SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 4 });
        let next = op.diff(&base).into_parts().0;
        assert_eq!(next.frame_cursor.stream_id.as_deref(), Some("stream-1"));
        assert_eq!(next.frame_cursor.frame_index, 4);

        let op = RemodelingConfigMutation::SetReportTable(SetReportTable { table: "gcps".into() });
        assert_eq!(op.diff(&base).diff().report_table, "gcps");

        let op = RemodelingConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: "measure".into() });
        assert_eq!(op.diff(&base).diff().active_utility_id, "measure");

        let op = RemodelingConfigMutation::SetLocale(SetLocale { value: "de-DE".into() });
        assert_eq!(op.diff(&base).diff().locale, "de-DE");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_mutations_roundtrip_through_op_text() {
        let config = RemodelingConfig::default();
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config }));
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetCamera(SetCamera { camera: RemodelingWorldCamera::default() }));
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetLayerVisibility(SetLayerVisibility { layer: "gcps".into(), visible: false }));
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetFrameCursor(SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 }));
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetFrameCursor(SetFrameCursor { stream_id: None, frame_index: 0 }));
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetReportTable(SetReportTable { table: "tracks".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: "gcpPlace".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&RemodelingConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    }
}
//#endregion 🧪️Tests

//! 🧮️ DAG play app — view state (`DagConfig`) and its operation enum (`DagConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.dag` document. It absorbs everything that used to live in the old
//! ui crate's `DagPlayRuntime` (an app-struct `RefCell`) AND the two fields the dag UI actually read off
//! the deleted host-pushed `ViewModel` (`locale`, via `dag_play_labels`/`app_labels`/`context_menu`): the
//! selected node ids, the free/live node-graph viewport camera, and the BCP-47 locale tag — session-only
//! view state round-trips through the config `ArtifactStore` exactly like document content, with a real
//! `backwards` per `DagConfigMutation` instead of never being VCS'd at all.

use semio_framework_artifact_infinite_dag::DagCamera;

//#region 🔖️Config
/// 🧮️ `DagPlayApp::Config` — the pure-trait `ArtifactEditor::Config` for the dag app.
///
/// The camera is flattened to its three scalar fields (`camera_x`/`camera_y`/`camera_zoom`) rather than
/// embedding `semio_framework_artifact_infinite_dag::DagCamera` as a `#[dsl(block)]`: that kernel type is
/// explicitly out of scope for this crate and doesn't derive `dsl::DslRecord` (only
/// `Clone`/`Debug`/`PartialEq`/`Serialize`/`Deserialize`), so it can't satisfy a nested-block field —
/// three plain `f64` fields need no such support at all. See `dag_config_camera` below for the seam back
/// to the real `DagCamera` type.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(extension = "dagcfg")]
#[dsl(id = "dag.config")]
#[dsl(layout = "lines")]
pub struct DagConfig {
    /// 🎥️ Viewport camera x — was `DagPlayRuntime::camera.x`.
    pub camera_x: f64,
    /// 🎥️ Viewport camera y — was `DagPlayRuntime::camera.y`.
    pub camera_y: f64,
    /// 🎥️ Viewport camera zoom — was `DagPlayRuntime::camera.zoom`.
    pub camera_zoom: f64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for DagConfig {
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
impl store::ArtifactPack for DagConfig {
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

impl Default for DagConfig {
    fn default() -> Self {
        // 🎥️ Matches `DagCamera`'s own implicit default (`x: 0.0, y: 0.0, zoom: 1.0`, see `DagFixture`'s
        // `Default` impl in the kernel crate) without needing to parse the bundled demo document just to
        // read a trivial camera default.
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, }
    }
}

store::impl_whole_record_config!(DagConfig);

/// 🎥️ Reassembles the kernel's `DagCamera` from `DagConfig`'s flattened scalar fields — the seam
/// `crate::editor::dag` uses wherever the old `DagPlayRuntime::camera` field was read.
pub fn dag_config_camera(config: &DagConfig) -> DagCamera {
    DagCamera { x: config.camera_x, y: config.camera_y, zoom: config.camera_zoom }
}
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-vectors/🦀️.rs"]
mod mutation_vectors;

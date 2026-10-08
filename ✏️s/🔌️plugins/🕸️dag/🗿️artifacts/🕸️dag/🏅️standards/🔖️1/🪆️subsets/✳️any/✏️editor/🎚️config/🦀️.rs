//! 🧮️ DAG play app — view state (`DagConfig`) and its operation enum (`DagConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.dag` document. It absorbs everything that used to live in the old
//! ui crate's `DagPlayRuntime` (an app-struct `RefCell`). The free/live node-graph viewport camera is
//! session-only view state and round-trips through the config `ArtifactStore` exactly like document
//! content, with a real `backwards` per `DagConfigMutation` instead of never being VCS'd at all.

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
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "dagcfg")]
#[artifact(id = "dag.config")]
#[dsl(layout = "lines")]
pub struct DagConfig {
    /// 🎥️ Viewport camera x — was `DagPlayRuntime::camera.x`.
    pub camera_x: f64,
    /// 🎥️ Viewport camera y — was `DagPlayRuntime::camera.y`.
    pub camera_y: f64,
    /// 🎥️ Viewport camera zoom — was `DagPlayRuntime::camera.zoom`.
    pub camera_zoom: f64,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for DagConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for DagConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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

//#endregion 🔖️ArtifactCodec

impl Default for DagConfig {
    fn default() -> Self {
        // 🎥️ Matches `DagCamera`'s own implicit default (`x: 0.0, y: 0.0, zoom: 1.0`, see `DagHostSnapshot`'s
        // `Default` impl in the kernel crate) without needing to parse the bundled demo document just to
        // read a trivial camera default.
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0 }
    }
}

impl store::ConfigRecord for DagConfig {}

/// 🔺️ Sparse field delta over [`DagConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct DagConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_x: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_y: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_zoom: Option<f64>,
}

impl protocol::MutationDiff<DagConfig> for DagConfigDiff {
    fn apply(&self, base: &DagConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<DagConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_x {
            next.camera_x = value.clone();
        }
        if let Some(value) = &self.camera_y {
            next.camera_y = value.clone();
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera_x.is_some() {
            self.camera_x = other.camera_x;
        }
        if other.camera_y.is_some() {
            self.camera_y = other.camera_y;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
    }
}

impl protocol::DiffAlgebra<DagConfig> for DagConfigDiff {
    fn inverse(&self, base: &DagConfig) -> Self {
        Self {
            camera_x: self.camera_x.as_ref().map(|_| base.camera_x.clone()),
            camera_y: self.camera_y.as_ref().map(|_| base.camera_y.clone()),
            camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom.clone()),
        }
    }
    fn between(base: &DagConfig, other: &DagConfig) -> Self {
        Self {
            camera_x: (base.camera_x != other.camera_x).then(|| other.camera_x.clone()),
            camera_y: (base.camera_y != other.camera_y).then(|| other.camera_y.clone()),
            camera_zoom: (base.camera_zoom != other.camera_zoom).then(|| other.camera_zoom.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_zoom.is_none()
    }
}


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

#[path = "🚪️io/🦀️.rs"]
pub mod io;

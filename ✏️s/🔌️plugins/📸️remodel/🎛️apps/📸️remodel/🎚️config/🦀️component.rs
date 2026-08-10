//! 🧮️ Remodel play app — the `ArtifactApp::Config` view state and its operation vocabulary.
//!
//! Every former `RemodelPlayRuntime` field (camera/selection/layers/frame cursor/report table) lives
//! here, written through `RemodelConfigMutation`s with a real `backwards`, never ad hoc runtime
//! mutation. This is app-level, not artifact-level, precisely because it is view state: the artifact
//! must never depend on the app, so nothing under `🗿️artifacts/` may reference these types.

use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎥️ Ephemeral viewport orbit camera — never persisted as document content, mirrors the pre-B1
/// `RemodelPlayRuntime::camera`'s shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelWorldCamera {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(coord)]
    pub target: [f64; 3],
    pub fov: f64,
}

impl Default for RemodelWorldCamera {
    fn default() -> Self {
        Self { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }
    }
}

/// 🖱️ Ephemeral face/vertex/object selection — was `RemodelPlayRuntime::selection`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelSelection {
    pub mode: String,
    pub ids: Vec<String>,
}

/// 👁️ Which `remodel-main` point-cloud/mesh layers are visible — was `RemodelPlayRuntime::layers`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelLayerVisibility {
    pub mesh: bool,
    pub dense: bool,
    pub sparse: bool,
    pub cameras: bool,
    pub gcps: bool,
}

impl Default for RemodelLayerVisibility {
    fn default() -> Self {
        Self { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }
    }
}

/// 🎞️ Which frame `remodel-frames` currently shows — was `RemodelPlayRuntime::frame_cursor`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelFrameCursor {
    pub stream_id: Option<String>,
    pub frame_index: u32,
}

/// 🧮️ Remodel's `ArtifactApp::Config` — absorbs every former `RemodelPlayRuntime` view/session field
/// (camera/selection/layers/frame cursor/report table selection) plus the two `ViewModel`-sourced
/// fields the UI actually reads (`active_utility_id`/`locale`).
/// The live `engine::reconstruction::ReconstructionEngine` and the video-import blur-gate rolling
/// window are deliberately NOT here: neither is `Clone + Serialize + Deserialize` in a way that
/// round-trips through a pure `&self` `handle()`. Both are rebuilt from already-persisted document
/// state instead of carried as hidden interior-mutable scratch — see `🎮️commands/🚀️reconstruction`
/// and `🎮️commands/📥️ingest` for how.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(id = "remodel.config", extension = "remodelcfg")]
#[dsl(layout = "lines")]
pub struct RemodelConfig {
    #[dsl(block)]
    pub camera: RemodelWorldCamera,
    #[dsl(block)]
    pub selection: RemodelSelection,
    #[dsl(block)]
    pub layers: RemodelLayerVisibility,
    #[dsl(block)]
    pub frame_cursor: RemodelFrameCursor,
    /// 📊️ Which `remodel-report` dataset is selected (`"frames"`/`"cameras"`/`"tracks"`/`"gcps"`/…).
    pub report_table: String,
    /// 🧰️ The active utility for `remodel-main`/`remodel-frames` — was read off `view_state.active_utility_id`.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for RemodelConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for RemodelConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


impl Default for RemodelConfig {
    fn default() -> Self {
        Self {
            camera: RemodelWorldCamera::default(),
            selection: RemodelSelection::default(),
            layers: RemodelLayerVisibility::default(),
            frame_cursor: RemodelFrameCursor::default(),
            report_table: "frames".into(),
            active_utility_id: "select".into(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(RemodelConfig);

//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `RemodelConfig`'s operation enum — one variant per settled interaction
/// (mirrors the former `RemodelPlayRuntime` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns. Mirrors `shooting_op::ShootingConfigOperation` exactly: a config-only "View"
/// dispatch is a plain `Apply` (never `AmendLast`), so each tick is its own distinct, real config edit
/// and "undo this tick" is exactly "restore the whole-config snapshot from just before it" — no
/// per-field reverse-patch bookkeeping needed. `Mutation::Diff` is the WHOLE `RemodelConfig`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum RemodelConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: RemodelConfig,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: RemodelWorldCamera,
    },
    #[dsl(key = "selection")]
    SetSelection { mode: String, ids: Vec<String> },
    #[dsl(key = "layer-visibility")]
    SetLayerVisibility { layer: String, visible: bool },
    #[dsl(key = "frame-cursor")]
    SetFrameCursor {
        #[serde(default)]
        stream_id: Option<String>,
        frame_index: u32,
    },
    #[dsl(key = "report-table")]
    SetReportTable { table: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for RemodelConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for RemodelConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


impl Mutation<RemodelConfig> for RemodelConfigMutation {
    type Diff = RemodelConfig;

    fn diff(&self, base: &RemodelConfig) -> RemodelConfig {
        let mut next = base.clone();
        match self {
            RemodelConfigMutation::Snapshot { config } => return config.clone(),
            RemodelConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            RemodelConfigMutation::SetSelection { mode, ids } => {
                next.selection.mode = mode.clone();
                next.selection.ids = ids.clone();
            }
            RemodelConfigMutation::SetLayerVisibility { layer, visible } => match layer.as_str() {
                "mesh" => next.layers.mesh = *visible,
                "dense" => next.layers.dense = *visible,
                "sparse" => next.layers.sparse = *visible,
                "cameras" => next.layers.cameras = *visible,
                "gcps" => next.layers.gcps = *visible,
                _ => {}
            },
            RemodelConfigMutation::SetFrameCursor { stream_id, frame_index } => {
                if stream_id.is_some() {
                    next.frame_cursor.stream_id = stream_id.clone();
                }
                next.frame_cursor.frame_index = *frame_index;
            }
            RemodelConfigMutation::SetReportTable { table } => next.report_table = table.clone(),
            RemodelConfigMutation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            RemodelConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &RemodelConfig) -> Vec<Self> {
        vec![RemodelConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remodel_config_default_matches_the_former_runtime_defaults() {
        let config = RemodelConfig::default();
        assert_eq!(config.camera, RemodelWorldCamera { position: [4.0, -4.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
        assert_eq!(config.selection, RemodelSelection::default());
        assert!(config.layers.mesh && config.layers.dense && config.layers.sparse && config.layers.cameras && config.layers.gcps);
        assert_eq!(config.frame_cursor, RemodelFrameCursor::default());
        assert_eq!(config.report_table, "frames");
        assert_eq!(config.active_utility_id, "select");
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn remodel_config_operation_diff_is_whole_record_replace() {
        let base = RemodelConfig::default();
        let mut next = base.clone();
        next.report_table = "gcps".into();
        assert_eq!(protocol::MutationDiff::apply(&next, &base), next, "apply ignores base entirely, like ShootingConfig");
    }

    #[test]
    fn config_mutations_apply_and_backwards_restore_the_pre_edit_snapshot() {
        let base = RemodelConfig::default();

        let camera = RemodelWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
        let op = RemodelConfigMutation::SetCamera { camera: camera.clone() };
        let next = op.diff(&base);
        assert_eq!(next.camera, camera);
        assert_eq!(op.inverse(&base), vec![RemodelConfigMutation::Snapshot { config: base.clone() }]);
        assert_eq!(op.inverse(&base)[0].diff(&next), base, "backwards restores the exact pre-edit config");

        let op = RemodelConfigMutation::SetSelection { mode: "rectangle".into(), ids: vec!["a".into()] };
        let next = op.diff(&base);
        assert_eq!(next.selection.mode, "rectangle");
        assert_eq!(next.selection.ids, vec!["a".to_string()]);

        let op = RemodelConfigMutation::SetLayerVisibility { layer: "dense".into(), visible: false };
        let next = op.diff(&base);
        assert!(!next.layers.dense);
        assert!(next.layers.mesh, "only the named layer flips");

        let op = RemodelConfigMutation::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 4 };
        let next = op.diff(&base);
        assert_eq!(next.frame_cursor.stream_id.as_deref(), Some("stream-1"));
        assert_eq!(next.frame_cursor.frame_index, 4);

        let op = RemodelConfigMutation::SetReportTable { table: "gcps".into() };
        assert_eq!(op.diff(&base).report_table, "gcps");

        let op = RemodelConfigMutation::SetActiveUtility { utility_id: "measure".into() };
        assert_eq!(op.diff(&base).active_utility_id, "measure");

        let op = RemodelConfigMutation::SetLocale { value: "de-DE".into() };
        assert_eq!(op.diff(&base).locale, "de-DE");
    }

    #[test]
    fn config_mutations_roundtrip_through_op_text() {
        let config = RemodelConfig::default();
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::Snapshot { config });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetCamera { camera: RemodelWorldCamera::default() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetSelection { mode: "rectangle".into(), ids: vec!["a".into(), "b".into()] });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetLayerVisibility { layer: "gcps".into(), visible: false });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetFrameCursor { stream_id: None, frame_index: 0 });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetReportTable { table: "tracks".into() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetActiveUtility { utility_id: "gcpPlace".into() });
        store::os_store::test_support::assert_op_line_round_trip(&RemodelConfigMutation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests

//! ⚙️ S Studio app — `ArtifactApp::Config` + its operation enum (constitutional: engine + op, merged
//! at app level: `Config`/`ConfigMutation` are inherently app-scoped, and this app owns no
//! document-side artifact — see `🦀️component.rs`'s module doc for why).

use crate::engine::space::S_PLAY_CATALOGUE_TAB_ID;
use semio_framework_os::OsWorkflowCamera;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Types
/// 🎥️ One window-instance's workflow-canvas camera — keyed by window id inside `SpaceConfig.camera`
/// (a `BTreeMap<String, SpaceWindowCamera>`, per the Configured Node Apps recipe's "camera/selection/
/// per-window options keyed by window-instance id" rule). Distinct from `semio_framework_os::OsWorkflowCamera`
/// (a plain, non-`dsl`-field data type this crate can't blanket-impl `dsl::DslField` for under the
/// orphan rule) — converts to/from it 1:1 at the render boundary.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SpaceWindowCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for SpaceWindowCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

impl From<OsWorkflowCamera> for SpaceWindowCamera {
    fn from(camera: OsWorkflowCamera) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}

impl From<SpaceWindowCamera> for OsWorkflowCamera {
    fn from(camera: SpaceWindowCamera) -> Self {
        Self { x: camera.x, y: camera.y, zoom: camera.zoom }
    }
}
//#endregion 🔖️Types

//#region 🔖️Config
/// 🧮️ Space's real `ArtifactApp::Config` — the studio app's config artifact. A node IS the app
/// instance now (see the kernel `🔁️workflow` crate's `🔖️InstanceIdentity` doc), so the old disjoint
/// `selected_media_node_ids`/`selected_app_instance_ids`/`clipboard_instance_ids` pairs collapse into
/// one `*_node_ids` field apiece. `camera`/per-window options are keyed by window id (`BTreeMap<String,
/// _>`, per the Configured Node Apps recipe) — today that's always
/// `crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW`, since split-pane
/// window *instances* aren't a thing anywhere in this codebase yet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(id = "s.spacecfg")]
#[dsl(layout = "lines")]
pub struct SpaceConfig {
    /// 🎥️ Workflow-canvas camera, keyed by window id.
    pub camera: BTreeMap<String, SpaceWindowCamera>,
    /// 🗂️ Collapsed workflow nodes — node-preview UI state, not yet driven by any command.
    pub collapsed_node_ids: Vec<String>,
    /// 🖼️ Workflow nodes with their live preview thumbnail turned off — node-preview UI state, not yet
    /// driven by any command.
    pub preview_off_node_ids: Vec<String>,
    /// 👁️ The "active app" measure selection.
    pub active_node_id: Option<String>,
    /// 👁️ The node currently open in its own plugin window.
    pub focused_node_id: Option<String>,
    /// 📋️ Copied node ids, pasted by `duplicateAppInstance`/`pasteAppInstance`.
    pub clipboard_node_ids: Vec<String>,
    pub workflow_engagement_input: String,
    pub compiled_dag_engagement_input: String,
    /// 📥️ In-flight media-import target.
    pub pending_import_node_id: Option<String>,
    pub pending_import_format: Option<String>,
    /// 👁️ Active studio panel tab.
    pub active_panel_tab: String,
    /// 🌱️ The currently open studio document's catalog id.
    pub space_id: Option<String>,
    /// 🫀️ This session's local presence identity.
    pub client_id: Option<String>,
    pub client_name: Option<String>,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for SpaceConfig {
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
impl store::ArtifactPack for SpaceConfig {
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


impl Default for SpaceConfig {
    fn default() -> Self {
        Self {
            camera: BTreeMap::new(),
            collapsed_node_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            active_node_id: None,
            focused_node_id: None,
            clipboard_node_ids: Vec::new(),
            workflow_engagement_input: String::new(),
            compiled_dag_engagement_input: String::new(),
            pending_import_node_id: None,
            pending_import_format: None,
            active_panel_tab: S_PLAY_CATALOGUE_TAB_ID.into(),
            space_id: None,
            client_id: None,
            client_name: None,
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(SpaceConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ `SpaceConfig`'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns: a config-only dispatch is a plain `Apply` (not an
/// `AmendLast`), so each tick is its own distinct, real config edit and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it". `Mutation::Diff` is the WHOLE `SpaceConfig`,
/// not a granular patch type.
// 🧯️ `large_enum_variant`: `Snapshot` deliberately carries the WHOLE `SpaceConfig` while every other row
// carries one or two scalars — that whole-config snapshot IS the inverse mechanism every variant's
// `backwards()` returns. Boxing it would change the derived `dsl::DslOps` wire encoding, which this
// migration must preserve byte-for-byte, so the size skew is accepted by design (same tradeoff as
// block3d's `Block3dConfigMutation`/gis's `Gis2dConfigMutation`).
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SpaceConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SpaceConfig,
    },
    #[dsl(key = "active-node")]
    SetActiveNode { node_id: Option<String> },
    #[dsl(key = "focused-node")]
    SetFocusedNode { node_id: Option<String> },
    #[dsl(key = "clipboard")]
    SetClipboard { node_ids: Vec<String> },
    #[dsl(key = "collapsed")]
    SetCollapsed { node_ids: Vec<String> },
    #[dsl(key = "preview-off")]
    SetPreviewOff { node_ids: Vec<String> },
    /// 🎥️ Sets one window's workflow camera — window-instance-keyed.
    #[dsl(key = "camera")]
    SetCamera {
        window_id: String,
        #[dsl(block)]
        camera: SpaceWindowCamera,
    },
    #[dsl(key = "workflow-engagement-input")]
    SetWorkflowEngagementInput { value: String },
    #[dsl(key = "compiled-dag-engagement-input")]
    SetCompiledDagEngagementInput { value: String },
    #[dsl(key = "pending-import")]
    SetPendingImport { node_id: Option<String>, format: Option<String> },
    #[dsl(key = "space-id")]
    SetSpaceId { space_id: Option<String> },
    #[dsl(key = "client")]
    SetClient { client_id: Option<String>, client_name: Option<String> },
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for SpaceConfigMutation {
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
impl protocol::OpBinary for SpaceConfigMutation {
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


impl protocol::Mutation<SpaceConfig> for SpaceConfigMutation {
    type Diff = SpaceConfig;

    fn diff(&self, base: &SpaceConfig) -> protocol::MutationOutcome<SpaceConfig> {
        let mut next = base.clone();
        match self {
            SpaceConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            SpaceConfigMutation::SetActiveNode { node_id } => next.active_node_id = node_id.clone(),
            SpaceConfigMutation::SetFocusedNode { node_id } => next.focused_node_id = node_id.clone(),
            SpaceConfigMutation::SetClipboard { node_ids } => next.clipboard_node_ids = node_ids.clone(),
            SpaceConfigMutation::SetCollapsed { node_ids } => next.collapsed_node_ids = node_ids.clone(),
            SpaceConfigMutation::SetPreviewOff { node_ids } => next.preview_off_node_ids = node_ids.clone(),
            SpaceConfigMutation::SetCamera { window_id, camera } => {
                next.camera.insert(window_id.clone(), *camera);
            }
            SpaceConfigMutation::SetWorkflowEngagementInput { value } => next.workflow_engagement_input = value.clone(),
            SpaceConfigMutation::SetCompiledDagEngagementInput { value } => next.compiled_dag_engagement_input = value.clone(),
            SpaceConfigMutation::SetPendingImport { node_id, format } => {
                next.pending_import_node_id = node_id.clone();
                next.pending_import_format = format.clone();
            }
            SpaceConfigMutation::SetSpaceId { space_id } => next.space_id = space_id.clone(),
            SpaceConfigMutation::SetClient { client_id, client_name } => {
                next.client_id = client_id.clone();
                next.client_name = client_name.clone();
            }
            SpaceConfigMutation::SetActivePanelTab { tab_id } => next.active_panel_tab = tab_id.clone(),
            SpaceConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &SpaceConfig) -> Vec<Self> {
        vec![SpaceConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::space::modes::main::windows::workflow::S_PLAY_WINDOW_WORKFLOW;
    use crate::engine::space::S_PLAY_PARAMETERS_TAB_ID;
    use protocol::Mutation;

    fn round_trip(config: &SpaceConfig, operation: &SpaceConfigMutation) -> SpaceConfig {
        let (forward, _messages) =
            vcs::apply_mutation(config, operation).expect("valid mutation");
        let backwards = operation.inverse(config);
        let mut restored = forward.clone();
        for back in &backwards {
            let (next, _messages) =
                vcs::apply_mutation(&restored, back).expect("valid inverse mutation");
            restored = next;
        }
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn space_config_default_matches_the_expected_sticky_defaults() {
        let config = SpaceConfig::default();
        assert_eq!(config.active_panel_tab, S_PLAY_CATALOGUE_TAB_ID);
        assert_eq!(config.locale, "en-US");
        assert!(config.camera.is_empty());
    }

    #[test]
    fn space_config_dsl_text_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&SpaceConfig::default());
    }

    #[test]
    fn set_camera_round_trips_and_keys_by_window_id() {
        let config = SpaceConfig::default();
        let camera = SpaceWindowCamera { x: 12.0, y: -4.0, zoom: 2.0 };
        let operation = SpaceConfigMutation::SetCamera { window_id: S_PLAY_WINDOW_WORKFLOW.into(), camera };
        let next = round_trip(&config, &operation);
        assert_eq!(next.camera.get(S_PLAY_WINDOW_WORKFLOW), Some(&camera));
    }

    #[test]
    fn set_active_panel_tab_round_trips() {
        let config = SpaceConfig::default();
        let operation = SpaceConfigMutation::SetActivePanelTab { tab_id: S_PLAY_PARAMETERS_TAB_ID.into() };
        let next = round_trip(&config, &operation);
        assert_eq!(next.active_panel_tab, S_PLAY_PARAMETERS_TAB_ID);
    }

    #[test]
    fn space_config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::Snapshot { config: SpaceConfig::default() });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetActiveNode { node_id: Some("a".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetFocusedNode { node_id: None });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetClipboard { node_ids: vec!["a".into()] });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetCollapsed { node_ids: vec!["a".into()] });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetPreviewOff { node_ids: vec!["a".into()] });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetCamera { window_id: "s-workflow".into(), camera: SpaceWindowCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetWorkflowEngagementInput { value: "draw draw".into() });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetCompiledDagEngagementInput { value: "".into() });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetPendingImport { node_id: Some("a".into()), format: Some("dwg".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetPendingImport { node_id: None, format: None });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetSpaceId { space_id: Some("demo".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetClient { client_id: Some("c1".into()), client_name: Some("Ada".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetActivePanelTab { tab_id: "s-play-catalogue".into() });
        store::os_store::test_support::assert_op_line_round_trip(&SpaceConfigMutation::SetLocale { value: "de".into() });
    }

    #[test]
    fn space_config_dsl_pack_equivalence() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&SpaceConfig::default());
    }
}
//#endregion 🧪️Tests

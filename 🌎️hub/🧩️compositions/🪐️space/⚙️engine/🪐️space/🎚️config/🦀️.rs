//! ⚙️ S Studio app — `ArtifactApp::Config` + its operation enum (constitutional: engine + op, merged
//! at app level: `Config`/`ConfigMutation` are inherently app-scoped, and this app owns no
//! document-side artifact — see `🦀️.rs`'s module doc for why).

use crate::engine::space::S_PLAY_CATALOGUE_TAB_ID;
use semio_framework_os::OsWorkflowCamera;
use std::collections::BTreeMap;

//#region 🔖️Types
/// 🎥️ One window-instance's workflow-canvas camera — keyed by window id inside `SpaceConfig.camera`
/// (a `BTreeMap<String, SpaceWindowCamera>`, per the Configured Node Apps recipe's "camera/selection/
/// per-window options keyed by window-instance id" rule). Distinct from `semio_framework_os::OsWorkflowCamera`
/// (a plain, non-`dsl`-field data type this crate can't blanket-impl `dsl::DslField` for under the
/// orphan rule) — converts to/from it 1:1 at the render boundary.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::DslArtifact, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
#[artifact(id = "s.spacecfg")]
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
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for SpaceConfig {
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
impl store::ArtifactPack for SpaceConfig {
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
        }
    }
}

store::config_diff! {
    record: SpaceConfig,
    diff: SpaceConfigDiff,
    fields: {
        collapsed_node_ids: Vec<String>,
        preview_off_node_ids: Vec<String>,
        active_node_id: Option<String>,
        focused_node_id: Option<String>,
        clipboard_node_ids: Vec<String>,
        workflow_engagement_input: String,
        compiled_dag_engagement_input: String,
        pending_import_node_id: Option<String>,
        pending_import_format: Option<String>,
        active_panel_tab: String,
        space_id: Option<String>,
    },
    keyed: { camera: SpaceWindowCamera },
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `SpaceConfig`'s operation enum — one variant per settled interaction, each setting only the slots (or keyed camera rows) of
/// the fields it owns, with the same variant carrying the base values as its inverse. There is no whole-config variant.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SpaceConfigMutation {
    #[dsl(key = "active-node")]
    SetActiveNode(ActiveNodeSetting),
    #[dsl(key = "focused-node")]
    SetFocusedNode(FocusedNodeSetting),
    #[dsl(key = "clipboard")]
    SetClipboard(ClipboardSetting),
    #[dsl(key = "collapsed")]
    SetCollapsed(CollapsedSetting),
    #[dsl(key = "preview-off")]
    SetPreviewOff(PreviewOffSetting),
    /// 🎥️ Sets one window's workflow camera — window-instance-keyed.
    #[dsl(key = "camera")]
    SetCamera(CameraSetting),
    /// 🎥️ Removes one window's workflow camera row — the inverse of the first `SetCamera` of a window.
    #[dsl(key = "remove-camera")]
    RemoveCamera(CameraRemoval),
    #[dsl(key = "workflow-engagement-input")]
    SetWorkflowEngagementInput(WorkflowEngagementInputSetting),
    #[dsl(key = "compiled-dag-engagement-input")]
    SetCompiledDagEngagementInput(CompiledDagEngagementInputSetting),
    #[dsl(key = "pending-import")]
    SetPendingImport(PendingImportSetting),
    #[dsl(key = "space-id")]
    SetSpaceId(SpaceIdSetting),
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab(ActivePanelTabSetting),
}

/// 🎯️ The active node a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "active-node")]
pub struct ActiveNodeSetting {
    pub node_id: Option<String>,
}

/// 🎯️ The focused node a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "focused-node")]
pub struct FocusedNodeSetting {
    pub node_id: Option<String>,
}

/// 📋️ The clipboard a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "clipboard")]
pub struct ClipboardSetting {
    pub node_ids: Vec<String>,
}

/// 🗂️ The collapsed nodes a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "collapsed")]
pub struct CollapsedSetting {
    pub node_ids: Vec<String>,
}

/// 🖼️ The preview-off nodes a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "preview-off")]
pub struct PreviewOffSetting {
    pub node_ids: Vec<String>,
}

/// 🎥️ The window camera a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "camera")]
pub struct CameraSetting {
    pub window_id: String,
    #[dsl(block)]
    pub camera: SpaceWindowCamera,
}

/// 🎥️ The window whose camera row a removal drops; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-camera")]
pub struct CameraRemoval {
    pub window_id: String,
}

/// ⌨️ The workflow engagement input a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "workflow-engagement-input")]
pub struct WorkflowEngagementInputSetting {
    pub value: String,
}

/// ⌨️ The compiled DAG engagement input a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "compiled-dag-engagement-input")]
pub struct CompiledDagEngagementInputSetting {
    pub value: String,
}

/// 📥️ The in-flight media-import target a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "pending-import")]
pub struct PendingImportSetting {
    pub node_id: Option<String>,
    pub format: Option<String>,
}

/// 🌱️ The open studio's catalog id a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "space-id")]
pub struct SpaceIdSetting {
    pub space_id: Option<String>,
}

/// 👁️ The active panel tab a setting installs; its wire record is exactly the former variant's fields.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "active-panel-tab")]
pub struct ActivePanelTabSetting {
    pub tab_id: String,
}

//#region 🔖️OpCodec
impl protocol::OpText for SpaceConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown mutation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for SpaceConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
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
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

impl protocol::Mutation<SpaceConfig> for SpaceConfigMutation {
    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate — one
    /// entry per variant, in declaration order. ⚠️ PROVISIONAL: no variant below has an authored leaf
    /// directory on disk yet, same precedent the sibling generation2d config aggregate sets.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-active-node",
            semantic_kind: "set-active-node",
            display_name: "Set Active Node",
            emoji: "⚙️",
            aggregate_variant: "SetActiveNode",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-focused-node",
            semantic_kind: "set-focused-node",
            display_name: "Set Focused Node",
            emoji: "⚙️",
            aggregate_variant: "SetFocusedNode",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-clipboard",
            semantic_kind: "set-clipboard",
            display_name: "Set Clipboard",
            emoji: "⚙️",
            aggregate_variant: "SetClipboard",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-collapsed",
            semantic_kind: "set-collapsed",
            display_name: "Set Collapsed",
            emoji: "⚙️",
            aggregate_variant: "SetCollapsed",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-preview-off",
            semantic_kind: "set-preview-off",
            display_name: "Set Preview Off",
            emoji: "⚙️",
            aggregate_variant: "SetPreviewOff",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-camera",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "⚙️",
            aggregate_variant: "SetCamera",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️remove-camera",
            semantic_kind: "remove-camera",
            display_name: "Remove Camera",
            emoji: "⚙️",
            aggregate_variant: "RemoveCamera",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-workflow-engagement-input",
            semantic_kind: "set-workflow-engagement-input",
            display_name: "Set Workflow Engagement Input",
            emoji: "⚙️",
            aggregate_variant: "SetWorkflowEngagementInput",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-compiled-dag-engagement-input",
            semantic_kind: "set-compiled-dag-engagement-input",
            display_name: "Set Compiled Dag Engagement Input",
            emoji: "⚙️",
            aggregate_variant: "SetCompiledDagEngagementInput",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-pending-import",
            semantic_kind: "set-pending-import",
            display_name: "Set Pending Import",
            emoji: "⚙️",
            aggregate_variant: "SetPendingImport",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-space-id",
            semantic_kind: "set-space-id",
            display_name: "Set Space Id",
            emoji: "⚙️",
            aggregate_variant: "SetSpaceId",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set-active-panel-tab",
            semantic_kind: "set-active-panel-tab",
            display_name: "Set Active Panel Tab",
            emoji: "⚙️",
            aggregate_variant: "SetActivePanelTab",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            SpaceConfigMutation::SetActiveNode(_) => &Self::DESCRIPTORS[0],
            SpaceConfigMutation::SetFocusedNode(_) => &Self::DESCRIPTORS[1],
            SpaceConfigMutation::SetClipboard(_) => &Self::DESCRIPTORS[2],
            SpaceConfigMutation::SetCollapsed(_) => &Self::DESCRIPTORS[3],
            SpaceConfigMutation::SetPreviewOff(_) => &Self::DESCRIPTORS[4],
            SpaceConfigMutation::SetCamera(_) => &Self::DESCRIPTORS[5],
            SpaceConfigMutation::RemoveCamera(_) => &Self::DESCRIPTORS[6],
            SpaceConfigMutation::SetWorkflowEngagementInput(_) => &Self::DESCRIPTORS[7],
            SpaceConfigMutation::SetCompiledDagEngagementInput(_) => &Self::DESCRIPTORS[8],
            SpaceConfigMutation::SetPendingImport(_) => &Self::DESCRIPTORS[9],
            SpaceConfigMutation::SetSpaceId(_) => &Self::DESCRIPTORS[10],
            SpaceConfigMutation::SetActivePanelTab(_) => &Self::DESCRIPTORS[11],
        }
    }

    type Diff = SpaceConfigDiff;

    fn diff(&self, base: &SpaceConfig) -> protocol::MutationOutcome<SpaceConfigDiff> {
        protocol::MutationOutcome::new(match self {
            SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id }) => SpaceConfigDiff { active_node_id: (base.active_node_id != *node_id).then(|| node_id.clone()), ..Default::default() },
            SpaceConfigMutation::SetFocusedNode(FocusedNodeSetting { node_id }) => SpaceConfigDiff { focused_node_id: (base.focused_node_id != *node_id).then(|| node_id.clone()), ..Default::default() },
            SpaceConfigMutation::SetClipboard(ClipboardSetting { node_ids }) => SpaceConfigDiff { clipboard_node_ids: (base.clipboard_node_ids != *node_ids).then(|| node_ids.clone()), ..Default::default() },
            SpaceConfigMutation::SetCollapsed(CollapsedSetting { node_ids }) => SpaceConfigDiff { collapsed_node_ids: (base.collapsed_node_ids != *node_ids).then(|| node_ids.clone()), ..Default::default() },
            SpaceConfigMutation::SetPreviewOff(PreviewOffSetting { node_ids }) => SpaceConfigDiff { preview_off_node_ids: (base.preview_off_node_ids != *node_ids).then(|| node_ids.clone()), ..Default::default() },
            SpaceConfigMutation::SetCamera(CameraSetting { window_id, camera }) => match base.camera.get(window_id) {
                Some(prior) if prior == camera => SpaceConfigDiff::default(),
                Some(_) => SpaceConfigDiff { camera: [(window_id.clone(), protocol::KeyedRow::Replace(*camera))].into(), ..Default::default() },
                None => SpaceConfigDiff { camera: [(window_id.clone(), protocol::KeyedRow::Insert(*camera))].into(), ..Default::default() },
            },
            SpaceConfigMutation::RemoveCamera(CameraRemoval { window_id }) => {
                if !base.camera.contains_key(window_id) {
                    return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMissing, "the window has no camera row", ["camera", window_id.as_str()]);
                }
                SpaceConfigDiff { camera: [(window_id.clone(), protocol::KeyedRow::Remove)].into(), ..Default::default() }
            }
            SpaceConfigMutation::SetWorkflowEngagementInput(WorkflowEngagementInputSetting { value }) => SpaceConfigDiff { workflow_engagement_input: (base.workflow_engagement_input != *value).then(|| value.clone()), ..Default::default() },
            SpaceConfigMutation::SetCompiledDagEngagementInput(CompiledDagEngagementInputSetting { value }) => SpaceConfigDiff { compiled_dag_engagement_input: (base.compiled_dag_engagement_input != *value).then(|| value.clone()), ..Default::default() },
            SpaceConfigMutation::SetPendingImport(PendingImportSetting { node_id, format }) => SpaceConfigDiff {
                pending_import_node_id: (base.pending_import_node_id != *node_id).then(|| node_id.clone()),
                pending_import_format: (base.pending_import_format != *format).then(|| format.clone()),
                ..Default::default()
            },
            SpaceConfigMutation::SetSpaceId(SpaceIdSetting { space_id }) => SpaceConfigDiff { space_id: (base.space_id != *space_id).then(|| space_id.clone()), ..Default::default() },
            SpaceConfigMutation::SetActivePanelTab(ActivePanelTabSetting { tab_id }) => SpaceConfigDiff { active_panel_tab: (base.active_panel_tab != *tab_id).then(|| tab_id.clone()), ..Default::default() },
        })
    }

    fn inverse(&self, base: &SpaceConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            SpaceConfigMutation::SetActiveNode(_) => SpaceConfigMutation::SetActiveNode(ActiveNodeSetting { node_id: base.active_node_id.clone() }),
            SpaceConfigMutation::SetFocusedNode(_) => SpaceConfigMutation::SetFocusedNode(FocusedNodeSetting { node_id: base.focused_node_id.clone() }),
            SpaceConfigMutation::SetClipboard(_) => SpaceConfigMutation::SetClipboard(ClipboardSetting { node_ids: base.clipboard_node_ids.clone() }),
            SpaceConfigMutation::SetCollapsed(_) => SpaceConfigMutation::SetCollapsed(CollapsedSetting { node_ids: base.collapsed_node_ids.clone() }),
            SpaceConfigMutation::SetPreviewOff(_) => SpaceConfigMutation::SetPreviewOff(PreviewOffSetting { node_ids: base.preview_off_node_ids.clone() }),
            SpaceConfigMutation::SetCamera(CameraSetting { window_id, .. }) => match base.camera.get(window_id) {
                Some(camera) => SpaceConfigMutation::SetCamera(CameraSetting { window_id: window_id.clone(), camera: *camera }),
                None => SpaceConfigMutation::RemoveCamera(CameraRemoval { window_id: window_id.clone() }),
            },
            SpaceConfigMutation::RemoveCamera(CameraRemoval { window_id }) => match base.camera.get(window_id) {
                Some(camera) => SpaceConfigMutation::SetCamera(CameraSetting { window_id: window_id.clone(), camera: *camera }),
                None => return Ok(Vec::new()),
            },
            SpaceConfigMutation::SetWorkflowEngagementInput(_) => SpaceConfigMutation::SetWorkflowEngagementInput(WorkflowEngagementInputSetting { value: base.workflow_engagement_input.clone() }),
            SpaceConfigMutation::SetCompiledDagEngagementInput(_) => SpaceConfigMutation::SetCompiledDagEngagementInput(CompiledDagEngagementInputSetting { value: base.compiled_dag_engagement_input.clone() }),
            SpaceConfigMutation::SetPendingImport(_) => SpaceConfigMutation::SetPendingImport(PendingImportSetting { node_id: base.pending_import_node_id.clone(), format: base.pending_import_format.clone() }),
            SpaceConfigMutation::SetSpaceId(_) => SpaceConfigMutation::SetSpaceId(SpaceIdSetting { space_id: base.space_id.clone() }),
            SpaceConfigMutation::SetActivePanelTab(_) => SpaceConfigMutation::SetActivePanelTab(ActivePanelTabSetting { tab_id: base.active_panel_tab.clone() }),
        }])
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

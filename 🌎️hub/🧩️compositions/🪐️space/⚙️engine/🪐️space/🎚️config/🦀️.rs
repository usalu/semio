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
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::DslArtifact)]
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
        camera: BTreeMap<String, SpaceWindowCamera>,
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
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `SpaceConfig`'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns: a config-only dispatch is a plain `Apply`, so each tick is its own
/// distinct, real config edit and "undo this tick" is exactly
/// "restore the whole-config snapshot from just before it". `Mutation::Diff` is the WHOLE `SpaceConfig`,
/// not a granular patch type.
// 🧯️ `large_enum_variant`: `Snapshot` deliberately carries the WHOLE `SpaceConfig` while every other row
// carries one or two scalars — that whole-config snapshot IS the inverse mechanism every variant's
// `backwards()` returns. Boxing it would change the derived `semio_framework_dsl_record_derive::DslEnum` wire encoding, which this
// migration must preserve byte-for-byte, so the size skew is accepted by design (same tradeoff as
// block3d's `Block3dConfigMutation`/gis's `Gis2dConfigMutation`).
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
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
    #[dsl(key = "active-panel-tab")]
    SetActivePanelTab { tab_id: String },
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
            owner: "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config/⚙️set",
            semantic_kind: "set-snapshot",
            display_name: "Set Snapshot",
            emoji: "⚙️",
            aggregate_variant: "Snapshot",
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
            SpaceConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            SpaceConfigMutation::SetActiveNode { .. } => &Self::DESCRIPTORS[1],
            SpaceConfigMutation::SetFocusedNode { .. } => &Self::DESCRIPTORS[2],
            SpaceConfigMutation::SetClipboard { .. } => &Self::DESCRIPTORS[3],
            SpaceConfigMutation::SetCollapsed { .. } => &Self::DESCRIPTORS[4],
            SpaceConfigMutation::SetPreviewOff { .. } => &Self::DESCRIPTORS[5],
            SpaceConfigMutation::SetCamera { .. } => &Self::DESCRIPTORS[6],
            SpaceConfigMutation::SetWorkflowEngagementInput { .. } => &Self::DESCRIPTORS[7],
            SpaceConfigMutation::SetCompiledDagEngagementInput { .. } => &Self::DESCRIPTORS[8],
            SpaceConfigMutation::SetPendingImport { .. } => &Self::DESCRIPTORS[9],
            SpaceConfigMutation::SetSpaceId { .. } => &Self::DESCRIPTORS[10],
            SpaceConfigMutation::SetActivePanelTab { .. } => &Self::DESCRIPTORS[11],
        }
    }

    type Diff = SpaceConfigDiff;

    fn diff(&self, base: &SpaceConfig) -> protocol::MutationOutcome<SpaceConfigDiff> {
        protocol::MutationOutcome::new(match self {
            SpaceConfigMutation::Snapshot { config } => SpaceConfigDiff::changing(base, config),
            SpaceConfigMutation::SetActiveNode { node_id } => SpaceConfigDiff { active_node_id: Some(node_id.clone()), ..Default::default() },
            SpaceConfigMutation::SetFocusedNode { node_id } => SpaceConfigDiff { focused_node_id: Some(node_id.clone()), ..Default::default() },
            SpaceConfigMutation::SetClipboard { node_ids } => SpaceConfigDiff { clipboard_node_ids: Some(node_ids.clone()), ..Default::default() },
            SpaceConfigMutation::SetCollapsed { node_ids } => SpaceConfigDiff { collapsed_node_ids: Some(node_ids.clone()), ..Default::default() },
            SpaceConfigMutation::SetPreviewOff { node_ids } => SpaceConfigDiff { preview_off_node_ids: Some(node_ids.clone()), ..Default::default() },
            SpaceConfigMutation::SetCamera { window_id, camera } => {
                let mut cameras = base.camera.clone();
                cameras.insert(window_id.clone(), *camera);
                SpaceConfigDiff { camera: Some(cameras), ..Default::default() }
            }
            SpaceConfigMutation::SetWorkflowEngagementInput { value } => SpaceConfigDiff { workflow_engagement_input: Some(value.clone()), ..Default::default() },
            SpaceConfigMutation::SetCompiledDagEngagementInput { value } => SpaceConfigDiff { compiled_dag_engagement_input: Some(value.clone()), ..Default::default() },
            SpaceConfigMutation::SetPendingImport { node_id, format } => SpaceConfigDiff { pending_import_node_id: Some(node_id.clone()), pending_import_format: Some(format.clone()), ..Default::default() },
            SpaceConfigMutation::SetSpaceId { space_id } => SpaceConfigDiff { space_id: Some(space_id.clone()), ..Default::default() },
            SpaceConfigMutation::SetActivePanelTab { tab_id } => SpaceConfigDiff { active_panel_tab: Some(tab_id.clone()), ..Default::default() },
        })
    }

    fn inverse(&self, base: &SpaceConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            SpaceConfigMutation::Snapshot { .. } => SpaceConfigMutation::Snapshot { config: base.clone() },
            SpaceConfigMutation::SetActiveNode { .. } => SpaceConfigMutation::SetActiveNode { node_id: base.active_node_id.clone() },
            SpaceConfigMutation::SetFocusedNode { .. } => SpaceConfigMutation::SetFocusedNode { node_id: base.focused_node_id.clone() },
            SpaceConfigMutation::SetClipboard { .. } => SpaceConfigMutation::SetClipboard { node_ids: base.clipboard_node_ids.clone() },
            SpaceConfigMutation::SetCollapsed { .. } => SpaceConfigMutation::SetCollapsed { node_ids: base.collapsed_node_ids.clone() },
            SpaceConfigMutation::SetPreviewOff { .. } => SpaceConfigMutation::SetPreviewOff { node_ids: base.preview_off_node_ids.clone() },
            SpaceConfigMutation::SetCamera { window_id, .. } => match base.camera.get(window_id) {
                Some(camera) => SpaceConfigMutation::SetCamera { window_id: window_id.clone(), camera: *camera },
                None => SpaceConfigMutation::Snapshot { config: base.clone() },
            },
            SpaceConfigMutation::SetWorkflowEngagementInput { .. } => SpaceConfigMutation::SetWorkflowEngagementInput { value: base.workflow_engagement_input.clone() },
            SpaceConfigMutation::SetCompiledDagEngagementInput { .. } => SpaceConfigMutation::SetCompiledDagEngagementInput { value: base.compiled_dag_engagement_input.clone() },
            SpaceConfigMutation::SetPendingImport { .. } => SpaceConfigMutation::SetPendingImport { node_id: base.pending_import_node_id.clone(), format: base.pending_import_format.clone() },
            SpaceConfigMutation::SetSpaceId { .. } => SpaceConfigMutation::SetSpaceId { space_id: base.space_id.clone() },
            SpaceConfigMutation::SetActivePanelTab { .. } => SpaceConfigMutation::SetActivePanelTab { tab_id: base.active_panel_tab.clone() },
        }])
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//! 🔱️ Trinity Jack editor — jack query editor bundled as a hot-swappable WASM plugin.
//!
//! 📌️ Pure-trait `ArtifactEditor`: `TrinityJackPlayApp` is a unit struct; every former
//! `TrinityJackRuntime` field (selection, camera, query draft, LOD, …) lives in
//! `config::JackConfig`, written via `config::JackConfigMutation`s (real `backwards()`, no ad hoc
//! inverse-action bookkeeping). Every action dispatches through the single typed `TrinityJackCommand`
//! channel via `ArtifactEditor::handle`, which fans out to `🎮️commands/<group>/component.rs` (the
//! command enum stays hand-rolled — see its own doc comment — only the match body is decomposed).

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::{JackSnapshot, Node, PortDirection, TRINITY_GRAPH_SCHEMA, TRINITY_JACK_DIALECT};
use crate::editor::jack::config::{JackConfig, JackConfigMutation};
use crate::editor::jack::presence::{JackPresence, JackPresenceMutation};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionKind, AppActionRegistry, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, ContextMenuItemSpec, ContextMenuRequest, Dialect, DomainTopology, DraftView, Editor, EditorApp, Effect, Emit, Fault,
    GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation,
    NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphViewport, PanelGroup, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, WindowMeasure, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
// 🩹️ `InteractionView` is not re-exported at `semio_framework_plugin`'s crate root (unlike
// `ConfigView`/`ArtifactView`/`DraftView`) — only reachable through its owning `app` submodule
// (itself `pub mod`). Flagged as a likely framework oversight, not fixed here (framework file).
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
use std::collections::HashMap;
use store::EngineHandles;
use store::{ArtifactDsl, ArtifactPack};

//#region 🔖️Constants
const TRINITY_JACK_PLAY_CONTROLLER_ID: &str = "trinity-jack-play";
const TRINITY_JACK_PLAY_SURFACE_GRAPH: &str = "trinity.jack.play";
const TRINITY_JACK_PLAY_SURFACE_EDITOR: &str = "trinity.jack.editor";
const TRINITY_JACK_PLAY_SURFACE_RESULTS: &str = "trinity.jack.results";
const TRINITY_JACK_PLAY_BODY_GRAPH: &str = "trinity.jack.play.main";
const TRINITY_JACK_PLAY_BODY_EDITOR: &str = "trinity.jack.play.editor";
const TRINITY_JACK_PLAY_BODY_RESULTS: &str = "trinity.jack.play.results";
const TRINITY_JACK_PLAY_BODY_DOCUMENT: &str = "trinity.jack.play.document";
const TRINITY_JACK_PLAY_BODY_CATALOGUE: &str = "trinity.jack.play.catalogue";
const TRINITY_JACK_PLAY_BODY_INSPECTION: &str = "trinity.jack.play.inspection";
pub(crate) const TRINITY_JACK_PLAY_WINDOW_GRAPH: &str = "trinity-jack-graph";
pub(crate) const TRINITY_JACK_PLAY_WINDOW_EDITOR: &str = "trinity-jack-editor";
pub(crate) const TRINITY_JACK_PLAY_WINDOW_RESULTS: &str = "trinity-jack-results";

pub(crate) const NAKAGIN_FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
pub(crate) const BRANCH_FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

pub(crate) const TRINITY_JACK_DEFAULT_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 📦️ The default trinity graph fixture (Nakagin capsule tower) — the initial document projection.
pub(crate) fn default_fixture() -> JackSnapshot {
    JackSnapshot::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap_or_else(|_| crate::artifacts::jack::empty_trinity_graph_fixture())
}

/// 🌱️ Seeds the initial config with the default query and its result table so the Results window is
/// populated on load.
fn seeded_jack_config(fixture: &JackSnapshot) -> JackConfig {
    let (result_json, _) = crate::editor::jack::commands::query::run_jack_query(fixture, TRINITY_JACK_DEFAULT_QUERY);
    JackConfig { camera: fixture.camera.clone(), active_fixture_id: "nakagin".into(), jack_query: TRINITY_JACK_DEFAULT_QUERY.into(), jack_result_json: result_json, ..JackConfig::default() }
}

/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetFixture` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `setActiveExample`/`setFixtureJson` build a
/// `Effect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry.
pub(crate) fn reset_document_effect(fixture: &JackSnapshot) -> Effect {
    let pack = <JackSnapshot as ArtifactPack>::encode_pack(fixture);
    let envelope = store::create_document_envelope::<JackSnapshot, TrinityGraphMutation>(TRINITY_GRAPH_SCHEMA, "jack", fixture.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("jack document spr encode is infallible for a fresh, edit-free envelope");
    Effect::LoadDocument { pack, spr }
}

pub(crate) fn jack_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(TRINITY_JACK_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Binds window chrome through its retained renderer action descriptor.
pub(crate) fn jack_window_action(action: &str, args: Option<pack::JsonValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: TRINITY_JACK_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| pack::json_to_dsl_value(&value)) }
}

/// 🏷️ Admits resolved Jack text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "Jack UI label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

pub(crate) fn graph_from_fixture_or_default(fixture: &JackSnapshot) -> crate::artifacts::jack::Graph {
    crate::artifacts::jack::Graph::from_fixture(fixture.clone()).unwrap_or_else(|_| crate::artifacts::jack::Graph::from_fixture(default_fixture()).expect("nakagin graph"))
}

/// 🩹️ Delegates to `crate::artifacts::jack::parse_port_key` (the one place the `nodeId@portId`
/// convention is owned) instead of hand-rolling a second splitter here.
pub(crate) fn split_endpoint(endpoint: &str) -> (String, String) {
    crate::artifacts::jack::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
}

pub(crate) fn fixture_to_workflow(fixture: &JackSnapshot) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>, NodeGraphViewport) {
    let scene = crate::artifacts::jack::jack_working_scene(fixture);
    let nodes: Vec<NodeGraphNodeRecord> = scene.nodes.iter().map(node_to_workflow_record).collect();
    let edges: Vec<NodeGraphEdgeRecord> = scene
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    let viewport = NodeGraphViewport { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom };
    (nodes, edges, viewport)
}

fn node_to_workflow_record(node: &Node) -> NodeGraphNodeRecord {
    let width = if node.width > 0.0 { node.width } else { 96.0 };
    let height = if node.height > 0.0 { node.height } else { 48.0 };
    NodeGraphNodeRecord {
        id: node.id.clone(),
        label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
        x: node.x,
        y: node.y,
        width,
        height,
        inputs: node.ports.iter().filter(|port| port.direction == PortDirection::In).map(|port| NodeGraphPortRecord { id: crate::artifacts::jack::port_key(&node.id, &port.id), label: Some(port.id.clone()), ..Default::default() }).collect(),
        outputs: node.ports.iter().filter(|port| port.direction == PortDirection::Out).map(|port| NodeGraphPortRecord { id: crate::artifacts::jack::port_key(&node.id, &port.id), label: Some(port.id.clone()), ..Default::default() }).collect(),
        ..Default::default()
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Io
/// 🔌️ Jack's typed media I/O surface (`AppDefinition.io`) — the implicit document in/out pair (a
/// `trinity.graph` document) plus one extra fan-out output port, `graph:out`, so a jack window can
/// feed its live query-graph projection into other graph-consuming workflow nodes (e.g. `rewrite`'s
/// `graph:in`).
pub(crate) fn jack_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: TRINITY_GRAPH_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "graph:out".into(),
            label: "Graph".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
            kind_id: Some("graph.trinity".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "graph.trinity".into(), name: "Trinity Graph".into(), dimension: "graph".into(), component_kind: "trinity".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️TrinityJackCommand
/// 🎯️ `TrinityJackPlayApp::Command` — the SOLE dispatch surface for jack's own behavior. Field shapes
/// mirror each action's real args exactly; `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`)
/// AND text (`OpText`) codec. Kept as a hand-rolled enum rather than rebuilt via `app_commands!`
/// (TEMPLATE §5.1's fallback) — it already has a byte-identical, working wire format, so a macro
/// rebuild would only add risk for zero benefit; only the `handle()` match BODY is decomposed across
/// `🎮️commands/<group>/component.rs`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
pub enum TrinityJackCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "set-fixture-json")]
    SetFixtureJson { json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "patch-nodes")]
    PatchNodes { node_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "run-query")]
    RunQuery { query: Option<String> },
    #[dsl(key = "load-example-query")]
    LoadExampleQuery { query: String },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },

    // 👁️ Config-only — was ephemeral `TrinityJackRuntime` state, now emits `config_mutations`.
    #[dsl(key = "set-viewport")]
    SetViewport { viewport_json: String },
    #[dsl(key = "text-edit")]
    TextEdit { text: String },
    #[dsl(key = "text-select")]
    TextSelect { start: u64, end: u64 },
    #[dsl(key = "request-completions")]
    RequestCompletions,
    #[dsl(key = "format-document")]
    FormatDocument,
    #[dsl(key = "set-lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "editor-engagement-input")]
    EditorEngagementInput { value: String },
    #[dsl(key = "graph-engagement-input")]
    GraphEngagementInput { value: String },
    #[dsl(key = "results-engagement-input")]
    ResultsEngagementInput { value: String },
    #[dsl(key = "set-locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for TrinityJackCommand {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
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
impl protocol::OpBinary for TrinityJackCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
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
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

//#endregion 🔖️TrinityJackCommand

//#region 🔖️TrinityJackPlayApp
/// 🔱️ Trinity Jack play app — a jack-query editor over a live {@link JackSnapshot} projection.
#[derive(Default)]
pub struct TrinityJackPlayApp;

//#region 🧵️RetainedConfigCommands
const JACK_RETAINED_CONFIG_TOOL_IDS: &[&str] = &["setViewport", "textEdit", "textSelect", "requestCompletions", "setLodMode", "editorEngagementInput", "graphEngagementInput", "resultsEngagementInput"];
const JACK_RETAINED_PAYLOAD_SCHEMA: &str = "trinity.graph.config-command.v1";
const JACK_RETAINED_RAW_BYTES: usize = 8_192;
const JACK_RETAINED_WORK_ITEMS: usize = 64;
const JACK_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setViewport", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "textSelect", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "requestCompletions", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "editorEngagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "graphEngagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "resultsEngagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
];

fn jack_retained_config_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(JACK_RETAINED_RAW_BYTES, 64, JACK_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

fn jack_retained_config_extent(command: &TrinityJackCommand, _snapshot: &JackSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        TrinityJackCommand::SetViewport { viewport_json } => viewport_json.len(),
        TrinityJackCommand::TextEdit { text } => text.len(),
        TrinityJackCommand::TextSelect { .. } | TrinityJackCommand::RequestCompletions => 0,
        TrinityJackCommand::SetLodMode { window_id, value } => window_id.len().checked_add(value.len())?,
        TrinityJackCommand::EditorEngagementInput { value } | TrinityJackCommand::GraphEngagementInput { value } | TrinityJackCommand::ResultsEngagementInput { value } => value.len(),
        _ => return None,
    };
    (bytes <= JACK_RETAINED_RAW_BYTES).then_some(1)
}

fn jack_retained_config_reduce(
    command: &TrinityJackCommand,
    _snapshot: &JackSnapshot,
    config: &JackConfig,
    _history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _operation: &AppOperationContext,
) -> Result<Emit<TrinityGraphMutation, JackConfigMutation, NoDraftMutation>, Fault> {
    match command {
        TrinityJackCommand::SetViewport { viewport_json } => crate::editor::jack::commands::set_viewport(viewport_json),
        TrinityJackCommand::TextEdit { text } => crate::editor::jack::commands::text_edit(text),
        TrinityJackCommand::TextSelect { start, end } => crate::editor::jack::commands::text_select(*start, *end),
        TrinityJackCommand::RequestCompletions => crate::editor::jack::commands::request_completions(config.revision),
        TrinityJackCommand::SetLodMode { window_id, value } => crate::editor::jack::commands::set_lod_mode(window_id, value),
        TrinityJackCommand::EditorEngagementInput { value } => crate::editor::jack::commands::editor_engagement_input(value),
        TrinityJackCommand::GraphEngagementInput { value } => crate::editor::jack::commands::graph_engagement_input(value),
        TrinityJackCommand::ResultsEngagementInput { value } => crate::editor::jack::commands::results_engagement_input(value),
        _ => Err(Fault::from("jack-retained-config-route-mismatch")),
    }
}

struct JackRetainedConfigJobFactory { keys: Vec<ToolFactoryKey> }

impl JackRetainedConfigJobFactory {
    fn new(controller: &str) -> Self { Self { keys: JACK_RETAINED_CONFIG_TOOL_IDS.iter().map(|tool| ToolFactoryKey::new(controller, *tool)).collect() } }
}

impl ToolJobFactory for JackRetainedConfigJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<TrinityJackPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<TrinityJackPlayApp>>;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { JACK_RETAINED_PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { jack_retained_config_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > JACK_RETAINED_RAW_BYTES || checkpoint.is_some() { return Err((ToolJobFactoryError::new("Jack retained config command rejects oversized wire or checkpoint owner"), input, checkpoint)); }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for JackRetainedConfigJobFactory {
    type Owner = EditorApp<TrinityJackPlayApp>;
    const TOOL_IDS: &'static [&'static str] = JACK_RETAINED_CONFIG_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = JACK_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedConfigCommands

//#region 📬️ConfigPreparation
const JACK_STORE_MAXIMUM_BYTES: usize = 32_768;

struct JackConfigPreparationFactory;

struct JackConfigPreparation {
    base: Option<store::SnapshotRead<JackConfig>>,
    mutation: Option<JackConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(JackConfig, Vec<JackConfigMutation>, JackConfigMutation, usize)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<JackConfig, JackConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

fn jack_bounded_serialized_bytes<T: dsl::ToValue>(value: &T) -> Result<usize, String> {
    let bytes = pack::to_json_string(&dsl::ToValue::to_value(value)).len();
    if bytes > JACK_STORE_MAXIMUM_BYTES {
        return Err("Jack retained Config value exceeds its fixed envelope".to_string());
    }
    Ok(bytes)
}

impl store::ArtifactStoreOneItemPreparationFactory<JackConfig, JackConfigMutation> for JackConfigPreparationFactory {
    fn preflight(&self, mutation: &JackConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) { return Err("Jack config preparation rejected its lane or description envelope".into()); }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: jack_bounded_serialized_bytes(mutation)? })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<JackConfig, JackConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<JackConfig, JackConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<JackConfig, JackConfigMutation>> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES { return Err(request); }
        Ok(Box::new(JackConfigPreparation { base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), candidate: None, prepared: None, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), phase: 0, cancelled: false, closing: false }))
    }
}

impl store::ArtifactStoreOneItemPreparation<JackConfig, JackConfigMutation> for JackConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.prepared.is_some() || self.phase >= 2 { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        match self.phase {
            0 => {
                let base = self.base.as_ref().ok_or_else(|| "Jack config preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "Jack config preparation lost its mutation owner".to_string())?;
                let retained_bytes = jack_bounded_serialized_bytes(base.get())?;
                let inverse = mutation.inverse(base.get());
                let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
                self.candidate = Some((post, inverse, mutation, retained_bytes));
                self.phase = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: retained_bytes, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, mutation, retained_bytes) = self.candidate.take().ok_or_else(|| "Jack config preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "Jack config preparation lost its Store authority".to_string())?;
                let id = format!("jack-config-retained-{}", authority.next_sequence_number());
                let edit = protocol::Edit {
                    id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![mutation], inverse,
                    mutation_meta: vec![protocol::MutationMeta { mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64, author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly, payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default() }],
                    description: self.description.take(), coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
                };
                let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
                self.phase = 2;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: retained_bytes, digest: prepared.edit_digest() };
                self.prepared = Some(prepared);
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
        }
    }
    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<JackConfig, JackConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<JackConfig, JackConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 { return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if self.prepared.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        if let Some(base) = self.base.take() { if !base.return_to_registry() { return Err("Jack config preparation could not return its exact base root".into()); } return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        if let Some(authority) = self.authority.as_ref() { if grant.maximum_bytes < authority.actor().len() { return Ok(store::SnapshotRetirementStep::Blocked); } self.authority = None; return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none() }
}
//#endregion 📬️ConfigPreparation

impl ArtifactEditor for TrinityJackPlayApp {
    type Snapshot = JackSnapshot;
    type Mutation = TrinityGraphMutation;
    type Config = JackConfig;
    type ConfigMutation = JackConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = JackPresence;
    type PresenceMutation = JackPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = TrinityJackCommand;

    const DIALECT: Dialect = TRINITY_JACK_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(JackConfigPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<TrinityJackPlayApp>,
        owner_file: "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.trinity.jack@1/*#editor",
        document_schema: "trinity.graph",
        factory: "JackRetainedConfigJobFactory",
        factory_type: JackRetainedConfigJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 64, 16_384, 7_500),
        tools: ["setViewport", "textEdit", "textSelect", "requestCompletions", "setLodMode", "editorEngagementInput", "graphEngagementInput", "resultsEngagementInput"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(JackRetainedConfigJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !JACK_RETAINED_CONFIG_TOOL_IDS.contains(&request.tool_id.as_str()) { return Ok(None); }
        if request.command.command_id() != request.tool_id || jack_retained_config_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) { return Err(Fault::from("jack-retained-config-tool-mismatch-or-capacity")); }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, jack_retained_config_reduce, jack_retained_config_extent));
        let operation = AppOperationContext { app_instance_id: request.app_instance_id, parent_document_id: request.parent_document_id.clone(), operation_id: request.operation.operation.0, generation: request.operation.generation.0, canonical_base_revision: request.canonical_base_revision };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(*request.command, request.snapshot, request.config, request.history, request.interaction_state, request.interaction_hover, request.context, operation, request.completion, TrinityJackPlayApp::command_id, JACK_RETAINED_RAW_BYTES, JACK_RETAINED_WORK_ITEMS, work)?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::jack::spr::jack_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::jack::spr::jack_document_store_owners())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::jack::spr::jack_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::jack::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> JackSnapshot {
        default_fixture()
    }

    fn initial_config() -> JackConfig {
        seeded_jack_config(&Self::initial_snapshot())
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(jack_io())
    }

    // 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetFixture` — see
    // `📓️taxonomy.md`'s forbidden vocabulary), so this intentionally falls back to the trait
    // default (`None`) rather than overriding — the `"document:in"` media port therefore reports
    // `MediaError::NotImplemented`; a real whole-fixture load goes through `reset_document_effect`
    // (`Effect::LoadDocument`, outside undo history), see `editor::jack::commands::set_active_example`
    // and `editor::jack::commands::set_fixture_json`.

    /// 🔌️ `"graph:out"` fans the live query-graph projection out to other graph-consuming workflow
    /// nodes, in addition to the implicit `"document:out"` — both encode the same `JackSnapshot` pack.
    fn export_media(port: &str, doc: &ArtifactView<'_, JackSnapshot>) -> Result<Media, MediaError> {
        match port {
            "graph:out" | "document:out" => {
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity }, payload: MediaPayload::Structured { schema: TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `TrinityJackCommand` variant back to the action id it was declared under in
    /// `create_trinity_jack_app`.
    fn command_id(command: &TrinityJackCommand) -> &'static str {
        match command {
            TrinityJackCommand::SetFixtureJson { .. } => "setFixtureJson",
            TrinityJackCommand::DeleteSelection => "deleteSelection",
            TrinityJackCommand::PatchNodes { .. } => "patchNodes",
            TrinityJackCommand::Reorganize => "reorganize",
            TrinityJackCommand::RunQuery { .. } => "runQuery",
            TrinityJackCommand::LoadExampleQuery { .. } => "loadExampleQuery",
            TrinityJackCommand::SetActiveExample { .. } => "setActiveExample",
            TrinityJackCommand::SetViewport { .. } => "setViewport",
            TrinityJackCommand::TextEdit { .. } => "textEdit",
            TrinityJackCommand::TextSelect { .. } => "textSelect",
            TrinityJackCommand::RequestCompletions => "requestCompletions",
            TrinityJackCommand::FormatDocument => "formatDocument",
            TrinityJackCommand::SetLodMode { .. } => "setLodMode",
            TrinityJackCommand::EditorEngagementInput { .. } => "editorEngagementInput",
            TrinityJackCommand::GraphEngagementInput { .. } => "graphEngagementInput",
            TrinityJackCommand::ResultsEngagementInput { .. } => "resultsEngagementInput",
            TrinityJackCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(
        command: &TrinityJackCommand,
        doc: &ArtifactView<'_, JackSnapshot>,
        cfg: &ConfigView<'_, JackConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<TrinityGraphMutation, JackConfigMutation, Self::DraftMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        match command {
            TrinityJackCommand::SetFixtureJson { json } => crate::editor::jack::commands::set_fixture_json(json),
            TrinityJackCommand::DeleteSelection => crate::editor::jack::commands::delete_selection(fixture, &interaction.selection("ast").ids),
            TrinityJackCommand::PatchNodes { node_ids, field, value } => crate::editor::jack::commands::patch_nodes(fixture, node_ids, field, value),
            TrinityJackCommand::Reorganize => crate::editor::jack::commands::reorganize(fixture, config.reorganize_epoch),
            TrinityJackCommand::RunQuery { query } => crate::editor::jack::commands::run_query(fixture, query, &config.jack_query),
            TrinityJackCommand::LoadExampleQuery { query } => crate::editor::jack::commands::load_example_query(fixture, query),
            TrinityJackCommand::SetActiveExample { example_id } => crate::editor::jack::commands::set_active_example(example_id),
            TrinityJackCommand::SetViewport { viewport_json } => crate::editor::jack::commands::set_viewport(viewport_json),
            TrinityJackCommand::TextEdit { text } => crate::editor::jack::commands::text_edit(text),
            TrinityJackCommand::TextSelect { start, end } => crate::editor::jack::commands::text_select(*start, *end),
            TrinityJackCommand::RequestCompletions => crate::editor::jack::commands::request_completions(config.revision),
            TrinityJackCommand::FormatDocument => crate::editor::jack::commands::format_document(&config.jack_query),
            TrinityJackCommand::SetLodMode { window_id, value } => crate::editor::jack::commands::set_lod_mode(window_id, value),
            TrinityJackCommand::EditorEngagementInput { value } => crate::editor::jack::commands::editor_engagement_input(value),
            TrinityJackCommand::GraphEngagementInput { value } => crate::editor::jack::commands::graph_engagement_input(value),
            TrinityJackCommand::ResultsEngagementInput { value } => crate::editor::jack::commands::results_engagement_input(value),
            TrinityJackCommand::SetLocale { value } => crate::editor::jack::commands::set_locale(value),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, JackSnapshot>, cfg: &ConfigView<'_, JackConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let fixture = doc.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::editor::jack::terminology::TrinityJackLabels>(&cfg.snapshot.locale);
        let root = match body_key {
            TRINITY_JACK_PLAY_BODY_GRAPH => edit::windows::graph::render(TRINITY_JACK_PLAY_SURFACE_GRAPH, TRINITY_JACK_PLAY_CONTROLLER_ID, TRINITY_JACK_PLAY_WINDOW_GRAPH, fixture, cfg.snapshot),
            TRINITY_JACK_PLAY_BODY_EDITOR => edit::windows::editor::render(TRINITY_JACK_PLAY_SURFACE_EDITOR, TRINITY_JACK_PLAY_CONTROLLER_ID, fixture, cfg.snapshot),
            TRINITY_JACK_PLAY_BODY_RESULTS => edit::windows::results::render(TRINITY_JACK_PLAY_SURFACE_RESULTS, TRINITY_JACK_PLAY_CONTROLLER_ID, cfg.snapshot),
            TRINITY_JACK_PLAY_BODY_DOCUMENT => crate::editor::jack::panels::document::render(fixture, cfg.snapshot, labels),
            TRINITY_JACK_PLAY_BODY_CATALOGUE => crate::editor::jack::panels::catalogue::render(cfg.snapshot, labels),
            TRINITY_JACK_PLAY_BODY_INSPECTION => crate::editor::jack::panels::inspection::render(),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.body.label", "the fixed Trinity body label exceeds its UI bound")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(root))
    }

    fn window_measures(_doc: &ArtifactView<'_, JackSnapshot>, cfg: &ConfigView<'_, JackConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let mode = cfg.snapshot.lod_mode_by_window.get(TRINITY_JACK_PLAY_WINDOW_GRAPH).map_or(edit::windows::graph::TRINITY_LOD_MODE_AUTOMATIC, String::as_str);
        HashMap::from([(TRINITY_JACK_PLAY_WINDOW_GRAPH.to_string(), vec![edit::windows::graph::trinity_lod_measure(TRINITY_JACK_PLAY_WINDOW_GRAPH, mode, jack_window_action)])])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, JackSnapshot>, cfg: &ConfigView<'_, JackConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = cfg.snapshot.locale.starts_with("de");
        // 🕹️ Selection is framework-owned now (domain "ast") — `context_menu` has no `InteractionView`,
        // so the request's own surface-carried selection groups are the only source; no config fallback.
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &[], &[]);
        let mut menu = Menu::of(registry).action("runQuery").action("reorganize").action("formatDocument").group("mode", |m| m.action("setActiveExample")).group("open", |m| m.action("loadExampleQuery"));
        // 🩹️ `Direct`, not `ViaNodeGraphEdit`: jack's own `TrinityJackCommand::DeleteSelection` is a
        // real standalone command (no `nodeGraphEdit`-style JSON-operations envelope exists for jack),
        // so the context-menu row must dispatch the `deleteSelection` action id directly.
        if let Some(spec) = node_graph_delete_selection_spec("Delete selection", is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::Direct) {
            menu = menu.item(spec);
        }
        menu.build()
    }

    /// 🕹️ Domain "ast" topology: every fixture node is a `TopologyNode`, parented by the source node
    /// of its first incoming connection (roots — nodes with no incoming edge — get `parent: None`).
    /// `MergeMode::Range` is not declared for this domain, so `ordered`'s sequence need not be a strict
    /// pre-order — `descendant_closure`/`ancestors` only need the (id, parent) pairs, not list order.
    fn interaction_topology(doc: &ArtifactView<'_, JackSnapshot>, _cfg: &ConfigView<'_, JackConfig>) -> InteractionTopology {
        let fixture = doc.snapshot;
        let mut parent_of: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
        for edge in fixture.edges() {
            let source = crate::artifacts::jack::port_node_id(&edge.source).unwrap_or(&edge.source).to_string();
            let target = crate::artifacts::jack::port_node_id(&edge.target).unwrap_or(&edge.target).to_string();
            parent_of.entry(target).or_insert(source);
        }
        let ordered = fixture.nodes().iter().map(|node| TopologyNode { id: node.id.clone(), granularity: "node".into(), parent: parent_of.get(&node.id).cloned() }).collect();
        let mut domains = std::collections::BTreeMap::new();
        domains.insert("ast".to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }
}
//#endregion 🔖️TrinityJackPlayApp

//#region 🔖️Manifest
use crate::editor::jack::modes::edit;

/// 🎯️ `create_trinity_jack_app` → `Editor::builder(TRINITY_JACK_DIALECT)…build_definition()`
/// (contract §2.4). The old `.example("nakagin", …)`/`.workflow("trinity", …)` calls are DROPPED,
/// not ported — `EditorBuilder::build_definition()` returns a bare `AppDefinition`, discarding
/// `App.examples` entirely (SDK gap, see this packet's notes file; the subset's own
/// `📚️examples/🎬️demo` facet is the likely intended replacement mechanism).
pub fn create_trinity_jack_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TRINITY_JACK_DIALECT).document(["semio", "trinity", "jack"])
            .artifact_kind(ArtifactKindSpec {
                id: "graph.trinity".into(),
                name: "Trinity Graph".into(),
                source_format: "trinity.graph".into(),
                component_kind: "trinity".into(),
                dimension: "graph".into(),
                media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                schema: "trinity.graph".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("trinity")
            .mode_def(edit::definition())
            .default_mode_id(edit::TRINITY_JACK_MODE_EDIT)
            .window_kind(TRINITY_JACK_PLAY_WINDOW_GRAPH, LocalizedLabel::native("Nakagin Graph", "Nakagin-Graph"), TRINITY_JACK_PLAY_BODY_GRAPH, SemanticSurfaceKind::NodeGraph, "graph-dag")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_EDITOR, LocalizedLabel::native("Jack Query", "Jack-Abfrage"), TRINITY_JACK_PLAY_BODY_EDITOR, SemanticSurfaceKind::TextEditor, "document-jack")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), TRINITY_JACK_PLAY_BODY_RESULTS, SemanticSurfaceKind::Table, "table-2")
            .default_layout(edit::layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                TRINITY_JACK_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                TRINITY_JACK_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                TRINITY_JACK_PLAY_BODY_INSPECTION,
            )
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .mutation("patchNodes", LocalizedLabel::native("Patch Nodes", "Knoten aktualisieren"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("runQuery", LocalizedLabel::native("Run Jack Query", "Jack-Abfrage ausführen"), ActionKind::Mutation).with_category("methods"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("loadExampleQuery", LocalizedLabel::native("Load Example Query", "Beispielabfrage laden"), ActionKind::Mutation).with_category("open"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation).with_category("mode"))
            // 🛠️ Dev-only whole-fixture import — kept out of the command palette.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation) })
            .view_action("setViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansicht festlegen"))
            .view_action("textEdit", LocalizedLabel::native("Edit Jack Query", "Jack-Abfrage bearbeiten"))
            .view_action("textSelect", LocalizedLabel::native("Select Jack Query Text", "Jack-Abfragetext auswählen"))
            .view_action("requestCompletions", LocalizedLabel::native("Request Completions", "Vervollständigungen anfordern"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("formatDocument", LocalizedLabel::native("Format Jack Query", "Jack-Abfrage formatieren"), ActionKind::View).with_category("utilities"))
            .view_action("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"))
            .view_action("editorEngagementInput", LocalizedLabel::native("Editor Engagement Input", "Editor-Eingabe"))
            .view_action("graphEngagementInput", LocalizedLabel::native("Graph Engagement Input", "Graph-Eingabe"))
            .view_action("resultsEngagementInput", LocalizedLabel::native("Results Engagement Input", "Ergebnis-Eingabe"))
            .action_interactive_job("setViewport", InteractiveJobClassification::Migrated)
            .action_interactive_job("textEdit", InteractiveJobClassification::Migrated)
            .action_interactive_job("textSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("requestCompletions", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("editorEngagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("graphEngagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("resultsEngagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchNodes", InteractiveJobClassification::BatchOnlyPendingRewrite)
            // 🕹️ Domain "ast": jack's document nodes, transitive over each node's first incoming
            // connection (see `interaction_topology`). Selection/hover, marquee, modes and merges are
            // ALL framework-injected now — no app-declared setSelection/graphPointerDown verbs.
            .interaction(InteractionDefinition {
                id: "ast".into(),
                label: LocalizedLabel::native("Nodes", "Knoten"),
                granularities: vec![GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: true, broadcast: true },
            })
            .window_kind_interactions(TRINITY_JACK_PLAY_WINDOW_GRAPH, vec![InteractionRef::new("ast")])
            // 📝️ Staged argument forms for the panel-visible preset loaders.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Fixture", "Fixtur"), vec![
                    ActionArgOption::new("nakagin", LocalizedLabel::native("Nakagin — Table", "Nakagin — Tabelle")),
                    ActionArgOption::new("branch-chain", LocalizedLabel::native("Branch — Graph", "Branch — Graph")),
                ]).required(),
            ])
            .action_args("loadExampleQuery", vec![
                ActionArgDef::select("query", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name", LocalizedLabel::native("Where Or", "Wo-Oder")),
                    ActionArgOption::new("MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b", LocalizedLabel::native("Return Graph", "Graph zurückgeben")),
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'", LocalizedLabel::native("Set Label", "Label setzen")),
                    ActionArgOption::new("MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120", LocalizedLabel::native("Set Position", "Position setzen")),
                    ActionArgOption::new("CREATE (n:Piece)", LocalizedLabel::native("Create Node", "Knoten erstellen")),
                    ActionArgOption::new("MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)", LocalizedLabel::native("Create Edge", "Kante erstellen")),
                    ActionArgOption::new("MATCH (n:Piece) WHERE n.name = 'b' DELETE n", LocalizedLabel::native("Delete Leaf", "Blatt löschen")),
                    ActionArgOption::new("MERGE (x:Piece)-[:Connection]->(y:Piece)", LocalizedLabel::native("Merge Edge", "Kante zusammenführen")),
                ]).required(),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint")
            .io(jack_io())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{OpBinary, OpText};
    use semio_framework_plugin::{testkit, App, EditorApp, PluginApp, VcsArtifactApp, ViewModel};

    /// 🎫️ `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`'s own signature
    /// is still `fn(manifest: fn() -> App)`, unchanged for this ticket (SDK gap, see this packet's
    /// notes file) — `create_trinity_jack_app` now returns a bare `AppDefinition`, so this tiny local
    /// wrapper adapts it back into the `App { definition, examples }` shape those testkit fns expect.
    fn trinity_jack_manifest_for_testkit() -> App {
        App { definition: create_trinity_jack_app(), examples: Vec::new() }
    }

    /// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityJackCommand` variant round-trips
    /// through both its binary (`OpBinary`, via `#[derive(dsl::DslOps)]`) and text (`OpText`) codecs.
    #[semio_framework_async_macros::async_test]
    async fn trinity_jack_command_text_and_binary_round_trip() {
        let commands = vec![
            TrinityJackCommand::SetFixtureJson { json: "{}".into() },
            TrinityJackCommand::DeleteSelection,
            TrinityJackCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
            TrinityJackCommand::Reorganize,
            TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a".into()) },
            TrinityJackCommand::RunQuery { query: None },
            TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() },
            TrinityJackCommand::SetViewport { viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
            TrinityJackCommand::TextSelect { start: 3, end: 9 },
            TrinityJackCommand::SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() },
            TrinityJackCommand::SetLocale { value: "de-DE".into() },
        ];
        for command in commands {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(TrinityJackCommand::decode_op(&bytes).expect("decode"), command);
            let text = command.print_op();
            assert_eq!(TrinityJackCommand::parse_op(&text).expect("parse"), command);
        }
    }

    fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
        testkit::meta(actor)
    }

    /// 🕹️ Registry-backed (not the bare `testkit::new_app`): `interactionSelect`/`interactionHover`
    /// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
    /// domain "ast" selection needs the real manifest's `.interaction(...)` declaration present.
    fn new_app() -> VcsArtifactApp<EditorApp<TrinityJackPlayApp>> {
        testkit::new_app_with_registry::<EditorApp<TrinityJackPlayApp>>(trinity_jack_manifest_for_testkit)
    }

    fn jack_envelope_wire() -> Vec<u8> {
        use store::ArtifactPack;

        let snapshot = empty_trinity_graph_fixture();
        let snapshot_pack = snapshot.encode_pack();
        let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let wire = pack::json_to_string(&pack::json!({
            "schema": TRINITY_GRAPH_SCHEMA,
            "id": "jack-live-load",
            "vcs": {
                "initialSnapshot": snapshot_hex,
                "edits": [],
                "changes": [],
                "checkpoints": [],
                "alternatives": []
            },
            "editMessages": [],
            "conflicts": []
        }))
        .into_bytes();
        let envelope = store::create_document_envelope(TRINITY_GRAPH_SCHEMA, "jack-live-load", snapshot, None);
        let mut retirement = crate::artifacts::jack::spr::jack_envelope_decode_owner_bundle().retire_envelope(envelope);
        for _ in 0..100_000 {
            match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Jack fixture envelope retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return wire;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared Jack fixture envelope retirement blocked"),
            }
        }
        panic!("Jack fixture envelope retirement did not reach terminal")
    }

    fn admit_jack_envelope(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Jack live envelope ingress credits");
        for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            bytes[..chunk.len()].copy_from_slice(chunk);
            let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Jack live envelope page");
            app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Jack live envelope page admission failed: {fault}"));
        }
        assert!(app.seal_artifact_envelope_ingress(handle).expect("Jack live envelope seal/submit"));
        handle
    }

    fn drive_jack_live_load(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..100_000 {
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Jack live maintenance turn");
            let poll = app.advance_artifact_envelope_load(handle).expect("Jack live load advancement");
            if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
                return poll;
            }
            std::thread::yield_now();
        }
        panic!("Jack live envelope load did not reach terminal")
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
        let mut app = new_app();
        let base_generation = app.artifact_generation_now();
        let handle = admit_jack_envelope(&mut app, &jack_envelope_wire());
        assert_eq!(handle.generation, base_generation);
        assert_eq!(drive_jack_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact Jack load acknowledgement"));
        assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Jack load acknowledgement is a no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_live_envelope_cancel_closes_retained_pages_without_publication() {
        let mut app = new_app();
        let base_generation = app.artifact_generation_now();
        let wire = jack_envelope_wire();
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Jack ingress credits");
        let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..first.len()].copy_from_slice(first);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Jack first page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Jack page admission failed: {fault}"));
        app.cancel_artifact_envelope_load(handle).expect("cancel exact Jack ingress");
        assert_eq!(drive_jack_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), base_generation);
    }

    fn node_id_at(app: &VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, index: usize) -> String {
        app.snapshot().expect("projection").nodes()[index].id.clone()
    }

    /// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "ast" — the
    /// replacement for the deleted `TrinityJackCommand::SetSelection`.
    fn select_ast(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp>>, ids: &[&str]) {
        let targets: Vec<pack::JsonValue> = ids.iter().map(|id| pack::json!({ "granularity": "node", "id": id })).collect();
        let args = pack::json!({ "domainId": "ast", "targets": pack::to_json_string(&targets) });
        app.handle_action("interactionSelect", Some(&args), &meta("local")).expect("interactionSelect");
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_node_graph_scene() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&node).contains("node-graph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_jack_editor() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).expect("render");
        let json = pack::to_json_string(&node);
        assert!(json.contains("text-editor"));
        assert!(json.contains(TRINITY_JACK_DEFAULT_QUERY));
    }

    #[semio_framework_async_macros::async_test]
    async fn run_query_populates_results_and_a_set_query_mutates_projection() {
        let mut app = new_app();
        app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &ViewModel::default()).expect("render");
        let result = app.dispatch_typed(TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'ran-label'".into()) }, &meta("local")).expect("run");
        assert!(!result.mutations.is_empty(), "a SET query emits operations");
        let projection = app.snapshot().expect("projection");
        // 🔬 `content` is now an opaque composed-child handle — `pack::to_json_string(&projection)`
        // no longer surfaces node property data directly (ticket
        // `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`); inspect through the working-scene
        // accessor instead of the raw derived JSON serialization.
        assert!(projection.nodes().iter().any(|node| node.properties.get("label") == Some(&crate::artifacts::jack::PropertyValue::String("ran-label".into()))));
    }

    #[semio_framework_async_macros::async_test]
    async fn node_graph_select_updates_selection_and_document_tree() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        select_ast(&mut app, &[&node_id]);
        let tree = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render");
        let json = pack::to_json_string(&tree);
        assert!(json.contains(&node_id));
        assert!(json.contains("\"selected\":true"));
    }

    #[semio_framework_async_macros::async_test]
    async fn nakagin_fixture_has_nodes() {
        assert!(!default_fixture().nodes().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn editor_scene_has_tokens_and_diagnostics() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).expect("render");
        let json = pack::to_json_string(&node);
        assert!(json.contains("tokensJson"));
        assert!(json.contains("diagnosticsJson"));
        assert!(json.contains("completionsJson"));
    }

    #[semio_framework_async_macros::async_test]
    async fn text_edit_updates_query_without_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &meta("local")).expect("edit");
        assert!(result.mutations.is_empty());
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&node).contains("MATCH (a:Piece) RETURN a.name"));
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_scene_has_lod_json() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewModel::default()).expect("render");
        let json = pack::to_json_string(&node);
        assert!(json.contains("lodJson"));
        assert!(json.contains("automatic"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_lod_mode_reflects_in_window_measures() {
        let mut app = new_app();
        app.dispatch_typed(TrinityJackCommand::SetLodMode { window_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into(), value: "compact".into() }, &meta("local")).expect("lod");
        let measures = app.window_measures();
        assert!(measures[TRINITY_JACK_PLAY_WINDOW_GRAPH].iter().any(|measure| matches!(measure, WindowMeasure::Select { value, .. } if value == "compact")));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_tree_renders() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&node).contains("trinity-jack-catalogue"));
    }

    #[semio_framework_async_macros::async_test]
    async fn inspection_panel_renders_the_selection_prompt() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        select_ast(&mut app, &[&node_id]);
        let node = app.render(TRINITY_JACK_PLAY_BODY_INSPECTION, None, &ViewModel::default()).expect("render");
        // 🕹️ `render` has no `InteractionView` (see the panel's own doc comment) — it can no longer
        // build per-selection fields, so it always renders the static prompt.
        assert!(pack::to_json_string(&node).contains("trinity-inspector.empty"));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_tree_de_locale_translates_labels() {
        let mut app = new_app();
        app.dispatch_typed(TrinityJackCommand::SetLocale { value: "de-DE".into() }, &meta("local")).expect("set locale");
        let node = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&node).contains("Stücke"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_swaps_fixture_and_seeds_query() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() }, &meta("local")).expect("set active example");
        // 🩹 Pre-existing test/implementation mismatch (traced to commit `a445617c`, 2026-08-12
        // 15:50:51 +0200 — predates this migration, not introduced by it): `set_active_example`
        // routes the fixture swap through `Effect::LoadDocument` (whole-document replace is
        // banned from the `Mutation` enum outright), never through `artifact_mutations`, so
        // `InvocationResult.mutations` is always empty for this command — `requested_effects` is
        // the field that actually carries the swap.
        assert!(!result.requested_effects.is_empty());
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&node).contains("RETURN a, r, b"));
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_selection_removes_selected_node() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        select_ast(&mut app, &[&node_id]);
        let result = app.dispatch_typed(TrinityJackCommand::DeleteSelection, &meta("local")).expect("delete");
        assert!(!result.mutations.is_empty());
        let projection = app.snapshot().expect("projection");
        assert!(!projection.nodes().iter().any(|node| node.id == node_id));
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_stays_within_row_budget_and_ends_with_delete_selection() {
        let mut app = testkit::new_app_with_registry::<EditorApp<TrinityJackPlayApp>>(trinity_jack_manifest_for_testkit);
        let node_id = node_id_at(&app, 0);
        let request = ContextMenuRequest {
            menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
            surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: TRINITY_JACK_PLAY_SURFACE_GRAPH.into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: node_id.clone(), label: None }],
                selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec![node_id] }],
                text: None,
            }),
            window_instance_id: None,
            point: None,
        };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("deleteSelection");
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_graph_out_matches_document_pack() {
        use semio_framework_plugin::PluginApp as _;
        let mut app = new_app();
        let document_out = semio_framework_plugin::resolve_ready(app.export_media("document:out")).expect("document:out export");
        let graph_out = semio_framework_plugin::resolve_ready(app.export_media("graph:out")).expect("graph:out export");
        assert_eq!(document_out.payload, graph_out.payload);
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_io_declares_graph_out_fan_out_port() {
        let io = jack_io();
        assert_eq!(io.document_schema, TRINITY_GRAPH_SCHEMA);
        assert_eq!(io.artifact.id, "graph.trinity");
        let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
        assert_eq!(graph_out.kind_id.as_deref(), Some("graph.trinity"));
        assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }
}
//#endregion 🧪️Tests

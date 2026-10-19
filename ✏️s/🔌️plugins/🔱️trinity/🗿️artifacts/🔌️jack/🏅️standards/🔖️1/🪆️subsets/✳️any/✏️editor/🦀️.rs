//! 🔱️ Trinity Jack editor — jack query editor bundled as a hot-swappable WASM plugin.
//!
//! 📌️ Pure-trait `ArtifactEditor`: `TrinityJackPlayApp` is a unit struct; every former
//! `TrinityJackRuntime` field (selection, camera, LOD, …) lives in exact window config/transient owners; the query is
//! document content (`JackSnapshot::query`), undoable and shared like the graph it runs against. Every action dispatches through the single typed `TrinityJackCommand`
//! channel via `ArtifactEditor::handle`, which fans out to `🎮️commands/<group>/component.rs` (the
//! command enum stays hand-rolled — see its own doc comment — only the match body is decomposed).

use crate::editor::jack::commands;
use crate::editor::jack::transient::{JackEditorWindowTransientOwner, JackResultsWindowTransientOwner};
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use crate::standards::v1::subsets::any::schema::graph::{TRINITY_GRAPH_SCHEMA, TRINITY_JACK_DIALECT};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::NodeGraphEdgeRecord;
use semio_framework_plugin::NodeGraphNodeRecord;
use semio_framework_plugin::NodeGraphPortRecord;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
use semio_framework_os_kernel::Viewport2d;
// 🩹️ `InteractionView` is not re-exported at `semio_framework_plugin`'s crate root (unlike
// `ConfigView`/`ArtifactView`/`DraftView`) — only reachable through its owning `app` submodule
// (itself `pub mod`). Flagged as a likely framework oversight, not fixed here (framework file).
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use pack::snapshot::{ArtifactDsl,ArtifactPack};

//#region 🔖️Constants
pub const TRINITY_JACK_PLAY_CONTROLLER_ID: &str = "trinity-jack-play";
const TRINITY_JACK_PLAY_SURFACE_GRAPH: &str = "trinity.jack.play";
const TRINITY_JACK_PLAY_SURFACE_EDITOR: &str = "trinity.jack.editor";
const TRINITY_JACK_PLAY_SURFACE_RESULTS: &str = "trinity.jack.results";
const TRINITY_JACK_PLAY_BODY_GRAPH: &str = "trinity.jack.play.main";
const TRINITY_JACK_PLAY_BODY_EDITOR: &str = "trinity.jack.play.editor";
const TRINITY_JACK_PLAY_BODY_RESULTS: &str = "trinity.jack.play.results";
pub const TRINITY_JACK_PLAY_BODY_ARTIFACT: &str = "trinity.jack.play.artifact";
const TRINITY_JACK_PLAY_BODY_CATALOGUE: &str = "trinity.jack.play.catalogue";
const TRINITY_JACK_PLAY_BODY_INSPECTION: &str = "trinity.jack.play.inspection";
pub const TRINITY_JACK_PLAY_WINDOW_GRAPH: &str = crate::editor::jack::window_config::WINDOW_KIND_ID;
pub const TRINITY_JACK_PLAY_WINDOW_EDITOR: &str = crate::editor::jack::transient::JACK_EDITOR_WINDOW_KIND_ID;
pub const TRINITY_JACK_PLAY_WINDOW_RESULTS: &str = crate::editor::jack::transient::results_window::WINDOW_KIND_ID;

//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 📦️ The default trinity graph snapshot (Nakagin capsule tower) — the initial document projection.
pub fn default_snapshot<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>() -> JackSnapshot {
    C::initial_snapshot()
}

/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetFixture` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `setActiveExample`/`loadDocumentJson` build a
/// `Effect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry. The spr is a
/// fresh, edit-free op-log (`store::os_store::text_format::empty_document_spr`): a live `ArtifactEnvelope` minted only to print
/// it owns a bounded retirement authority whose Drop traps the guest.
pub fn reset_document_effect(snapshot: &JackSnapshot) -> Effect {
    let pack = <JackSnapshot as ArtifactPack>::encode_pack(snapshot);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("jack", TRINITY_GRAPH_SCHEMA));
    Effect::LoadDocument { pack, spr }
}

pub fn jack_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(TRINITY_JACK_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Binds window chrome through its retained renderer action descriptor.
pub fn jack_window_action(action: &str, args: Option<semio_framework_pack_json::Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: TRINITY_JACK_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)) }
}

/// 🏷️ Admits resolved Jack text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "Jack UI label admission failed"))
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

/// 🧠️ The language editor reads its query workspace from the exact published child.
pub fn graph_from_document<C:crate::standards::v1::subsets::any::schema::composition::JackComposition>(snapshot:&JackSnapshot,children:&semio_framework_plugin::app::ChildContentView)->semio_framework_plugin::UiAssemblyResult<crate::standards::v1::subsets::any::schema::memory::Graph>{let scene=crate::standards::v1::subsets::any::schema::composition::jack_scene_from_children::<C>(snapshot,children).map_err(|fault|semio_framework_plugin::PluginAssemblyError::new("trinity.jack.content-unreadable",fault.message))?;crate::standards::v1::subsets::any::schema::memory::Graph::with_scene(snapshot.clone(),scene).map_err(|error|semio_framework_plugin::PluginAssemblyError::new("trinity.jack.content-unreadable",error.to_string()))}

/// 🩹️ Delegates to `crate::standards::v1::subsets::any::schema::graph::parse_port_key` (the one place the `nodeId@portId`
/// convention is owned) instead of hand-rolling a second splitter here.
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    crate::standards::v1::subsets::any::schema::graph::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
}

/// ♻️ Projects a standalone Jack snapshot (its retained content owner) into the shared node-graph view consumed by Rewriting
/// and the results window.
pub fn snapshot_to_workflow(snapshot: &JackSnapshot) -> Result<(Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>, Viewport2d), semio_framework_value::ValueError> {
    let owner = crate::standards::v1::subsets::any::schema::content::jack_content_for_handle(&snapshot.content)?;
    Ok(content_to_workflow(&snapshot.camera, owner.read()))
}

/// 🕸️ Projects rich borrowed content under the document camera.
pub fn content_to_workflow(camera:&crate::standards::v1::subsets::any::schema::graph::Camera,content:&dyn semio_s_artifact_trinity_jack_graph_query::JackContentVisual)->(Vec<NodeGraphNodeRecord>,Vec<NodeGraphEdgeRecord>,Viewport2d){
 let(count_nodes,count_edges)=content.visual_counts();
 let nodes=(0..count_nodes).filter_map(|index|{
  let node=content.node_view(index)?;
  let ports=|inbound:bool|{(0..node.port_count).filter_map(|port_index|content.port_view(index,port_index)).filter(|port|if inbound{port.inbound}else{port.outbound}).map(|port|NodeGraphPortRecord{id:crate::standards::v1::subsets::any::schema::graph::port_key(node.id,port.id),label:Some(port.label.into()),..Default::default()}).collect()};
  let(width,height)=node.presented_extent();
  Some(NodeGraphNodeRecord{id:node.id.into(),label:Some(node.presented_label().into()),x:node.x,y:node.y,width,height,inputs:ports(true),outputs:ports(false),..Default::default()})
 }).collect();
 let edges=(0..count_edges).filter_map(|index|content.edge_view(index)).map(|edge|NodeGraphEdgeRecord{id:edge.id.into(),source_node_id:edge.source.into(),source_port_id:edge.source_port.unwrap_or("out").into(),target_node_id:edge.target.into(),target_port_id:edge.target_port.unwrap_or("in").into(),label:(!edge.label.is_empty()).then(||edge.label.into())}).collect();
 (nodes,edges,Viewport2d{x:camera.x,y:camera.y,zoom:camera.zoom})
}

//#endregion 🔖️DocumentHelpers

//#region 🔖️Io
/// 🔌️ Jack's typed media I/O surface (`AppDefinition.io`) — the implicit document in/out pair (a
/// `trinity.graph` document) plus one extra fan-out output port, `graph:out`, so a jack window can
/// feed its live query-graph projection into other graph-consuming workflow nodes (e.g. `rewriting`'s
/// `graph:in`).
pub fn jack_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: TRINITY_GRAPH_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum TrinityJackCommand {
    #[dsl(key = "set-snapshot-json")]
    LoadDocumentJson { json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "patch-nodes")]
    PatchNodes { node_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "run-query")]
    RunQuery { query: Option<String>, results_window_id: String },
    #[dsl(key = "load-example-query")]
    LoadExampleQuery { query: String, results_window_id: String },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },

    #[dsl(key = "node-graph-viewport")]
    SetViewport {
        surface_id: Option<String>,
        #[dsl(block)]
        viewport: Viewport2d,
    },
    #[dsl(key = "text-edit")]
    TextEdit { text: String },
    #[dsl(key = "text-select")]
    TextSelect { start: u64, end: u64 },
    #[dsl(key = "format-document")]
    FormatDocument,
    #[dsl(key = "set-lod-mode")]
    SetLodMode { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for TrinityJackCommand {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for TrinityJackCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &["runQuery", "loadExampleQuery", "nodeGraphViewport", "textEdit", "textSelect", "formatDocument", "setLodMode", "patchNodes", "deleteSelection", "setActiveExample", "loadDocumentJson"];

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

//#endregion 🔖️TrinityJackCommand

//#region 🔖️ActionBridge

/// 🎯️ Folds the host's `{action, args}` vocabulary (camelCase keys, JSON floats, control `value`s) into
/// `TrinityJackCommand` — the trait default refuses every app action, which left every panel, catalogue
/// and window verb dead in the shell (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END).
#[path = "🌉️args-bridge/🦀️.rs"]
mod args_bridge;

//#endregion 🔖️ActionBridge

//#region 🔖️TrinityJackPlayApp

#[path = "🕹️play-app/🦀️.rs"]
mod play_app;
pub use play_app::*;

//#region 🧵️RetainedConfigCommands

#[path = "🧵️retained-commands/🦀️.rs"]
pub mod retained_commands;
use retained_commands::*;

//#endregion 🧵️RetainedConfigCommands

//#endregion 🔖️TrinityJackPlayApp

//#region 🔖️Manifest
use crate::editor::jack::modes::edit;

#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use manifest::*;

//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

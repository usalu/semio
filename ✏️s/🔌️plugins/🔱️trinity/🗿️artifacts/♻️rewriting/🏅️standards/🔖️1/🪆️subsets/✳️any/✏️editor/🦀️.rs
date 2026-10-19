//! ♻️ Trinity Rewriting editor — parametric rewriting editor bundled as a hot-swappable WASM plugin.
//!
//! 📌️ Pure-trait `ArtifactEditor`: `TrinityRewritingPlayApp` is a unit struct; every former
//! `RewritingPlayRuntime` field (selection, hover/select var, camera, LOD, …) lives in
//! concrete-window configuration; the app itself has no config record. Every rule/parameter/
//! working-graph edit flows through the semantic `RewriteRuleMutation` vocabulary (`edit-*` body
//! replaces, `change-*`/`remove-*` map upserts): every command builds exactly the leaves its intent means, never a diff of a
//! scratch copy of the whole rule. The `TrinityRewritingCommand` enum stays hand-rolled (TEMPLATE §5.1 fallback, same rationale
//! as `jack`).

use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::JackWorkingScene;
use crate::editor::rewriting::window_config;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::{self};
use crate::standards::v1::subsets::any::schema::rule::{ParameterKind, Rhs};
use crate::standards::v1::subsets::any::schema::model::{LayoutPoint, REWRITE_RULE_SCHEMA, TRINITY_REWRITING_DIALECT};
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use semio_framework_graph_manifest::{PropertyBag};
#[cfg(test)] use semio_framework_graph_manifest::{PropertyValue};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
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
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
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
use semio_framework_plugin::{EditorApp, NoConfig, NoConfigMutation};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::graph::{Camera, Node};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
// 🩹️ `InteractionView` is not re-exported at `semio_framework_plugin`'s crate root (unlike
// `ConfigView`/`ArtifactView`/`DraftView`) — only reachable through its owning `app` submodule
// (itself `pub mod`). Flagged as a likely framework oversight, not fixed here (framework file).
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
use std::collections::{BTreeMap, HashMap};
use semio_framework_2d_compute::EngineHandles;
use store::{pack::snapshot::ArtifactPack};
#[cfg(test)] use store::{pack::snapshot::ArtifactDsl};





use crate::editor::rewriting::document::*;
use crate::editor::rewriting::ids::*;
use crate::editor::rewriting::ui::*;

//#region 🔖️Io
/// 🔌️ Rewriting's typed media I/O surface (`AppDefinition.io`) — the implicit document in/out pair (a
/// `trinity.rewrite.rule` document) plus a graph in/out pair: `graph:in` loads an incoming
/// `trinity.graph` as this rule's `working_graph` working graph, and `graph:out` re-emits the
/// rule-applied result graph.
pub fn rewriting_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: REWRITE_RULE_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "graph:in".into(),
                label: "Graph".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                kind_id: Some("graph.trinity".into()),
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::One,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "graph:out".into(),
                label: "Graph".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                kind_id: Some("graph.trinity".into()),
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: crate::standards::v1::subsets::any::schema::model::artifact_kind().id, name: "Trinity Rewrite Rule".into(), dimension: "graph".into(), component_kind: "trinity".into() },
    }
}
//#endregion 🔖️Io



//#region 🔖️TrinityRewritingCommand
/// 🎯️ `TrinityRewritingPlayApp::Command` — the SOLE dispatch surface for rewriting's own behavior. Kept
/// hand-rolled (see `jack::TrinityJackCommand`'s doc comment for the rationale). `NodeGraphEdit` keeps
/// its JSON-array `operations` shape (rather than a typed sub-enum) — the same
/// node-graph record rows (`connect`/`disconnect`/`move`/`setSlider`/`insertPort`/`delete`) `commands::node_graph_edit::node_graph_edit`
/// already parses, carried as an opaque string field.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
pub enum TrinityRewritingCommand {
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit { surface_id: String, operations_json: String },
    #[dsl(key = "set-lhs-json")]
    SetLhsJson { value: String },
    #[dsl(key = "set-rhs-json")]
    SetRhsJson { value: String },
    #[dsl(key = "set-parameter")]
    SetParameter { name: String, value: String },
    #[dsl(key = "add-rule-clause")]
    AddRuleClause { kind: String },
    #[dsl(key = "reset-rule")]
    ResetRule,
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "patch-nodes")]
    PatchNodes { node_ids: Vec<String>, field: String, value: String },

    #[dsl(key = "node-graph-viewport")]
    SetViewport {
        surface_id: Option<String>,
        #[dsl(block)]
        viewport: Viewport2d,
    },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "set-lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "add-working-node")]
    AddWorkingNode { kind: Option<String>, name: Option<String>, x: f64, y: f64 },
}

//#region 🔖️OpCodec
impl protocol::OpText for TrinityRewritingCommand {
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
impl protocol::OpBinary for TrinityRewritingCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &["nodeGraphViewport", "setLodMode", "addRuleClause", "resetRule", "setActiveExample", "setParameter", "patchNodes", "nodeGraphEdit", "setLhsJson", "setRhsJson", "reorganize", "addWorkingNode"];

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

//#endregion 🔖️TrinityRewritingCommand

//#region 🔖️ActionBridge

/// 🎯️ Folds the host's `{action, args}` vocabulary (camelCase keys, JSON floats, control `value`s, the
/// node-graph host's `operations` array) into `TrinityRewritingCommand` — the trait default refuses every
/// app action, which left every panel and window verb dead in the shell (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END).
#[path = "🌉️args-bridge/🦀️.rs"]
pub mod args_bridge;

//#endregion 🔖️ActionBridge

//#region 🧵️RetainedDocumentCommands

#[path = "🧵️retained-document-commands/🦀️.rs"]
pub mod retained_document_commands;
use retained_document_commands::*;

//#endregion 🧵️RetainedDocumentCommands

//#region 🔖️TrinityRewritingPlayApp

#[path = "🕹️play-app/🦀️.rs"]
mod play_app;
pub use play_app::*;

//#endregion 🔖️TrinityRewritingPlayApp

//#region 🔖️Manifest
use crate::editor::rewriting::modes::edit;

#[path = "📋️manifest/🦀️.rs"]
pub mod manifest;
pub use manifest::*;

//#endregion 🔖️Manifest

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🪢️TaxonomyMounts
pub use semio_s_artifact_trinity_rewriting_operations::editor::rewriting::delete_rule_clause;
//#endregion 🪢️TaxonomyMounts

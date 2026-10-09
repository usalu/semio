//! ♻️ Trinity Rewriting editor — parametric rewriting editor bundled as a hot-swappable WASM plugin.
//!
//! 📌️ Pure-trait `ArtifactEditor`: `TrinityRewritingPlayApp` is a unit struct; every former
//! `RewritingPlayRuntime` field (selection, hover/select var, camera, LOD, …) lives in
//! concrete-window configuration; the app itself has no config record. Every rule/parameter/
//! working-graph edit flows through the semantic `RewriteRuleMutation` vocabulary (`edit-*` body
//! replaces, `change-*`/`remove-*` map upserts): every command builds exactly the leaves its intent means, never a diff of a
//! scratch copy of the whole rule. The `TrinityRewritingCommand` enum stays hand-rolled (TEMPLATE §5.1 fallback, same rationale
//! as `jack`).

use semio_s_artifact_trinity_jack::JackWorkingScene;
use crate::editor::rewriting::window_config;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::{self, ParameterKind, Rhs};
use crate::{LayoutPoint, RewritingSnapshot, REWRITE_RULE_SCHEMA, TRINITY_REWRITING_DIALECT};
use semio_framework_graph::manifest::{PropertyBag,PropertyValue};
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
use semio_s_artifact_trinity_jack::{Camera, JackSnapshot, Node};
// 🩹️ `InteractionView` is not re-exported at `semio_framework_plugin`'s crate root (unlike
// `ConfigView`/`ArtifactView`/`DraftView`) — only reachable through its owning `app` submodule
// (itself `pub mod`). Flagged as a likely framework oversight, not fixed here (framework file).
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
use std::collections::{BTreeMap, HashMap};
use semio_framework_2d::compute::EngineHandles;
use store::{ArtifactDsl, ArtifactPack};

//#region 🔖️Constants
pub(crate) const TRINITY_REWRITING_PLAY_CONTROLLER_ID: &str = "trinity-rewriting-play";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_BEFORE: &str = "trinity.rewriting.before";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_AFTER: &str = "trinity.rewriting.after";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_LHS: &str = "trinity.rewriting.lhs";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_RHS: &str = "trinity.rewriting.rhs";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_JACK: &str = "trinity.rewriting.jack";
pub(crate) const TRINITY_REWRITING_PLAY_BODY_BEFORE: &str = "trinity.rewriting.play.before";
const TRINITY_REWRITING_PLAY_BODY_AFTER: &str = "trinity.rewriting.play.after";
const TRINITY_REWRITING_PLAY_BODY_LHS: &str = "trinity.rewriting.play.lhs";
const TRINITY_REWRITING_PLAY_BODY_RHS: &str = "trinity.rewriting.play.rhs";
const TRINITY_REWRITING_PLAY_BODY_JACK: &str = "trinity.rewriting.play.jack";
const TRINITY_REWRITING_PLAY_BODY_PARAMETERS: &str = "trinity.rewriting.play.parameters";
pub(crate) const TRINITY_REWRITING_PLAY_BODY_ARTIFACT: &str = "trinity.rewriting.play.artifact";
const TRINITY_REWRITING_PLAY_BODY_CATALOGUE: &str = "trinity.rewriting.play.catalogue";
const TRINITY_REWRITING_PLAY_BODY_INSPECTION: &str = "trinity.rewriting.play.inspection";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_BEFORE: &str = "trinity-rewriting-before";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_AFTER: &str = "trinity-rewriting-after";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_LHS: &str = "trinity-rewriting-lhs";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_RHS: &str = "trinity-rewriting-rhs";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_JACK: &str = "trinity-rewriting-jack";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_PARAMETERS: &str = "trinity-rewriting-parameters";
const TRINITY_REWRITING_PLAY_RULE_NAME: &str = "label-core";

#[cfg(test)]
const NAKAGIN_CHILD: &str = include_str!("../🧫️fixtures/🪆️child/🏢️initial/🪆️content/🔣️.json");

const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 🪆️ The authored Nakagin asset materializes its actual typed child.
#[cfg(test)]
fn nakagin_fixture() -> JackSnapshot {
    let child=semio_s_artifact_stdio_semio::standards::v1::subsets::graph::io::text::snapshot::decode_semio_graph_snapshot_json(NAKAGIN_CHILD).expect("declared complete Nakagin Semio child");
    let content=store::ArtifactChild::new("nakagin-jack-demo-content".into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:"nakagin-jack-demo-content".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.stdio.semio".into(),standard:"v1".into(),subset:"graph".into()}});
    let mut graph=JackSnapshot{schema:JackSnapshot::SCHEMA.into(),name:"Nakagin Capsule Tower".into(),manifest_id:Some("nakagin".into()),manifest:semio_s_artifact_trinity_jack::Manifest::nakagin_default(),camera:semio_s_artifact_trinity_jack::Camera::default(),content,root_node_id:Some("7dc5b737-3b6b-4068-b315-b7bacc91c2e1".into()),query:semio_s_artifact_trinity_jack::TRINITY_JACK_DEFAULT_QUERY.into()};
    semio_s_artifact_trinity_jack::materialize_jack_snapshot(&mut graph.content,child);
    graph
}
pub(crate) fn default_parameter_bindings(rhs: &Rhs) -> PropertyBag {
    rhs.parameters.iter().map(|param| (param.name.clone(), param.default.clone())).collect()
}
/// 🔧️ The parameter-binding leaves that carry `before` to `after`: one `change-parameter-binding` per key whose value differs or
/// is new, one `remove-parameter-binding` per key `after` drops, in key order.
pub(crate) fn parameter_binding_mutations(before: &PropertyBag, after: &PropertyBag) -> Vec<RewriteRuleMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::{change_parameter_binding,remove_parameter_binding};

    let changed = after.iter().filter(|(key, value)| before.get(*key) != Some(*value)).map(|(key, value)| change_parameter_binding(key.clone(), value.clone()));
    let removed = before.keys().filter(|key| !after.contains_key(*key)).map(|key| remove_parameter_binding(key.clone()));
    changed.chain(removed).collect()
}

#[cfg(test)]
pub(crate) fn fixture_rule_state() -> RewritingSnapshot {
    let lhs = schema::Lhs { pattern: schema::Pattern { left_var: "a".into(), left_kind: "Piece".into(), edge_var: Some("r".into()), edge_kind: Some("Connection".into()), right_var: Some("b".into()), right_kind: Some("Piece".into()) }, where_clause: Some("a.name = 'b'".into()) };
    let rhs = Rhs { set: vec![schema::Assignment { var: "a".into(), prop: "label".into(), value: PropertyValue::String("$label".into()) }], parameters: vec![schema::ParameterSpec { name: "label".into(), kind: ParameterKind::String, default: PropertyValue::String("nakagin-core".into()) }], ..Rhs::default() };
    let parameter_bindings = default_parameter_bindings(&rhs);
    RewritingSnapshot { working_graph: nakagin_fixture(), lhs, rhs, parameter_bindings, rule_layout: schema::RuleLayout::new() }
}
/// 🆕️ A new editor begins with the canonical empty rule and graph.
pub(crate) fn default_rule_state() -> RewritingSnapshot {
    RewritingSnapshot::default()
}
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetState` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `resetRule` builds a `Effect::LoadDocument`
/// (outside undo history) instead of an `artifact_mutations` entry.
pub(crate) fn reset_document_effect(state: &RewritingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <RewritingSnapshot as ArtifactPack>::encode_pack(state);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("rewriting", REWRITE_RULE_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

pub(crate) fn rewriting_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(TRINITY_REWRITING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Binds window chrome through its retained renderer action descriptor.
pub(crate) fn rewriting_window_action(action: &str, args: Option<semio_framework_pack_json::Value>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: TRINITY_REWRITING_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| semio_framework_pack_json::to_dsl_value(&value)) }
}

/// 🏷️ Admits resolved Rewriting text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "Rewriting UI label admission failed"))
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

fn build_rule_from_state(state: &RewritingSnapshot) -> schema::Rule {
    schema::Rule { name: TRINITY_REWRITING_PLAY_RULE_NAME.into(), lhs: state.lhs.clone(), rhs: state.rhs.clone() }
}
pub(crate) fn compiled_jack_query(state: &RewritingSnapshot) -> String {
    schema::build_rule_query(&build_rule_from_state(state), &state.parameter_bindings)
}
/// ♻️ The checked query projection produces a new typed graph without changing the retained source child.
pub(crate) fn rewritten_graph(state: &RewritingSnapshot) -> Result<JackSnapshot, String> {
    let mut graph = semio_s_artifact_trinity_jack::Graph::from_snapshot(state.working_graph.clone()).map_err(|error|error.to_string())?;
    schema::apply_rule(&mut graph, &build_rule_from_state(state), &state.parameter_bindings).map_err(|error|error.to_string())?;
    Ok(graph.to_snapshot())
}
/// 🧾️ The working graph's resolved manifest (embedded, else named by `manifestId`); `None` when it resolves to none.
pub(crate) fn resolved_working_manifest(state: &RewritingSnapshot) -> Option<semio_s_artifact_trinity_jack::Manifest> {
    let mut graph = state.working_graph.clone();
    graph.resolve_manifest().ok().map(|()| graph.manifest.clone())
}
/// 🧩️ One semantic rule-graph node at its `rule_layout` point, else at its default slot ([`schema::lhs_graph_slots`],
/// [`schema::rhs_graph_slots`] — the same positions a node drag moves from).
fn semantic_rule_node(id: &str, kind: &str, name: &str, slots: &[(String, LayoutPoint)], rule_layout: &schema::RuleLayout) -> Node {
    let default = slots.iter().find(|(slot, _)| slot == id).map_or(LayoutPoint { x: 0.0, y: 0.0 }, |(_, point)| *point);
    let point = rule_layout.get(id).copied().unwrap_or(default);
    Node { id: id.into(), name: name.into(), kind: kind.into(), x: point.x, y: point.y, width: 160.0, height: 56.0, ports: vec![], properties: Default::default() }
}

fn lhs_semantic_graph_snapshot(lhs: &schema::Lhs, rule_layout: &schema::RuleLayout) -> JackSnapshot {
    let slots = schema::lhs_graph_slots(lhs);
    let mut nodes = vec![semantic_rule_node("lhs-match", "rewriting.match", &format!("{}:{}", lhs.pattern.left_var, lhs.pattern.left_kind), &slots, rule_layout)];
    let mut edges = Vec::new();
    if let Some(where_clause) = lhs.where_clause.as_deref().filter(|value| !value.trim().is_empty()) {
        nodes.push(semantic_rule_node("lhs-where", "rewriting.where", where_clause, &slots, rule_layout));
        edges.push(semio_s_artifact_trinity_jack::Edge { id: "lhs-match-where".into(), kind: "rewriting.flow".into(), source: "lhs-match@out".into(), target: "lhs-where@in".into(), properties: Default::default() });
    }
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "lhs".into(), Some("nakagin".into()), semio_s_artifact_trinity_jack::Manifest::nakagin_default(), Camera { x: 0.0, y: 0.0, zoom: 1.0 }, JackWorkingScene { nodes: nodes, edges: edges }, None)
}

fn rhs_semantic_graph_snapshot(rhs: &Rhs, rule_layout: &schema::RuleLayout) -> JackSnapshot {
    let slots = schema::rhs_graph_slots(rhs);
    let node = |id: String, kind: &str, name: String| semantic_rule_node(&id, kind, &name, &slots, rule_layout);
    let mut nodes = Vec::new();
    nodes.extend(rhs.create.iter().enumerate().map(|(index, pattern)| node(format!("rhs-create-{index}"), "rewriting.create", format!("{}:{}", pattern.left_var, pattern.left_kind))));
    nodes.extend(rhs.merge.iter().enumerate().map(|(index, pattern)| node(format!("rhs-merge-{index}"), "rewriting.merge", format!("{}:{}", pattern.left_var, pattern.left_kind))));
    nodes.extend(rhs.set.iter().enumerate().map(|(index, assignment)| node(format!("rhs-set-{index}"), "rewriting.set", format!("{}.{} = {:?}", assignment.var, assignment.prop, assignment.value))));
    nodes.extend(rhs.delete.iter().enumerate().map(|(index, name)| node(format!("rhs-delete-{index}"), "rewriting.delete", name.clone())));
    nodes.extend(rhs.parameters.iter().enumerate().map(|(index, parameter)| {
        let kind = match parameter.kind {
            ParameterKind::String => "string",
            ParameterKind::Number => "number",
            ParameterKind::Boolean => "boolean",
        };
        node(format!("rhs-parameter-{index}"), "rewriting.parameter", format!("{}:{kind}", parameter.name))
    }));
    if nodes.is_empty() {
        nodes.push(node("rhs-empty".into(), "rewriting.create", "result:Piece".into()));
    }
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "rhs".into(), Some("nakagin".into()), semio_s_artifact_trinity_jack::Manifest::nakagin_default(), Camera { x: 0.0, y: 0.0, zoom: 1.0 }, JackWorkingScene { nodes: nodes, edges: Vec::new() }, None)
}

pub(crate) fn lhs_graph_snapshot(lhs: &schema::Lhs, rule_layout: &schema::RuleLayout) -> JackSnapshot {
    lhs_semantic_graph_snapshot(lhs, rule_layout)
}
pub(crate) fn rhs_graph_snapshot(rhs: &Rhs, rule_layout: &schema::RuleLayout) -> JackSnapshot {
    rhs_semantic_graph_snapshot(rhs, rule_layout)
}
/// 🕹️ Used by `interaction_topology` to hang a var-reference `TopologyNode` off its graph node
/// (domain "graph" — "AST parents + variable references").
fn var_from_node_name(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if let Some((var, _)) = trimmed.split_once(':') {
        return Some(var.trim().into());
    }
    if let Some((var, _)) = trimmed.split_once(" : ") {
        return Some(var.trim().into());
    }
    None
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Io
/// 🔌️ Rewriting's typed media I/O surface (`AppDefinition.io`) — the implicit document in/out pair (a
/// `trinity.rewrite.rule` document) plus a graph in/out pair: `graph:in` loads an incoming
/// `trinity.graph` as this rule's `working_graph` working graph, and `graph:out` re-emits the
/// rule-applied result graph.
pub(crate) fn rewriting_io() -> semio_framework_plugin::AppIo {
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
        artifact: semio_framework_plugin::ArtifactPresentation { id: crate::artifact_kind().id, name: "Trinity Rewrite Rule".into(), dimension: "graph".into(), component_kind: "trinity".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Render
fn rewriting_lod_json_for_window(cfg: &window_config::RewritingWindowConfig) -> String {
    let mode = cfg.lod_mode.as_str();
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        semio_framework_pack_json::json!({ "automatic": true }).to_string()
    } else {
        semio_framework_pack_json::json!({ "automatic": false, "forcedLabel": mode }).to_string()
    }
}

fn trinity_rewriting_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![semio_framework_plugin::MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    let rows: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::parse(&semio_s_artifact_trinity_jack::editor::jack::lod::trinity_lod_scale_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).ok().and_then(|value| value.as_array().map(|values| values.to_vec())).unwrap_or_default();
    items.extend(rows.into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(semio_framework_plugin::MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: rewriting_window_action("setLodMode", Some(semio_framework_pack_json::json!({ "windowId": window_id }))) }
}

/// 🕹️ `selection`/`hover` are left unset: `ArtifactApp::render` has no `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` gained one — see `📌️panels/🔍️inspection`'s doc comment
/// on `editor::jack` for the same framework-side gap) and this static scene isn't a `UiNode::Tree` the
/// wrapper's `stamp_and_cache_interaction_ui` post-pass would stamp either. The live node-graph host
/// reads domain "graph"'s `DomainSelection`/`DomainHover` directly, so the interactive surface stays
/// correct even though this snapshot doesn't carry it.
pub(crate) fn render_graph_snapshot(surface_id: &str, snapshot: &JackSnapshot, cfg: &window_config::RewritingWindowConfig, editable: bool) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let (nodes, edges, scene_viewport) = semio_s_artifact_trinity_jack::snapshot_to_workflow(snapshot).map_err(|error|semio_framework_plugin::PluginAssemblyError::new("trinity.child.unavailable",error.into_message()))?;
    let viewport = cfg.camera.as_ref().map_or(scene_viewport, |camera| Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom });
    semio_framework_plugin::scene_surface(
        surface_id,
        SemanticSurfaceKind::NodeGraph,
        &semio_framework_plugin::NodeGraphScene { lod_json: Some(rewriting_lod_json_for_window(cfg)), editable: editable.then_some(true), ..semio_framework_plugin::NodeGraphScene::base(nodes, edges, viewport) },
    )
}
//#endregion 🔖️Render

//#region 🔖️TrinityRewritingCommand
/// 🎯️ `TrinityRewritingPlayApp::Command` — the SOLE dispatch surface for rewriting's own behavior. Kept
/// hand-rolled (see `jack::TrinityJackCommand`'s doc comment for the rationale). `NodeGraphEdit` keeps
/// its JSON-array `operations` shape (rather than a typed sub-enum) — the same
/// node-graph record rows (`connect`/`disconnect`/`move`/`setSlider`/`insertPort`/`delete`) `commands::node_graph_edit`
/// already parses, carried as an opaque string field.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum TrinityRewritingCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
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

    // 👁️ Config-only — was ephemeral `RewritingPlayRuntime` state, now emits `config_mutations`.
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
mod args_bridge {
    use super::TrinityRewritingCommand;
    use semio_framework_os_kernel::Viewport2d;
    use semio_framework_plugin::{DslValue, Fault, FaultCode, FaultOrigin};

    fn invalid(action: &str, detail: impl std::fmt::Display) -> Fault {
        Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("rewriting action '{action}' arguments do not decode: {detail}"))
    }

    fn field<'a>(args: Option<&'a DslValue>, keys: &[&str]) -> Option<&'a DslValue> {
        keys.iter().find_map(|key| args.and_then(|args| args.get(key))).filter(|value| !matches!(value, DslValue::Null))
    }

    /// 📝️ Prints a host control value into the `String` field a verb carries (`"512"`, `"true"`, or the text itself).
    fn text(args: Option<&DslValue>, keys: &[&str]) -> Option<String> {
        field(args, keys).map(|value| match value {
            DslValue::String(text) => text.clone(),
            DslValue::Number(number) => {
                let float = number.as_f64();
                if float.is_finite() && float.fract() == 0.0 { format!("{}", float as i64) } else { format!("{float}") }
            }
            DslValue::Bool(flag) => flag.to_string(),
            other => semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(other)),
        })
    }

    fn required(action: &str, args: Option<&DslValue>, keys: &[&str]) -> Result<String, Fault> {
        text(args, keys).ok_or_else(|| invalid(action, format!("missing {}", keys[0])))
    }

    /// 🔡️ `nodeIds` arrives as a list, a JSON-array string, or the rail's text field listing ids
    /// separated by commas or whitespace. An empty field means "the current node selection".
    fn ids(args: Option<&DslValue>) -> Vec<String> {
        match field(args, &["nodeIds", "node_ids", "ids"]) {
            Some(DslValue::Array(items)) => items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect(),
            Some(DslValue::String(text)) if text.trim_start().starts_with('[') => semio_framework_pack_json::from_json_str::<Vec<String>>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default(),
            Some(DslValue::String(text)) => text.split(|character: char| character == ',' || character.is_whitespace()).filter(|id| !id.is_empty()).map(str::to_string).collect(),
            _ => Vec::new(),
        }
    }

    /// 📷️ The node-graph host nests its pose under `viewport`; a flat `{x, y, zoom}` payload is the pose itself.
    fn viewport(action: &str, args: Option<&DslValue>) -> Result<Viewport2d, Fault> {
        let coordinate = |value: Option<&DslValue>, fallback: f64| value.and_then(DslValue::as_f64).unwrap_or(fallback);
        let pose = field(args, &["viewport"]).or(args).ok_or_else(|| invalid(action, "missing viewport"))?;
        let viewport = Viewport2d { x: coordinate(pose.get("x"), 0.0), y: coordinate(pose.get("y"), 0.0), zoom: coordinate(pose.get("zoom"), 1.0) };
        viewport.validate().map_err(|error| invalid(action, format!("{error:?}")))?;
        Ok(viewport)
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<TrinityRewritingCommand, Fault> {
        const SURFACE: &[&str] = &["surfaceId", "surface_id"];
        Ok(match action {
            "nodeGraphEdit" => TrinityRewritingCommand::NodeGraphEdit { surface_id: text(args, SURFACE).unwrap_or_default(), operations_json: required(action, args, &["operationsJson", "operations_json", "operations"])? },
            "setLhsJson" => TrinityRewritingCommand::SetLhsJson { value: required(action, args, &["value", "json"])? },
            "setRhsJson" => TrinityRewritingCommand::SetRhsJson { value: required(action, args, &["value", "json"])? },
            "setParameter" => TrinityRewritingCommand::SetParameter { name: required(action, args, &["name"])?, value: text(args, &["value"]).unwrap_or_default() },
            "addRuleClause" => TrinityRewritingCommand::AddRuleClause { kind: required(action, args, &["kind", "value"])? },
            "resetRule" => TrinityRewritingCommand::ResetRule,
            "setActiveExample" => TrinityRewritingCommand::SetActiveExample { example_id: required(action, args, &["exampleId", "example_id", "value", "id"])? },
            "patchNodes" => TrinityRewritingCommand::PatchNodes { node_ids: ids(args), field: text(args, &["field"]).unwrap_or_else(|| "name".into()), value: required(action, args, &["value"])? },
            "nodeGraphViewport" => TrinityRewritingCommand::SetViewport { surface_id: text(args, SURFACE), viewport: viewport(action, args)? },
            "reorganize" => TrinityRewritingCommand::Reorganize,
            "setLodMode" => TrinityRewritingCommand::SetLodMode { value: required(action, args, &["value", "mode"])? },
            "addWorkingNode" => {
                let coordinate = |key: &str| field(args, &[key]).and_then(DslValue::as_f64).ok_or_else(|| invalid(action, format!("missing numeric {key}")));
                TrinityRewritingCommand::AddWorkingNode { kind: text(args, &["kind"]), name: text(args, &["name"]), x: coordinate("x")?, y: coordinate("y")? }
            }
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the trinity rewriting editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🧵️RetainedDocumentCommands
/// 🧾️ Document verbs as bounded first-step tools (ticket 26/09/17/TRINITY-PLUGIN-END-TO-END): the framework
/// refuses UI dispatch of every command not classified `Migrated`, which left every rule edit dead in the
/// shell. Each verb publishes granular `RewriteRuleMutation`s on the artifact lane — the working-graph verbs (`patchNodes`,
/// `addWorkingNode`, a working-canvas `nodeGraphEdit`) ONE edit of the composed `workingGraph` child (design §20.15) — except
/// `resetRule`, a host-applied `Effect::LoadDocument`.
const REWRITING_DOCUMENT_TOOL_IDS: &[&str] = &["addRuleClause", "resetRule", "setActiveExample", "setParameter", "patchNodes", "nodeGraphEdit", "setLhsJson", "setRhsJson", "reorganize", "addWorkingNode"];
const REWRITING_DOCUMENT_PAYLOAD_SCHEMA: &str = "trinity.rewriting.document-command.v1";
const REWRITING_DOCUMENT_RAW_BYTES: usize = 32_768;
/// 📬️ One retained rule mutation: `edit-working-graph` carries the whole working graph JSON (the Nakagin
/// snapshot is the largest), every other body replace or map upsert stays far below one page.
const REWRITING_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 60_000;

fn rewriting_document_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(REWRITING_DOCUMENT_RAW_BYTES, 64, 1, 262_144, 7_500)
}

fn rewriting_document_extent(command: &TrinityRewritingCommand, _snapshot: &RewritingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => surface_id.len().checked_add(operations_json.len())?,
        TrinityRewritingCommand::SetLhsJson { value } | TrinityRewritingCommand::SetRhsJson { value } => value.len(),
        TrinityRewritingCommand::SetParameter { name, value } => name.len().checked_add(value.len())?,
        TrinityRewritingCommand::AddRuleClause { kind } => kind.len(),
        TrinityRewritingCommand::SetActiveExample { example_id } => example_id.len(),
        TrinityRewritingCommand::PatchNodes { node_ids, field, value } => node_ids.iter().map(String::len).try_fold(field.len().checked_add(value.len())?, usize::checked_add)?,
        TrinityRewritingCommand::AddWorkingNode { kind, name, .. } => kind.as_ref().map_or(0, String::len).checked_add(name.as_ref().map_or(0, String::len))?.checked_add(16)?,
        TrinityRewritingCommand::ResetRule | TrinityRewritingCommand::Reorganize => 1,
        _ => return None,
    };
    (bytes <= REWRITING_DOCUMENT_RAW_BYTES).then_some(1)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn rewriting_document_reduce(
    command: &TrinityRewritingCommand,
    state: &RewritingSnapshot,
    _config: &NoConfig,
    _history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TrinityRewritingPlayApp>>>,
    operation: &semio_framework_plugin::AppOperationContext,
) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    use crate::editor::rewriting::commands;
    let children = || context.map(|context| context.children.as_ref()).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.rewriting.child-refused"), "the retained document command carries no child view"));
    Ok(match command {
        TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => commands::node_graph_edit(state, children()?, surface_id, operations_json, &operation.authoring_seed)?,
        TrinityRewritingCommand::SetLhsJson { value } => commands::set_lhs(state, value)?,
        TrinityRewritingCommand::SetRhsJson { value } => commands::set_rhs(state, value)?,
        TrinityRewritingCommand::SetParameter { name, value } => commands::set_parameter(state, name, value),
        TrinityRewritingCommand::AddRuleClause { kind } => commands::add_rule_clause_command(state, kind)?,
        TrinityRewritingCommand::ResetRule => commands::reset_rule(state),
        TrinityRewritingCommand::SetActiveExample { example_id } => commands::set_active_example(example_id),
        TrinityRewritingCommand::PatchNodes { node_ids, field, value } => commands::patch_nodes(state, children()?, node_ids, interaction.selection.get("graph").map_or(&[][..], |selection| selection.ids.as_slice()), field, value)?,
        TrinityRewritingCommand::Reorganize => commands::reorganize(state),
        TrinityRewritingCommand::AddWorkingNode { kind, name, x, y } => commands::add_working_node_command(state, children()?, kind.as_deref(), name.as_deref(), *x, *y)?,
        _ => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "the rewriting document reducer received a command outside its tool roster")),
    })
}

struct RewritingDocumentJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl RewritingDocumentJobFactory {
    fn new(controller: &str) -> Self {
        Self { keys: REWRITING_DOCUMENT_TOOL_IDS.iter().map(|tool| semio_framework::ToolFactoryKey::new(controller, *tool)).collect() }
    }
}

impl semio_framework::ToolJobFactory for RewritingDocumentJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<TrinityRewritingPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<TrinityRewritingPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        REWRITING_DOCUMENT_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        rewriting_document_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > REWRITING_DOCUMENT_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("Rewriting document command rejects oversized wire or a checkpoint"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for RewritingDocumentJobFactory {
    type Owner = EditorApp<TrinityRewritingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = REWRITING_DOCUMENT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addRuleClause", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "resetRule", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setParameter", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchNodes", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLhsJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setRhsJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWorkingNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    ];
}

fn rewriting_build_document_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<TrinityRewritingPlayApp>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandInputs, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
    let tool_id = TrinityRewritingPlayApp::command_id(&request.command);
    if tool_id != request.tool_id {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Rewriting command does not match its exact registered tool"));
    }
    if rewriting_document_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("trinity.rewriting.retained-capacity"), "the rewriting command exceeds the capacity of one bounded edit"));
    }
    let work: Box<dyn ArtifactCommandWork<EditorApp<TrinityRewritingPlayApp>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, rewriting_document_reduce, rewriting_document_extent));
    let operation = semio_framework_plugin::AppOperationContext {
        app_instance_id: request.app_instance_id,
        parent_document_id: request.parent_document_id.clone(),
        operation_id: request.operation.operation.0,
        generation: request.operation.generation.0,
        canonical_base_revision: request.canonical_base_revision,
        authoring_seed: request.authoring_seed.clone(),
    };
    let payload = ArtifactRetainedCommandPayload::new(
        ArtifactRetainedCommandInputs {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
            history: request.history,
            interaction_state: request.interaction_state,
            interaction_hover: request.interaction_hover,
            context: Some(request.context),
            operation,
            completion: request.completion,
        },
        TrinityRewritingPlayApp::command_id,
        REWRITING_DOCUMENT_RAW_BYTES,
        1,
        work,
    );
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}
//#endregion 🧵️RetainedDocumentCommands

//#region 🔖️TrinityRewritingPlayApp
/// ♻️ Trinity Rewriting play app — a parametric-rewriting editor over a {@link RewritingSnapshot} projection.
#[derive(Default)]
pub struct TrinityRewritingPlayApp;

impl ArtifactEditor for TrinityRewritingPlayApp {
    /// 🧩️ The roster the composed `s.stdio.semio` `workingGraph` child opens through (design §20.15).
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = RewritingSnapshot;
    type Mutation = RewriteRuleMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = TrinityRewritingCommand;

    const DIALECT: Dialect = TRINITY_REWRITING_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::rewriting_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
        crate::rewriting_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(schema::retirement::document_store_owners())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }
    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }
    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }
    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let documents = REWRITING_DOCUMENT_TOOL_IDS.iter().map(|tool| {
            semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(
                "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
                "s.trinity.rewriting@1/*#editor",
                "RewritingDocumentJobFactory",
                tool,
                REWRITE_RULE_SCHEMA,
                semio_framework::ToolExecutionContract::bounded_first_step(32_768, 64, 1, 262_144, 7_500),
            )
            .with_factory_type::<EditorApp<Self>, RewritingDocumentJobFactory>()
        });
        window_config::job::TOOL_IDS
            .iter()
            .map(|tool| {
                semio_framework_plugin::ArtifactBoundedFirstStepProof::new::<EditorApp<Self>>(
                    "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧵️job/🦀️.rs",
                    "s.trinity.rewriting@1/*#editor",
                    "RewritingWindowConfigJobFactory",
                    tool,
                    REWRITE_RULE_SCHEMA,
                    window_config::job::contract(),
                )
                .with_factory_type::<EditorApp<Self>, window_config::job::RewritingWindowConfigJobFactory>()
            })
            .chain(documents)
            .collect()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(window_config::job::RewritingWindowConfigJobFactory::new(&controller))?;
        registry.register(RewritingDocumentJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::app::ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if REWRITING_DOCUMENT_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return rewriting_build_document_tool_job(request);
        }
        window_config::job::build_job(request)
    }

    /// 🧾️ Store publication authority for the `Artifact` lane — without it the host refuses every document
    /// verb at dispatch (`declares the unsupported artifact publication lane`).
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("trinity-rewriting-artifact-retained", REWRITING_ARTIFACT_MUTATION_MAXIMUM_BYTES))
    }

    /// 🎯️ Host-action bridge into the closed `TrinityRewritingCommand` enum — see `args_bridge`.
    fn command_from_action(action: &str, args: Option<&semio_framework_plugin::DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_config::register(registry)
    }

    fn initial_snapshot() -> RewritingSnapshot {
        default_rule_state()
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>, semio_framework_value::ValueError> {
        crate::content::genesis_working_child_pack(snapshot, slot, child_id)
    }

    /// 📢️ The localized notices of the editor's own refusal codes (design §20.12).
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
        crate::content::rewriting_fault_notices()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(rewriting_io())
    }

    // 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetState` — a
    // whole-snapshot LWW register wearing a mutation costume, see `📓️taxonomy.md`'s forbidden
    // vocabulary), so this intentionally falls back to the trait default (`None`) rather than
    // overriding — the `"artifact:in"` media port therefore reports `MediaError::NotImplemented`;
    // there is no import mutation (locked decision).

    /// 🔌️ `"graph:in"` loads an incoming `trinity.graph` pack as this rule's `working_graph`
    /// working graph — a single targeted field edit, not a whole-document replace.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RewritingSnapshot>) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "graph:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "graph:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let _ = doc;
                Ok(Emit::mutations(vec![schema::mutations::edit_working_graph(snapshot)]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🔌️ `"graph:out"` re-emits the rule-applied result graph, alongside the implicit `"artifact:out"`.
    fn export_media(port: &str, doc: &ArtifactView<'_, RewritingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "graph:out" => {
                let state = crate::content::composed(doc.snapshot, &doc.children).map_err(|fault| MediaError::Payload(port.to_string(), fault.message))?;
                let snapshot = rewritten_graph(&state).map_err(|error| MediaError::Payload(port.to_string(), error))?;
                let bytes = ArtifactPack::encode_pack(&snapshot);
                Ok(Media {
                    media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                    payload: MediaPayload::Structured { schema: semio_s_artifact_trinity_jack::TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `TrinityRewritingCommand` variant back to the action id it was declared under in
    /// `create_rewriting_app`.
    fn command_id(command: &TrinityRewritingCommand) -> &'static str {
        match command {
            TrinityRewritingCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            TrinityRewritingCommand::SetLhsJson { .. } => "setLhsJson",
            TrinityRewritingCommand::SetRhsJson { .. } => "setRhsJson",
            TrinityRewritingCommand::SetParameter { .. } => "setParameter",
            TrinityRewritingCommand::AddRuleClause { .. } => "addRuleClause",
            TrinityRewritingCommand::ResetRule => "resetRule",
            TrinityRewritingCommand::SetActiveExample { .. } => "setActiveExample",
            TrinityRewritingCommand::PatchNodes { .. } => "patchNodes",
            TrinityRewritingCommand::SetViewport { .. } => "nodeGraphViewport",
            TrinityRewritingCommand::Reorganize => "reorganize",
            TrinityRewritingCommand::SetLodMode { .. } => "setLodMode",
            TrinityRewritingCommand::AddWorkingNode { .. } => "addWorkingNode",
        }
    }

    fn handle(
        command: &TrinityRewritingCommand,
        doc: &ArtifactView<'_, RewritingSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RewriteRuleMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let state = doc.snapshot;
        Ok(match command {
            TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => crate::editor::rewriting::commands::node_graph_edit(state, &doc.children, surface_id, operations_json, "")?,
            TrinityRewritingCommand::SetLhsJson { value } => crate::editor::rewriting::commands::set_lhs(state, value)?,
            TrinityRewritingCommand::SetRhsJson { value } => crate::editor::rewriting::commands::set_rhs(state, value)?,
            TrinityRewritingCommand::SetParameter { name, value } => crate::editor::rewriting::commands::set_parameter(state, name, value),
            TrinityRewritingCommand::AddRuleClause { kind } => crate::editor::rewriting::commands::add_rule_clause_command(state, kind)?,
            TrinityRewritingCommand::ResetRule => crate::editor::rewriting::commands::reset_rule(state),
            TrinityRewritingCommand::SetActiveExample { example_id } => crate::editor::rewriting::commands::set_active_example(example_id),
            TrinityRewritingCommand::PatchNodes { node_ids, field, value } => crate::editor::rewriting::commands::patch_nodes(state, &doc.children, node_ids, &interaction.selection("graph").ids, field, value)?,
            TrinityRewritingCommand::SetViewport { surface_id, viewport } => crate::editor::rewriting::commands::set_viewport(surface_id, viewport, view_state)?,
            TrinityRewritingCommand::Reorganize => crate::editor::rewriting::commands::reorganize(state),
            TrinityRewritingCommand::SetLodMode { value } => crate::editor::rewriting::commands::set_lod_mode(value, view_state)?,
            TrinityRewritingCommand::AddWorkingNode { kind, name, x, y } => crate::editor::rewriting::commands::add_working_node_command(state, &doc.children, kind.as_deref(), name.as_deref(), *x, *y)?,
        })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RewritingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let state = doc.snapshot;
        let config = cfg.snapshot;
        let window_config = window_config::current(cfg).cloned().unwrap_or_default();
        let labels = semio_framework_plugin::resolve_labels::<crate::editor::rewriting::terminology::TrinityRewritingLabels>(view_state);
        let composed = || crate::content::composed(state, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("trinity.rewriting.working-graph", fault.message));
        let root = match body_key {
            TRINITY_REWRITING_PLAY_BODY_BEFORE => edit::windows::before::render(&composed()?, &window_config),
            TRINITY_REWRITING_PLAY_BODY_AFTER => edit::windows::after::render(&composed()?, &window_config),
            TRINITY_REWRITING_PLAY_BODY_LHS => edit::windows::lhs::render(state, &window_config),
            TRINITY_REWRITING_PLAY_BODY_RHS => edit::windows::rhs::render(state, &window_config),
            TRINITY_REWRITING_PLAY_BODY_JACK => edit::windows::jack::render(state, config),
            TRINITY_REWRITING_PLAY_BODY_PARAMETERS => edit::windows::parameters::render(state, labels),
            TRINITY_REWRITING_PLAY_BODY_ARTIFACT => crate::editor::rewriting::panels::document::render(&composed()?, config, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, TRINITY_REWRITING_PLAY_BODY_ARTIFACT)),
            TRINITY_REWRITING_PLAY_BODY_CATALOGUE => crate::editor::rewriting::panels::catalogue::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, TRINITY_REWRITING_PLAY_BODY_CATALOGUE)),
            TRINITY_REWRITING_PLAY_BODY_INSPECTION => crate::editor::rewriting::panels::inspection::render(),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.body.label", "the fixed Trinity body label exceeds its UI bound")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(root))
    }

    fn window_measures(_doc: &ArtifactView<'_, RewritingSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let Some(config) = window_config::current(cfg) else { return HashMap::new() };
        HashMap::from([(window_id.to_string(), vec![trinity_rewriting_lod_measure(window_id, &config.lod_mode)])])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, RewritingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        // 🕹️ Selection is framework-owned now (domain "graph") — `context_menu` has no `InteractionView`,
        // so the request's own surface-carried selection groups are the only source; no config fallback.
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &[], &[]);

        let menu = Menu::of(registry, view_state)
            .action("addRuleClause")
            .action("setParameter")
            .action("reorganize")
            .group("create", |m| m.action("addWorkingNode"))
            .group("transform", |m| m.action("patchNodes"))
            .group("history", |m| m.action("resetRule"))
            .group("mode", |m| m.action("setLodMode").action("setActiveExample"))
            .group("tools", |m| m.action("setLhsJson").action("setRhsJson"));
        menu.item(node_graph_delete_selection_spec(semio_framework_plugin::delete_selection().resolve(view_state.terminology, view_state.locale), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()
    }

    /// 🕹️ Domain "graph" topology: unions three node universes under one "node" granularity —
    /// (1) the Before snapshot's own nodes, parented by the source node of their first incoming
    /// connection, each with a variable-reference child when its name resolves one (`var_from_node_name`
    /// — "AST parents + variable references"); (2) the LHS semantic graph (`lhs-where` parented by
    /// `lhs-match` via their one edge); (3) the RHS semantic graph (its clause nodes have no inherent
    /// parent order, so they're roots). `MergeMode::Range` is not declared for this domain, so
    /// `ordered`'s sequence need not be a strict pre-order.
    fn interaction_topology(doc: &ArtifactView<'_, RewritingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 let composed=crate::content::composed(doc.snapshot,&doc.children).map_err(|fault|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,fault.message))?;let state=&composed;
 semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::inferences::topology::compute_topology(&state.working_graph)?;
 let owner=semio_s_artifact_trinity_jack::jack_content_for_handle(&state.working_graph.content)?;let raw=owner.snapshot();let mut ordered=Vec::new();let mut parent_of=BTreeMap::new();
 for edge in &raw.edges{parent_of.entry(edge.target.value.clone()).or_insert_with(||edge.source.value.clone());}
 for node in &raw.nodes{ordered.push(TopologyNode{id:node.id.value.clone(),granularity:"node".into(),parent:parent_of.get(&node.id.value).cloned()});if let Some(var)=var_from_node_name(&node.label){ordered.push(TopologyNode{id:var,granularity:"node".into(),parent:Some(node.id.value.clone())});}}
 let lhs=lhs_semantic_graph_snapshot(&state.lhs,&state.rule_layout);let lhs_owner=semio_s_artifact_trinity_jack::jack_content_for_handle(&lhs.content)?;let lhs_raw=lhs_owner.snapshot();let mut parent_of=BTreeMap::new();for edge in &lhs_raw.edges{parent_of.entry(edge.target.value.clone()).or_insert_with(||edge.source.value.clone());}
 for node in &lhs_raw.nodes{ordered.push(TopologyNode{id:node.id.value.clone(),granularity:"node".into(),parent:parent_of.get(&node.id.value).cloned()});}
 let rhs=rhs_semantic_graph_snapshot(&state.rhs,&state.rule_layout);let rhs_owner=semio_s_artifact_trinity_jack::jack_content_for_handle(&rhs.content)?;for node in &rhs_owner.snapshot().nodes{ordered.push(TopologyNode{id:node.id.value.clone(),granularity:"node".into(),parent:None});}
 let mut domains=BTreeMap::new();domains.insert("graph".into(),DomainTopology{ordered});Ok(InteractionTopology{domains})
 }
}
//#endregion 🔖️TrinityRewritingPlayApp

//#region 🔖️Manifest
use crate::editor::rewriting::modes::edit;

/// 🎯️ `create_rewriting_app` → `Editor::builder(TRINITY_REWRITING_DIALECT)…build_definition()` (contract
/// §2.4). The old `.example("label-core", …)`/`.workflow("trinity-rewriting", …)` calls are DROPPED,
/// not ported — same SDK gap `jack`'s `create_trinity_jack_app` doc comment records.
///
/// 🕸️ `nodeGraphEdit` is the node-graph host's gesture verb: every canvas edit carries the edited
/// graph's `surfaceId` and an `operations` list, neither of which a rail, palette or context-menu press
/// can supply (it was refused `missing operationsJson`), so it stays out of all three. `patchNodes`'
/// `nodeIds` is optional: left empty, the verb patches the selected nodes.
pub fn create_rewriting_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TRINITY_REWRITING_DIALECT).document(["semio", "trinity", "rewriting"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("trinity-rewriting")
            .mode_def(edit::definition())
            .default_mode_id(edit::TRINITY_REWRITING_MODE_EDIT)
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_BEFORE, LocalizedLabel::native("Before", "Vorher"), TRINITY_REWRITING_PLAY_BODY_BEFORE, SemanticSurfaceKind::NodeGraph, "git-branch")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_AFTER, LocalizedLabel::native("After", "Nachher"), TRINITY_REWRITING_PLAY_BODY_AFTER, SemanticSurfaceKind::NodeGraph, "arrow-right")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_LHS, LocalizedLabel::native("LHS", "LHS"), TRINITY_REWRITING_PLAY_BODY_LHS, SemanticSurfaceKind::NodeGraph, "trinity-lhs")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_RHS, LocalizedLabel::native("RHS", "RHS"), TRINITY_REWRITING_PLAY_BODY_RHS, SemanticSurfaceKind::NodeGraph, "trinity-rhs")
            .window_kind(TRINITY_REWRITING_PLAY_WINDOW_JACK, LocalizedLabel::native("Jack", "Jack"), TRINITY_REWRITING_PLAY_BODY_JACK, SemanticSurfaceKind::TextEditor, "document-jack")
            .window_kind(
                TRINITY_REWRITING_PLAY_WINDOW_PARAMETERS,
                LocalizedLabel::native("Parameters", "Parameter"),
                TRINITY_REWRITING_PLAY_BODY_PARAMETERS,
                SemanticSurfaceKind::Canvas2d,
                "settings-2",
            )
            .default_layout(edit::layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
                PanelGroup::Workbench,
                TRINITY_REWRITING_PLAY_BODY_ARTIFACT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                TRINITY_REWRITING_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                TRINITY_REWRITING_PLAY_BODY_INSPECTION,
            )
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("addRuleClause", LocalizedLabel::native("Add Rule Clause", "Regelklausel hinzufügen"), ActionKind::Mutation).with_category("create"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("resetRule", LocalizedLabel::native("Reset Rule", "Regel zurücksetzen"), ActionKind::Mutation).with_category("history"))
            // 🎬️ The navbar example picker dispatches `setActiveExample` at boot and on every pick
            // (`🧱️elements/🛠️ShellHelpers/🟦️.tsx:373`). Without this declaration the shell's very first
            // dispatch was dropped `undeclared-action` — an error line on every boot and an inert
            // picker — while the sibling `🔌️jack` app declared it all along. It stays UNSCOPED (no
            // `window_kind_action_refs`) because it reads the document, not a pane.
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left").with_category("mode"))
            .action_destructive("setActiveExample")
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("setParameter", LocalizedLabel::native("Set Parameter", "Parameter festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("patchNodes", LocalizedLabel::native("Patch Nodes", "Knoten aktualisieren"), ActionKind::Mutation).with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation).with_category("transform") })
            // 🛠️ Dev-only raw rule editors — kept out of the command palette.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("setLhsJson", LocalizedLabel::native("Set LHS Json", "LHS-JSON festlegen"), ActionKind::Mutation).with_category("tools") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("setRhsJson", LocalizedLabel::native("Set RHS Json", "RHS-JSON festlegen"), ActionKind::Mutation).with_category("tools") })
            // 👁️ Ephemeral view state — viewport, recompute/layout, LOD. Selection/hover/text-cursor
            // cross-highlighting is framework-owned now (domain "graph") — no app-declared verbs.
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nodeGraphViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansicht festlegen"), ActionKind::View).with_category("view"))
            .action_with(semio_framework_plugin::ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw").with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View, "layers").with_category("mode"))
            .action_interactive_job("nodeGraphViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addRuleClause", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("resetRule", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_destructive("resetRule")
            .action_interactive_job("setParameter", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("patchNodes", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLhsJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setRhsJson", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("addWorkingNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"), ActionKind::Mutation).with_category("create"))
            .action_interactive_job("addWorkingNode", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🕹️ Domain "graph": before/after/lhs/rhs graph nodes plus rule-clause nodes plus variable
            // references, transitive over each node's first incoming connection / variable binding
            // (see `interaction_topology`). Selection/hover, modes and merges are ALL
            // framework-injected now — no app-declared setSelection/nodeGraphHover/textSelect/
            // textHover/graphPointerDown verbs.
            .interaction(InteractionDefinition {
                id: "graph".into(),
                label: LocalizedLabel::native("Nodes", "Knoten"),
                granularities: vec![GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() }],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: true, broadcast: true },
            })
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_BEFORE, vec![InteractionRef::new("graph")])
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_AFTER, vec![InteractionRef::new("graph")])
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_LHS, vec![InteractionRef::new("graph")])
            .window_kind_interactions(TRINITY_REWRITING_PLAY_WINDOW_RHS, vec![InteractionRef::new("graph")])
            // 📝️ Staged argument forms.
            // 🎬️ The option list IS the subset's registered example set — the navbar picker only ever
            // dispatches a registered id.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(crate::examples::demo::ID, crate::examples::demo::label()),
                ]).required(),
            ])
            .action_args("addRuleClause", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Clause", "Klausel"), vec![
                    ActionArgOption::new("where", LocalizedLabel::native("Where", "Wo")),
                    ActionArgOption::new("create", LocalizedLabel::native("Create", "Erstellen")),
                    ActionArgOption::new("merge", LocalizedLabel::native("Merge", "Zusammenführen")),
                    ActionArgOption::new("set", LocalizedLabel::native("Set", "Setzen")),
                    ActionArgOption::new("delete", LocalizedLabel::native("Delete", "Löschen")),
                    ActionArgOption::new("parameter", LocalizedLabel::native("Parameter", "Parameter")),
                ]).required(),
            ])
            .action_args("patchNodes", vec![
                ActionArgDef::entity_ids("nodeIds", LocalizedLabel::native("Nodes (empty: selection)", "Knoten (leer: Auswahl)"), "graph", "node"),
                ActionArgDef::select("field", LocalizedLabel::native("Field", "Feld"), vec![
                    ActionArgOption::new("name", LocalizedLabel::native("Name", "Name")),
                    ActionArgOption::new("kind", LocalizedLabel::native("Kind", "Art")),
                ]).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required(),
            ])
            .action_args("setParameter", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Parameter", "Parameter")).required(),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required(),
            ])
            .action_args("addWorkingNode", vec![
                ActionArgDef::text("kind", LocalizedLabel::native("Kind (empty: the graph's first node kind)", "Art (leer: erste Knotenart des Graphen)")),
                ActionArgDef::text("name", LocalizedLabel::native("Name (empty: the new id)", "Name (leer: die neue Id)")),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
            ])
            .action_args("setLhsJson", vec![ActionArgDef::text("value", LocalizedLabel::native("LHS JSON", "LHS-JSON")).required()])
            .action_args("setRhsJson", vec![ActionArgDef::text("value", LocalizedLabel::native("RHS JSON", "RHS-JSON")).required()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint")
            .io(rewriting_io())
            .action_describe("setLodMode", LocalizedLabel::native("Sets the level of detail the rule's graph window draws with; only that window's view changes.", "Legt die Detailstufe fest, mit der das Graphfenster der Regel zeichnet; nur die Ansicht dieses Fensters ändert sich."))
            .action_describe("patchNodes", LocalizedLabel::native("Sets the name or kind of the given nodes in the rule's example graph.", "Setzt Name oder Art der angegebenen Knoten im Beispielgraphen der Regel."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole rewriting rule with the bundled demo rule, or with the blank default rule, by example id.", "Ersetzt die gesamte Umschreiberegel durch die mitgelieferte Demo-Regel oder die leere Standardregel, anhand der Beispiel-Id."))
            .action_describe("addRuleClause", LocalizedLabel::native("Adds a clause of the given kind (where, create, merge, set, delete or parameter) to the rewriting rule; a where clause is only added once.", "Fügt der Umschreiberegel eine Klausel der angegebenen Art hinzu (where, create, merge, set, delete oder parameter); eine where-Klausel wird nur einmal hinzugefügt."))
            .action_describe("resetRule", LocalizedLabel::native("Resets the rewriting rule to the blank default rule, discarding its pattern, clauses and parameters.", "Setzt die Umschreiberegel auf die leere Standardregel zurück und verwirft Muster, Klauseln und Parameter."))
            .action_describe("setParameter", LocalizedLabel::native("Sets the bound value of one named rule parameter, parsed as a number, boolean or string according to its declared kind.", "Setzt den gebundenen Wert eines benannten Regelparameters, gelesen als Zahl, Wahrheitswert oder Zeichenkette gemäß seiner deklarierten Art."))
            .action_describe("setLhsJson", LocalizedLabel::native("Replaces the rule's left-hand side, the graph pattern it matches, with the given JSON.", "Ersetzt die linke Seite der Regel, das Graphmuster, das sie erkennt, durch das angegebene JSON."))
            .action_describe("setRhsJson", LocalizedLabel::native("Replaces the rule's right-hand side, what it writes for each match, with the given JSON and resets the parameter bindings to their defaults.", "Ersetzt die rechte Seite der Regel, was sie für jeden Treffer schreibt, durch das angegebene JSON und setzt die Parameterbindungen auf ihre Standardwerte zurück."))
            .action_describe("reorganize", LocalizedLabel::native("Drops every manual node position of the rule graph so it is laid out automatically again.", "Verwirft alle manuellen Knotenpositionen des Regelgraphen, sodass er wieder automatisch angeordnet wird."))
            .action_describe("addWorkingNode", LocalizedLabel::native("Adds one node to the rule's example graph at x, y, named with the first free id unless a name is given.", "Fügt dem Beispielgraphen der Regel einen Knoten an x, y hinzu, benannt mit der ersten freien Id, sofern kein Name angegeben ist."))
            .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("setLhsJson")
            .action_destructive("setRhsJson")
            .action_destructive("reorganize")
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🪢️TaxonomyMounts
#[path = "🎮️commands/🗑️delete-rule-clause/🦀️.rs"]
pub mod delete_rule_clause;
//#endregion 🪢️TaxonomyMounts

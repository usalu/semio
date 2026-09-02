//! ♻️ Trinity Rewriting editor — parametric rewriting editor bundled as a hot-swappable WASM plugin.
//!
//! 📌️ Pure-trait `ArtifactEditor`: `TrinityRewritingPlayApp` is a unit struct; every former
//! `RewritingPlayRuntime` field (selection, hover/select var, camera, LOD, …) lives in
//! `config::RewritingConfig`, written via `config::RewritingConfigMutation`s. Every rule/parameter/
//! before-fixture edit flows through the semantic `RewriteRuleMutation` vocabulary (`edit-*` body
//! replaces, `change-*`/`remove-*` map upserts) — see
//! `crate::artifacts::rewriting::rewriting_snapshot_mutations`, the seam commands that still
//! compute a whole `next: RewritingSnapshot` use to emit granular mutations. The
//! `TrinityRewritingCommand` enum stays hand-rolled (TEMPLATE §5.1 fallback, same rationale as `jack`).

use crate::artifacts::jack::{Camera, JackSnapshot, Node, PropertyValue};
use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::artifacts::rewriting::schema::{ParameterKind, Rhs};
use crate::artifacts::rewriting::{LayoutPoint, RewritingSnapshot, REWRITE_RULE_SCHEMA, TRINITY_REWRITING_DIALECT};
use crate::editor::rewriting::config::{RewritingConfig, RewritingConfigMutation};
use crate::editor::rewriting::presence::{RewritingPresence, RewritingPresenceMutation};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionKind, AppActionRegistry, ArtifactEditor, ArtifactView, ConfigView, ContextMenuItemSpec, ContextMenuRequest, Dialect, DomainTopology, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider,
    HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, NodeGraphViewport, PanelGroup, SelectionMethod,
    SelectionMode, SelectionSpec, TopologyNode, WindowMeasure, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
// 🩹️ `InteractionView` is not re-exported at `semio_framework_plugin`'s crate root (unlike
// `ConfigView`/`ArtifactView`/`DraftView`) — only reachable through its owning `app` submodule
// (itself `pub mod`). Flagged as a likely framework oversight, not fixed here (framework file).
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as SemanticSurfaceKind;
use std::collections::{BTreeMap, HashMap};
use store::EngineHandles;
use store::{ArtifactDsl, ArtifactPack};

//#region 🔖️Constants
pub(crate) const TRINITY_REWRITING_PLAY_CONTROLLER_ID: &str = "trinity-rewriting-play";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_BEFORE: &str = "trinity.rewriting.before";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_AFTER: &str = "trinity.rewriting.after";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_LHS: &str = "trinity.rewriting.lhs";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_RHS: &str = "trinity.rewriting.rhs";
pub(crate) const TRINITY_REWRITING_PLAY_SURFACE_JACK: &str = "trinity.rewriting.jack";
const TRINITY_REWRITING_PLAY_BODY_BEFORE: &str = "trinity.rewriting.play.before";
const TRINITY_REWRITING_PLAY_BODY_AFTER: &str = "trinity.rewriting.play.after";
const TRINITY_REWRITING_PLAY_BODY_LHS: &str = "trinity.rewriting.play.lhs";
const TRINITY_REWRITING_PLAY_BODY_RHS: &str = "trinity.rewriting.play.rhs";
const TRINITY_REWRITING_PLAY_BODY_JACK: &str = "trinity.rewriting.play.jack";
const TRINITY_REWRITING_PLAY_BODY_PARAMETERS: &str = "trinity.rewriting.play.parameters";
const TRINITY_REWRITING_PLAY_BODY_DOCUMENT: &str = "trinity.rewriting.play.document";
const TRINITY_REWRITING_PLAY_BODY_CATALOGUE: &str = "trinity.rewriting.play.catalogue";
const TRINITY_REWRITING_PLAY_BODY_INSPECTION: &str = "trinity.rewriting.play.inspection";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_BEFORE: &str = "trinity-rewriting-before";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_AFTER: &str = "trinity-rewriting-after";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_LHS: &str = "trinity-rewriting-lhs";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_RHS: &str = "trinity-rewriting-rhs";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_JACK: &str = "trinity-rewriting-jack";
pub(crate) const TRINITY_REWRITING_PLAY_WINDOW_PARAMETERS: &str = "trinity-rewriting-parameters";
const TRINITY_REWRITING_PLAY_RULE_NAME: &str = "label-core";

const NAKAGIN_FIXTURE_DSL: &str = include_str!("../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

const DEFAULT_LHS_JSON: &str = r#"{
  "pattern": {
    "leftVar": "a",
    "leftKind": "Piece",
    "edgeVar": "r",
    "edgeKind": "Connection",
    "rightVar": "b",
    "rightKind": "Piece"
  },
  "whereClause": "a.name = 'b'"
}"#;

const DEFAULT_RHS_JSON: &str = r#"{
  "create": [],
  "delete": [],
  "set": [{ "var": "a", "prop": "label", "value": "$label" }],
  "merge": [],
  "parameters": [{ "name": "label", "kind": "string", "default": "nakagin-core" }]
}"#;

const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 📦️ JSON text of the bundled Nakagin fixture — `RewritingSnapshot`'s own `_json` fields keep their
/// JSON contract, so the `.trinity` DSL source is parsed once and re-serialized here.
fn nakagin_fixture_json() -> String {
    JackSnapshot::parse_dsl(NAKAGIN_FIXTURE_DSL).expect("bundled nakagin fixture parses").to_json().expect("fixture serializes")
}

pub(crate) fn default_parameter_bindings(rhs_json: &str) -> BTreeMap<String, PropertyValue> {
    let Ok(rhs) = pack::from_json_str::<Rhs>(rhs_json) else {
        return BTreeMap::new();
    };
    rhs.parameters.iter().map(|param| (param.name.clone(), param.default.clone())).collect()
}

pub(crate) fn default_rule_state() -> RewritingSnapshot {
    let mut state = RewritingSnapshot { before_fixture_json: nakagin_fixture_json(), lhs_json: DEFAULT_LHS_JSON.into(), rhs_json: DEFAULT_RHS_JSON.into(), parameter_bindings: BTreeMap::new(), rule_layout: BTreeMap::new() };
    state.parameter_bindings = default_parameter_bindings(&state.rhs_json);
    state
}

/// 🌱️ Reads `RewritingSnapshot.before_fixture_json`'s seed-only `camera` field once — the one place a
/// before-fixture's initial framing is consumed into the app's live config camera.
pub(crate) fn seed_before_pane_camera(state: &RewritingSnapshot) -> Camera {
    parse_fixture_json(&state.before_fixture_json).map(|fixture| fixture.camera).unwrap_or_default()
}

/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetState` — see
/// `📓️taxonomy.md`'s forbidden vocabulary), so `resetRule` builds a `Effect::LoadDocument`
/// (outside undo history) instead of an `artifact_mutations` entry.
pub(crate) fn reset_document_effect(state: &RewritingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <RewritingSnapshot as ArtifactPack>::encode_pack(state);
    let envelope = store::create_document_envelope::<RewritingSnapshot, RewriteRuleMutation>(REWRITE_RULE_SCHEMA, "rewriting", state.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("rewriting document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

pub(crate) fn rewriting_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(TRINITY_REWRITING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Binds window chrome through its retained renderer action descriptor.
pub(crate) fn rewriting_window_action(action: &str, args: Option<pack::JsonValue>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: TRINITY_REWRITING_PLAY_CONTROLLER_ID.into(), action: action.into(), args: args.map(|value| pack::json_to_dsl_value(&value)) }
}

/// 🏷️ Admits resolved Rewriting text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "Rewriting UI label admission failed"))
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

pub(crate) fn parse_fixture_json(json: &str) -> Option<JackSnapshot> {
    JackSnapshot::from_json(json).ok()
}

fn build_rule_from_state(state: &RewritingSnapshot) -> Result<crate::artifacts::rewriting::schema::Rule, String> {
    let lhs: crate::artifacts::rewriting::schema::Lhs = pack::from_json_str(&state.lhs_json).map_err(|e| e.to_string())?;
    let rhs: Rhs = pack::from_json_str(&state.rhs_json).map_err(|e| e.to_string())?;
    Ok(crate::artifacts::rewriting::schema::Rule { name: TRINITY_REWRITING_PLAY_RULE_NAME.into(), lhs, rhs })
}

pub(crate) fn compiled_jack_query(state: &RewritingSnapshot) -> String {
    let rule_json = match build_rule_from_state(state) {
        Ok(rule) => pack::to_json_string(&rule).unwrap_or_default(),
        Err(_) => return String::new(),
    };
    let bindings_json = pack::to_json_string(&state.parameter_bindings);
    crate::artifacts::rewriting::schema::rule_query_json(&rule_json, &bindings_json)
        .ok()
        .and_then(|json| pack::parse_json(&json).ok())
        .and_then(|value| value.get("query").and_then(|query| query.as_str()).map(str::to_string))
        .unwrap_or_else(|| build_rule_from_state(state).map(|rule| crate::artifacts::rewriting::schema::build_rule_query(&rule, &state.parameter_bindings)).unwrap_or_default())
}

fn apply_rewriting_to_fixture(before_json: &str, state: &RewritingSnapshot) -> String {
    let Ok(mut graph) = crate::artifacts::jack::Graph::load_json(before_json) else {
        return before_json.into();
    };
    let Ok(rule) = build_rule_from_state(state) else {
        return before_json.into();
    };
    if crate::artifacts::rewriting::schema::apply_rule(&mut graph, &rule, &state.parameter_bindings).is_ok() {
        graph.fixture_json().unwrap_or_else(|_| before_json.into())
    } else {
        before_json.into()
    }
}

/// ♻️ Pure computation of the rule-applied result graph — reused both by the `After` window's render
/// and by `ArtifactApp::export_media`'s `"graph:out"` port.
pub(crate) fn after_fixture_json(state: &RewritingSnapshot) -> String {
    apply_rewriting_to_fixture(&state.before_fixture_json, state)
}

fn semantic_rule_node(id: &str, kind: &str, name: &str, x: f64, y: f64, rule_layout: &BTreeMap<String, LayoutPoint>) -> Node {
    let (x, y) = rule_layout.get(id).map_or((x, y), |point| (point.x, point.y));
    Node { id: id.into(), name: name.into(), kind: kind.into(), x, y, width: 160.0, height: 56.0, ports: vec![], properties: Default::default() }
}

fn lhs_semantic_graph_fixture(lhs: &crate::artifacts::rewriting::schema::Lhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> JackSnapshot {
    let mut nodes = vec![semantic_rule_node("lhs-match", "rewriting.match", &format!("{}:{}", lhs.pattern.left_var, lhs.pattern.left_kind), 0.0, 0.0, rule_layout)];
    let mut edges = Vec::new();
    if let Some(where_clause) = lhs.where_clause.as_deref().filter(|value| !value.trim().is_empty()) {
        nodes.push(semantic_rule_node("lhs-where", "rewriting.where", where_clause, 220.0, 80.0, rule_layout));
        edges.push(crate::artifacts::jack::Edge { id: "lhs-match-where".into(), kind: "rewriting.flow".into(), source: "lhs-match@out".into(), target: "lhs-where@in".into(), properties: Default::default() });
    }
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "lhs".into(), Some("nakagin".into()), crate::artifacts::jack::Manifest::nakagin_default(), Camera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges, None)
}

fn rhs_semantic_graph_fixture(rhs: &Rhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> JackSnapshot {
    let mut nodes = Vec::new();
    let edges = Vec::new();
    let mut y = 0.0;
    for (index, pattern) in rhs.create.iter().enumerate() {
        let id = format!("rhs-create-{index}");
        nodes.push(semantic_rule_node(&id, "rewriting.create", &format!("{}:{}", pattern.left_var, pattern.left_kind), (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, pattern) in rhs.merge.iter().enumerate() {
        let id = format!("rhs-merge-{index}");
        nodes.push(semantic_rule_node(&id, "rewriting.merge", &format!("{}:{}", pattern.left_var, pattern.left_kind), (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, assignment) in rhs.set.iter().enumerate() {
        let id = format!("rhs-set-{index}");
        nodes.push(semantic_rule_node(&id, "rewriting.set", &format!("{}.{} = {:?}", assignment.var, assignment.prop, assignment.value), (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, name) in rhs.delete.iter().enumerate() {
        let id = format!("rhs-delete-{index}");
        nodes.push(semantic_rule_node(&id, "rewriting.delete", name, (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, parameter) in rhs.parameters.iter().enumerate() {
        let id = format!("rhs-parameter-{index}");
        let kind = match parameter.kind {
            ParameterKind::String => "string",
            ParameterKind::Number => "number",
            ParameterKind::Boolean => "boolean",
        };
        nodes.push(semantic_rule_node(&id, "rewriting.parameter", &format!("{}:{kind}", parameter.name), (index as f64) * 220.0, y, rule_layout));
    }
    if nodes.is_empty() {
        nodes.push(semantic_rule_node("rhs-empty", "rewriting.create", "result:Piece", 0.0, 0.0, rule_layout));
    }
    JackSnapshot::with_content(JackSnapshot::SCHEMA.into(), "rhs".into(), Some("nakagin".into()), crate::artifacts::jack::Manifest::nakagin_default(), Camera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges, None)
}

pub(crate) fn lhs_graph_fixture_json(lhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
    let Ok(lhs) = pack::from_json_str::<crate::artifacts::rewriting::schema::Lhs>(lhs_json) else {
        return nakagin_fixture_json();
    };
    crate::artifacts::jack::Graph::from_fixture(lhs_semantic_graph_fixture(&lhs, rule_layout)).ok().and_then(|graph| graph.fixture_json().ok()).unwrap_or_else(nakagin_fixture_json)
}

pub(crate) fn rhs_graph_fixture_json(rhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
    let Ok(rhs) = pack::from_json_str::<Rhs>(rhs_json) else {
        return nakagin_fixture_json();
    };
    crate::artifacts::jack::Graph::from_fixture(rhs_semantic_graph_fixture(&rhs, rule_layout)).ok().and_then(|graph| graph.fixture_json().ok()).unwrap_or_else(nakagin_fixture_json)
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
/// `trinity.graph` as this rule's `before_fixture_json` working graph, and `graph:out` re-emits the
/// rule-applied result graph.
pub(crate) fn rewriting_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: REWRITE_RULE_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
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
        artifact: semio_framework_plugin::ArtifactPresentation { id: "trinity.rewriting".into(), name: "Trinity Rewrite Rule".into(), dimension: "graph".into(), component_kind: "trinity".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Render
fn rewriting_lod_json_for_window(cfg: &RewritingConfig, window_id: &str) -> Option<String> {
    let mode = cfg.lod_mode_by_window.get(window_id).map_or(TRINITY_LOD_MODE_AUTOMATIC, String::as_str);
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        Some(pack::json!({ "automatic": true }).to_string())
    } else {
        Some(pack::json!({ "automatic": false, "forcedLabel": mode }).to_string())
    }
}

fn trinity_rewriting_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![semio_framework_plugin::MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    let rows: Vec<pack::JsonValue> = pack::parse_json(&crate::editor::rewriting::world::trinity_lod_scale_json()).ok().and_then(|value| value.as_array().map(<[pack::JsonValue]>::to_vec)).unwrap_or_default();
    items.extend(rows.into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(semio_framework_plugin::MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: rewriting_window_action("setLodMode", Some(pack::json!({ "windowId": window_id }))) }
}

/// 🕹️ `selection`/`hover` are left unset: `ArtifactApp::render` has no `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` gained one — see `📌️panels/🔍️inspection`'s doc comment
/// on `editor::jack` for the same framework-side gap) and this static scene isn't a `UiNode::Tree` the
/// wrapper's `stamp_and_cache_interaction_ui` post-pass would stamp either. The live node-graph host
/// reads domain "graph"'s `DomainSelection`/`DomainHover` directly, so the interactive surface stays
/// correct even though this snapshot doesn't carry it.
pub(crate) fn render_fixture_graph(surface_id: &str, window_id: &str, fixture_json: &str, cfg: &RewritingConfig, editable: bool, camera_override: Option<&Camera>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture = parse_fixture_json(fixture_json).unwrap_or_else(|| JackSnapshot::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap());
    let (nodes, edges, fixture_viewport) = crate::editor::jack::fixture_to_workflow(&fixture);
    let viewport = camera_override.map_or(fixture_viewport, |camera| NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom });
    semio_framework_plugin::scene_surface(
        surface_id,
        SemanticSurfaceKind::NodeGraph,
        &semio_framework_plugin::NodeGraphScene { lod_json: rewriting_lod_json_for_window(cfg, window_id), editable: editable.then_some(true), ..semio_framework_plugin::NodeGraphScene::base(nodes, edges, viewport) },
    )
}
//#endregion 🔖️Render

//#region 🔖️TrinityRewritingCommand
/// 🎯️ `TrinityRewritingPlayApp::Command` — the SOLE dispatch surface for rewriting's own behavior. Kept
/// hand-rolled (see `jack::TrinityJackCommand`'s doc comment for the rationale). `NodeGraphEdit` keeps
/// its JSON-array `operations` shape (rather than a typed sub-enum) — the same
/// `{"operation":"setFixture"|"deleteSelection", ...}` payload `apply_rewriting_node_graph_edit_operations`
/// already parses, carried as an opaque string field.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
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
    #[dsl(key = "patch-nodes")]
    PatchNodes { node_ids: Vec<String>, field: String, value: String },

    // 👁️ Config-only — was ephemeral `RewritingPlayRuntime` state, now emits `config_mutations`.
    #[dsl(key = "set-viewport")]
    SetViewport { surface_id: Option<String>, viewport_json: String },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "set-lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "set-locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for TrinityRewritingCommand {
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
impl protocol::OpBinary for TrinityRewritingCommand {
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

//#endregion 🔖️TrinityRewritingCommand

//#region 🔖️TrinityRewritingPlayApp
/// ♻️ Trinity Rewriting play app — a parametric-rewriting editor over a {@link RewritingSnapshot} projection.
#[derive(Default)]
pub struct TrinityRewritingPlayApp;

impl ArtifactEditor for TrinityRewritingPlayApp {
    type Snapshot = RewritingSnapshot;
    type Mutation = RewriteRuleMutation;
    type Config = RewritingConfig;
    type ConfigMutation = RewritingConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = RewritingPresence;
    type PresenceMutation = RewritingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = TrinityRewritingCommand;

    const DIALECT: Dialect = TRINITY_REWRITING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::rewriting::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> RewritingSnapshot {
        default_rule_state()
    }

    fn initial_config() -> RewritingConfig {
        let projection = Self::initial_snapshot();
        RewritingConfig { before_pane_camera: seed_before_pane_camera(&projection), ..RewritingConfig::default() }
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(rewriting_io())
    }

    // 🧬️ Whole-document replace is banned from the `Mutation` enum outright (`SetState` — a
    // whole-snapshot LWW register wearing a mutation costume, see `📓️taxonomy.md`'s forbidden
    // vocabulary), so this intentionally falls back to the trait default (`None`) rather than
    // overriding — the `"document:in"` media port therefore reports `MediaError::NotImplemented`;
    // there is no import mutation (locked decision).

    /// 🔌️ `"graph:in"` loads an incoming `trinity.graph` pack as this rule's `before_fixture_json`
    /// working graph — a single targeted field edit, not a whole-document replace.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, RewritingSnapshot>) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "graph:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "graph:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture_json = fixture.to_json().map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let _ = doc;
                Ok(Emit::mutations(vec![crate::artifacts::rewriting::mutations::edit_before_fixture(fixture_json)]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🔌️ `"graph:out"` re-emits the rule-applied result graph, alongside the implicit `"document:out"`.
    fn export_media(port: &str, doc: &ArtifactView<'_, RewritingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "graph:out" => {
                let fixture_json = after_fixture_json(doc.snapshot);
                let fixture = JackSnapshot::from_json(&fixture_json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let bytes = ArtifactPack::encode_pack(&fixture);
                Ok(Media {
                    media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity },
                    payload: MediaPayload::Structured { schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
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
            TrinityRewritingCommand::PatchNodes { .. } => "patchNodes",
            TrinityRewritingCommand::SetViewport { .. } => "setViewport",
            TrinityRewritingCommand::Reorganize => "reorganize",
            TrinityRewritingCommand::SetLodMode { .. } => "setLodMode",
            TrinityRewritingCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(
        command: &TrinityRewritingCommand,
        doc: &ArtifactView<'_, RewritingSnapshot>,
        cfg: &ConfigView<'_, RewritingConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation, Self::DraftMutation>, Fault> {
        let state = doc.snapshot;
        let config = cfg.snapshot;
        match command {
            TrinityRewritingCommand::NodeGraphEdit { surface_id, operations_json } => crate::editor::rewriting::commands::node_graph_edit(state, &interaction.selection("graph").ids, surface_id, operations_json),
            TrinityRewritingCommand::SetLhsJson { value } => crate::editor::rewriting::commands::set_lhs_json(state, value),
            TrinityRewritingCommand::SetRhsJson { value } => crate::editor::rewriting::commands::set_rhs_json(state, value),
            TrinityRewritingCommand::SetParameter { name, value } => crate::editor::rewriting::commands::set_parameter(state, name, value),
            TrinityRewritingCommand::AddRuleClause { kind } => crate::editor::rewriting::commands::add_rule_clause_command(state, kind),
            TrinityRewritingCommand::ResetRule => crate::editor::rewriting::commands::reset_rule(state),
            TrinityRewritingCommand::PatchNodes { node_ids, field, value } => crate::editor::rewriting::commands::patch_nodes(state, node_ids, field, value),
            TrinityRewritingCommand::SetViewport { surface_id, viewport_json } => crate::editor::rewriting::commands::set_viewport(surface_id, viewport_json),
            TrinityRewritingCommand::Reorganize => crate::editor::rewriting::commands::reorganize(config.reorganize_epoch),
            TrinityRewritingCommand::SetLodMode { window_id, value } => crate::editor::rewriting::commands::set_lod_mode(window_id, value),
            TrinityRewritingCommand::SetLocale { value } => crate::editor::rewriting::commands::set_locale(value),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, RewritingSnapshot>, cfg: &ConfigView<'_, RewritingConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let state = doc.snapshot;
        let config = cfg.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::editor::rewriting::terminology::TrinityRewritingLabels>(&config.locale);
        let root = match body_key {
            TRINITY_REWRITING_PLAY_BODY_BEFORE => edit::windows::before::render(state, config),
            TRINITY_REWRITING_PLAY_BODY_AFTER => edit::windows::after::render(state, config),
            TRINITY_REWRITING_PLAY_BODY_LHS => edit::windows::lhs::render(state, config),
            TRINITY_REWRITING_PLAY_BODY_RHS => edit::windows::rhs::render(state, config),
            TRINITY_REWRITING_PLAY_BODY_JACK => edit::windows::jack::render(state, config),
            TRINITY_REWRITING_PLAY_BODY_PARAMETERS => edit::windows::parameters::render(state, labels),
            TRINITY_REWRITING_PLAY_BODY_DOCUMENT => crate::editor::rewriting::panels::document::render(state, config, labels),
            TRINITY_REWRITING_PLAY_BODY_CATALOGUE => crate::editor::rewriting::panels::catalogue::render(labels),
            TRINITY_REWRITING_PLAY_BODY_INSPECTION => crate::editor::rewriting::panels::inspection::render(),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.body.label", "the fixed Trinity body label exceeds its UI bound")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(root))
    }

    fn window_measures(_doc: &ArtifactView<'_, RewritingSnapshot>, cfg: &ConfigView<'_, RewritingConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let mode_for = |window_id: &str| config.lod_mode_by_window.get(window_id).map_or(TRINITY_LOD_MODE_AUTOMATIC, String::as_str);
        HashMap::from([
            (TRINITY_REWRITING_PLAY_WINDOW_BEFORE.to_string(), vec![trinity_rewriting_lod_measure(TRINITY_REWRITING_PLAY_WINDOW_BEFORE, mode_for(TRINITY_REWRITING_PLAY_WINDOW_BEFORE))]),
            (TRINITY_REWRITING_PLAY_WINDOW_AFTER.to_string(), vec![trinity_rewriting_lod_measure(TRINITY_REWRITING_PLAY_WINDOW_AFTER, mode_for(TRINITY_REWRITING_PLAY_WINDOW_AFTER))]),
            (TRINITY_REWRITING_PLAY_WINDOW_LHS.to_string(), vec![trinity_rewriting_lod_measure(TRINITY_REWRITING_PLAY_WINDOW_LHS, mode_for(TRINITY_REWRITING_PLAY_WINDOW_LHS))]),
            (TRINITY_REWRITING_PLAY_WINDOW_RHS.to_string(), vec![trinity_rewriting_lod_measure(TRINITY_REWRITING_PLAY_WINDOW_RHS, mode_for(TRINITY_REWRITING_PLAY_WINDOW_RHS))]),
        ])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &ArtifactView<'_, RewritingSnapshot>, cfg: &ConfigView<'_, RewritingConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = cfg.snapshot.locale.starts_with("de");
        // 🕹️ Selection is framework-owned now (domain "graph") — `context_menu` has no `InteractionView`,
        // so the request's own surface-carried selection groups are the only source; no config fallback.
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &[], &[]);

        // 🩹️ `nodeGraphEdit` folds into the `transform` group alongside `patchNodes` (both are
        // mechanical graph-mutation actions, not primary verbs) — keeping it top-level alongside
        // `addRuleClause`/`setParameter`/`reorganize` plus all four groups plus the separator plus the
        // destructive row exceeds the 9-row top-level budget `organize_context_menu` enforces.
        let mut menu = Menu::of(registry)
            .action("addRuleClause")
            .action("setParameter")
            .action("reorganize")
            .group("transform", |m| m.action("patchNodes").action("nodeGraphEdit"))
            .group("history", |m| m.action("resetRule"))
            .group("mode", |m| m.action("setLodMode"))
            .group("tools", |m| m.action("setLhsJson").action("setRhsJson"));
        if let Some(spec) = node_graph_delete_selection_spec("Delete selection", is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }

    /// 🕹️ Domain "graph" topology: unions three node universes under one "node" granularity —
    /// (1) the Before fixture's own nodes, parented by the source node of their first incoming
    /// connection, each with a variable-reference child when its name resolves one (`var_from_node_name`
    /// — "AST parents + variable references"); (2) the LHS semantic graph (`lhs-where` parented by
    /// `lhs-match` via their one edge); (3) the RHS semantic graph (its clause nodes have no inherent
    /// parent order, so they're roots). `MergeMode::Range` is not declared for this domain, so
    /// `ordered`'s sequence need not be a strict pre-order.
    fn interaction_topology(doc: &ArtifactView<'_, RewritingSnapshot>, _cfg: &ConfigView<'_, RewritingConfig>) -> InteractionTopology {
        let state = doc.snapshot;
        let mut ordered = Vec::new();

        if let Some(fixture) = parse_fixture_json(&state.before_fixture_json) {
            let mut parent_of: BTreeMap<String, String> = BTreeMap::new();
            for edge in fixture.edges() {
                let source = crate::artifacts::jack::port_node_id(&edge.source).unwrap_or(&edge.source).to_string();
                let target = crate::artifacts::jack::port_node_id(&edge.target).unwrap_or(&edge.target).to_string();
                parent_of.entry(target).or_insert(source);
            }
            for node in fixture.nodes() {
                ordered.push(TopologyNode { id: node.id.clone(), granularity: "node".into(), parent: parent_of.get(&node.id).cloned() });
                if let Some(var) = var_from_node_name(&node.name) {
                    ordered.push(TopologyNode { id: var, granularity: "node".into(), parent: Some(node.id.clone()) });
                }
            }
        }
        // 🩹️ Reads the semantic rule graphs directly (`lhs_semantic_graph_fixture`/
        // `rhs_semantic_graph_fixture`), NOT via `lhs_graph_fixture_json`/`rhs_graph_fixture_json`:
        // those round-trip through `Graph::from_fixture`, which validates node kinds against the
        // "nakagin" manifest — the synthetic `rewriting.*` clause kinds fail that validation and the
        // wrapper silently falls back to the nakagin fixture, which would leave the "graph" domain's
        // topology missing every `lhs-*`/`rhs-*` id entirely.
        if let Ok(lhs) = pack::from_json_str::<crate::artifacts::rewriting::schema::Lhs>(&state.lhs_json) {
            let lhs_fixture = lhs_semantic_graph_fixture(&lhs, &state.rule_layout);
            let mut parent_of: BTreeMap<String, String> = BTreeMap::new();
            for edge in lhs_fixture.edges() {
                let source = crate::artifacts::jack::port_node_id(&edge.source).unwrap_or(&edge.source).to_string();
                let target = crate::artifacts::jack::port_node_id(&edge.target).unwrap_or(&edge.target).to_string();
                parent_of.entry(target).or_insert(source);
            }
            for node in lhs_fixture.nodes() {
                ordered.push(TopologyNode { id: node.id.clone(), granularity: "node".into(), parent: parent_of.get(&node.id).cloned() });
            }
        }
        if let Ok(rhs) = pack::from_json_str::<Rhs>(&state.rhs_json) {
            for node in rhs_semantic_graph_fixture(&rhs, &state.rule_layout).nodes() {
                ordered.push(TopologyNode { id: node.id.clone(), granularity: "node".into(), parent: None });
            }
        }

        let mut domains = BTreeMap::new();
        domains.insert("graph".to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }
}
//#endregion 🔖️TrinityRewritingPlayApp

//#region 🔖️Manifest
use crate::editor::rewriting::modes::edit;

/// 🎯️ `create_rewriting_app` → `Editor::builder(TRINITY_REWRITING_DIALECT)…build_definition()` (contract
/// §2.4). The old `.example("label-core", …)`/`.workflow("trinity-rewriting", …)` calls are DROPPED,
/// not ported — same SDK gap `jack`'s `create_trinity_jack_app` doc comment records.
pub fn create_rewriting_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(TRINITY_REWRITING_DIALECT).document(["semio", "trinity", "rewriting"])
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
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                TRINITY_REWRITING_PLAY_BODY_DOCUMENT,
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
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("setParameter", LocalizedLabel::native("Set Parameter", "Parameter festlegen"), ActionKind::Mutation).with_category("settings"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("patchNodes", LocalizedLabel::native("Patch Nodes", "Knoten aktualisieren"), ActionKind::Mutation).with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation).with_category("transform"))
            // 🛠️ Dev-only raw rule editors — kept out of the command palette.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("setLhsJson", LocalizedLabel::native("Set LHS Json", "LHS-JSON festlegen"), ActionKind::Mutation).with_category("tools") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("setRhsJson", LocalizedLabel::native("Set RHS Json", "RHS-JSON festlegen"), ActionKind::Mutation).with_category("tools") })
            // 👁️ Ephemeral view state — viewport, recompute/layout, LOD. Selection/hover/text-cursor
            // cross-highlighting is framework-owned now (domain "graph") — no app-declared verbs.
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("setViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansicht festlegen"), ActionKind::View).with_category("view"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::View).with_category("view"))
            .action_with(semio_framework_plugin::ActionDefinition::bounded_catalog("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View).with_category("mode"))
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
            .action_args("setLhsJson", vec![ActionArgDef::text("value", LocalizedLabel::native("LHS JSON", "LHS-JSON")).required()])
            .action_args("setRhsJson", vec![ActionArgDef::text("value", LocalizedLabel::native("RHS JSON", "RHS-JSON")).required()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+alt+s", "commitCheckpoint")
            .io(rewriting_io())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::rewriting::schema::Rhs;
    use protocol::{OpBinary, OpText};
    use semio_framework_plugin::{testkit, App, EditorApp, Locale, PluginApp, Terminology, VcsArtifactApp, ViewModel};

    /// 🎫️ See `jack`'s `trinity_jack_manifest_for_testkit` doc comment for why this wrapper exists
    /// (SDK gap, `testkit::new_app_with_registry`'s signature is still `fn(manifest: fn() -> App)`).
    fn trinity_rewriting_manifest_for_testkit() -> App {
        App { definition: create_rewriting_app(), examples: Vec::new() }
    }

    fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
        testkit::meta(actor)
    }

    /// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityRewritingCommand` variant round-trips
    /// through both its binary (`OpBinary`) and text (`OpText`) codecs.
    #[semio_framework_async_macros::async_test]
    async fn trinity_rewriting_command_text_and_binary_round_trip() {
        let commands = vec![
            TrinityRewritingCommand::NodeGraphEdit { surface_id: "trinity.rewriting.before".into(), operations_json: "[]".into() },
            TrinityRewritingCommand::SetLhsJson { value: "{}".into() },
            TrinityRewritingCommand::SetRhsJson { value: "{}".into() },
            TrinityRewritingCommand::SetParameter { name: "label".into(), value: "hi".into() },
            TrinityRewritingCommand::AddRuleClause { kind: "where".into() },
            TrinityRewritingCommand::ResetRule,
            TrinityRewritingCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
            TrinityRewritingCommand::SetViewport { surface_id: Some("trinity.rewriting.before".into()), viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
            TrinityRewritingCommand::Reorganize,
            TrinityRewritingCommand::SetLodMode { window_id: "trinity-rewriting-before".into(), value: "compact".into() },
            TrinityRewritingCommand::SetLocale { value: "de-DE".into() },
        ];
        for command in commands {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(TrinityRewritingCommand::decode_op(&bytes).expect("decode"), command);
            let text = command.print_op();
            assert_eq!(TrinityRewritingCommand::parse_op(&text).expect("parse"), command);
        }
    }

    /// 🕹️ Registry-backed (not the bare `testkit::new_app`): `interactionSelect`/`interactionHover`
    /// resolve the dispatching app's declared `AppActionRegistry.interactions`, so any test exercising
    /// domain "graph" selection needs the real manifest's `.interaction(...)` declaration present.
    fn new_app() -> VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>> {
        testkit::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(trinity_rewriting_manifest_for_testkit)
    }

    /// 🕹️ Dispatches the framework-injected `interactionSelect` verb against domain "graph" — the
    /// replacement for the deleted `TrinityRewritingCommand::SetSelection`.
    fn select_graph(app: &mut VcsArtifactApp<EditorApp<TrinityRewritingPlayApp>>, ids: &[&str]) {
        let targets: Vec<pack::JsonValue> = ids.iter().map(|id| pack::json!({ "granularity": "node", "id": id })).collect();
        let args = pack::json!({ "domainId": "graph", "targets": pack::to_json_string(&targets) });
        app.handle_action("interactionSelect", Some(&args), &meta("local")).expect("interactionSelect");
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        let mut app = testkit::new_app_with_registry::<EditorApp<TrinityRewritingPlayApp>>(trinity_rewriting_manifest_for_testkit);
        let request = ContextMenuRequest {
            menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
            surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: TRINITY_REWRITING_PLAY_SURFACE_BEFORE.into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
                selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec!["n1".into(), "n2".into()] }],
                text: None,
            }),
            window_instance_id: None,
            point: None,
        };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive delete-selection must be last: {menu:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_before_and_after_graphs() {
        let mut app = new_app();
        let before = app.render(TRINITY_REWRITING_PLAY_BODY_BEFORE, None, &ViewModel::default()).expect("render");
        let after = app.render(TRINITY_REWRITING_PLAY_BODY_AFTER, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&before).contains("node-graph"));
        assert!(pack::to_json_string(&after).contains("node-graph"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_viewport_writes_before_pane_config_camera_without_artifact_mutations() {
        let mut app = new_app();
        let before_state = app.snapshot().unwrap();
        let result = app
            .dispatch_typed(TrinityRewritingCommand::SetViewport { surface_id: Some(TRINITY_REWRITING_PLAY_SURFACE_BEFORE.into()), viewport_json: pack::json!({ "x": 10.0, "y": 20.0, "zoom": 2.5 }).to_string() }, &meta("local"))
            .expect("viewport");
        assert!(result.mutations.is_empty(), "camera is a config-only command, no document operations");
        assert_eq!(app.snapshot().unwrap(), before_state, "document is untouched by a viewport pan");
        let before = app.render(TRINITY_REWRITING_PLAY_BODY_BEFORE, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&before).contains("2.5"), "render reads the live config camera");
    }

    #[semio_framework_async_macros::async_test]
    async fn compiles_jack_query_from_rule() {
        let query = compiled_jack_query(&default_rule_state());
        assert!(query.contains("MATCH"));
        assert!(query.contains("SET"));
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_rewriting_changes_after_fixture() {
        let state = default_rule_state();
        assert_ne!(state.before_fixture_json, after_fixture_json(&state));
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_lhs_rhs_graphs() {
        let mut app = new_app();
        let lhs_json = pack::to_json_string(&app.render(TRINITY_REWRITING_PLAY_BODY_LHS, None, &ViewModel::default()).expect("render"));
        let rhs_json = pack::to_json_string(&app.render(TRINITY_REWRITING_PLAY_BODY_RHS, None, &ViewModel::default()).expect("render"));
        assert!(lhs_json.contains("node-graph"));
        assert!(rhs_json.contains("node-graph"));
        assert!(lhs_json.contains("\"editable\":true"));
        assert!(rhs_json.contains("\"editable\":true"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_parameter_emits_one_op_and_is_undoable() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityRewritingCommand::SetParameter { name: "label".into(), value: "changed".into() }, &meta("local")).expect("set parameter");
        assert_eq!(result.mutations.len(), 1, "a single-key parameter edit is one ChangeParameterBinding operation");
        assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.snapshot().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
    }

    #[semio_framework_async_macros::async_test]
    async fn add_and_delete_rhs_set_clause() {
        let mut app = new_app();
        app.dispatch_typed(TrinityRewritingCommand::AddRuleClause { kind: "set".into() }, &meta("local")).expect("add clause");
        let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 2);
        select_graph(&mut app, &["rhs-set-1"]);
        let result = app
            .dispatch_typed(TrinityRewritingCommand::NodeGraphEdit { surface_id: TRINITY_REWRITING_PLAY_SURFACE_RHS.into(), operations_json: pack::json!([{ "operation": "deleteSelection" }]).to_string() }, &meta("local"))
            .expect("delete selection");
        assert!(!result.mutations.is_empty());
        let rhs: Rhs = pack::from_json_str(&app.snapshot().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn jack_view_renders_compiled_query_tokens() {
        let mut app = new_app();
        let node = app.render(TRINITY_REWRITING_PLAY_BODY_JACK, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&node).contains("tokensJson"));
    }

    #[semio_framework_async_macros::async_test]
    async fn graph_scenes_have_lod_json() {
        let mut app = new_app();
        let before = app.render(TRINITY_REWRITING_PLAY_BODY_BEFORE, None, &ViewModel::default()).expect("render");
        assert!(pack::to_json_string(&before).contains("lodJson"));
    }

    #[semio_framework_async_macros::async_test]
    async fn app_definition_declares_reorganize_and_history_actions() {
        let definition = create_rewriting_app();
        let action_ids: Vec<&str> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|action| action.id.as_str()).collect();
        assert!(action_ids.contains(&"undo"));
        assert!(action_ids.contains(&"reorganize"));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_rewriting_labels_resolve_native_by_default() {
        let mut app = new_app();
        let json = pack::to_json_string(&app.render(TRINITY_REWRITING_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render"));
        assert!(json.contains("\"Pieces\""));
        assert!(!json.contains("Stücke"));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_rewriting_labels_translate_panels_in_german() {
        let mut app = new_app();
        app.dispatch_typed(TrinityRewritingCommand::SetLocale { value: "de-DE".into() }, &meta("local")).expect("set locale");
        let document_json = pack::to_json_string(&app.render(TRINITY_REWRITING_PLAY_BODY_DOCUMENT, None, &ViewModel::default()).expect("render"));
        assert!(document_json.contains("Stücke"));
        assert!(!document_json.contains("\"Pieces\""));
        let catalogue_json = pack::to_json_string(&app.render(TRINITY_REWRITING_PLAY_BODY_CATALOGUE, None, &ViewModel::default()).expect("render"));
        assert!(catalogue_json.contains("Katalog"));
        assert!(catalogue_json.contains("Zu LHS hinzufügen"));
        assert!(catalogue_json.contains("Zu RHS hinzufügen"));
        let parameters_json = pack::to_json_string(&app.render(TRINITY_REWRITING_PLAY_BODY_PARAMETERS, None, &ViewModel::default()).expect("render"));
        assert!(parameters_json.contains("\"Parameter\""));
        let definition = create_rewriting_app();
        let reset_rule = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "resetRule").expect("resetRule action");
        assert_eq!(reset_rule.label.resolve(Terminology::Native, Locale::De), "Regel zurücksetzen");
    }

    #[semio_framework_async_macros::async_test]
    async fn set_lhs_json_undo_redo_round_trip() {
        let mut app = new_app();
        let original = app.snapshot().unwrap().lhs_json;
        let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
        app.dispatch_typed(TrinityRewritingCommand::SetLhsJson { value: next_lhs.into() }, &meta("local")).expect("set lhs");
        assert_eq!(app.snapshot().unwrap().lhs_json, next_lhs);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.snapshot().unwrap().lhs_json, original);
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(app.snapshot().unwrap().lhs_json, next_lhs);
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_graph_out_reflects_rule_applied_fixture() {
        let mut app = new_app();
        let graph_out = semio_framework_plugin::resolve_ready(app.export_media("graph:out")).expect("graph:out export");
        let MediaPayload::Structured { json, .. } = graph_out.payload else { panic!("structured payload") };
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64");
        let fixture = <JackSnapshot as ArtifactPack>::decode_pack(&bytes).expect("decode pack");
        let expected = JackSnapshot::from_json(&after_fixture_json(&app.snapshot().unwrap())).unwrap();
        assert_eq!(fixture.nodes().len(), expected.nodes().len());
    }

    #[semio_framework_async_macros::async_test]
    async fn rewriting_io_declares_graph_in_and_graph_out_ports() {
        let io = rewriting_io();
        assert_eq!(io.document_schema, REWRITE_RULE_SCHEMA);
        let graph_in = io.ports.iter().find(|port| port.id == "graph:in").expect("graph:in declared");
        assert_eq!(graph_in.kind_id.as_deref(), Some("graph.trinity"));
        assert_eq!(graph_in.multiplicity, semio_framework_plugin::PortMultiplicity::One);
        let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
        assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }
}
//#endregion 🧪️Tests

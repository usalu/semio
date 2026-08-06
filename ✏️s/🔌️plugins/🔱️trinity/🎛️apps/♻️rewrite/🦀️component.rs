//! ♻️ Trinity Rewrite app — parametric rewrite play app bundled as a hot-swappable WASM plugin.
//!
//! 📌️ Pure-trait `DocumentApp`: `TrinityRewritePlayApp` is a unit struct; every former
//! `RewritePlayRuntime` field (selection, hover/select var, camera, LOD, …) lives in
//! `config::RewriteConfig`, written via `config::RewriteConfigOperation`s. Every rule/parameter/
//! before-fixture mutation flows through the single LWW `RewriteRuleOperation::SetState`. The
//! `TrinityRewriteCommand` enum stays hand-rolled (TEMPLATE §5.1 fallback, same rationale as `jack`).

use crate::artifacts::jack::{Camera, GraphFixture, Node, PropertyValue};
use crate::artifacts::rewrite::engine::{ParameterKind, Rhs};
use crate::artifacts::rewrite::op::RewriteRuleOperation;
use crate::artifacts::rewrite::{LayoutPoint, RewriteRuleState, REWRITE_RULE_SCHEMA};
use crate::apps::rewrite::config::{RewriteConfig, RewriteConfigOperation};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, 
    ActionArgDef, ActionArgOption, ActionKind, App, AppActionRegistry, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload,
    MediaType, NodeGraphViewport, PanelGroup, SurfaceKind, UiNode, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use store::EngineHandles;
use std::collections::{BTreeMap, HashMap};
use store::{DocumentDsl, DocumentPack};

//#region 🔖️Constants
pub(crate) const TRINITY_REWRITE_PLAY_APP_ID: &str = "trinity-rewrite-play";
pub(crate) const TRINITY_REWRITE_PLAY_CONTROLLER_ID: &str = "trinity-rewrite-play";
pub(crate) const TRINITY_REWRITE_PLAY_SURFACE_BEFORE: &str = "trinity.rewrite.before";
pub(crate) const TRINITY_REWRITE_PLAY_SURFACE_AFTER: &str = "trinity.rewrite.after";
pub(crate) const TRINITY_REWRITE_PLAY_SURFACE_LHS: &str = "trinity.rewrite.lhs";
pub(crate) const TRINITY_REWRITE_PLAY_SURFACE_RHS: &str = "trinity.rewrite.rhs";
pub(crate) const TRINITY_REWRITE_PLAY_SURFACE_JACK: &str = "trinity.rewrite.jack";
const TRINITY_REWRITE_PLAY_BODY_BEFORE: &str = "trinity.rewrite.play.before";
const TRINITY_REWRITE_PLAY_BODY_AFTER: &str = "trinity.rewrite.play.after";
const TRINITY_REWRITE_PLAY_BODY_LHS: &str = "trinity.rewrite.play.lhs";
const TRINITY_REWRITE_PLAY_BODY_RHS: &str = "trinity.rewrite.play.rhs";
const TRINITY_REWRITE_PLAY_BODY_JACK: &str = "trinity.rewrite.play.jack";
const TRINITY_REWRITE_PLAY_BODY_PARAMETERS: &str = "trinity.rewrite.play.parameters";
const TRINITY_REWRITE_PLAY_BODY_DOCUMENT: &str = "trinity.rewrite.play.document";
const TRINITY_REWRITE_PLAY_BODY_CATALOGUE: &str = "trinity.rewrite.play.catalogue";
const TRINITY_REWRITE_PLAY_BODY_INSPECTION: &str = "trinity.rewrite.play.inspection";
pub(crate) const TRINITY_REWRITE_PLAY_WINDOW_BEFORE: &str = "trinity-rewrite-before";
pub(crate) const TRINITY_REWRITE_PLAY_WINDOW_AFTER: &str = "trinity-rewrite-after";
pub(crate) const TRINITY_REWRITE_PLAY_WINDOW_LHS: &str = "trinity-rewrite-lhs";
pub(crate) const TRINITY_REWRITE_PLAY_WINDOW_RHS: &str = "trinity-rewrite-rhs";
const TRINITY_REWRITE_PLAY_WINDOW_JACK: &str = "trinity-rewrite-jack";
const TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS: &str = "trinity-rewrite-parameters";
const TRINITY_REWRITE_PLAY_RULE_NAME: &str = "label-core";

const NAKAGIN_FIXTURE_DSL: &str = include_str!("../../🗿️artifacts/🔌️jack/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.trinity.jack.dsl.semio");

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
/// 📦️ JSON text of the bundled Nakagin fixture — `RewriteRuleState`'s own `_json` fields keep their
/// JSON contract, so the `.trinity` DSL source is parsed once and re-serialized here.
fn nakagin_fixture_json() -> String {
    GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).expect("bundled nakagin fixture parses").to_json().expect("fixture serializes")
}

pub(crate) fn default_parameter_bindings(rhs_json: &str) -> BTreeMap<String, PropertyValue> {
    let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
        return BTreeMap::new();
    };
    rhs.parameters.iter().map(|param| (param.name.clone(), param.default.clone())).collect()
}

pub(crate) fn default_rule_state() -> RewriteRuleState {
    let mut state = RewriteRuleState { before_fixture_json: nakagin_fixture_json(), lhs_json: DEFAULT_LHS_JSON.into(), rhs_json: DEFAULT_RHS_JSON.into(), parameter_bindings: BTreeMap::new(), rule_layout: BTreeMap::new() };
    state.parameter_bindings = default_parameter_bindings(&state.rhs_json);
    state
}

/// 🌱️ Reads `RewriteRuleState.before_fixture_json`'s seed-only `camera` field once — the one place a
/// before-fixture's initial framing is consumed into the app's live config camera.
pub(crate) fn seed_before_pane_camera(state: &RewriteRuleState) -> Camera {
    parse_fixture_json(&state.before_fixture_json).map(|fixture| fixture.camera).unwrap_or_default()
}

pub(crate) fn rewrite_action(action: &str, args: Option<serde_json::Value>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(TRINITY_REWRITE_PLAY_CONTROLLER_ID).action(action, args)
}

pub(crate) fn parse_fixture_json(json: &str) -> Option<GraphFixture> {
    GraphFixture::from_json(json).ok()
}

fn build_rule_from_state(state: &RewriteRuleState) -> Result<crate::artifacts::rewrite::engine::Rule, String> {
    let lhs: crate::artifacts::rewrite::engine::Lhs = serde_json::from_str(&state.lhs_json).map_err(|e| e.to_string())?;
    let rhs: Rhs = serde_json::from_str(&state.rhs_json).map_err(|e| e.to_string())?;
    Ok(crate::artifacts::rewrite::engine::Rule { name: TRINITY_REWRITE_PLAY_RULE_NAME.into(), lhs, rhs })
}

pub(crate) fn compiled_jack_query(state: &RewriteRuleState) -> String {
    let rule_json = match build_rule_from_state(state) {
        Ok(rule) => serde_json::to_string(&rule).unwrap_or_default(),
        Err(_) => return String::new(),
    };
    let bindings_json = serde_json::to_string(&state.parameter_bindings).unwrap_or_else(|_| "{}".into());
    crate::artifacts::rewrite::engine::rule_query_json(&rule_json, &bindings_json)
        .ok()
        .and_then(|json| serde_json::from_str::<serde_json::Value>(&json).ok())
        .and_then(|value| value.get("query").and_then(|query| query.as_str()).map(str::to_string))
        .unwrap_or_else(|| build_rule_from_state(state).map(|rule| crate::artifacts::rewrite::engine::build_rule_query(&rule, &state.parameter_bindings)).unwrap_or_default())
}

fn apply_rewrite_to_fixture(before_json: &str, state: &RewriteRuleState) -> String {
    let Ok(mut graph) = crate::artifacts::jack::Graph::load_json(before_json) else {
        return before_json.into();
    };
    let Ok(rule) = build_rule_from_state(state) else {
        return before_json.into();
    };
    if crate::artifacts::rewrite::engine::apply_rule(&mut graph, &rule, &state.parameter_bindings).is_ok() {
        graph.fixture_json().unwrap_or_else(|_| before_json.into())
    } else {
        before_json.into()
    }
}

/// ♻️ Pure computation of the rule-applied result graph — reused both by the `After` window's render
/// and by `DocumentApp::export_media`'s `"graph:out"` port.
pub(crate) fn after_fixture_json(state: &RewriteRuleState) -> String {
    apply_rewrite_to_fixture(&state.before_fixture_json, state)
}

pub(crate) fn sync_select_var_from_node(fixture_json: &str, node_id: &str) -> Option<String> {
    let fixture = parse_fixture_json(fixture_json)?;
    let node = fixture.nodes.iter().find(|node| node.id == node_id)?;
    var_from_node_name(&node.name)
}

/// 🧭️ Resolves which fixture backs a given rewrite graph surface (Before/After/LHS/RHS), for hover/select var lookup.
pub(crate) fn fixture_json_for_surface(surface_id: &str, state: &RewriteRuleState) -> String {
    if surface_id == TRINITY_REWRITE_PLAY_SURFACE_AFTER {
        after_fixture_json(state)
    } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_LHS {
        lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout)
    } else if surface_id == TRINITY_REWRITE_PLAY_SURFACE_RHS {
        rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout)
    } else {
        state.before_fixture_json.clone()
    }
}

fn semantic_rule_node(id: &str, kind: &str, name: &str, x: f64, y: f64, rule_layout: &BTreeMap<String, LayoutPoint>) -> Node {
    let (x, y) = rule_layout.get(id).map_or((x, y), |point| (point.x, point.y));
    Node { id: id.into(), name: name.into(), kind: kind.into(), x, y, width: 160.0, height: 56.0, ports: vec![], properties: Default::default() }
}

fn lhs_semantic_graph_fixture(lhs: &crate::artifacts::rewrite::engine::Lhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> GraphFixture {
    let mut nodes = vec![semantic_rule_node("lhs-match", "rewrite.match", &format!("{}:{}", lhs.pattern.left_var, lhs.pattern.left_kind), 0.0, 0.0, rule_layout)];
    let mut edges = Vec::new();
    if let Some(where_clause) = lhs.where_clause.as_deref().filter(|value| !value.trim().is_empty()) {
        nodes.push(semantic_rule_node("lhs-where", "rewrite.where", where_clause, 220.0, 80.0, rule_layout));
        edges.push(crate::artifacts::jack::Edge { id: "lhs-match-where".into(), kind: "rewrite.flow".into(), source: "lhs-match@out".into(), target: "lhs-where@in".into(), properties: Default::default() });
    }
    GraphFixture {
        schema: GraphFixture::SCHEMA.into(),
        name: "lhs".into(),
        manifest_id: Some("nakagin".into()),
        manifest: crate::artifacts::jack::Manifest::nakagin_default(),
        camera: Camera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes,
        edges,
        root_node_id: None,
    }
}

fn rhs_semantic_graph_fixture(rhs: &Rhs, rule_layout: &BTreeMap<String, LayoutPoint>) -> GraphFixture {
    let mut nodes = Vec::new();
    let edges = Vec::new();
    let mut y = 0.0;
    for (index, pattern) in rhs.create.iter().enumerate() {
        let id = format!("rhs-create-{index}");
        nodes.push(semantic_rule_node(&id, "rewrite.create", &format!("{}:{}", pattern.left_var, pattern.left_kind), (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, pattern) in rhs.merge.iter().enumerate() {
        let id = format!("rhs-merge-{index}");
        nodes.push(semantic_rule_node(&id, "rewrite.merge", &format!("{}:{}", pattern.left_var, pattern.left_kind), (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, assignment) in rhs.set.iter().enumerate() {
        let id = format!("rhs-set-{index}");
        nodes.push(semantic_rule_node(&id, "rewrite.set", &format!("{}.{} = {:?}", assignment.var, assignment.prop, assignment.value), (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, name) in rhs.delete.iter().enumerate() {
        let id = format!("rhs-delete-{index}");
        nodes.push(semantic_rule_node(&id, "rewrite.delete", name, (index as f64) * 220.0, y, rule_layout));
    }
    y += 80.0;
    for (index, parameter) in rhs.parameters.iter().enumerate() {
        let id = format!("rhs-parameter-{index}");
        let kind = match parameter.kind {
            ParameterKind::String => "string",
            ParameterKind::Number => "number",
            ParameterKind::Boolean => "boolean",
        };
        nodes.push(semantic_rule_node(&id, "rewrite.parameter", &format!("{}:{kind}", parameter.name), (index as f64) * 220.0, y, rule_layout));
    }
    if nodes.is_empty() {
        nodes.push(semantic_rule_node("rhs-empty", "rewrite.create", "result:Piece", 0.0, 0.0, rule_layout));
    }
    GraphFixture {
        schema: GraphFixture::SCHEMA.into(),
        name: "rhs".into(),
        manifest_id: Some("nakagin".into()),
        manifest: crate::artifacts::jack::Manifest::nakagin_default(),
        camera: Camera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes,
        edges,
        root_node_id: None,
    }
}

pub(crate) fn lhs_graph_fixture_json(lhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
    let Ok(lhs) = serde_json::from_str::<crate::artifacts::rewrite::engine::Lhs>(lhs_json) else {
        return nakagin_fixture_json();
    };
    crate::artifacts::jack::Graph::from_fixture(lhs_semantic_graph_fixture(&lhs, rule_layout)).ok().and_then(|graph| graph.fixture_json().ok()).unwrap_or_else(nakagin_fixture_json)
}

pub(crate) fn rhs_graph_fixture_json(rhs_json: &str, rule_layout: &BTreeMap<String, LayoutPoint>) -> String {
    let Ok(rhs) = serde_json::from_str::<Rhs>(rhs_json) else {
        return nakagin_fixture_json();
    };
    crate::artifacts::jack::Graph::from_fixture(rhs_semantic_graph_fixture(&rhs, rule_layout)).ok().and_then(|graph| graph.fixture_json().ok()).unwrap_or_else(nakagin_fixture_json)
}

fn node_id_for_var(fixture_json: &str, var: &str) -> Option<String> {
    if var.is_empty() {
        return None;
    }
    let fixture = GraphFixture::from_json(fixture_json).ok()?;
    fixture.nodes.iter().find(|node| node.name.starts_with(&format!("{var}:")) || node.name == var || var_from_node_name(&node.name).as_deref() == Some(var)).map(|node| node.id.clone())
}

fn graph_hover(fixture_json: &str, hover_var: &str, hover_node_id: &str) -> Option<semio_framework_plugin::NodeGraphHover> {
    let node_id = if !hover_node_id.is_empty() { Some(hover_node_id.to_string()) } else { node_id_for_var(fixture_json, hover_var) }?;
    Some(semio_framework_plugin::NodeGraphHover { node_id: Some(node_id) })
}

fn graph_selection(fixture_json: &str, select_var: &str, selected_ids: &[String]) -> Vec<String> {
    if !selected_ids.is_empty() {
        return selected_ids.to_vec();
    }
    node_id_for_var(fixture_json, select_var).into_iter().collect()
}

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
/// 🔌️ Rewrite's typed media I/O surface (`AppDefinition.io`) — the implicit document in/out pair (a
/// `trinity.rewrite.rule` document) plus a graph in/out pair: `graph:in` loads an incoming
/// `trinity.graph` as this rule's `before_fixture_json` working graph, and `graph:out` re-emits the
/// rule-applied result graph.
pub(crate) fn rewrite_io() -> semio_framework_plugin::AppIo {
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
        artifact: semio_framework_plugin::ArtifactPresentation { id: "trinity.rewrite".into(), name: "Trinity Rewrite Rule".into(), dimension: "graph".into(), component_kind: "trinity".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Render
fn rewrite_lod_json_for_window(cfg: &RewriteConfig, window_id: &str) -> Option<String> {
    let mode = cfg.lod_mode_by_window.get(window_id).map_or(TRINITY_LOD_MODE_AUTOMATIC, String::as_str);
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        Some(serde_json::json!({ "automatic": true }).to_string())
    } else {
        Some(serde_json::json!({ "automatic": false, "forcedLabel": mode }).to_string())
    }
}

fn trinity_rewrite_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![semio_framework_plugin::MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    let rows: Vec<serde_json::Value> = serde_json::from_str(&crate::apps::rewrite::world::trinity_lod_scale_json()).unwrap_or_default();
    items.extend(rows.into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(semio_framework_plugin::MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: rewrite_action("setLodMode", Some(serde_json::json!({ "windowId": window_id }))) }
}

pub(crate) fn render_rule_graph(surface_id: &str, window_id: &str, fixture_json: &str, cfg: &RewriteConfig, hover_node_id: &str, editable: bool, camera_override: Option<&Camera>) -> UiNode {
    let fixture = parse_fixture_json(fixture_json).unwrap_or_else(|| GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap());
    let (nodes, edges, fixture_viewport) = crate::apps::jack::fixture_to_workflow(&fixture);
    let viewport = camera_override.map_or(fixture_viewport, |camera| NodeGraphViewport { x: camera.x, y: camera.y, zoom: camera.zoom });
    let hover = graph_hover(fixture_json, &cfg.active_hover_var, hover_node_id);
    let selection = graph_selection(fixture_json, &cfg.active_select_var, &cfg.selected_node_ids);
    semio_framework_plugin::build_node_graph_scene(
        surface_id,
        TRINITY_REWRITE_PLAY_CONTROLLER_ID,
        semio_framework_plugin::NodeGraphScene { hover, selection, lod_json: rewrite_lod_json_for_window(cfg, window_id), editable: editable.then_some(true), ..semio_framework_plugin::NodeGraphScene::base(nodes, edges, viewport) },
    )
}

pub(crate) fn render_fixture_graph(surface_id: &str, window_id: &str, fixture_json: &str, cfg: &RewriteConfig, editable: bool, camera_override: Option<&Camera>) -> UiNode {
    render_rule_graph(surface_id, window_id, fixture_json, cfg, "", editable, camera_override)
}
//#endregion 🔖️Render

//#region 🔖️TrinityRewriteCommand
/// 🎯️ `TrinityRewritePlayApp::Command` — the SOLE dispatch surface for rewrite's own behavior. Kept
/// hand-rolled (see `jack::TrinityJackCommand`'s doc comment for the rationale). `NodeGraphEdit` keeps
/// its JSON-array `operations` shape (rather than a typed sub-enum) — the same
/// `{"operation":"setFixture"|"deleteSelection", ...}` payload `apply_rewrite_node_graph_edit_operations`
/// already parses, carried as an opaque string field.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
pub enum TrinityRewriteCommand {
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

    // 👁️ Config-only — was ephemeral `RewritePlayRuntime` state, now emits `config_operations`.
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String>, surface_id: Option<String> },
    #[dsl(key = "node-graph-hover")]
    NodeGraphHover { surface_id: Option<String>, node_id: Option<String> },
    #[dsl(key = "set-viewport")]
    SetViewport { surface_id: Option<String>, viewport_json: String },
    #[dsl(key = "graph-pointer-down")]
    GraphPointerDown { node_id: Option<String> },
    #[dsl(key = "text-select")]
    TextSelect { var: Option<String>, start: Option<u64> },
    #[dsl(key = "text-hover")]
    TextHover { var: Option<String>, offset: Option<u64> },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "set-lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "set-locale")]
    SetLocale { value: String },
}
//#endregion 🔖️TrinityRewriteCommand

//#region 🔖️TrinityRewritePlayApp
/// ♻️ Trinity Rewrite play app — a parametric-rewrite editor over a {@link RewriteRuleState} projection.
#[derive(Default)]
pub struct TrinityRewritePlayApp;

impl DocumentApp for TrinityRewritePlayApp {
    type Projection = RewriteRuleState;
    type Operation = RewriteRuleOperation;
    type Config = RewriteConfig;
    type ConfigOperation = RewriteConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = TrinityRewriteCommand;

    const APP_ID: &'static str = TRINITY_REWRITE_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;

    fn initial_projection() -> RewriteRuleState {
        default_rule_state()
    }

    fn initial_config() -> RewriteConfig {
        let projection = self.initial_projection();
        RewriteConfig { before_pane_camera: seed_before_pane_camera(&projection), ..RewriteConfig::default() }
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(rewrite_io())
    }

    fn whole_document_operation(projection: RewriteRuleState) -> Option<RewriteRuleOperation> {
        Some(RewriteRuleOperation::SetState { state: projection })
    }

    /// 🔌️ `"graph:in"` loads an incoming `trinity.graph` pack as this rule's `before_fixture_json`
    /// working graph. `"document:in"` reimplements the default `DocumentApp::import_media` body for
    /// the rule document itself.
    fn import_media(port: &str, media: &Media, doc: &DocumentView<'_, RewriteRuleState>) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation, Self::DraftOperation>, MediaError> {
        match port {
            "graph:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "graph:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture = <GraphFixture as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let fixture_json = fixture.to_json().map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let mut next = doc.projection.clone();
                next.before_fixture_json = fixture_json;
                Ok(Emit::operations(vec![RewriteRuleOperation::SetState { state: next }]))
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <RewriteRuleState as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match Self::whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🔌️ `"graph:out"` re-emits the rule-applied result graph, alongside the implicit `"document:out"`.
    fn export_media(port: &str, doc: &DocumentView<'_, RewriteRuleState>) -> Result<Media, MediaError> {
        match port {
            "graph:out" => {
                let fixture_json = after_fixture_json(doc.projection);
                let fixture = GraphFixture::from_json(&fixture_json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let bytes = DocumentPack::encode_pack(&fixture);
                Ok(Media { media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity }, payload: MediaPayload::Structured { schema: crate::artifacts::jack::TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `TrinityRewriteCommand` variant back to the action id it was declared under in
    /// `create_rewrite_app`.
    fn command_id(command: &TrinityRewriteCommand) -> &'static str {
        match command {
            TrinityRewriteCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            TrinityRewriteCommand::SetLhsJson { .. } => "setLhsJson",
            TrinityRewriteCommand::SetRhsJson { .. } => "setRhsJson",
            TrinityRewriteCommand::SetParameter { .. } => "setParameter",
            TrinityRewriteCommand::AddRuleClause { .. } => "addRuleClause",
            TrinityRewriteCommand::ResetRule => "resetRule",
            TrinityRewriteCommand::PatchNodes { .. } => "patchNodes",
            TrinityRewriteCommand::SetSelection { .. } => "setSelection",
            TrinityRewriteCommand::NodeGraphHover { .. } => "nodeGraphHover",
            TrinityRewriteCommand::SetViewport { .. } => "setViewport",
            TrinityRewriteCommand::GraphPointerDown { .. } => "graphPointerDown",
            TrinityRewriteCommand::TextSelect { .. } => "textSelect",
            TrinityRewriteCommand::TextHover { .. } => "textHover",
            TrinityRewriteCommand::Reorganize => "reorganize",
            TrinityRewriteCommand::SetLodMode { .. } => "setLodMode",
            TrinityRewriteCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(command: &TrinityRewriteCommand, doc: &DocumentView<'_, RewriteRuleState>, cfg: &ConfigView<'_, RewriteConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<RewriteRuleOperation, RewriteConfigOperation, Self::DraftOperation>, Fault> {
        let state = doc.projection;
        let config = cfg.projection;
        match command {
            TrinityRewriteCommand::NodeGraphEdit { surface_id, operations_json } => crate::apps::rewrite::commands::rule::node_graph_edit(state, &config.selected_node_ids, surface_id, operations_json),
            TrinityRewriteCommand::SetLhsJson { value } => crate::apps::rewrite::commands::rule::set_lhs_json(state, value),
            TrinityRewriteCommand::SetRhsJson { value } => crate::apps::rewrite::commands::rule::set_rhs_json(state, value),
            TrinityRewriteCommand::SetParameter { name, value } => crate::apps::rewrite::commands::rule::set_parameter(state, name, value),
            TrinityRewriteCommand::AddRuleClause { kind } => crate::apps::rewrite::commands::rule::add_rule_clause_command(state, kind),
            TrinityRewriteCommand::ResetRule => crate::apps::rewrite::commands::rule::reset_rule(state),
            TrinityRewriteCommand::PatchNodes { node_ids, field, value } => crate::apps::rewrite::commands::rule::patch_nodes(state, node_ids, field, value),
            TrinityRewriteCommand::SetSelection { ids, surface_id } => crate::apps::rewrite::commands::view::set_selection(state, ids, surface_id, config.select_epoch),
            TrinityRewriteCommand::NodeGraphHover { surface_id, node_id } => crate::apps::rewrite::commands::view::node_graph_hover(state, surface_id, node_id, config.hover_epoch),
            TrinityRewriteCommand::SetViewport { surface_id, viewport_json } => crate::apps::rewrite::commands::view::set_viewport(surface_id, viewport_json),
            TrinityRewriteCommand::GraphPointerDown { node_id } => crate::apps::rewrite::commands::view::graph_pointer_down(node_id),
            TrinityRewriteCommand::TextSelect { var, start } => crate::apps::rewrite::commands::view::text_select(state, var, start, config.select_epoch),
            TrinityRewriteCommand::TextHover { var, offset } => crate::apps::rewrite::commands::view::text_hover(state, var, offset, config.hover_epoch),
            TrinityRewriteCommand::Reorganize => crate::apps::rewrite::commands::view::reorganize(config.reorganize_epoch),
            TrinityRewriteCommand::SetLodMode { window_id, value } => crate::apps::rewrite::commands::view::set_lod_mode(window_id, value),
            TrinityRewriteCommand::SetLocale { value } => crate::apps::rewrite::commands::view::set_locale(value),
        }
    }

    fn render(body_key: &str, doc: &DocumentView<'_, RewriteRuleState>, cfg: &ConfigView<'_, RewriteConfig>) -> UiNode {
        let state = doc.projection;
        let config = cfg.projection;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::apps::rewrite::terminology::TrinityRewriteLabels>(&config.locale);
        match body_key {
            TRINITY_REWRITE_PLAY_BODY_BEFORE => crate::apps::rewrite::windows::before::render(state, config),
            TRINITY_REWRITE_PLAY_BODY_AFTER => crate::apps::rewrite::windows::after::render(state, config),
            TRINITY_REWRITE_PLAY_BODY_LHS => crate::apps::rewrite::windows::lhs::render(state, config),
            TRINITY_REWRITE_PLAY_BODY_RHS => crate::apps::rewrite::windows::rhs::render(state, config),
            TRINITY_REWRITE_PLAY_BODY_JACK => crate::apps::rewrite::windows::jack::render(state, config),
            TRINITY_REWRITE_PLAY_BODY_PARAMETERS => crate::apps::rewrite::windows::parameters::render(state, labels),
            TRINITY_REWRITE_PLAY_BODY_DOCUMENT => crate::apps::rewrite::panels::document::render(state, config, labels),
            TRINITY_REWRITE_PLAY_BODY_CATALOGUE => crate::apps::rewrite::panels::catalogue::render(labels),
            TRINITY_REWRITE_PLAY_BODY_INSPECTION => crate::apps::rewrite::panels::inspection::render(state, config, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(_doc: &DocumentView<'_, RewriteRuleState>, cfg: &ConfigView<'_, RewriteConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let mode_for = |window_id: &str| config.lod_mode_by_window.get(window_id).map_or(TRINITY_LOD_MODE_AUTOMATIC, String::as_str);
        HashMap::from([
            (TRINITY_REWRITE_PLAY_WINDOW_BEFORE.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, mode_for(TRINITY_REWRITE_PLAY_WINDOW_BEFORE))]),
            (TRINITY_REWRITE_PLAY_WINDOW_AFTER.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_AFTER, mode_for(TRINITY_REWRITE_PLAY_WINDOW_AFTER))]),
            (TRINITY_REWRITE_PLAY_WINDOW_LHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_LHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_LHS))]),
            (TRINITY_REWRITE_PLAY_WINDOW_RHS.to_string(), vec![trinity_rewrite_lod_measure(TRINITY_REWRITE_PLAY_WINDOW_RHS, mode_for(TRINITY_REWRITE_PLAY_WINDOW_RHS))]),
        ])
    }

    fn context_menu(request: &ContextMenuRequest, _doc: &DocumentView<'_, RewriteRuleState>, cfg: &ConfigView<'_, RewriteConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = cfg.projection.locale.starts_with("de");
        let selected = cfg.projection.selected_node_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);

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
}
//#endregion 🔖️TrinityRewritePlayApp

//#region 🔖️Manifest
fn rewrite_window_stack(id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: id.into(), title: Some(title.into()), instance_id: None, template_id: None }],
    })
}

fn rewrite_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "column".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "row".into(),
                    size: Some(0.5),
                    children: vec![
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_LHS, "LHS", Some(0.34)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_RHS, "RHS", Some(0.34)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_JACK, "Jack", Some(0.32)),
                    ],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "row".into(),
                    size: Some(0.5),
                    children: vec![
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS, "Parameters", Some(0.34)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, "Before", Some(0.33)),
                        rewrite_window_stack(TRINITY_REWRITE_PLAY_WINDOW_AFTER, "After", Some(0.33)),
                    ],
                }),
            ],
        }),
    }
}

pub fn create_rewrite_app() -> App {
    App::from_builder(
        App::builder(TRINITY_REWRITE_PLAY_APP_ID, LocalizedLabel::native("Trinity Rewrite", "Trinity Rewrite")).document(["semio", "trinity", "rewrite"])
            .icon_id("trinity-rewrite")
            .mode("explore", LocalizedLabel::native("Explore", "Erkunden"), "focus")
            .default_mode_id("explore")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_BEFORE, LocalizedLabel::native("Before", "Vorher"), TRINITY_REWRITE_PLAY_BODY_BEFORE, SurfaceKind::NodeGraph, "git-branch")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_AFTER, LocalizedLabel::native("After", "Nachher"), TRINITY_REWRITE_PLAY_BODY_AFTER, SurfaceKind::NodeGraph, "arrow-right")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_LHS, LocalizedLabel::native("LHS", "LHS"), TRINITY_REWRITE_PLAY_BODY_LHS, SurfaceKind::NodeGraph, "trinity-lhs")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_RHS, LocalizedLabel::native("RHS", "RHS"), TRINITY_REWRITE_PLAY_BODY_RHS, SurfaceKind::NodeGraph, "trinity-rhs")
            .window_kind(TRINITY_REWRITE_PLAY_WINDOW_JACK, LocalizedLabel::native("Jack", "Jack"), TRINITY_REWRITE_PLAY_BODY_JACK, SurfaceKind::TextEditor, "document-jack")
            .window_kind(
                TRINITY_REWRITE_PLAY_WINDOW_PARAMETERS,
                LocalizedLabel::native("Parameters", "Parameter"),
                TRINITY_REWRITE_PLAY_BODY_PARAMETERS,
                SurfaceKind::Canvas2d,
                "settings-2",
            )
            .default_layout(rewrite_layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                TRINITY_REWRITE_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                TRINITY_REWRITE_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                TRINITY_REWRITE_PLAY_BODY_INSPECTION,
            )
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("addRuleClause", LocalizedLabel::native("Add Rule Clause", "Regelklausel hinzufügen"), ActionKind::Operation).with_category("create"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("resetRule", LocalizedLabel::native("Reset Rule", "Regel zurücksetzen"), ActionKind::Operation).with_category("history"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("setParameter", LocalizedLabel::native("Set Parameter", "Parameter festlegen"), ActionKind::Operation).with_category("settings"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("patchNodes", LocalizedLabel::native("Patch Nodes", "Knoten aktualisieren"), ActionKind::Operation).with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Operation).with_category("transform"))
            // 🛠️ Dev-only raw rule editors — kept out of the command palette.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new_catalog("setLhsJson", LocalizedLabel::native("Set LHS Json", "LHS-JSON festlegen"), ActionKind::Operation).with_category("tools") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new_catalog("setRhsJson", LocalizedLabel::native("Set RHS Json", "RHS-JSON festlegen"), ActionKind::Operation).with_category("tools") })
            // 👁️ Ephemeral view state — selection, hover, text cursor, recompute/layout, LOD.
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View).with_category("selection"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("nodeGraphHover", LocalizedLabel::native("Hover Graph Node", "Graph-Knoten hovern"), ActionKind::View).with_category("hand"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("setViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansicht festlegen"), ActionKind::View).with_category("view"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"), ActionKind::View).with_category("hand"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("textSelect", LocalizedLabel::native("Select Text", "Text auswählen"), ActionKind::View).with_category("selection"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("textHover", LocalizedLabel::native("Hover Text", "Text hovern"), ActionKind::View).with_category("hand"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::View).with_category("view"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View).with_category("mode"))
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
            .io(rewrite_io()),
    )
    .example("label-core", LocalizedLabel::native("Label Core", "Label-Kern"), default_rule_state().print_dsl(), "file-text")
    .workflow("trinity-rewrite", "Trinity Rewrite", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::rewrite::engine::Rhs;
    use protocol::{OpBinary, OpText};
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, Locale, Terminology, ViewState};

    fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
        testkit::meta(actor)
    }

    /// 🎫️ Permanent wire guard (TEMPLATE.md §7): every `TrinityRewriteCommand` variant round-trips
    /// through both its binary (`OpBinary`) and text (`OpText`) codecs.
    #[test]
    fn trinity_rewrite_command_text_and_binary_round_trip() {
        let commands = vec![
            TrinityRewriteCommand::NodeGraphEdit { surface_id: "trinity.rewrite.before".into(), operations_json: "[]".into() },
            TrinityRewriteCommand::SetLhsJson { value: "{}".into() },
            TrinityRewriteCommand::SetRhsJson { value: "{}".into() },
            TrinityRewriteCommand::SetParameter { name: "label".into(), value: "hi".into() },
            TrinityRewriteCommand::AddRuleClause { kind: "where".into() },
            TrinityRewriteCommand::ResetRule,
            TrinityRewriteCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
            TrinityRewriteCommand::SetSelection { ids: vec!["n1".into()], surface_id: Some("trinity.rewrite.before".into()) },
            TrinityRewriteCommand::NodeGraphHover { surface_id: Some("trinity.rewrite.before".into()), node_id: Some("n1".into()) },
            TrinityRewriteCommand::SetViewport { surface_id: Some("trinity.rewrite.before".into()), viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
            TrinityRewriteCommand::GraphPointerDown { node_id: Some("n1".into()) },
            TrinityRewriteCommand::TextSelect { var: Some("a".into()), start: None },
            TrinityRewriteCommand::TextHover { var: None, offset: Some(3) },
            TrinityRewriteCommand::Reorganize,
            TrinityRewriteCommand::SetLodMode { window_id: "trinity-rewrite-before".into(), value: "compact".into() },
            TrinityRewriteCommand::SetLocale { value: "de-DE".into() },
        ];
        for command in commands {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(TrinityRewriteCommand::decode_op(&bytes).expect("decode"), command);
            let text = command.print_op();
            assert_eq!(TrinityRewriteCommand::parse_op(&text).expect("parse"), command);
        }
    }

    fn new_app() -> VcsDocumentApp<TrinityRewritePlayApp> {
        testkit::new_app::<TrinityRewritePlayApp>()
    }

    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        let mut app = testkit::new_app_with_registry::<TrinityRewritePlayApp>(create_rewrite_app);
        app.dispatch_typed(TrinityRewriteCommand::SetSelection { ids: vec!["n1".into(), "n2".into()], surface_id: None }, &meta("local")).expect("select");
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("nodeGraphEdit");
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive delete-selection must be last: {menu:?}");
    }

    #[test]
    fn renders_before_and_after_graphs() {
        let mut app = new_app();
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        let after = app.render(TRINITY_REWRITE_PLAY_BODY_AFTER, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("node-graph"));
        assert!(serde_json::to_string(&after).unwrap().contains("node-graph"));
    }

    #[test]
    fn set_viewport_writes_before_pane_config_camera_without_document_operations() {
        let mut app = new_app();
        let before_state = app.projection().unwrap();
        let result = app.dispatch_typed(TrinityRewriteCommand::SetViewport { surface_id: Some(TRINITY_REWRITE_PLAY_SURFACE_BEFORE.into()), viewport_json: serde_json::json!({ "x": 10.0, "y": 20.0, "zoom": 2.5 }).to_string() }, &meta("local")).expect("viewport");
        assert!(result.operations.is_empty(), "camera is a config-only command, no document operations");
        assert_eq!(app.projection().unwrap(), before_state, "document is untouched by a viewport pan");
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("2.5"), "render reads the live config camera");
    }

    #[test]
    fn compiles_jack_query_from_rule() {
        let query = compiled_jack_query(&default_rule_state());
        assert!(query.contains("MATCH"));
        assert!(query.contains("SET"));
    }

    #[test]
    fn apply_rewrite_changes_after_fixture() {
        let state = default_rule_state();
        assert_ne!(state.before_fixture_json, after_fixture_json(&state));
    }

    #[test]
    fn renders_lhs_rhs_graphs() {
        let mut app = new_app();
        let lhs_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_LHS, None, &ViewState::default()).expect("render")).unwrap();
        let rhs_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_RHS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(lhs_json.contains("node-graph"));
        assert!(rhs_json.contains("node-graph"));
        assert!(lhs_json.contains("\"editable\":true"));
        assert!(rhs_json.contains("\"editable\":true"));
    }

    #[test]
    fn set_parameter_emits_one_op_and_is_undoable() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityRewriteCommand::SetParameter { name: "label".into(), value: "changed".into() }, &meta("local")).expect("set parameter");
        assert_eq!(result.operations.len(), 1, "a parameter edit is a single SetState operation");
        assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("changed".into())));
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.projection().unwrap().parameter_bindings.get("label").cloned(), Some(PropertyValue::String("nakagin-core".into())));
    }

    #[test]
    fn add_and_delete_rhs_set_clause() {
        let mut app = new_app();
        app.dispatch_typed(TrinityRewriteCommand::AddRuleClause { kind: "set".into() }, &meta("local")).expect("add clause");
        let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 2);
        app.dispatch_typed(TrinityRewriteCommand::SetSelection { ids: vec!["rhs-set-1".into()], surface_id: Some(TRINITY_REWRITE_PLAY_SURFACE_RHS.into()) }, &meta("local")).expect("select");
        let result =
            app.dispatch_typed(TrinityRewriteCommand::NodeGraphEdit { surface_id: TRINITY_REWRITE_PLAY_SURFACE_RHS.into(), operations_json: serde_json::json!([{ "operation": "deleteSelection" }]).to_string() }, &meta("local")).expect("delete selection");
        assert!(!result.operations.is_empty());
        let rhs: Rhs = serde_json::from_str(&app.projection().unwrap().rhs_json).unwrap();
        assert_eq!(rhs.set.len(), 1);
    }

    #[test]
    fn jack_view_has_occurrences_after_select() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityRewriteCommand::TextSelect { var: Some("a".into()), start: None }, &meta("local")).expect("text select");
        assert!(result.operations.is_empty(), "text selection is a config-only command, no document operations");
        let node = app.render(TRINITY_REWRITE_PLAY_BODY_JACK, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("occurrencesJson"));
    }

    #[test]
    fn graph_scenes_have_lod_json() {
        let mut app = new_app();
        let before = app.render(TRINITY_REWRITE_PLAY_BODY_BEFORE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&before).unwrap().contains("lodJson"));
    }

    #[test]
    fn app_definition_declares_reorganize_and_history_actions() {
        let definition = create_rewrite_app().definition;
        let action_ids: Vec<&str> = definition.actions.iter().map(|action| action.id.as_str()).collect();
        assert!(action_ids.contains(&"undo"));
        assert!(action_ids.contains(&"reorganize"));
    }

    #[test]
    fn trinity_rewrite_labels_resolve_native_by_default() {
        let mut app = new_app();
        let json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(json.contains("\"Pieces\""));
        assert!(!json.contains("Stücke"));
    }

    #[test]
    fn trinity_rewrite_labels_translate_panels_in_german() {
        let mut app = new_app();
        app.dispatch_typed(TrinityRewriteCommand::SetLocale { value: "de-DE".into() }, &meta("local")).expect("set locale");
        let document_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render")).unwrap();
        assert!(document_json.contains("Stücke"));
        assert!(!document_json.contains("\"Pieces\""));
        let catalogue_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render")).unwrap();
        assert!(catalogue_json.contains("Katalog"));
        assert!(catalogue_json.contains("Zu LHS hinzufügen"));
        assert!(catalogue_json.contains("Zu RHS hinzufügen"));
        let parameters_json = serde_json::to_string(&app.render(TRINITY_REWRITE_PLAY_BODY_PARAMETERS, None, &ViewState::default()).expect("render")).unwrap();
        assert!(parameters_json.contains("\"Parameter\""));
        let definition = create_rewrite_app().definition;
        let reset_rule = definition.actions.iter().find(|action| action.id == "resetRule").expect("resetRule action");
        assert_eq!(reset_rule.label.resolve(Terminology::Native, Locale::De), "Regel zurücksetzen");
    }

    #[test]
    fn set_lhs_json_undo_redo_round_trip() {
        let mut app = new_app();
        let original = app.projection().unwrap().lhs_json;
        let next_lhs = r#"{"pattern":{"leftVar":"x","leftKind":"Piece","edgeVar":"r","edgeKind":"Connection","rightVar":"y","rightKind":"Piece"}}"#;
        app.dispatch_typed(TrinityRewriteCommand::SetLhsJson { value: next_lhs.into() }, &meta("local")).expect("set lhs");
        assert_eq!(app.projection().unwrap().lhs_json, next_lhs);
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.projection().unwrap().lhs_json, original);
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(app.projection().unwrap().lhs_json, next_lhs);
    }

    #[test]
    fn export_media_graph_out_reflects_rule_applied_fixture() {
        let mut app = new_app();
        let graph_out = app.export_media("graph:out").expect("graph:out export");
        let MediaPayload::Structured { json, .. } = graph_out.payload else { panic!("structured payload") };
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64");
        let fixture = <GraphFixture as DocumentPack>::decode_pack(&bytes).expect("decode pack");
        let expected = GraphFixture::from_json(&after_fixture_json(&app.projection().unwrap())).unwrap();
        assert_eq!(fixture.nodes.len(), expected.nodes.len());
    }

    #[test]
    fn rewrite_io_declares_graph_in_and_graph_out_ports() {
        let io = rewrite_io();
        assert_eq!(io.document_schema, REWRITE_RULE_SCHEMA);
        let graph_in = io.ports.iter().find(|port| port.id == "graph:in").expect("graph:in declared");
        assert_eq!(graph_in.kind_id.as_deref(), Some("graph.trinity"));
        assert_eq!(graph_in.multiplicity, semio_framework_plugin::PortMultiplicity::One);
        let graph_out = io.ports.iter().find(|port| port.id == "graph:out").expect("graph:out declared");
        assert_eq!(graph_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }
}
//#endregion 🧪️Tests

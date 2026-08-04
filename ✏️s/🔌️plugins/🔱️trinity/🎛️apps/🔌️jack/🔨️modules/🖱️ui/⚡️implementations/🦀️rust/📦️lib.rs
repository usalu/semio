//! 🔱️ Trinity Jack plugin — jack query play app bundled as a hot-swappable WASM plugin.
//!
//! 📌️ B1: the pure-trait migration — `TrinityJackPlayApp` is a unit struct; every former
//! `TrinityJackRuntime` field (selection, camera, query draft, LOD, …) now lives in
//! `trinity_jack_engine::JackConfig`, written via `trinity_jack_op::JackConfigOperation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `trinity_jack_protocol::TrinityJackCommand` channel via `DocumentApp::handle`. Mirrors
//! `shooting_ui::ShootingPlayApp` (the B1 pilot) — see its doc comments for the full rationale.

use semio_framework_plugin::{
        app_labels, build_node_graph_scene, build_table_scene, build_text_editor_scene, text_identifier_occurrences_json, tree_item, tree_item_with_action, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_text,
    ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDescriptor, ActionKind, App, AppActionRegistry, AppLabels, ArtifactKindSpec, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView, Emit, Label,
    Locale, LocalizedLabel, MeasureSelectItem, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord, NodeGraphScene, NodeGraphViewport, PanelGroup, PanelTreeBuilder,
    SurfaceKind, TableScene, Terminology, TextEditorScene, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, UiTreeItemNode, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode,
    WindowLayoutWindowNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::{DocumentDsl, DocumentPack};
use trinity_jack::{complete, execute, format as jack_format, lint, parse, semantic_tokens, QueryResult, QueryResultKind};
use trinity_jack_engine::{jack_io, JackConfig, JackEditorSelection};
use trinity_jack_op::JackConfigOperation;
use trinity_jack_protocol::TrinityJackCommand;
use trinity_ram::{Camera, Graph, GraphFixture, Node, PortDirection, PropertyValue, TrinityGraphOperation, TRINITY_GRAPH_SCHEMA};

//#region 🔖️Constants
const TRINITY_JACK_PLAY_APP_ID: &str = "trinity-jack-play";
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
const TRINITY_JACK_PLAY_WINDOW_GRAPH: &str = "trinity-jack-graph";
const TRINITY_JACK_PLAY_WINDOW_EDITOR: &str = "trinity-jack-editor";
const TRINITY_JACK_PLAY_WINDOW_RESULTS: &str = "trinity-jack-results";

const NAKAGIN_FIXTURE_DSL: &str = include_str!("../../../../../../../../../✏️s/🔌️plugins/🔱️trinity/📚️examples/🔱️nakagin-capsule-tower.trinity");
const BRANCH_FIXTURE_DSL: &str = include_str!("../../../../../../../../../✏️s/🔌️plugins/🔱️trinity/📚️examples/🔱️branch-chain.trinity");

const TRINITY_JACK_DEFAULT_QUERY: &str = "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";

const TRINITY_LOD_MODE_AUTOMATIC: &str = "automatic";
//#endregion 🔖️Constants

//#region 🔖️Locale

//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
/// 📦️ The default trinity graph fixture (Nakagin capsule tower) — the initial document projection.
fn default_fixture() -> GraphFixture {
    GraphFixture::parse_dsl(NAKAGIN_FIXTURE_DSL).unwrap_or_else(|_| trinity_ram::empty_trinity_graph_fixture())
}

/// 🌱️ Seeds the initial config with the default query and its result table so the Results window is
/// populated on load — was `seeded_jack_runtime()`.
fn seeded_jack_config(fixture: &GraphFixture) -> JackConfig {
    let (result_json, _) = run_jack_query(fixture, TRINITY_JACK_DEFAULT_QUERY);
    JackConfig { camera: fixture.camera.clone(), active_fixture_id: "nakagin".into(), jack_query: TRINITY_JACK_DEFAULT_QUERY.into(), jack_result_json: result_json, ..JackConfig::default() }
}

/// 🔎️ Runs a jack query against the fixture, returning `(result_json, forward operations)`; a parse/execute
/// failure yields an error result and no operations (no document mutation).
fn run_jack_query(fixture: &GraphFixture, query: &str) -> (String, Vec<TrinityGraphOperation>) {
    let graph = match Graph::from_fixture(fixture.clone()) {
        Ok(graph) => graph,
        Err(error) => return (error_result_json(&error.to_string()), Vec::new()),
    };
    let parsed = match parse(query) {
        Ok(parsed) => parsed,
        Err(error) => return (error_result_json(&error), Vec::new()),
    };
    match execute(&graph, &parsed) {
        Ok((result, operations)) => (serde_json::to_string(&result).unwrap_or_default(), operations),
        Err(error) => (error_result_json(&error), Vec::new()),
    }
}

fn error_result_json(message: &str) -> String {
    json!({ "error": message }).to_string()
}

fn jack_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(TRINITY_JACK_PLAY_CONTROLLER_ID).action(action, args)
}

fn graph_from_fixture_or_default(fixture: &GraphFixture) -> Graph {
    Graph::from_fixture(fixture.clone()).unwrap_or_else(|_| Graph::from_fixture(default_fixture()).expect("nakagin graph"))
}

/// 🧮️ Clones the fixture and recomputes its derived (flat-position) node properties for the inspector.
fn fixture_with_derived(fixture: &GraphFixture) -> Option<GraphFixture> {
    let mut graph = Graph::from_fixture(fixture.clone()).ok()?;
    graph.recompute_derived();
    Some(graph.to_fixture())
}

fn fixture_dsl_for_preset(preset_id: &str) -> Option<&'static str> {
    match preset_id {
        "nakagin" | "nakagin-capsule-tower" => Some(NAKAGIN_FIXTURE_DSL),
        "branch-chain" => Some(BRANCH_FIXTURE_DSL),
        _ => None,
    }
}

fn preset_query(preset_id: &str) -> &'static str {
    match preset_id {
        "branch-chain" => "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
        _ => TRINITY_JACK_DEFAULT_QUERY,
    }
}

fn property_value_to_string(value: &PropertyValue) -> String {
    match value {
        PropertyValue::String(text) => text.clone(),
        PropertyValue::Number(number) => number.to_string(),
        PropertyValue::Bool(flag) => flag.to_string(),
        PropertyValue::Null => "null".into(),
        PropertyValue::Array(items) => serde_json::to_string(items).unwrap_or_else(|_| "[]".into()),
        PropertyValue::Object(map) => serde_json::to_string(map).unwrap_or_else(|_| "{}".into()),
    }
}

/// 🧲️ Re-runs force layout on the fixture, returning the repositioned fixture (or `None` if empty).
fn force_layout_fixture(fixture: &GraphFixture) -> Option<GraphFixture> {
    let mut fixture = fixture.clone();
    if fixture.nodes.is_empty() {
        return None;
    }
    use mathematical_geometry::Vec2;
    use mathematical_graph_drawing::force::{run_force_layout, ForceLayoutOptions};
    let mut positions: Vec<Vec2> = fixture.nodes.iter().map(|node| Vec2::new(node.x, node.y)).collect();
    let radii: Vec<f64> = fixture.nodes.iter().map(|node| (node.width.max(48.0) + node.height.max(24.0)) * 0.25).collect();
    let id_to_index: HashMap<String, usize> = fixture.nodes.iter().enumerate().map(|(index, node)| (node.id.clone(), index)).collect();
    let mut edge_pairs = Vec::new();
    for edge in &fixture.edges {
        let (source_node, _) = split_endpoint(&edge.source);
        let (target_node, _) = split_endpoint(&edge.target);
        if let (Some(a), Some(b)) = (id_to_index.get(&source_node), id_to_index.get(&target_node)) {
            edge_pairs.push((*a, *b));
        }
    }
    let pin = vec![None; positions.len()];
    run_force_layout(&mut positions, &radii, &edge_pairs, &pin, &ForceLayoutOptions { iterations: 120, ..ForceLayoutOptions::default() });
    for (index, node) in fixture.nodes.iter_mut().enumerate() {
        node.x = positions[index].x;
        node.y = positions[index].y;
    }
    Some(fixture)
}

/// 🧭️ Emits a `Reposition` operation for every node whose position differs between `before` and `after`.
fn reposition_operations(before: &GraphFixture, after: &GraphFixture) -> Vec<TrinityGraphOperation> {
    after
        .nodes
        .iter()
        .filter_map(|node| {
            let prev = before.nodes.iter().find(|entry| entry.id == node.id)?;
            if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
                Some(TrinityGraphOperation::Reposition { id: node.id.clone(), x: node.x, y: node.y })
            } else {
                None
            }
        })
        .collect()
}

fn trinity_lod_tier_rows() -> Vec<Value> {
    serde_json::from_str(&trinity_rewrite::trinity_lod_scale_json()).unwrap_or_default()
}

fn trinity_lod_measure(window_id: &str, current_mode: &str) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: TRINITY_LOD_MODE_AUTOMATIC.into(), value: TRINITY_LOD_MODE_AUTOMATIC.into(), label: "Automatic".into() }];
    items.extend(trinity_lod_tier_rows().into_iter().filter_map(|row| {
        let id = row.get("id")?.as_str()?.to_string();
        let name = row.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: format!("{window_id}-lod"), label: Some("LOD".into()), value: current_mode.into(), items, on_change: jack_action("setLodMode", Some(json!({ "windowId": window_id }))) }
}

fn trinity_lod_json_for_window(cfg: &JackConfig, window_id: &str) -> Option<String> {
    let mode = cfg.lod_mode_by_window.get(window_id).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
    if mode == TRINITY_LOD_MODE_AUTOMATIC {
        Some(json!({ "automatic": true }).to_string())
    } else {
        Some(json!({ "automatic": false, "forcedLabel": mode }).to_string())
    }
}
/// 🩹️ Delegates to `trinity_ram::parse_port_key` (the one place the `nodeId@portId` convention is
/// owned) instead of hand-rolling a second splitter here.
fn split_endpoint(endpoint: &str) -> (String, String) {
    trinity_ram::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), "in".into()), |(n, p)| (n.to_string(), p.to_string()))
}

fn fixture_to_workflow(fixture: &GraphFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>, NodeGraphViewport) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture.nodes.iter().map(|node| node_to_workflow_record(node)).collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
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
        inputs: node.ports.iter().filter(|port| port.direction == PortDirection::In).map(|port| NodeGraphPortRecord { id: trinity_ram::port_key(&node.id, &port.id), label: Some(port.id.clone()), ..Default::default() }).collect(),
        outputs: node.ports.iter().filter(|port| port.direction == PortDirection::Out).map(|port| NodeGraphPortRecord { id: trinity_ram::port_key(&node.id, &port.id), label: Some(port.id.clone()), ..Default::default() }).collect(),
        ..Default::default()
    }
}

fn result_to_table(result_json: &str) -> (String, String) {
    let parsed: QueryResult = serde_json::from_str(result_json).unwrap_or(QueryResult::table(vec![], vec![]));
    let columns: Vec<Value> = parsed.columns.iter().map(|column| json!({ "id": column, "label": column })).collect();
    let rows: Vec<Value> = parsed
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let mut record = serde_json::Map::new();
            record.insert("index".into(), json!(index + 1));
            for (column, value) in parsed.columns.iter().zip(row.iter()) {
                record.insert(column.clone(), json!(property_value_to_string(value)));
            }
            Value::Object(record)
        })
        .collect();
    (serde_json::to_string(&columns).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()))
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the Jack query app; one field per label makes every locale×terminology
    /// combination compile-checked. No distinct reuse-terminology concept for this app, so reuse repeats native.
    struct TrinityJackLabels {
        pieces: native_en "Pieces", native_de "Stücke", reuse_en "Pieces", reuse_de "Stücke";
        connections: native_en "Connections", native_de "Verbindungen", reuse_en "Connections", reuse_de "Verbindungen";
        fixtures: native_en "Fixtures", native_de "Fixturen", reuse_en "Fixtures", reuse_de "Fixturen";
        example_queries: native_en "Example queries", native_de "Beispielabfragen", reuse_en "Example queries", reuse_de "Beispielabfragen";
        manifest_kinds: native_en "Manifest kinds", native_de "Manifestarten", reuse_en "Manifest kinds", reuse_de "Manifestarten";
        piece: native_en "Piece", native_de "Stück", reuse_en "Piece", reuse_de "Stück";
        connection: native_en "Connection", native_de "Verbindung", reuse_en "Connection", reuse_de "Verbindung";
        connector: native_en "Connector", native_de "Verbinder", reuse_en "Connector", reuse_de "Verbinder";
        geometry: native_en "Geometry", native_de "Geometrie", reuse_en "Geometry", reuse_de "Geometrie";
        identity: native_en "Identity", native_de "Identität", reuse_en "Identity", reuse_de "Identität";
        history: native_en "History", native_de "Verlauf", reuse_en "History", reuse_de "Verlauf";
        query: native_en "Query", native_de "Abfrage", reuse_en "Query", reuse_de "Abfrage";
        window_graph: native_en "Nakagin Graph", native_de "Nakagin-Graph", reuse_en "Nakagin Graph", reuse_de "Nakagin-Graph";
        window_editor: native_en "Jack Query", native_de "Jack-Abfrage", reuse_en "Jack Query", reuse_de "Jack-Abfrage";
        window_results: native_en "Results", native_de "Ergebnisse", reuse_en "Results", reuse_de "Ergebnisse";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn flat_position_uv(node: &Node) -> (String, String) {
    let Some(flat) = node.properties.get("flatPosition").and_then(PropertyValue::as_object) else {
        return (String::new(), String::new());
    };
    let format_axis = |axis: &str| flat.get(axis).and_then(PropertyValue::as_f64).map(|value| format!("{value:.2}")).unwrap_or_default();
    (format_axis("u"), format_axis("v"))
}

fn build_document_tree(fixture: &GraphFixture, cfg: &JackConfig, labels: &TrinityJackLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("trinity-document");
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(builder.item_id("node", &node.id), Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }), Some(node.kind.clone()), jack_action("setSelection", Some(json!({ "ids": [node.id] }))))
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture.edges.iter().map(|edge| tree_item(builder.item_id("edge", &edge.id), Label::data(format!("{} → {}", edge.source, edge.target)))).collect();
    let selected = cfg.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
    builder
        .section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)
        .section("trinity-document.edges", Some(labels.connections.into()), false, edge_items)
        .selected(selected)
        .selection_change(jack_action("setSelection", Some(json!({ "ids": [] }))))
        .build()
}

fn build_catalogue_tree(cfg: &JackConfig, labels: &TrinityJackLabels) -> UiNode {
    let fixtures = [("nakagin", "Nakagin — Table"), ("branch-chain", "Branch — Graph")];
    let examples = [
        ("where-or", "Where Or", "MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name"),
        ("return-graph", "Return Graph", "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b"),
        ("set-label", "Set Label", "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'"),
        ("set-position", "Set Position", "MATCH (a:Piece) WHERE a.name = 'b' SET a.x = 300, a.y = 120"),
        ("create-node", "Create Node", "CREATE (n:Piece)"),
        ("create-edge", "Create Edge", "MATCH (a:Piece), (b:Piece) WHERE a.name = 'b' AND b.name != 'b' CREATE (a)-[:Connection]->(b)"),
        ("delete-leaf", "Delete Leaf", "MATCH (n:Piece) WHERE n.name = 'b' DELETE n"),
        ("merge-edge", "Merge Edge", "MERGE (x:Piece)-[:Connection]->(y:Piece)"),
    ];
    let builder = PanelTreeBuilder::new("trinity-jack-catalogue");
    let fixture_items: Vec<UiTreeItemNode> =
        fixtures.iter().map(|(id, label)| tree_item_with_action(builder.item_id("fixture", id), Label::data(*label), Some(preset_query(id).into()), jack_action("setActiveExample", Some(json!({ "exampleId": id }))))).collect();
    let example_items: Vec<UiTreeItemNode> =
        examples.iter().map(|(id, label, query)| tree_item_with_action(builder.item_id("example", id), Label::data(*label), Some((*query).into()), jack_action("loadExampleQuery", Some(json!({ "query": query }))))).collect();
    let selected = if cfg.active_fixture_id.is_empty() { vec![] } else { vec![builder.item_id("fixture", &cfg.active_fixture_id)] };
    builder
        .section("trinity-jack-catalogue.fixtures", Some(labels.fixtures.into()), true, fixture_items)
        .section("trinity-jack-catalogue.examples", Some(labels.example_queries.into()), true, example_items)
        .section(
            "trinity-jack-catalogue.kinds",
            Some(labels.manifest_kinds.into()),
            false,
            vec![tree_item("trinity-jack-catalogue.piece", labels.piece), tree_item("trinity-jack-catalogue.connection", labels.connection), tree_item("trinity-jack-catalogue.connector", labels.connector)],
        )
        .selected(selected)
        .build()
}

fn build_inspector_tree(fixture: &GraphFixture, cfg: &JackConfig, term_labels: &TrinityJackLabels) -> UiNode {
    if cfg.selected_node_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(Label::data("Select one or more pieces"))],
            menu: None,
        }]);
    }
    let nodes: Vec<&Node> = cfg.selected_node_ids.iter().filter_map(|id| fixture.nodes.iter().find(|node| &node.id == id)).collect();
    if nodes.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "trinity-inspector.missing".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(Label::data("Piece not found"))],
            menu: None,
        }]);
    }
    let node_ids: Vec<String> = nodes.iter().map(|node| node.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.name.clone()).collect::<Vec<_>>());
    let kind_mixed = ui_inspector_mixed_text(&nodes.iter().map(|node| node.kind.clone()).collect::<Vec<_>>());
    let port_counts: Vec<String> = nodes.iter().map(|node| node.ports.len().to_string()).collect();
    let ports_mixed = ui_inspector_mixed_text(&port_counts);
    let derived_fixture = fixture_with_derived(fixture);
    let derived_uv = |id: &str| -> (String, String) { derived_fixture.as_ref().and_then(|fixture| fixture.nodes.iter().find(|node| node.id == id)).map(flat_position_uv).unwrap_or_default() };
    let u_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).0).collect();
    let v_values: Vec<String> = node_ids.iter().map(|id| derived_uv(id).1).collect();
    let u_mixed = ui_inspector_mixed_text(&u_values);
    let v_mixed = ui_inspector_mixed_text(&v_values);
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "trinity-inspector.geometry".into(),
            label: term_labels.geometry.into(),
            default_open: None,
            fields: vec![
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-u",
                    Label::data("Flat U"),
                    if u_mixed.placeholder.is_none() { u_values.first().cloned().unwrap_or_default() } else { u_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
                ui_inspector_readonly_field(
                    "trinity-inspector.flat-v",
                    Label::data("Flat V"),
                    if v_mixed.placeholder.is_none() { v_values.first().cloned().unwrap_or_default() } else { v_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
                ui_inspector_readonly_field(
                    "trinity-inspector.ports",
                    Label::data("Connectors"),
                    if ports_mixed.placeholder.is_none() { port_counts.first().cloned().unwrap_or_default() } else { ports_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
            ],
        },
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "trinity-inspector.identity".into(),
            label: term_labels.identity.into(),
            default_open: None,
            fields: vec![
                UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
                    id: "trinity-inspector.name".into(),
                    label: Label::data("Name"),
                    child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                        presence: UiPresence::default(),
                        id: "trinity-inspector.name.input".into(),
                        input_kind: "text".into(),
                        value: name_mixed.value,
                        placeholder: name_mixed.placeholder.map(Label::data),
                        commit: None,
                        on_change: jack_action("patchNodes", Some(json!({ "nodeIds": node_ids, "field": "name" }))),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }),
                ui_inspector_readonly_field(
                    "trinity-inspector.kind",
                    Label::data("Kind"),
                    if kind_mixed.placeholder.is_none() { nodes.first().map(|node| node.kind.clone()).unwrap_or_default() } else { kind_mixed.placeholder.unwrap_or_else(|| UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                ),
                ui_inspector_readonly_field("trinity-inspector.id", Label::data("Id"), if node_ids.len() == 1 { node_ids.first().cloned().unwrap_or_default() } else { format!("{} selected", node_ids.len()) }),
            ],
        },
    ])
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_graph(fixture: &GraphFixture, cfg: &JackConfig) -> UiNode {
    let (nodes, edges, _) = fixture_to_workflow(fixture);
    let viewport = NodeGraphViewport { x: cfg.camera.x, y: cfg.camera.y, zoom: cfg.camera.zoom };
    let selection = cfg.selected_node_ids.clone();
    build_node_graph_scene(TRINITY_JACK_PLAY_SURFACE_GRAPH, TRINITY_JACK_PLAY_CONTROLLER_ID, NodeGraphScene { selection, lod_json: trinity_lod_json_for_window(cfg, TRINITY_JACK_PLAY_WINDOW_GRAPH), ..NodeGraphScene::base(nodes, edges, viewport) })
}

fn render_editor(fixture: &GraphFixture, cfg: &JackConfig) -> UiNode {
    let query = &cfg.jack_query;
    let graph = graph_from_fixture_or_default(fixture);
    let cursor = cfg.editor_selection.as_ref().map(|selection| selection.end as usize).unwrap_or(0);
    let selection_json = cfg.editor_selection.as_ref().map(|selection| json!({ "start": selection.start, "end": selection.end }).to_string());
    build_text_editor_scene(
        TRINITY_JACK_PLAY_SURFACE_EDITOR,
        TRINITY_JACK_PLAY_CONTROLLER_ID,
        TextEditorScene {
            selection_json,
            tokens_json: serde_json::to_string(&semantic_tokens(query)).ok(),
            diagnostics_json: serde_json::to_string(&lint(&graph, query)).ok(),
            completions_json: serde_json::to_string(&complete(&graph, query, cursor)).ok(),
            occurrences_json: text_identifier_occurrences_json(query, cursor),
            ..TextEditorScene::base(query.clone(), Some("jack".into()), None)
        },
    )
}

fn render_results(cfg: &JackConfig) -> UiNode {
    let result: QueryResult = serde_json::from_str(&cfg.jack_result_json).unwrap_or(QueryResult::table(vec![], vec![]));
    if result.kind == QueryResultKind::Graph {
        if let Some(fixture) = &result.graph_fixture {
            let (nodes, edges, viewport) = fixture_to_workflow(fixture);
            return build_node_graph_scene(TRINITY_JACK_PLAY_SURFACE_RESULTS, TRINITY_JACK_PLAY_CONTROLLER_ID, NodeGraphScene::base(nodes, edges, viewport));
        }
    }
    let (columns_json, rows_json) = result_to_table(&cfg.jack_result_json);
    build_table_scene(TRINITY_JACK_PLAY_SURFACE_RESULTS, TRINITY_JACK_PLAY_CONTROLLER_ID, TableScene::base(columns_json, rows_json))
}
//#endregion 🔖️Render

//#region 🔖️TrinityJackPlayApp
/// 🔱️ Trinity Jack play app — a jack-query editor over a live {@link GraphFixture} projection. B1:
/// unit struct — every former `TrinityJackRuntime`/`self.runtime` field now lives in
/// `trinity_jack_engine::JackConfig` (see `DocumentApp::Config`), written through
/// `trinity_jack_op::JackConfigOperation`s.
#[derive(Default)]
pub struct TrinityJackPlayApp;

impl DocumentApp for TrinityJackPlayApp {
    type Projection = GraphFixture;
    type Operation = TrinityGraphOperation;
    type Config = JackConfig;
    type ConfigOperation = JackConfigOperation;
    type Command = TrinityJackCommand;

    fn app_id(&self) -> &str {
        TRINITY_JACK_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        TRINITY_GRAPH_SCHEMA
    }

    fn initial_projection(&self) -> GraphFixture {
        default_fixture()
    }

    fn initial_config(&self) -> JackConfig {
        seeded_jack_config(&self.initial_projection())
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(jack_io())
    }

    fn whole_document_operation(&self, projection: GraphFixture) -> Option<TrinityGraphOperation> {
        Some(TrinityGraphOperation::SetFixture { fixture: projection })
    }

    /// 🔌️ `"graph:out"` fans the live query-graph projection out to other graph-consuming workflow
    /// nodes, in addition to the implicit `"document:out"` — both encode the same `GraphFixture` pack,
    /// so this reimplements (rather than delegates to, no supertrait call exists in Rust) the default
    /// `DocumentApp::export_media` body for `"document:out"` alongside the new port.
    fn export_media(&self, port: &str, doc: &DocumentView<'_, GraphFixture>) -> Result<Media, MediaError> {
        match port {
            "graph:out" | "document:out" => {
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Trinity }, payload: MediaPayload::Structured { schema: TRINITY_GRAPH_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ Maps each `TrinityJackCommand` variant back to the action id it was declared under in
    /// `create_trinity_jack_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(&self, command: &TrinityJackCommand) -> &str {
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
            TrinityJackCommand::GraphPointerDown { .. } => "graphPointerDown",
            TrinityJackCommand::SetSelection { .. } => "setSelection",
            TrinityJackCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &TrinityJackCommand, doc: &DocumentView<'_, GraphFixture>, cfg: &ConfigView<'_, JackConfig>) -> Result<Emit<TrinityGraphOperation, JackConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        match command {
            TrinityJackCommand::SetFixtureJson { json } => match GraphFixture::from_json(json) {
                Ok(next) => Ok(Emit::operations(vec![TrinityGraphOperation::SetFixture { fixture: next }]),
                Err(_) => Ok(Emit::default()),
            },
            TrinityJackCommand::DeleteSelection => {
                let deletes: Vec<TrinityGraphOperation> = config.selected_node_ids.iter().filter(|id| fixture.nodes.iter().any(|node| &node.id == *id)).map(|id| TrinityGraphOperation::DeleteNode { id: id.clone() }).collect();
                if deletes.is_empty() {
                    Ok(Emit::default()
                } else {
                    Emit { document_operations: deletes, config_operations: vec![JackConfigOperation::SetSelection { node_ids: Vec::new() }], ..Default::default() }
                }
            }
            TrinityJackCommand::PatchNodes { node_ids, field, value } => {
                if field == "name" && !node_ids.is_empty() && !value.trim().is_empty() {
                    let operations: Vec<TrinityGraphOperation> = node_ids.iter().filter(|id| fixture.nodes.iter().any(|node| &node.id == *id)).map(|id| TrinityGraphOperation::Rename { id: id.clone(), name: value.trim().into() }).collect();
                    Ok(Emit::operations(operations)
                } else {
                    Ok(Emit::default()
                }
            }
            TrinityJackCommand::Reorganize => {
                let config_operations = vec![JackConfigOperation::SetReorganizeEpoch { value: config.reorganize_epoch + 1 }];
                match force_layout_fixture(fixture) {
                    Some(after) => Ok(Emit { document_operations: reposition_operations(fixture, &after), config_operations, ..Default::default() },
                    None => Ok(Emit::config(config_operations)),
                }
            }
            TrinityJackCommand::RunQuery { query } => {
                let resolved = query.as_deref().filter(|value| !value.trim().is_empty()).map(str::to_string).unwrap_or_else(|| config.jack_query.clone());
                let (result_json, operations) = run_jack_query(fixture, &resolved);
                Ok(Emit {
                    document_operations: operations,
                    config_operations: vec![JackConfigOperation::SetQuery { value: resolved }, JackConfigOperation::SetResult { value: result_json }, JackConfigOperation::SetResultsEngagementInput { value: String::new() }],
                    ..Default::default()
                })
            }
            TrinityJackCommand::LoadExampleQuery { query } => {
                let (result_json, operations) = run_jack_query(fixture, query);
                Emit { document_operations: operations, config_operations: vec![JackConfigOperation::SetQuery { value: query.clone() }, JackConfigOperation::SetResult { value: result_json }], ..Default::default() }
            }
            TrinityJackCommand::SetActiveExample { example_id } => match fixture_dsl_for_preset(example_id).and_then(|dsl| GraphFixture::parse_dsl(dsl).ok()) {
                Some(next) => {
                    let query = preset_query(example_id).to_string();
                    let (result_json, _) = run_jack_query(&next, &query);
                    Ok(Emit {
                        document_operations: vec![TrinityGraphOperation::SetFixture { fixture: next.clone() }],
                        config_operations: vec![
                            JackConfigOperation::SetActiveFixture { value: example_id.clone() },
                            JackConfigOperation::SetCamera { camera: next.camera.clone() },
                            JackConfigOperation::SetQuery { value: query },
                            JackConfigOperation::SetResult { value: result_json },
                        ],
                        ..Default::default()
                    })
                }
                None => Ok(Emit::default()),
            },
            TrinityJackCommand::SetViewport { viewport_json } => match serde_json::from_str::<Camera>(viewport_json) {
                Ok(camera) => Ok(Emit::config(vec![JackConfigOperation::SetCamera { camera }])),
                Err(_) => Ok(Emit::default()),
            },
            TrinityJackCommand::TextEdit { text } => Ok(Emit::config(vec![JackConfigOperation::SetQuery { value: text.clone() }])),
            TrinityJackCommand::TextSelect { start, end } => Ok(Emit::config(vec![JackConfigOperation::SetEditorSelection { selection: Some(JackEditorSelection { start: *start, end: *end }) }])),
            TrinityJackCommand::RequestCompletions => Ok(Emit::config(vec![JackConfigOperation::SetRevision { value: config.revision + 1 }])),
            TrinityJackCommand::FormatDocument => match jack_format(&config.jack_query) {
                Ok(formatted) => Ok(Emit::config(vec![JackConfigOperation::SetQuery { value: formatted }])),
                Err(_) => Ok(Emit::default()),
            },
            TrinityJackCommand::SetLodMode { window_id, value } => Ok(Emit::config(vec![JackConfigOperation::SetLodMode { window_id: window_id.clone(), value: value.clone() }])),
            TrinityJackCommand::EditorEngagementInput { value } => Ok(Emit::config(vec![JackConfigOperation::SetEditorEngagementInput { value: value.clone() }])),
            TrinityJackCommand::GraphEngagementInput { value } => Ok(Emit::config(vec![JackConfigOperation::SetGraphEngagementInput { value: value.clone() }])),
            TrinityJackCommand::ResultsEngagementInput { value } => Ok(Emit::config(vec![JackConfigOperation::SetResultsEngagementInput { value: value.clone() }])),
            TrinityJackCommand::GraphPointerDown { node_id } => Ok(Emit::config(vec![JackConfigOperation::SetSelection { node_ids: node_id.clone().map(|id| vec![id]).unwrap_or_default() }])),
            TrinityJackCommand::SetSelection { ids } => Ok(Emit::config(vec![JackConfigOperation::SetSelection { node_ids: ids.clone() }])),
            TrinityJackCommand::SetLocale { value } => Ok(Emit::config(vec![JackConfigOperation::SetLocale { value: value.clone() }])),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, GraphFixture>, cfg: &ConfigView<'_, JackConfig>) -> UiNode {
        let fixture = doc.projection;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<TrinityJackLabels>(&cfg.projection.locale);
        match body_key {
            TRINITY_JACK_PLAY_BODY_GRAPH => render_graph(fixture, cfg.projection),
            TRINITY_JACK_PLAY_BODY_EDITOR => render_editor(fixture, cfg.projection),
            TRINITY_JACK_PLAY_BODY_RESULTS => render_results(cfg.projection),
            TRINITY_JACK_PLAY_BODY_DOCUMENT => build_document_tree(fixture, cfg.projection, labels),
            TRINITY_JACK_PLAY_BODY_CATALOGUE => build_catalogue_tree(cfg.projection, labels),
            TRINITY_JACK_PLAY_BODY_INSPECTION => build_inspector_tree(fixture, cfg.projection, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, GraphFixture>, cfg: &ConfigView<'_, JackConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let mode = cfg.projection.lod_mode_by_window.get(TRINITY_JACK_PLAY_WINDOW_GRAPH).map(String::as_str).unwrap_or(TRINITY_LOD_MODE_AUTOMATIC);
        HashMap::from([(TRINITY_JACK_PLAY_WINDOW_GRAPH.to_string(), vec![trinity_lod_measure(TRINITY_JACK_PLAY_WINDOW_GRAPH, mode)])])
    }

    fn context_menu(&self, request: &ContextMenuRequest, _doc: &DocumentView<'_, GraphFixture>, cfg: &ConfigView<'_, JackConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};

        let is_de = cfg.projection.locale.starts_with("de");
        let selected = cfg.projection.selected_node_ids.clone();
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected, &[]);
        let mut menu = Menu::of(registry).action("runQuery").action("reorganize").action("formatDocument").group("mode", |m| m.action("setActiveExample")).group("open", |m| m.action("loadExampleQuery"));
        if let Some(spec) = node_graph_delete_selection_spec("Delete selection", is_de, nodes.len(), edges.len(), NodeGraphDeleteDispatch::ViaNodeGraphEdit) {
            menu = menu.item(spec);
        }
        menu.build()
    }
}
//#endregion 🔖️TrinityJackPlayApp

//#region 🔖️Manifest
fn jack_window_stack(id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: id.into(), title: Some(title.into()), instance_id: None, template_id: None }],
    })
}

fn jack_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(0.6),
                    active_window_kind_id: None,
                    children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into(), title: Some("Nakagin Graph".into()), instance_id: None, template_id: None }],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.4),
                    children: vec![jack_window_stack(TRINITY_JACK_PLAY_WINDOW_EDITOR, "Jack Query", Some(0.55)), jack_window_stack(TRINITY_JACK_PLAY_WINDOW_RESULTS, "Results", Some(0.45))],
                }),
            ],
        }),
    }
}

pub fn create_trinity_jack_app() -> App {
    App::from_builder(
        App::builder(TRINITY_JACK_PLAY_APP_ID, LocalizedLabel::native("Trinity Jack", "Trinity Jack")).document(["semio", "trinity", "jack"])
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
            })
            .icon_id("trinity")
            .mode("explore", LocalizedLabel::native("Explore", "Erkunden"), "focus")
            .default_mode_id("explore")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_GRAPH, LocalizedLabel::native("Nakagin Graph", "Nakagin-Graph"), TRINITY_JACK_PLAY_BODY_GRAPH, SurfaceKind::NodeGraph, "graph-dag")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_EDITOR, LocalizedLabel::native("Jack Query", "Jack-Abfrage"), TRINITY_JACK_PLAY_BODY_EDITOR, SurfaceKind::TextEditor, "document-jack")
            .window_kind(TRINITY_JACK_PLAY_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), TRINITY_JACK_PLAY_BODY_RESULTS, SurfaceKind::Table, "table-2")
            .default_layout(jack_layout())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
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
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .operation("patchNodes", LocalizedLabel::native("Patch Nodes", "Knoten aktualisieren"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("runQuery", LocalizedLabel::native("Run Jack Query", "Jack-Abfrage ausführen"), ActionKind::Operation).with_category("methods"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("loadExampleQuery", LocalizedLabel::native("Load Example Query", "Beispielabfrage laden"), ActionKind::Operation).with_category("open"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Operation).with_category("mode"))
            // 🛠️ Dev-only whole-fixture import — kept out of the command palette.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new_catalog("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Operation) })
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("setViewport", LocalizedLabel::native("Set Graph Viewport", "Graph-Ansicht festlegen"))
            .view_action("textEdit", LocalizedLabel::native("Edit Jack Query", "Jack-Abfrage bearbeiten"))
            .view_action("textSelect", LocalizedLabel::native("Select Jack Query Text", "Jack-Abfragetext auswählen"))
            .view_action("requestCompletions", LocalizedLabel::native("Request Completions", "Vervollständigungen anfordern"))
            .action_with(semio_framework_plugin::ActionDefinition::new_catalog("formatDocument", LocalizedLabel::native("Format Jack Query", "Jack-Abfrage formatieren"), ActionKind::View).with_category("utilities"))
            .view_action("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"))
            .view_action("editorEngagementInput", LocalizedLabel::native("Editor Engagement Input", "Editor-Eingabe"))
            .view_action("graphEngagementInput", LocalizedLabel::native("Graph Engagement Input", "Graph-Eingabe"))
            .view_action("resultsEngagementInput", LocalizedLabel::native("Results Engagement Input", "Ergebnis-Eingabe"))
            .view_action("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"))
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
            .io(jack_io()),
    )
    .example("nakagin", LocalizedLabel::native("Nakagin", "Nakagin"), default_fixture().print_dsl(), "building")
    .workflow("trinity", "Trinity", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, ViewState};

    fn meta(actor: &str) -> semio_framework_plugin::ActionMeta {
        testkit::meta(actor)
    }

    fn new_app() -> VcsDocumentApp<TrinityJackPlayApp> {
        testkit::new_app::<TrinityJackPlayApp>()
    }

    fn node_id_at(app: &VcsDocumentApp<TrinityJackPlayApp>, index: usize) -> String {
        app.projection().expect("projection").nodes[index].id.clone()
    }

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn renders_jack_editor() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("text-editor"));
        assert!(json.contains(TRINITY_JACK_DEFAULT_QUERY));
    }

    #[test]
    fn run_query_populates_results_and_a_set_query_mutates_projection() {
        let mut app = new_app();
        // The seeded default query (read-only) already populated the initial config on construction.
        app.render(TRINITY_JACK_PLAY_BODY_RESULTS, None, &ViewState::default()).expect("render");
        let result = app.dispatch_typed(TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'ran-label'".into()) }, &meta("local")).expect("run");
        assert!(!result.operations.is_empty(), "a SET query emits operations");
        let projection = app.projection().expect("projection");
        assert!(serde_json::to_string(&projection).unwrap().contains("ran-label"));
    }

    #[test]
    fn node_graph_select_updates_selection_and_document_tree() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        let result = app.dispatch_typed(TrinityJackCommand::SetSelection { ids: vec![node_id.clone()] }, &meta("local")).expect("select");
        assert!(result.operations.is_empty(), "selection is a config-only command, no document operations");
        let tree = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&tree).unwrap().contains(&format!("trinity-document.node.{node_id}")));
    }

    #[test]
    fn nakagin_fixture_has_nodes() {
        assert!(!default_fixture().nodes.is_empty());
    }

    #[test]
    fn editor_scene_has_tokens_and_diagnostics() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("tokensJson"));
        assert!(json.contains("diagnosticsJson"));
        assert!(json.contains("completionsJson"));
    }

    #[test]
    fn text_edit_updates_query_without_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityJackCommand::TextEdit { text: "MATCH (a:Piece) RETURN a.name".into() }, &meta("local")).expect("edit");
        assert!(result.operations.is_empty());
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("MATCH (a:Piece) RETURN a.name"));
    }

    #[test]
    fn graph_scene_has_lod_json() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_GRAPH, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("lodJson"));
        assert!(json.contains("automatic"));
    }

    #[test]
    fn set_lod_mode_reflects_in_window_measures() {
        let mut app = new_app();
        app.dispatch_typed(TrinityJackCommand::SetLodMode { window_id: TRINITY_JACK_PLAY_WINDOW_GRAPH.into(), value: "compact".into() }, &meta("local")).expect("lod");
        let measures = app.window_measures();
        assert!(measures[TRINITY_JACK_PLAY_WINDOW_GRAPH].iter().any(|measure| matches!(measure, WindowMeasure::Select { value, .. } if value == "compact")));
    }

    #[test]
    fn catalogue_tree_renders() {
        let mut app = new_app();
        let node = app.render(TRINITY_JACK_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("trinity-jack-catalogue"));
    }

    #[test]
    fn inspection_tree_reflects_selection() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        app.dispatch_typed(TrinityJackCommand::SetSelection { ids: vec![node_id] }, &meta("local")).expect("select");
        let node = app.render(TRINITY_JACK_PLAY_BODY_INSPECTION, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("trinity-inspector.identity"));
    }

    #[test]
    fn document_tree_de_locale_translates_labels() {
        let mut app = new_app();
        app.dispatch_typed(TrinityJackCommand::SetLocale { value: "de-DE".into() }, &meta("local")).expect("set locale");
        let node = app.render(TRINITY_JACK_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("Stücke"));
    }

    #[test]
    fn set_active_example_swaps_fixture_and_seeds_query() {
        let mut app = new_app();
        let result = app.dispatch_typed(TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() }, &meta("local")).expect("set active example");
        assert!(!result.operations.is_empty());
        let node = app.render(TRINITY_JACK_PLAY_BODY_EDITOR, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("RETURN a, r, b"));
    }

    #[test]
    fn delete_selection_removes_selected_node() {
        let mut app = new_app();
        let node_id = node_id_at(&app, 0);
        app.dispatch_typed(TrinityJackCommand::SetSelection { ids: vec![node_id.clone()] }, &meta("local")).expect("select");
        let result = app.dispatch_typed(TrinityJackCommand::DeleteSelection, &meta("local")).expect("delete");
        assert!(!result.operations.is_empty());
        let projection = app.projection().expect("projection");
        assert!(!projection.nodes.iter().any(|node| node.id == node_id));
    }

    #[test]
    fn context_menu_stays_within_row_budget_and_ends_with_delete_selection() {
        let mut app = testkit::new_app_with_registry::<TrinityJackPlayApp>(create_trinity_jack_app);
        let node_id = node_id_at(&app, 0);
        app.dispatch_typed(TrinityJackCommand::SetSelection { ids: vec![node_id.clone()] }, &meta("local")).expect("select");
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
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).map(|child| child.destructive == Some(true)).unwrap_or(false);
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
    }

    #[test]
    fn export_media_graph_out_matches_document_pack() {
        use semio_framework_plugin::PluginApp as _;
        let mut app = new_app();
        let document_out = app.export_media("document:out").expect("document:out export");
        let graph_out = app.export_media("graph:out").expect("graph:out export");
        assert_eq!(document_out.payload, graph_out.payload);
    }
}
//#endregion 🧪️Tests

//! 🌐️ Trinity Rewriting app — retained WASM canvas host + LOD scale (scene compute needing both the
//! live document AND its own view-only camera/LOD state, so — like `block`/`cad`'s app-level
//! `world.rs` precedent — this lives at app level rather than in the artifact's `🧬️schema`).

use crate::artifacts::jack::mutations::move_node;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::{port_key, Graph, JackSnapshot, Node, PortDirection};
use crate::artifacts::rewriting::schema::{ApplyRuleResult, Rule};
use crate::ast::QueryResult;
use crate::executor::execute;
use crate::language_service::{complete as complete_jack, parse};
use crate::lexer::tokenize as tokenize_jack;
use infinite_board_port_directed::{
    compute_edge_bezier_points, distance_between,
    force_graph::{apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions},
    BoardEngine, CanvasPalette, HandleRole,
};
use infinite_board_port_directed_normal::BoardHost;
pub use infinite_canvas as canvas;
use std::cell::Cell;
use std::collections::HashMap;

use crate::artifacts::rewriting::TrinityRewritingError;

type TrinityBoardEngine = BoardEngine;

const TRINITY_HANDLE_RADIUS: f64 = 5.0;
const TRINITY_BOARD_PORT_HANDLE_KIND: &str = "port";
const TRINITY_DEFAULT_NODE_RADIUS: f64 = 44.0;
const TRINITY_BOARD_KIND_CATALOGS_JSON: &str = "{\"handleKinds\":[{\"id\":\"port\",\"name\":\"Port\",\"color\":\"#6b7280\"}],\"edgeKinds\":[{\"id\":\"Connection\",\"name\":\"Connection\",\"color\":\"#94a3b8\"}]}";

//#region 🔖️Lod
use canvas::lod::{Lod, LodScale};

const TRINITY_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; edges and node fills only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Topology without labels or port handles.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Abbreviated node names.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Full node names.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Node names and port handles.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Maximum port-graph fidelity.", max_zoom: f64::INFINITY },
];

const TRINITY_LOD_SCALE: LodScale = LodScale { lods: TRINITY_LODS };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrinityDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl TrinityDrawLod {
    fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        Some(match id {
            "minimap" => Self::Minimap,
            "overview" => Self::Overview,
            "compact" => Self::Compact,
            "normal" => Self::Normal,
            "detail" => Self::Detail,
            "micro" => Self::Micro,
            _ => return None,
        })
    }

    fn from_scale_index(index: usize) -> Self {
        match index {
            0 => Self::Minimap,
            1 => Self::Overview,
            2 => Self::Compact,
            3 => Self::Normal,
            4 => Self::Detail,
            _ => Self::Micro,
        }
    }

    #[cfg(test)]
    fn handles_visible(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    #[cfg(test)]
    fn labels_visible(self) -> bool {
        !matches!(self, Self::Minimap | Self::Overview)
    }

    #[cfg(test)]
    fn full_labels(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }
}

#[cfg(test)]
fn trinity_abbreviate_label(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.len() <= 4 {
        return trimmed.to_string();
    }
    trimmed.chars().take(3).collect()
}

fn trinity_lod_index(zoom: f64) -> usize {
    TRINITY_LOD_SCALE.resolve_index(zoom.max(0.05))
}

pub fn trinity_lod_scale_json() -> String {
    let rows: Vec<pack::JsonValue> = TRINITY_LODS
        .iter()
        .map(|lod| {
            pack::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
            })
        })
        .collect();
    pack::json_to_string(&pack::json_array(rows))
}

fn trinity_node_radius(node: &Node) -> f64 {
    let w = if node.width > 0.0 { node.width } else { 88.0 };
    let h = if node.height > 0.0 { node.height } else { 40.0 };
    (w.max(h) * 0.5).max(TRINITY_DEFAULT_NODE_RADIUS * 0.5)
}

fn trinity_circle_port_angle(index: usize, count: usize, left: bool) -> f64 {
    let base = if left { std::f64::consts::PI } else { 0.0 };
    let spread = 0.35;
    let t = (index as f64 + 0.5) / count.max(1) as f64 - 0.5;
    base + t * spread
}

fn trinity_graph_to_board_fixture(graph: &Graph) -> pack::JsonValue {
    let nodes: Vec<pack::JsonValue> = graph
        .nodes
        .values()
        .map(|node| {
            let radius = trinity_node_radius(node);
            let in_ports: Vec<_> = node.ports.iter().filter(|port| port.direction == PortDirection::In).collect();
            let out_ports: Vec<_> = node.ports.iter().filter(|port| port.direction == PortDirection::Out).collect();
            let mut handles = Vec::new();
            for (index, port) in in_ports.iter().enumerate() {
                handles.push(pack::json!({
                    "id": port_key(&node.id, &port.id),
                    "handleKind": TRINITY_BOARD_PORT_HANDLE_KIND,
                    "angle": trinity_circle_port_angle(index, in_ports.len(), true),
                }));
            }
            for (index, port) in out_ports.iter().enumerate() {
                handles.push(pack::json!({
                    "id": port_key(&node.id, &port.id),
                    "handleKind": TRINITY_BOARD_PORT_HANDLE_KIND,
                    "angle": trinity_circle_port_angle(index, out_ports.len(), false),
                }));
            }
            pack::json!({
                "id": node.id,
                "x": node.x,
                "y": node.y,
                "radius": radius,
                "shape": "circle",
                "text": node.name,
                "nodeKind": node.kind,
                "handles": handles,
            })
        })
        .collect();
    let edges: Vec<pack::JsonValue> = graph
        .edges
        .values()
        .map(|edge| {
            pack::json!({
                "id": edge.id,
                "source": edge.source,
                "target": edge.target,
                "edgeKind": edge.kind,
            })
        })
        .collect();
    pack::json!({
        "schema": "puzzle.2d.fixture",
        "camera": {
            "x": graph.camera.x,
            "y": graph.camera.y,
            "zoom": graph.camera.zoom,
        },
        "nodes": nodes,
        "edges": edges,
    })
}

fn trinity_graph_to_force_layout_fixture(graph: &Graph) -> pack::JsonValue {
    let nodes: Vec<pack::JsonValue> = graph
        .nodes
        .values()
        .map(|node| {
            let radius = trinity_node_radius(node);
            let handles: Vec<pack::JsonValue> = node.ports.iter().map(|port| pack::json!({ "id": port_key(&node.id, &port.id) })).collect();
            pack::json!({
                "id": node.id,
                "x": node.x,
                "y": node.y,
                "radius": radius,
                "shape": "circle",
                "handles": handles,
            })
        })
        .collect();
    let edges: Vec<pack::JsonValue> = graph.edges.values().map(|edge| pack::json!({ "source": edge.source, "target": edge.target })).collect();
    pack::json!({
        "schema": JackSnapshot::SCHEMA,
        "nodes": nodes,
        "edges": edges,
    })
}

fn apply_force_layout_positions_to_trinity_graph(graph: &mut Graph, fixture: &pack::JsonValue) -> Result<(), TrinityRewritingError> {
    let nodes = fixture.get("nodes").and_then(|v| v.as_array()).ok_or(TrinityRewritingError::ForceLayoutFixtureMissingNodes)?;
    for node in nodes {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let Some(id) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(entry) = graph.nodes.get_mut(id) else {
            continue;
        };
        if let Some(x) = obj.get("x").and_then(|v| v.as_f64()) {
            entry.x = x;
        }
        if let Some(y) = obj.get("y").and_then(|v| v.as_f64()) {
            entry.y = y;
        }
    }
    Ok(())
}

fn force_layout_reposition_operations(fixture: &JackSnapshot) -> Result<Vec<TrinityGraphMutation>, TrinityRewritingError> {
    let mut graph = Graph::from_fixture(fixture.clone())?;
    apply_force_layout_to_trinity_graph(&mut graph)?;
    let next = graph.to_fixture();
    let next_nodes = next.nodes();
    let prev_nodes = fixture.nodes();
    let mut operations = Vec::new();
    for node in &next_nodes {
        let Some(prev) = prev_nodes.iter().find(|entry| entry.id == node.id) else {
            continue;
        };
        if (prev.x - node.x).abs() > 1e-6 || (prev.y - node.y).abs() > 1e-6 {
            operations.push(move_node(node.id.clone(), node.x, node.y));
        }
    }
    Ok(operations)
}

fn apply_force_layout_to_trinity_graph(graph: &mut Graph) -> Result<(), TrinityRewritingError> {
    let mut fixture = trinity_graph_to_force_layout_fixture(graph);
    apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &ForceGraphLayoutOptions::default()).map_err(TrinityRewritingError::Layout)?;
    apply_force_layout_positions_to_trinity_graph(graph, &fixture)
}
//#endregion 🔖️Lod

//#region 🔖️TrinityBridge
/// 🖥️ Retained trinity graph host on the directed port board engine.
pub struct TrinityBridge {
    pub graph: Graph,
    store: crate::artifacts::jack::op::TrinityGraphStore,
    pub engine: TrinityBoardEngine,
    board: BoardHost,
    pub canvas_theme: CanvasPalette,
    width: u32,
    height: u32,
    dpr: f64,
    node_id_map: HashMap<u64, String>,
    handle_key_map: HashMap<u64, String>,
    edge_id_map: HashMap<u64, String>,
    last_logged_lod: Cell<i8>,
    automatic_lod: bool,
    forced_draw_lod: Option<TrinityDrawLod>,
}

impl TrinityBridge {
    pub fn from_graph(graph: &Graph) -> Self {
        let fixture = graph.to_fixture();
        let store = crate::artifacts::jack::op::TrinityGraphStore::new(crate::artifacts::jack::op::create_trinity_graph_envelope("trinity-host", fixture)).expect("failed to create trinity graph store");
        let graph = Graph::from_fixture(store.snapshot().expect("projection")).expect("graph");
        let mut host = Self {
            graph,
            store,
            engine: TrinityBoardEngine::new(),
            board: BoardHost::new(),
            canvas_theme: CanvasPalette::default(),
            width: 1,
            height: 1,
            dpr: 1.0,
            node_id_map: HashMap::new(),
            handle_key_map: HashMap::new(),
            edge_id_map: HashMap::new(),
            last_logged_lod: Cell::new(-1),
            automatic_lod: true,
            forced_draw_lod: None,
        };
        host.rebuild_engine();
        host
    }

    pub fn load_fixture_json(json: &str) -> Result<Self, TrinityRewritingError> {
        let graph = Graph::load_json(json)?;
        Ok(Self::from_graph(&graph))
    }

    fn refresh_graph_from_store(&mut self) -> Result<(), TrinityRewritingError> {
        self.graph = Graph::from_fixture(self.store.snapshot()?)?;
        Ok(())
    }

    fn dispatch(&mut self, operations: Vec<TrinityGraphMutation>) -> Result<(), TrinityRewritingError> {
        crate::artifacts::jack::op::dispatch_trinity_graph_mutations(&mut self.store, operations)?;
        self.refresh_graph_from_store()
    }

    pub fn undo(&mut self) -> Result<(), TrinityRewritingError> {
        use store::ArtifactCommand;
        self.store.dispatch(ArtifactCommand::Undo)?;
        self.refresh_graph_from_store()?;
        self.rebuild_engine();
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), TrinityRewritingError> {
        use store::ArtifactCommand;
        self.store.dispatch(ArtifactCommand::Redo)?;
        self.refresh_graph_from_store()?;
        self.rebuild_engine();
        Ok(())
    }

    pub fn commit_checkpoint(&mut self, message: Option<String>) -> Result<(), TrinityRewritingError> {
        use store::ArtifactCommand;
        self.store.dispatch(ArtifactCommand::CommitCheckpoint { message, authors: Vec::new() }).map_err(TrinityRewritingError::from).map(|_| ())
    }

    pub fn store_generation(&self) -> u64 {
        self.store.generation()
    }

    pub fn fixture_json(&self) -> Result<String, TrinityRewritingError> {
        Ok(self.graph.fixture_json()?)
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
        self.board.set_size(self.width, self.height, self.dpr);
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.graph.camera.x = x;
        self.graph.camera.y = y;
        self.graph.camera.zoom = zoom;
        self.engine.set_camera(x, y, zoom);
        self.board.set_camera_silent(x, y, zoom);
    }

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), TrinityRewritingError> {
        self.canvas_theme.merge_from_json(json).map_err(TrinityRewritingError::CanvasTheme)?;
        self.board.canvas_theme = self.canvas_theme;
        Ok(())
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_down(world.x, world.y, extend);
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_move(world.x, world.y);
        self.sync_ephemeral_positions_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_up(world.x, world.y);
        if let Err(err) = self.commit_drag_positions() {
            eprintln!("[DEBUG] trinity drag commit failed: {err}");
        }
        self.rebuild_engine();
    }

    pub fn reorganize(&mut self) {
        match force_layout_reposition_operations(&self.store.snapshot().unwrap_or_else(|_| self.graph.to_fixture())) {
            Ok(operations) if !operations.is_empty() => {
                if let Err(err) = self.dispatch(operations) {
                    eprintln!("[DEBUG] trinity reorganize dispatch failed: {err}");
                    return;
                }
                self.rebuild_engine();
            }
            Ok(_) => {}
            Err(err) => eprintln!("[DEBUG] trinity reorganize force layout failed: {err}"),
        }
    }

    pub fn run_jack(&mut self, query: &str) -> Result<QueryResult, TrinityRewritingError> {
        let parsed = parse(query).map_err(TrinityRewritingError::Jack)?;
        let (result, operations) = execute(&self.graph, &parsed).map_err(TrinityRewritingError::Jack)?;
        if !operations.is_empty() {
            self.dispatch(operations)?;
            self.rebuild_engine();
        }
        Ok(result)
    }

    pub fn run_jack_json(&mut self, query: &str) -> Result<String, TrinityRewritingError> {
        let result = self.run_jack(query)?;
        Ok(pack::to_json_string(&result))
    }

    pub fn run_jack_with_fixture_json(&mut self, query: &str) -> Result<String, TrinityRewritingError> {
        let result = self.run_jack(query)?;
        let fixture_json = self.fixture_json()?;
        let out = JackRunWithFixture { result, fixture_json };
        Ok(pack::to_json_string(&out))
    }

    pub fn tokenize_jack_json(&self, source: &str) -> Result<String, TrinityRewritingError> {
        let tokens = tokenize_jack(source);
        Ok(pack::to_json_string(&tokens))
    }

    pub fn complete_jack_json(&self, source: &str, cursor: usize) -> Result<String, TrinityRewritingError> {
        let items = complete_jack(&self.graph, source, cursor);
        Ok(pack::to_json_string(&items))
    }

    pub fn apply_rewriting_json(&mut self, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
        let rule: Rule = pack::from_json_str(rule_json)?;
        let bindings = crate::artifacts::rewriting::schema::parse_bindings_json(bindings_json)?;
        let query = crate::artifacts::rewriting::schema::build_rule_query(&rule, &bindings);
        let parsed = parse(&query).map_err(TrinityRewritingError::Jack)?;
        let (result, operations) = execute(&self.graph, &parsed).map_err(TrinityRewritingError::Jack)?;
        if !operations.is_empty() {
            self.dispatch(operations)?;
            self.rebuild_engine();
        }
        Ok(pack::to_json_string(&ApplyRuleResult { fixture: self.fixture_json()?, query: result }))
    }

    pub fn node_overlays_json(&self) -> Result<String, TrinityRewritingError> {
        Ok("[]".into())
    }

    pub fn draw_lod_label(&self) -> &'static str {
        self.draw_lod_for_frame().label()
    }

    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        self.board.set_automatic_lod(enabled);
    }

    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        self.forced_draw_lod = if label.is_empty() { None } else { TrinityDrawLod::from_id(label) };
        self.board.set_forced_draw_lod_label(label);
    }

    fn draw_lod_for_frame(&self) -> TrinityDrawLod {
        if !self.automatic_lod {
            if let Some(forced) = self.forced_draw_lod {
                return forced;
            }
        }
        TrinityDrawLod::from_scale_index(trinity_lod_index(self.graph.camera.zoom))
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        use canvas::camera::{wheel_screen, Camera as CanvasCamera, Viewport};
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        let mut cam = CanvasCamera { x: self.graph.camera.x, y: self.graph.camera.y, zoom: self.graph.camera.zoom };
        wheel_screen(&mut cam, &viewport, sx, sy, delta_y);
        self.set_camera(cam.x, cam.y, cam.zoom);
    }

    pub fn selected_node_ids_json(&self) -> Result<String, TrinityRewritingError> {
        let mut ids = Vec::new();
        for &nid in &self.engine.selection.node_ids {
            if let Some(tid) = self.node_id_map.get(&nid) {
                ids.push(tid.clone());
            }
        }
        for &hid in &self.engine.selection.handle_ids {
            if let Some(handle) = self.engine.handles.get(&hid) {
                if let Some(tid) = self.node_id_map.get(&handle.node_id) {
                    if !ids.iter().any(|row| row == tid) {
                        ids.push(tid.clone());
                    }
                }
            }
        }
        Ok(pack::to_json_string(&ids))
    }

    pub fn set_highlighted_node_ids_json(&mut self, json: &str) -> Result<(), TrinityRewritingError> {
        let ids: Vec<String> = pack::from_json_str(json)?;
        self.board.set_highlighted_ids(ids);
        Ok(())
    }

    fn screen_to_world(&self, sx: f64, sy: f64) -> canvas::Point {
        use canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
        use canvas::Point;
        let cam = CanvasCamera { x: self.graph.camera.x, y: self.graph.camera.y, zoom: self.graph.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn sync_ephemeral_positions_from_engine(&mut self) {
        for (&nid, widget_id) in &self.node_id_map {
            if let Some(node) = self.engine.nodes.get(&nid) {
                if let Some(entry) = self.graph.nodes.get_mut(widget_id) {
                    entry.x = node.center.x;
                    entry.y = node.center.y;
                }
            }
        }
        self.sync_board_from_graph();
    }

    fn commit_drag_positions(&mut self) -> Result<(), TrinityRewritingError> {
        let projection = self.store.snapshot()?;
        let projection_nodes = projection.nodes();
        let mut operations = Vec::new();
        for (nid, widget_id) in &self.node_id_map {
            let Some(engine_node) = self.engine.nodes.get(nid) else {
                continue;
            };
            let Some(fixture_node) = projection_nodes.iter().find(|node| node.id == *widget_id) else {
                continue;
            };
            if (fixture_node.x - engine_node.center.x).abs() > 1e-6 || (fixture_node.y - engine_node.center.y).abs() > 1e-6 {
                operations.push(move_node(widget_id.clone(), engine_node.center.x, engine_node.center.y));
            }
        }
        if operations.is_empty() {
            return Ok(());
        }
        self.dispatch(operations)
    }

    fn sync_board_from_graph(&mut self) {
        let _ = self.board.set_board_kind_catalogs_from_json(TRINITY_BOARD_KIND_CATALOGS_JSON);
        let fixture = trinity_graph_to_board_fixture(&self.graph);
        if !self.board.parse_fixture_v1(&fixture) {
            eprintln!("[DEBUG] trinity board fixture parse failed");
        }
        self.board.set_size(self.width, self.height, self.dpr);
        self.board.canvas_theme = self.canvas_theme;
        self.board.set_automatic_lod(self.automatic_lod);
        if let Some(lod) = self.forced_draw_lod {
            self.board.set_forced_draw_lod_label(lod.label());
        }
    }

    // 📌️ `next_node`/`next_handle`/`eid` are three independent manually-assigned id counters (node
    // ids, handle ids starting at a different base, edge ids) interleaved across nested loops — no
    // single `.zip()` range captures all three, so the explicit-counter-loop suggestion doesn't apply.
    #[allow(clippy::explicit_counter_loop)]
    fn rebuild_engine(&mut self) {
        self.engine = TrinityBoardEngine::new();
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.edge_id_map.clear();
        let (cx, cy, zoom) = (self.graph.camera.x, self.graph.camera.y, self.graph.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        let mut next_node: u64 = 1;
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for node in self.graph.nodes.values() {
            let nid = next_node;
            next_node += 1;
            self.node_id_map.insert(nid, node.id.clone());
            let radius = trinity_node_radius(node);
            self.engine.create_node(nid, node.x, node.y, radius, true);
            let in_count = node.ports.iter().filter(|port| port.direction == PortDirection::In).count();
            let out_count = node.ports.iter().filter(|port| port.direction == PortDirection::Out).count();
            let mut in_idx = 0usize;
            let mut out_idx = 0usize;
            for port in &node.ports {
                let hid = next_handle;
                next_handle += 1;
                let angle = match port.direction {
                    PortDirection::In => {
                        let angle = trinity_circle_port_angle(in_idx, in_count, true);
                        in_idx += 1;
                        angle
                    }
                    PortDirection::Out => {
                        let angle = trinity_circle_port_angle(out_idx, out_count, false);
                        out_idx += 1;
                        angle
                    }
                };
                let public_key = port_key(&node.id, &port.id);
                handle_map.insert(trinity_port_handle_key(&node.id, &port.id, port.direction == PortDirection::In), hid);
                self.handle_key_map.insert(hid, public_key);
                self.engine.create_handle(hid, nid, angle);
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = TRINITY_HANDLE_RADIUS;
                }
                self.engine.set_handle_role(hid, if port.direction == PortDirection::In { HandleRole::Target } else { HandleRole::Source });
            }
        }
        let mut eid: u64 = 100;
        for edge in self.graph.edges.values() {
            let (source_node, source_port) = trinity_port_endpoint_parts(&edge.source);
            let (target_node, target_port) = trinity_port_endpoint_parts(&edge.target);
            let src = handle_map.get(&trinity_port_handle_key(&source_node, &source_port, false)).copied();
            let tgt = handle_map.get(&trinity_port_handle_key(&target_node, &target_port, true)).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                self.engine.create_edge(eid, s, t);
                self.edge_id_map.insert(eid, edge.id.clone());
                eid += 1;
            }
        }
        self.engine.set_next_edge_id(eid);
        self.sync_board_from_graph();
    }

    pub fn paint_scene(&self, scene: &mut canvas::Scene, _viewport_w: u32, _viewport_h: u32, _dpr: f64) {
        let lod_index = trinity_lod_index(self.graph.camera.zoom) as i8;
        if self.last_logged_lod.get() != lod_index {
            self.last_logged_lod.set(lod_index);
        }
        let board_scene = self.board.build_vector_scene();
        scene.append(&board_scene, None);
        let _ = distance_between;
        let _ = compute_edge_bezier_points;
    }
}

/// 🩹️ Delegates to `crate::artifacts::jack::parse_port_key` (the one place the `nodeId@portId`
/// convention is owned) instead of hand-rolling a second splitter here.
fn trinity_port_endpoint_parts(endpoint: &str) -> (String, String) {
    crate::artifacts::jack::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), String::new()), |(n, p)| (n.to_string(), p.to_string()))
}

fn trinity_port_handle_key(node_id: &str, port_id: &str, input: bool) -> String {
    format!("{}:{}:{}", node_id, if input { "in" } else { "out" }, port_id)
}

struct JackRunWithFixture {
    result: QueryResult,
    fixture_json: String,
}

/// 🌱️ Hand-written `ToValue` — the derive has no `#[serde(flatten)]` equivalent (fan-out
/// playbook's "not supported" list), so `result`'s own object entries are spliced directly into
/// the parent object rather than nested under a `"result"` key, matching the old flattened wire
/// shape byte-for-byte.
impl dsl::ToValue for JackRunWithFixture {
    fn to_value(&self) -> dsl::DslValue {
        let dsl::DslValue::Object(mut entries) = dsl::ToValue::to_value(&self.result) else {
            unreachable!("QueryResult::to_value always produces an object")
        };
        entries.push(("fixtureJson".to_string(), dsl::ToValue::to_value(&self.fixture_json)));
        dsl::DslValue::Object(entries)
    }
}
//#endregion 🔖️TrinityBridge

//#region 🔖️WasmBridge
// 🌉️ The wasm-bindgen `mod wasm_bridge` (envelope-loading VCS class) and `mod wasm_session`
// (`TrinitySession` WebGPU canvas host, with its DOM `HtmlCanvasElement` attach) that used to
// follow the helpers below were deleted — nothing ever built either for `wasm32-unknown-unknown`
// (no engine entry, no `wasm` script target) — see
// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`. The helpers themselves
// stay: they are exercised directly by native `#[cfg(test)]` tests below.
#[cfg(any(target_arch = "wasm32", test))]
const TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
#[cfg(any(target_arch = "wasm32", test))]
const TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

#[cfg(any(target_arch = "wasm32", test))]
fn trinity_rewriting_envelope_credits_are_valid(maximum_pages: usize, maximum_bytes: usize) -> bool {
    maximum_pages != 0 && maximum_pages <= TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES && maximum_bytes != 0 && maximum_bytes <= TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES
}

#[cfg(any(target_arch = "wasm32", test))]
struct TrinityRewritingCallerPageOwner<Page> {
    page: Option<Page>,
}

#[cfg(any(target_arch = "wasm32", test))]
impl<Page> TrinityRewritingCallerPageOwner<Page> {
    fn new(page: Page) -> Self {
        Self { page: Some(page) }
    }

    fn has_page(&self) -> bool {
        self.page.is_some()
    }

    fn take_page(&mut self) -> Option<Page> {
        self.page.take()
    }

    fn close_step(&mut self) -> bool {
        if self.page.take().is_some() {
            return false;
        }
        true
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn trinity_rewriting_page_handle_matches(operation: u64, generation: u64, expected_operation: u64, expected_generation: u64) -> bool {
    operation == expected_operation && generation == expected_generation
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jack::PropertyValue;
    use crate::artifacts::rewriting::schema::{AssignmentJson, Lhs, PatternJson, Rhs};
    use crate::lexer::TokenSpan as JackTokenSpan;
    use graph::dsl::Completion as JackCompletion;
    use store::ArtifactDsl;

    #[test]
    fn trinity_rewriting_envelope_credits_admit_exact_caps_and_reject_zero_or_plus_one() {
        assert!(trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES));
        assert!(!trinity_rewriting_envelope_credits_are_valid(0, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES));
        assert!(!trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES, 0));
        assert!(!trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES + 1, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES));
        assert!(!trinity_rewriting_envelope_credits_are_valid(TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES, TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES + 1));
    }

    #[test]
    fn trinity_rewriting_rejected_page_preserves_pointer_content_and_retry_owner() {
        let caller = std::rc::Rc::new(vec![1_u8, 2, 3, 4]);
        let pointer = caller.as_ptr();
        let mut rejected = TrinityRewritingCallerPageOwner::new(std::rc::Rc::clone(&caller));
        let retry = rejected.take_page().expect("exact rejected caller page");
        assert_eq!(retry.as_ptr(), pointer);
        assert_eq!(retry.as_slice(), &[1, 2, 3, 4]);
        assert!(!rejected.has_page());
        let retried = TrinityRewritingCallerPageOwner::new(retry);
        assert_eq!(retried.page.as_ref().expect("same retry owner").as_ptr(), pointer);
    }

    #[test]
    fn trinity_rewriting_page_cap_plus_one_rejects_before_owner_construction() {
        let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }).expect("one exact page reservation");
        let bytes_plus_one = vec![8_u8; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1].into_boxed_slice();
        let bytes_plus_one_pointer = bytes_plus_one.as_ptr();
        assert_eq!(pages.preflight_page_bytes(bytes_plus_one.len()), Err(store::OwnedSchemaDecodeAdmissionFault::ByteCapacity));
        assert_eq!(bytes_plus_one.as_ptr(), bytes_plus_one_pointer);
        assert_eq!(bytes_plus_one[0], 8);
        let bytes = [7; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        store::ArtifactEnvelopeDecodePage::from_preflighted_array(bytes, bytes.len()).admit_preflighted_into(&mut pages);
        let caller = Box::new([9; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]);
        let pointer = caller.as_ptr();
        assert_eq!(pages.preflight_page_bytes(caller.len()), Err(store::OwnedSchemaDecodeAdmissionFault::PageCapacity));
        assert_eq!(caller.as_ptr(), pointer);
        assert_eq!(caller[0], 9);
    }

    #[test]
    fn trinity_rewriting_stale_generation_and_slot_aba_never_match() {
        assert!(trinity_rewriting_page_handle_matches(17, 23, 17, 23));
        assert!(!trinity_rewriting_page_handle_matches(17, 23, 17, 24));
        assert!(!trinity_rewriting_page_handle_matches(17, 23, 18, 23));
    }

    #[test]
    fn trinity_rewriting_checked_out_drop_preserves_raw_caller_authority() {
        let caller = std::rc::Rc::new(vec![5_u8; 64]);
        let pointer = caller.as_ptr();
        let checked_out = TrinityRewritingCallerPageOwner::new(std::rc::Rc::clone(&caller));
        drop(checked_out);
        assert_eq!(caller.as_ptr(), pointer);
        assert_eq!(caller.as_slice(), &[5_u8; 64]);
        assert_eq!(std::rc::Rc::strong_count(&caller), 1);
    }

    #[test]
    fn trinity_rewriting_rejected_page_close_retires_one_owner_per_grant() {
        let mut rejected = TrinityRewritingCallerPageOwner::new(Box::new([3_u8; 32]));
        assert!(!rejected.close_step());
        assert!(rejected.close_step());
        assert!(!rejected.has_page());
    }

    fn nakagin_graph() -> Graph {
        let dsl = include_str!("../../../../../../../🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        Graph::from_fixture(JackSnapshot::parse_dsl(dsl).unwrap()).unwrap()
    }

    #[semio_framework_async_macros::async_test]
    async fn nakagin_fixture_loads() {
        let g = nakagin_graph();
        assert_eq!(g.nodes.len(), 9);
        assert_eq!(g.edges.len(), 6);
    }

    #[semio_framework_async_macros::async_test]
    async fn nakagin_flat_position_derived() {
        let g = nakagin_graph();
        let flat = crate::artifacts::jack::schema::inferences::flat_position::compute_flat_position(&g.to_fixture());
        let root_uv = flat.positions.get("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert_eq!(root_uv.u, 0.0);
        let capsule_uv = flat.positions.get("6947a41b-8c6d-4291-bdd8-96cd535c78fc").unwrap();
        assert!(capsule_uv.v > 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_rebuilds_engine() {
        let host = TrinityBridge::from_graph(&nakagin_graph());
        assert_eq!(host.engine.nodes.len(), 9);
        assert!(!host.engine.edges.is_empty());
        assert!(!host.engine.enforce_acyclic);
        assert_eq!(host.board.nodes.len(), 9);
        assert!(host.board.nodes.values().all(|node| matches!(node.shape, infinite_board_port_directed::NodeShape::Circle)));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_reorganize_moves_nodes() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        let before: Vec<(f64, f64)> = host.graph.nodes.values().map(|n| (n.x, n.y)).collect();
        host.reorganize();
        let after: Vec<(f64, f64)> = host.graph.nodes.values().map(|n| (n.x, n.y)).collect();
        assert_ne!(before, after);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_tokenize_jack_json() {
        let host = TrinityBridge::from_graph(&nakagin_graph());
        let json = host.tokenize_jack_json("MATCH (a:Piece)").unwrap();
        let tokens: Vec<JackTokenSpan> = pack::from_json_str(&json).unwrap();
        assert!(tokens.iter().any(|row| row.start == 0));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_complete_jack_json() {
        let host = TrinityBridge::from_graph(&nakagin_graph());
        let json = host.complete_jack_json("MAT", 3).unwrap();
        let items: Vec<JackCompletion> = pack::from_json_str(&json).unwrap();
        assert!(items.iter().any(|row| row.label == "MATCH"));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_jack_create_undo() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        let before = host.graph.nodes.len();
        host.run_jack("CREATE (n:Piece)").unwrap();
        assert_eq!(host.graph.nodes.len(), before + 1);
        host.undo().unwrap();
        assert_eq!(host.graph.nodes.len(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_lod_scale_json_lists_all_six_lods() {
        let json = trinity_lod_scale_json();
        let rows: Vec<pack::JsonValue> = pack::parse_json(&json).unwrap().as_array().unwrap().to_vec();
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0]["id"], "minimap");
        assert_eq!(rows[5]["id"], "micro");
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_abbreviate_label_short_passthrough_and_long_truncated() {
        assert_eq!(trinity_abbreviate_label("abcd"), "abcd");
        assert_eq!(trinity_abbreviate_label("  abcd  "), "abcd");
        assert_eq!(trinity_abbreviate_label("abcdef"), "abc");
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_draw_lod_from_id_and_visibility_flags() {
        assert_eq!(TrinityDrawLod::from_id("bogus"), None);
        assert_eq!(TrinityDrawLod::from_id("micro"), Some(TrinityDrawLod::Micro));
        assert!(TrinityDrawLod::Detail.handles_visible());
        assert!(!TrinityDrawLod::Normal.handles_visible());
        assert!(!TrinityDrawLod::Minimap.labels_visible());
        assert!(TrinityDrawLod::Compact.labels_visible());
        assert!(!TrinityDrawLod::Compact.full_labels());
        assert!(TrinityDrawLod::Detail.full_labels());
        assert_eq!(TrinityDrawLod::from_scale_index(5), TrinityDrawLod::Micro);
        assert_eq!(TrinityDrawLod::from_scale_index(0), TrinityDrawLod::Minimap);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_node_radius_uses_dimensions_or_default() {
        let mut node = Node { id: "n".into(), kind: "Piece".into(), name: "n".into(), x: 0.0, y: 0.0, width: 0.0, height: 0.0, properties: Default::default(), ports: vec![] };
        assert_eq!(trinity_node_radius(&node), 44.0);
        node.width = 10.0;
        node.height = 10.0;
        assert_eq!(trinity_node_radius(&node), TRINITY_DEFAULT_NODE_RADIUS * 0.5);
        node.width = 200.0;
        node.height = 10.0;
        assert_eq!(trinity_node_radius(&node), 100.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_circle_port_angle_left_right_spread() {
        assert!((trinity_circle_port_angle(0, 1, true) - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(trinity_circle_port_angle(0, 1, false), 0.0);
        assert!(trinity_circle_port_angle(0, 2, false) < trinity_circle_port_angle(1, 2, false));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_port_endpoint_parts_splits_on_at() {
        assert_eq!(trinity_port_endpoint_parts("node1@portA"), ("node1".to_string(), "portA".to_string()));
        assert_eq!(trinity_port_endpoint_parts("no-at"), ("no-at".to_string(), String::new()));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_port_handle_key_direction_prefix() {
        assert_eq!(trinity_port_handle_key("n", "p", true), "n:in:p");
        assert_eq!(trinity_port_handle_key("n", "p", false), "n:out:p");
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_graph_to_board_fixture_includes_handles_and_edges() {
        let g = nakagin_graph();
        let fixture = trinity_graph_to_board_fixture(&g);
        assert_eq!(fixture["schema"], "puzzle.2d.fixture");
        assert_eq!(fixture["nodes"].as_array().unwrap().len(), 9);
        assert_eq!(fixture["edges"].as_array().unwrap().len(), 6);
        let root_node = fixture["nodes"].as_array().unwrap().iter().find(|n| n["id"] == "7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert!(!root_node["handles"].as_array().unwrap().is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn force_layout_reposition_operations_produces_repositions() {
        let fixture = nakagin_graph().to_fixture();
        let operations = force_layout_reposition_operations(&fixture).unwrap();
        assert!(!operations.is_empty());
        assert!(operations.iter().all(|op| matches!(op, TrinityGraphMutation::MoveNode(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_force_layout_positions_errors_when_nodes_missing() {
        let mut g = nakagin_graph();
        let fixture = pack::json!({});
        let err = apply_force_layout_positions_to_trinity_graph(&mut g, &fixture).unwrap_err();
        assert!(matches!(err, TrinityRewritingError::ForceLayoutFixtureMissingNodes));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_apply_rewriting_json_end_to_end() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        let rule = Rule {
            name: "label-core".into(),
            lhs: Lhs { pattern: PatternJson { left_var: "a".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }, where_clause: Some("a.name = 'b'".into()) },
            rhs: Rhs { create: vec![], delete: vec![], set: vec![AssignmentJson { var: "a".into(), prop: "label".into(), value: PropertyValue::String("nakagin-core".into()) }], merge: vec![], parameters: vec![] },
        };
        let rule_json = pack::to_json_string(&rule);
        let out = host.apply_rewriting_json(&rule_json, "{}").unwrap();
        let value: pack::JsonValue = pack::parse_json(&out).unwrap();
        assert!(value.get("fixture").is_some());
        let core = host.graph.node("7dc5b737-3b6b-4068-b315-b7bacc91c2e1").unwrap();
        assert_eq!(core.properties.get("label"), Some(&PropertyValue::String("nakagin-core".into())));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_run_jack_json_and_with_fixture() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        let json = host.run_jack_json("MATCH (a:Piece) WHERE a.name = 'b' RETURN a.name").unwrap();
        let result: QueryResult = pack::from_json_str(&json).unwrap();
        assert_eq!(result.rows.len(), 1);

        let before = host.graph.nodes.len();
        let out = host.run_jack_with_fixture_json("CREATE (n:Piece)").unwrap();
        let value: pack::JsonValue = pack::parse_json(&out).unwrap();
        assert!(value.get("fixtureJson").is_some());
        assert_eq!(host.graph.nodes.len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_selected_and_highlighted_node_ids() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        host.set_viewport(800, 600, 1.0);
        host.pointer_down(400.0, 300.0, false);
        let json = host.selected_node_ids_json().unwrap();
        let ids: Vec<String> = pack::from_json_str(&json).unwrap();
        assert_eq!(ids, vec!["7dc5b737-3b6b-4068-b315-b7bacc91c2e1".to_string()]);
        assert!(host.set_highlighted_node_ids_json("[\"7dc5b737-3b6b-4068-b315-b7bacc91c2e1\"]").is_ok());
        assert_eq!(host.node_overlays_json().unwrap(), "[]");
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_viewport_camera_and_wheel() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        host.set_viewport(800, 600, 1.0);
        let before_zoom = host.graph.camera.zoom;
        host.wheel_screen(400.0, 300.0, -100.0);
        assert!(host.graph.camera.zoom > before_zoom);
        host.set_camera(10.0, 20.0, 2.0);
        assert_eq!((host.graph.camera.x, host.graph.camera.y, host.graph.camera.zoom), (10.0, 20.0, 2.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_pointer_drag_commits_position() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        host.set_viewport(800, 600, 1.0);
        let node_id = "7dc5b737-3b6b-4068-b315-b7bacc91c2e1";
        assert_eq!((host.graph.nodes[node_id].x, host.graph.nodes[node_id].y), (0.0, 0.0));
        host.pointer_down(400.0, 300.0, false);
        host.pointer_move(460.0, 360.0);
        host.pointer_up(460.0, 360.0);
        let after = (host.graph.nodes[node_id].x, host.graph.nodes[node_id].y);
        assert!((after.0 - 60.0).abs() < 1e-6);
        assert!((after.1 - 60.0).abs() < 1e-6);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_commit_checkpoint_and_redo_and_store_generation() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        let gen0 = host.store_generation();
        host.run_jack("CREATE (n:Piece)").unwrap();
        assert!(host.store_generation() > gen0);
        let count_after_create = host.graph.nodes.len();
        host.commit_checkpoint(None).unwrap();
        host.undo().unwrap();
        assert_eq!(host.graph.nodes.len(), count_after_create - 1);
        host.redo().unwrap();
        assert_eq!(host.graph.nodes.len(), count_after_create);
    }

    #[semio_framework_async_macros::async_test]
    async fn trinity_host_forced_and_automatic_draw_lod_label() {
        let mut host = TrinityBridge::from_graph(&nakagin_graph());
        host.set_camera(0.0, 0.0, 0.05);
        assert_eq!(host.draw_lod_label(), "minimap");
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("micro");
        assert_eq!(host.draw_lod_label(), "micro");
        host.set_forced_draw_lod_label("");
        assert_eq!(host.draw_lod_label(), "minimap");
        host.set_forced_draw_lod_label("bogus");
        assert_eq!(host.draw_lod_label(), "minimap");
    }
}
//#endregion 🧪️Tests

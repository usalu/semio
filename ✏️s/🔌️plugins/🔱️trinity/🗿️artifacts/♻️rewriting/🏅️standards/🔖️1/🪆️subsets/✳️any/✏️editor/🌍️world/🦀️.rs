//! 🌐️ Trinity Rewriting app — retained WASM canvas host + LOD scale (scene compute needing both the
//! live document AND its own view-only camera/LOD state, so — like `block`/`cad`'s app-level
//! `world.rs` precedent — this lives at app level rather than in the artifact's `🧬️schema`).

use crate::standards::v1::subsets::any::schema::{self, ApplyRuleResult, Rule};
pub use semio_framework_os_infinite::canvas;
use semio_framework_os_infinite::BoardHost;
use semio_framework_os_infinite::{compute_edge_bezier_points, distance_between, force_graph::apply_force_graph_layout_to_fixture_v1_json, BoardEngine, CanvasPalette, HandleRole};
use semio_s_artifact_trinity_jack::ast::QueryResult;
use semio_s_artifact_trinity_jack::executor::execute;
use semio_s_artifact_trinity_jack::language_service::{complete as complete_jack, parse};
use semio_s_artifact_trinity_jack::lexer::tokenize as tokenize_jack;
use semio_s_artifact_trinity_jack::{move_node, TrinityGraphMutation};
use semio_s_artifact_trinity_jack::{port_key, Graph, JackSnapshot, Node, PortDirection};
use std::cell::Cell;
use std::collections::HashMap;

use crate::TrinityRewritingError;

type TrinityBoardEngine = BoardEngine;

const TRINITY_HANDLE_RADIUS: f64 = 5.0;
const TRINITY_BOARD_PORT_HANDLE_KIND: &str = "port";
const TRINITY_DEFAULT_NODE_RADIUS: f64 = 44.0;
const TRINITY_BOARD_KIND_CATALOGS_JSON: &str = "{\"handleKinds\":[{\"id\":\"port\",\"name\":\"Port\",\"color\":\"#6b7280\"}],\"edgeKinds\":[{\"id\":\"Connection\",\"name\":\"Connection\",\"color\":\"#94a3b8\"}]}";

//#region 🔖️Lod
use semio_s_artifact_trinity_jack::editor::jack::lod::TRINITY_LOD_SCALE;

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
    let fixture = trinity_graph_to_force_layout_fixture(graph);
    let positioned = apply_force_graph_layout_to_fixture_v1_json(&pack::json_to_string(&fixture), "").map_err(TrinityRewritingError::Layout)?;
    let fixture = pack::parse_json(&positioned).map_err(|error| TrinityRewritingError::Layout(error.to_string()))?;
    apply_force_layout_positions_to_trinity_graph(graph, &fixture)
}
//#endregion 🔖️Lod

//#region 🔖️TrinityBridge
/// 🖥️ Retained trinity graph host on the directed port board engine.
pub struct TrinityBridge {
    pub graph: Graph,
    store: semio_s_artifact_trinity_jack::TrinityGraphStore,
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
    pub async fn from_graph(graph: &Graph) -> Self {
        let fixture = graph.to_fixture();
        let store = semio_s_artifact_trinity_jack::TrinityGraphStore::new(semio_s_artifact_trinity_jack::create_trinity_graph_envelope("trinity-host", fixture)).await.expect("failed to create trinity graph store");
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

    pub async fn load_fixture_json(json: &str) -> Result<Self, TrinityRewritingError> {
        let graph = Graph::load_json(json)?;
        Ok(Self::from_graph(&graph).await)
    }

    fn refresh_graph_from_store(&mut self) -> Result<(), TrinityRewritingError> {
        self.graph = Graph::from_fixture(self.store.snapshot()?)?;
        Ok(())
    }

    async fn dispatch(&mut self, operations: Vec<TrinityGraphMutation>) -> Result<(), TrinityRewritingError> {
        semio_s_artifact_trinity_jack::dispatch_trinity_graph_mutations(&mut self.store, operations).await?;
        self.refresh_graph_from_store()
    }

    pub async fn undo(&mut self) -> Result<(), TrinityRewritingError> {
        use store::ArtifactCommand;
        self.store.dispatch(ArtifactCommand::Undo).await?;
        self.refresh_graph_from_store()?;
        self.rebuild_engine();
        Ok(())
    }

    pub async fn redo(&mut self) -> Result<(), TrinityRewritingError> {
        use store::ArtifactCommand;
        self.store.dispatch(ArtifactCommand::Redo).await?;
        self.refresh_graph_from_store()?;
        self.rebuild_engine();
        Ok(())
    }

    pub async fn commit_checkpoint(&mut self, message: Option<String>) -> Result<(), TrinityRewritingError> {
        use store::ArtifactCommand;
        self.store.dispatch(ArtifactCommand::CommitCheckpoint { message, authors: Vec::new() }).await.map_err(TrinityRewritingError::from).map(|_| ())
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

    pub async fn pointer_up(&mut self, x: f64, y: f64) {
        let world = self.screen_to_world(x, y);
        self.engine.pointer_up(world.x, world.y);
        if let Err(err) = self.commit_drag_positions().await {
            eprintln!("[DEBUG] trinity drag commit failed: {err}");
        }
        self.rebuild_engine();
    }

    pub async fn reorganize(&mut self) {
        match force_layout_reposition_operations(&self.store.snapshot().unwrap_or_else(|_| self.graph.to_fixture())) {
            Ok(operations) if !operations.is_empty() => {
                if let Err(err) = self.dispatch(operations).await {
                    eprintln!("[DEBUG] trinity reorganize dispatch failed: {err}");
                    return;
                }
                self.rebuild_engine();
            }
            Ok(_) => {}
            Err(err) => eprintln!("[DEBUG] trinity reorganize force layout failed: {err}"),
        }
    }

    pub async fn run_jack(&mut self, query: &str) -> Result<QueryResult, TrinityRewritingError> {
        let parsed = parse(query).map_err(TrinityRewritingError::Jack)?;
        let (result, operations) = execute(&self.graph, &parsed).map_err(TrinityRewritingError::Jack)?;
        if !operations.is_empty() {
            self.dispatch(operations).await?;
            self.rebuild_engine();
        }
        Ok(result)
    }

    pub async fn run_jack_json(&mut self, query: &str) -> Result<String, TrinityRewritingError> {
        let result = self.run_jack(query).await?;
        Ok(pack::to_json_string(&result))
    }

    pub async fn run_jack_with_fixture_json(&mut self, query: &str) -> Result<String, TrinityRewritingError> {
        let result = self.run_jack(query).await?;
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

    pub async fn apply_rewriting_json(&mut self, rule_json: &str, bindings_json: &str) -> Result<String, TrinityRewritingError> {
        let rule: Rule = pack::from_json_str(rule_json)?;
        let bindings = schema::parse_bindings_json(bindings_json)?;
        let query = schema::build_rule_query(&rule, &bindings);
        let parsed = parse(&query).map_err(TrinityRewritingError::Jack)?;
        let (result, operations) = execute(&self.graph, &parsed).map_err(TrinityRewritingError::Jack)?;
        if !operations.is_empty() {
            self.dispatch(operations).await?;
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

    async fn commit_drag_positions(&mut self) -> Result<(), TrinityRewritingError> {
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
        self.dispatch(operations).await
    }

    fn sync_board_from_graph(&mut self) {
        let _ = self.board.set_board_kind_catalogs_from_json(TRINITY_BOARD_KIND_CATALOGS_JSON);
        let fixture = trinity_graph_to_board_fixture(&self.graph);
        if !self.board.parse_fixture_json(&pack::json_to_string(&fixture)) {
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

/// 🩹️ Delegates to `semio_s_artifact_trinity_jack::parse_port_key` (the one place the `nodeId@portId`
/// convention is owned) instead of hand-rolling a second splitter here.
fn trinity_port_endpoint_parts(endpoint: &str) -> (String, String) {
    semio_s_artifact_trinity_jack::parse_port_key(endpoint).map_or_else(|| (endpoint.to_string(), String::new()), |(n, p)| (n.to_string(), p.to_string()))
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
        let dsl::DslValue::Object(mut entries) = dsl::ToValue::to_value(&self.result) else { unreachable!("QueryResult::to_value always produces an object") };
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
#[cfg(test)]
const TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
#[cfg(test)]
const TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

#[cfg(test)]
fn trinity_rewriting_envelope_credits_are_valid(maximum_pages: usize, maximum_bytes: usize) -> bool {
    maximum_pages != 0 && maximum_pages <= TRINITY_REWRITING_ENVELOPE_MAXIMUM_PAGES && maximum_bytes != 0 && maximum_bytes <= TRINITY_REWRITING_ENVELOPE_MAXIMUM_BYTES
}

#[cfg(test)]
struct TrinityRewritingCallerPageOwner<Page> {
    page: Option<Page>,
}

#[cfg(test)]
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

#[cfg(test)]
fn trinity_rewriting_page_handle_matches(operation: u64, generation: u64, expected_operation: u64, expected_generation: u64) -> bool {
    operation == expected_operation && generation == expected_generation
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

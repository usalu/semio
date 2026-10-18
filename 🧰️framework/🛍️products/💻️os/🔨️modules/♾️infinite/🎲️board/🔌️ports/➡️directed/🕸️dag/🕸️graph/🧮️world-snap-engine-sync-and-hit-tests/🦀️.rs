//! 🧮️ World snap, engine sync and hit tests of the DAG graph: grid snap helpers, screen to world conversion, engine position and event sync and the channel, anchor, port and connection hit tests. See [`super`].

use super::*;

fn lod_visible_grid_snap_step_world(viewport: &DagViewportState, graph: &DagGraphState) -> Option<f64> {
    match draw_lod_for_frame(viewport, graph) {
        DagDrawLod::Minimap => None,
        DagDrawLod::Overview | DagDrawLod::Compact => Some(viewport.grid_step_large_world()),
        DagDrawLod::Normal => Some(viewport.grid_step_medium_world()),
        DagDrawLod::Detail => Some(viewport.grid_step_small_world()),
        DagDrawLod::Micro => Some(viewport.grid_step_micro_world()),
    }
}

fn snap_world_scalar(viewport: &DagViewportState, graph: &DagGraphState, value: f64) -> f64 {
    if !viewport.grid_snap_enabled {
        return value;
    }
    let Some(step) = lod_visible_grid_snap_step_world(viewport, graph) else {
        return value;
    };
    (value / step).round() * step
}

fn snap_world_pair(viewport: &DagViewportState, graph: &DagGraphState, x: f64, y: f64) -> (f64, f64) {
    (snap_world_scalar(viewport, graph, x), snap_world_scalar(viewport, graph, y))
}

pub fn screen_to_world_point(viewport_state: &DagViewportState, graph: &DagGraphState, sx: f64, sy: f64) -> canvas::Point {
    use canvas::camera::{screen_to_world, Camera as CanvasCamera, Viewport};
    use canvas::Point;
    let cam = CanvasCamera { x: graph.host_snapshot.camera.x, y: graph.host_snapshot.camera.y, zoom: graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: viewport_state.width, height: viewport_state.height, dpr: viewport_state.dpr };
    screen_to_world(&cam, &viewport, Point::new(sx, sy))
}

pub fn sync_node_positions_from_engine(viewport: &mut DagViewportState, graph: &mut DagGraphState) {
    for (&nid, &idx) in &graph.node_id_map {
        let Some(engine_node) = graph.engine.nodes.get(&nid) else {
            continue;
        };
        let (mut x, mut y) = (engine_node.center.x, engine_node.center.y);
        if viewport.grid_snap_enabled {
            (x, y) = snap_world_pair(viewport, graph, x, y);
        }
        graph.host_snapshot.nodes[idx].x = x;
        graph.host_snapshot.nodes[idx].y = y;
        if viewport.grid_snap_enabled {
            if let Some(engine_node) = graph.engine.nodes.get_mut(&nid) {
                engine_node.center = canvas::Point::new(x, y);
            }
        }
    }
}

pub fn process_engine_events(viewport: &mut DagViewportState, graph: &mut DagGraphState) {
    let events = graph.engine.drain_events();
    let mut moved = false;
    let mut wired = false;
    for event in events {
        match event {
            BoardEvent::NodeMoved { .. } => moved = true,
            BoardEvent::EdgeConnected { source, target, .. } => {
                wired = true;
                graph.journal_connect(source, target);
            }
            BoardEvent::EdgeRemoved { id } => {
                wired = true;
                graph.journal_disconnect(id);
            }
            BoardEvent::SelectionChanged { .. } | BoardEvent::PreselectChanged { .. } | BoardEvent::HoverChanged { .. } => {}
        }
    }
    if moved {
        sync_node_positions_from_engine(viewport, graph);
    }
    if wired {
        graph.sync_edges_from_engine();
    }
}

fn channel_row_handle_hit(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> Option<HandleId> {
    if !draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        return None;
    }
    graph.port_row_handle_hit(world_x, world_y, true, true)
}

fn handle_anchor_hit(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> Option<HandleId> {
    if !draw_lod_for_frame(viewport, graph).allows_connection_hit_picking() {
        return None;
    }
    use canvas::Point;
    let p = Point::new(world_x, world_y);
    for (&hid, handle) in graph.engine.handles.iter().rev() {
        let Some(node) = graph.engine.nodes.get(&handle.node_id) else {
            continue;
        };
        let pos = handle_position(node, handle);
        let dx = p.x - pos.x;
        let dy = p.y - pos.y;
        let tol = handle.radius + 6.0;
        if dx * dx + dy * dy <= tol * tol {
            return Some(hid);
        }
    }
    None
}

/// 🔌️ The ONE port hit — the CONNECTOR rect, which is also the one rect
/// [`Self::entity_screen_json`] publishes for `"handle"`. Grabbing and publishing used to be two
/// different shapes (a 6.5-world-unit rim disc against the whole 20-unit row), so the published
/// centre never wired at any zoom, and a radius-11 anchor disc reached past the row centre and
/// stole the node drag (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️generate-mode-interactions-2026-09-13.md` §4).
pub fn port_pointer_handle_hit(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> Option<HandleId> {
    if !draw_lod_for_frame(viewport, graph).allows_connection_hit_picking() {
        return None;
    }
    graph.port_connector_handle_hit(world_x, world_y)
}

/// 🔗️ Wire snapping belongs to a wire: while a node is being DRAGGED, snapping the pointer onto a
/// port anchor teleports the node to that anchor. Only a gesture that can still land on a port —
/// a press, or a wire in flight — is snapped.
pub fn connection_hit_world_while_wiring(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> (f64, f64) {
    if matches!(graph.engine.interaction, InteractionMode::DrawEdge { .. }) {
        return connection_hit_world(viewport, graph, world_x, world_y);
    }
    (world_x, world_y)
}

pub fn connection_hit_world(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> (f64, f64) {
    let Some(hid) = port_pointer_handle_hit(viewport, graph, world_x, world_y) else {
        return (world_x, world_y);
    };
    let Some(handle) = graph.engine.handles.get(&hid) else {
        return (world_x, world_y);
    };
    let Some(node) = graph.engine.nodes.get(&handle.node_id) else {
        return (world_x, world_y);
    };
    let pos = handle_position(node, handle);
    (pos.x, pos.y)
}

pub fn world_hits_handle(viewport: &DagViewportState, graph: &DagGraphState, world_x: f64, world_y: f64) -> bool {
    port_pointer_handle_hit(viewport, graph, world_x, world_y).is_some()
}

pub fn sync_channel_row_pointer_hover(viewport: &mut DagViewportState, graph: &mut DagGraphState, world_x: f64, world_y: f64) {
    if !draw_lod_for_frame(viewport, graph).uses_channel_row_pick() {
        return;
    }
    if !matches!(graph.engine.interaction, InteractionMode::Idle) {
        return;
    }
    if handle_anchor_hit(viewport, graph, world_x, world_y).is_some() {
        return;
    }
    if let Some(hid) = channel_row_handle_hit(viewport, graph, world_x, world_y) {
        graph.engine.hover = Some(hid);
    }
}

pub fn sync_connection_hit_picking_for_lod(viewport: &mut DagViewportState, graph: &mut DagGraphState) {
    let allow = draw_lod_for_frame(viewport, graph).allows_connection_hit_picking();
    graph.engine.handle_pointer_picking = allow;
    if !allow && matches!(graph.engine.interaction, InteractionMode::DrawEdge { .. }) {
        graph.engine.interaction = InteractionMode::Idle;
    }
}

/// 🧭️ Minimap LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
pub fn lod_uses_bounded_drag(viewport: &DagViewportState, graph: &DagGraphState) -> bool {
    matches!(draw_lod_for_frame(viewport, graph), DagDrawLod::Minimap)
}

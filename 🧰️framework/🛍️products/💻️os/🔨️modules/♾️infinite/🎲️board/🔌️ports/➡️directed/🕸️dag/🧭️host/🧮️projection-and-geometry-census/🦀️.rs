//! 🧮️ Projection and geometry census of the DAG host: the bounded node hit, the down and move projections, the screen geometry census JSON and the first screen point probe. See [`super`].

use super::*;

fn bounded_node_hit_index(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, sx: f64, sy: f64) -> Result<Option<usize>, DagInteractionPlanFault> {
    if minimap_widget_pointer_hit(viewport, graph, minimap, sx, sy).is_some() {
        return Err(DagInteractionPlanFault::Unsupported);
    }
    let world = screen_to_world_point(viewport, graph, sx, sy);
    if graph.port_insert_hit(world.x, world.y, graph.host_snapshot.camera.zoom).is_some() || world_hits_handle(viewport, graph, world.x, world.y) || graph.widget_hit_at(world.x, world.y).is_some() {
        return Err(DagInteractionPlanFault::Unsupported);
    }
    Ok(graph.snapshot_draggable_node_hit(world.x, world.y).and_then(|node_id| graph.node_id_map.get(&node_id).copied()))
}

pub(super) fn derive_projection_down(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, next: &mut DagInteractionProjection, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
    if intent.pan || intent.button == 1 {
        next.gesture = DagProjectionGesture::Pan { start_x: intent.x, start_y: intent.y, camera: next.camera };
        return Ok(());
    }
    if intent.button != 0 || intent.alt {
        return Err(DagInteractionPlanFault::Unsupported);
    }
    let Some(index) = bounded_node_hit_index(viewport, graph, minimap, intent.x, intent.y)? else {
        next.gesture = DagProjectionGesture::Select { start_x: intent.x, start_y: intent.y, initial: next.selected };
        return Ok(());
    };
    if intent.ctrl_or_meta {
        let selected = !dag_bit_contains(&next.selected, index);
        dag_bit_set(&mut next.selected, index, selected);
        if !selected {
            next.gesture = DagProjectionGesture::Idle;
            return Ok(());
        }
    } else if intent.shift {
        dag_bit_set(&mut next.selected, index, true);
    } else if !dag_bit_contains(&next.selected, index) {
        next.selected = [0; DAG_INTERACTION_WORD_CAPACITY];
        dag_bit_set(&mut next.selected, index, true);
    }
    next.hover = u16::try_from(index).ok();
    let mut starts = [None; DAG_INTERACTION_NODE_CAPACITY];
    let mut len = 0usize;
    for node_index in 0..graph.host_snapshot.nodes.len() {
        if !dag_bit_contains(&next.selected, node_index) {
            continue;
        }
        let node = &graph.host_snapshot.nodes[node_index];
        starts[len] = Some(DagNodeMove { index: node_index as u16, x: node.x, y: node.y });
        len += 1;
    }
    next.gesture = DagProjectionGesture::Drag { press: next.revision, start_x: intent.x, start_y: intent.y, starts, len: len as u16 };
    Ok(())
}

pub(super) fn derive_projection_move(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, next: &mut DagInteractionProjection, moves: &mut [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY], move_len: &mut u16, drag: &mut Option<DagDragOffset>, intent: DagPointerIntent) -> Result<(), DagInteractionPlanFault> {
    match next.gesture {
        DagProjectionGesture::Pan { start_x, start_y, camera } => {
            let zoom = camera[2].max(1e-9);
            next.camera = [camera[0] - (intent.x - start_x) / zoom, camera[1] - (intent.y - start_y) / zoom, camera[2]];
        }
        DagProjectionGesture::Drag { press, start_x, start_y, starts, len } => {
            let zoom = next.camera[2].max(1e-9);
            let dx = (intent.x - start_x) / zoom;
            let dy = (intent.y - start_y) / zoom;
            *drag = (dx != 0.0 || dy != 0.0).then_some(DagDragOffset { press, dx, dy });
            for index in 0..usize::from(len) {
                let Some(start) = starts[index] else {
                    continue;
                };
                moves[index] = Some(DagNodeMove { index: start.index, x: start.x + dx, y: start.y + dy });
            }
            *move_len = len;
        }
        DagProjectionGesture::Select { start_x, start_y, initial } => {
            let min_x = start_x.min(intent.x);
            let max_x = start_x.max(intent.x);
            let min_y = start_y.min(intent.y);
            let max_y = start_y.max(intent.y);
            let mut selected = if intent.shift || intent.ctrl_or_meta { initial } else { [0; DAG_INTERACTION_WORD_CAPACITY] };
            for index in 0..graph.host_snapshot.nodes.len() {
                let node = &graph.host_snapshot.nodes[index];
                let screen = world_to_screen_point(viewport, graph, node.x, node.y);
                if screen.0 >= min_x && screen.0 <= max_x && screen.1 >= min_y && screen.1 <= max_y {
                    dag_bit_set(&mut selected, index, true);
                }
            }
            next.selected = selected;
        }
        DagProjectionGesture::Idle => {
            next.hover = bounded_node_hit_index(viewport, graph, minimap, intent.x, intent.y)?.and_then(|index| u16::try_from(index).ok());
        }
    }
    Ok(())
}

/// 🗺️ Every node and port this graph paints, each with the SCREEN geometry
/// [`Self::entity_screen_json`] publishes for it — one census of where everything IS, for a
/// renderer diagnostic and for a browser probe that would otherwise have to FIND the graph by
/// sweeping the pointer across it. A sweep is not a cheap way to ask: measured on the wgpu serve,
/// a 224-point hover sweep spends the host's whole attention budget and leaves nothing for the
/// gesture it was sweeping to find (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// Rects are SURFACE-local, in the same coordinates `pointer_*_screen` takes.
pub fn screen_geometry_census_json(viewport: &DagViewportState, graph: &DagGraphState, minimap_state: &DagMinimapState) -> String {
    let mut rows: Vec<String> = Vec::new();
    for node in &graph.host_snapshot.nodes {
        let body = match entity_screen_rect(viewport, graph, "node", &node.id) {
            Some(rect) => first_screen_point_in(viewport, graph, minimap_state, rect, |hit| hit.is_draggable_body() && hit.node_id.as_deref() == Some(node.id.as_str())),
            None => None,
        };
        let body_json = body.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"));
        rows.push(format!("{{\"kind\":\"node\",\"id\":{},\"body\":{body_json},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&node.id), entity_screen_json(viewport, graph, "node", &node.id)));
        for (port, direction) in node.inputs().iter().map(|port| (port, "in")).chain(node.outputs().iter().map(|port| (port, "out"))) {
            let channel = format!("{}@{}", node.id, port.id);
            rows.push(format!("{{\"kind\":\"handle\",\"direction\":\"{direction}\",\"id\":{},\"geometry\":{}}}", semio_framework_pack_json::to_json_string(&channel), entity_screen_json(viewport, graph, "handle", &channel)));
        }
    }
    let panel = [0.0, 0.0, viewport.width as f64, viewport.height as f64];
    let navigate = first_screen_point_in(viewport, graph, minimap_state, panel, |hit| hit.minimap && !hit.minimap_viewport);
    rows.push(format!("{{\"kind\":\"minimap\",\"navigate\":{}}}", navigate.map_or_else(|| "null".to_string(), |(x, y)| format!("[{x:.1},{y:.1}]"))));
    format!("[{}]", rows.join(","))
}

/// 🔎️ The first point of a screen rect whose classification satisfies `wanted`, scanned on a fixed
/// grid inset from the rim. Which part of a node is draggable body and which is inline widget is
/// the ENGINE's answer, and this is how it is asked without a pointer ever moving.
fn first_screen_point_in(viewport: &DagViewportState, graph: &DagGraphState, minimap: &DagMinimapState, rect: [f64; 4], wanted: impl Fn(&DagScreenHit) -> bool) -> Option<(f64, f64)> {
    const STEPS: usize = 12;
    for row in 0..STEPS {
        for column in 0..STEPS {
            let x = rect[0] + rect[2] * (0.06 + 0.88 * column as f64 / (STEPS - 1) as f64);
            let y = rect[1] + rect[3] * (0.06 + 0.88 * row as f64 / (STEPS - 1) as f64);
            if x < 0.0 || y < 0.0 || x > viewport.width as f64 || y > viewport.height as f64 {
                continue;
            }
            if wanted(&screen_hit(viewport, graph, minimap, x, y)) {
                return Some((x, y));
            }
        }
    }
    None
}

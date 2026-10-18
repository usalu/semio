//! 🔘️ Handle geometry of the board scene: the handle positions JSON, effective scale and radius, world positions, cap attachment and the indirect handle marker radius. See [`super`].

use super::*;

/// 🩺️ Every handle the pane can actually be POINTED AT, as one bounded probe row:
/// world position, owning node, handle kind, and whether the handle is still free (no incident
/// edge — the precondition `openHandleSuggestions` and `createEdge` both need). `data-board-
/// positions-json` carries NODES only, so nothing in the DOM ever named a handle and no
/// headless caller could aim at one.
///
/// 🧱️ Bounded twice over, because Nakagin carries 358 handles: the row set is the camera's own
/// viewport (a handle off-screen cannot be clicked, so publishing it buys nothing) and then a
/// hard [`BOARD_HANDLE_VITALS_CAP`]. `total`/`onScreen`/`capped` say exactly what was left out.
pub fn handle_positions_json(scene: &BoardSceneState, style: &BoardStyleState) -> String {
    let (w, h) = (f64::from(scene.width), f64::from(scene.height));
    let mut rows: Vec<(&str, Point, &str, &str, bool)> = Vec::new();
    let mut on_screen = 0usize;
    for (id, handle) in &scene.handles {
        let Some(world) = handle_world_pos(scene, style, handle) else { continue };
        let screen = scene.world_to_screen(world);
        if screen.x < 0.0 || screen.y < 0.0 || screen.x > w || screen.y > h || !scene.handle_effectively_visible(id.as_str()) {
            continue;
        }
        on_screen += 1;
        if rows.len() < BOARD_HANDLE_VITALS_CAP {
            rows.push((id.as_str(), world, handle.node_id.as_str(), handle.handle_kind.as_str(), !scene.handle_has_incident_edge(id.as_str())));
        }
    }
    let quote = |value: &str, out: &mut String| {
        out.push('"');
        out.push_str(&value.replace('\\', "\\\\").replace('"', "\\\""));
        out.push('"');
    };
    let mut out = String::from("{\"total\":");
    out.push_str(&scene.handles.len().to_string());
    out.push_str(",\"onScreen\":");
    out.push_str(&on_screen.to_string());
    out.push_str(",\"published\":");
    out.push_str(&rows.len().to_string());
    out.push_str(",\"capped\":");
    out.push_str(if on_screen > rows.len() { "true" } else { "false" });
    out.push_str(",\"rows\":[");
    for (index, (id, world, node_id, handle_kind, open)) in rows.iter().copied().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('[');
        quote(id, &mut out);
        out.push(',');
        out.push_str(&world.x.to_string());
        out.push(',');
        out.push_str(&world.y.to_string());
        out.push(',');
        quote(node_id, &mut out);
        out.push(',');
        quote(handle_kind, &mut out);
        out.push(',');
        out.push_str(if open { "true" } else { "false" });
        out.push(']');
    }
    out.push_str("]}");
    out
}

fn effective_handle_scale(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    let node_scale = scene.nodes.get(h.node_id.as_str()).map_or(1.0, |n| style.effective_node_scale(n));
    (node_scale * h.scale * style.handle_kind_scale(h.handle_kind.as_str())).max(1e-9)
}

pub fn effective_handle_radius(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    h.radius * effective_handle_scale(scene, style, h)
}

pub fn handle_world_pos(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<Point> {
    let n = scene.nodes.get(&h.node_id)?;
    Some(match n.shape {
        NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), style.scaled_node_radius(n), h.angle),
        NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), style.scaled_node_width(n), style.scaled_node_height(n), h.angle),
    })
}

/// 🎯️ Where an edge meets a handle: the cap's outer peak, the outward normal, and the node center.
pub fn handle_cap_attachment(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<(Point, Vec2, Point)> {
    let rim = handle_world_pos(scene, style, h)?;
    let n = scene.nodes.get(&h.node_id)?;
    let node_center = Point::new(n.x, n.y);
    let outward = handle_outward_at_node_rim(rim, node_center, n.shape, style.scaled_node_radius(n), style.scaled_node_width(n), style.scaled_node_height(n))?;
    let peak = handle_exterior_cap_peak(rim, outward, effective_handle_radius(scene, style, h));
    Some((peak, outward, node_center))
}

/// 📐️ Ghost link handles sit on a rim offset by `INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× node half-extent from the node body so ring spacing scales with the node at every zoom.
pub fn indirect_handle_world_pos(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> Option<Point> {
    let n = scene.nodes.get(&h.node_id)?;
    let offset = style.indirect_handle_ring_offset_world(n);
    Some(match n.shape {
        NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), style.scaled_node_radius(n) + offset, h.angle),
        NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), style.scaled_node_width(n) + 2.0 * offset, style.scaled_node_height(n) + 2.0 * offset, h.angle),
    })
}

/// 📐️ Indirect-connect marker radius in world units: `INDIRECT_HANDLE_MARKER_NODE_SCALE`× circle radius or × half the shorter rectangle side.
pub fn indirect_handle_marker_radius_world(scene: &BoardSceneState, style: &BoardStyleState, h: &HandleData) -> f64 {
    let Some(n) = scene.nodes.get(&h.node_id) else {
        return (effective_handle_radius(scene, style, h) * INDIRECT_HANDLE_MARKER_NODE_SCALE).max(1e-9);
    };
    let handle_local_scale = (h.scale * style.handle_kind_scale(h.handle_kind.as_str())).max(1e-9);
    (style.indirect_node_half_extent(n) * INDIRECT_HANDLE_MARKER_NODE_SCALE * handle_local_scale).max(1e-9)
}

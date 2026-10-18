//! 🔗️ Link compatibility and curves of the board scene: the compatible link targets and handles, the free handle probes, edge and wire curves, the level of detail gates and node hits. See [`super`].

use super::*;

/// 🧭️ Visible target node ids that expose at least one free handle compatible with `source_handle_id`.
pub fn link_drag_compatible_target_node_ids(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str) -> Vec<String> {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return Vec::new();
    };
    let source_node_id = source.node_id.as_str();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for (hid, h) in &scene.handles {
        if h.node_id == source_node_id || !scene.handle_effectively_visible(hid.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(hid.as_str()) {
            continue;
        }
        if !handles_link_compatible_for_drag(scene, style, source, h) {
            continue;
        }
        if !scene.nodes.get(&h.node_id).is_some_and(|n| n.visible) {
            continue;
        }
        if seen.insert(h.node_id.clone()) {
            out.push(h.node_id.clone());
        }
    }
    out.sort();
    out
}

/// 🧭️ Count of visible free handles on `node_id` compatible with `source_handle_id`.
pub fn link_compatible_handle_count_on_node(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, node_id: &str) -> usize {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return 0;
    };
    if source.node_id == node_id {
        return 0;
    }
    scene.handles.iter().filter(|(id, h)| h.node_id == node_id && handle_eligible_link_target_ring(scene, style, id.as_str(), source_handle_id) && handles_link_compatible_for_drag(scene, style, source, h)).count()
}

/// 🧭️ Free compatible handle ids on `node_id` for an active link from `source_handle_id`.
pub fn link_compatible_handle_ids_on_node(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, node_id: &str) -> Vec<String> {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return Vec::new();
    };
    let mut out: Vec<String> = scene
        .handles
        .iter()
        .filter_map(|(id, h)| {
            if h.node_id != node_id {
                return None;
            }
            if !handle_eligible_link_target_ring(scene, style, id.as_str(), source_handle_id) {
                return None;
            }
            handles_link_compatible_for_drag(scene, style, source, h).then(|| id.clone())
        })
        .collect();
    out.sort();
    out
}

/// 🧭️ Compatible target node under `world` while a link wire is active (node body hit).
pub fn link_drag_ring_target_node_id(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, world: Point) -> Option<String> {
    let nid = resolve_node_hit_world(scene, style, world)?;
    if scene.handles.get(source_handle_id)?.node_id == nid {
        return None;
    }
    node_has_any_free_link_compatible_handle(scene, style, source_handle_id, nid.as_str()).then_some(nid)
}

/// 🧭️ When the drop target has exactly one free handle compatible with `source_handle_id`, returns that handle id (otherwise `None`).
pub fn node_sole_free_link_compatible_handle(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, target_node_id: &str) -> Option<String> {
    let source = scene.handles.get(source_handle_id)?;
    if source.node_id == target_node_id {
        return None;
    }
    let mut found: Option<String> = None;
    for (id, h) in &scene.handles {
        if h.node_id != target_node_id || !scene.handle_effectively_visible(id.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(id.as_str()) {
            continue;
        }
        if !handles_link_compatible_for_drag(scene, style, source, h) {
            continue;
        }
        if found.is_some() {
            return None;
        }
        found = Some(id.clone());
    }
    found
}

/// 🧭️ True when `target_node_id` hosts at least one visible free handle that can pair with `source_handle_id` under link-compat rules.
pub fn node_has_any_free_link_compatible_handle(scene: &BoardSceneState, style: &BoardStyleState, source_handle_id: &str, target_node_id: &str) -> bool {
    let Some(source) = scene.handles.get(source_handle_id) else {
        return false;
    };
    if source.node_id == target_node_id {
        return false;
    }
    for (hid, h) in &scene.handles {
        if h.node_id != target_node_id || !scene.handle_effectively_visible(hid.as_str()) {
            continue;
        }
        if scene.handle_has_incident_edge(hid.as_str()) {
            continue;
        }
        if handles_link_compatible_for_drag(scene, style, source, h) {
            return true;
        }
    }
    false
}

/// 💫️ True when the handle may appear on a link-target ghost ring (`overview`/`normal` LOD).
pub fn handle_eligible_link_target_ring(scene: &BoardSceneState, style: &BoardStyleState, handle_id: &str, source_handle_id: &str) -> bool {
    if !scene.handle_effectively_visible(handle_id) || scene.handle_has_incident_edge(handle_id) {
        return false;
    }
    let Some(source) = scene.handles.get(source_handle_id) else {
        return false;
    };
    let Some(target) = scene.handles.get(handle_id) else {
        return false;
    };
    if source.node_id == target.node_id {
        return false;
    }
    handles_link_compatible_for_drag(scene, style, source, target)
}

pub fn board_edge_curve(scene: &BoardSceneState, style: &BoardStyleState, e: &EdgeData) -> Option<CubicBez> {
    if !style.has_ports() {
        let source_node = scene.nodes.get(&e.source)?;
        let target_node = scene.nodes.get(&e.target)?;
        let source_center = Point::new(source_node.x, source_node.y);
        let target_center = Point::new(target_node.x, target_node.y);
        let source_pos = style.node_rim_point_toward(source_node, target_center);
        let target_pos = style.node_rim_point_toward(target_node, source_center);
        return Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center));
    }
    let source_handle = scene.handles.get(&e.source)?;
    let target_handle = scene.handles.get(&e.target)?;
    let (source_pos, source_out, _) = handle_cap_attachment(scene, style, source_handle)?;
    let (target_pos, target_out, _) = handle_cap_attachment(scene, style, target_handle)?;
    Some(compute_edge_bezier_outward(source_pos, target_pos, source_out, target_out))
}

pub fn link_drag_wire_curve_world(scene: &BoardSceneState, style: &BoardStyleState, source_id: &str, target_id: Option<&str>, end_world: Point) -> Option<CubicBez> {
    let source_handle = scene.handles.get(source_id)?;
    let (source_pos, source_out, _) = handle_cap_attachment(scene, style, source_handle)?;
    let (target_pos, target_out) = if let Some(tid) = target_id {
        let (peak, outward, _) = handle_cap_attachment(scene, style, scene.handles.get(tid)?)?;
        (peak, outward)
    } else {
        (end_world, Vec2::ZERO)
    };
    Some(compute_edge_bezier_outward(source_pos, target_pos, source_out, target_out))
}

pub fn wire_curve(scene: &BoardSceneState, style_state: &BoardStyleState, w: &WireData) -> Option<CubicBez> {
    let end_world = match (&w.target, w.end_x, w.end_y) {
        (None, Some(x), Some(y)) if x.is_finite() && y.is_finite() => Point::new(x, y),
        (Some(tid), _, _) => {
            scene.handles.get(tid)?;
            return board_edge_curve(scene, style_state, &EdgeData {
                id: w.id.clone(),
                source: w.source.clone(),
                target: tid.clone(),
                selected: w.selected,
                visible: w.visible,
                locked: w.locked,
                style: w.style.clone(),
                edge_kind: String::new(),
                source_tip: None,
                target_tip: None,
                properties: w.properties.clone(),
            });
        }
        _ => return None,
    };
    link_drag_wire_curve_world(scene, style_state, w.source.as_str(), None, end_world)
}

/// 🧭️ Minimap/overview LOD: group selection and bounded drag only — no per-node/edge/handle picks.
pub fn lod_disables_discrete_pick(scene: &BoardSceneState, style: &BoardStyleState) -> bool {
    matches!(current_draw_lod(scene, style), BoardDrawLod::Minimap | BoardDrawLod::Overview)
}

/// 🧭️ Minimap/overview LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
pub fn board_lod_uses_bounded_drag(scene: &BoardSceneState, style: &BoardStyleState) -> bool {
    matches!(current_draw_lod(scene, style), BoardDrawLod::Minimap | BoardDrawLod::Overview)
}

pub fn resolve_node_hit_world(scene: &BoardSceneState, style: &BoardStyleState, point: Point) -> Option<String> {
    for n in scene.nodes.values().rev() {
        if !n.visible {
            continue;
        }
        match n.shape {
            NodeShape::Rectangle => {
                let hw = style.scaled_node_width(n) / 2.0;
                let hh = style.scaled_node_height(n) / 2.0;
                if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                    return Some(n.id.clone());
                }
            }
            NodeShape::Circle => {
                if distance_between(point, Point::new(n.x, n.y)) <= style.scaled_node_radius(n) {
                    return Some(n.id.clone());
                }
            }
        }
    }
    None
}

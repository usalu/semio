//! 🔗️ Link gesture and hit resolution of the board focus: link gesture events, the active link wire curve and the world hover and hit resolution. See [`super`].

use super::*;

pub fn sync_link_gesture_events(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState) {
    if let Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } = &focus.interaction {
        if compatible_node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
            outbox.event_schema_fault = true;
            return;
        }
        let compat_key = format!("{}|{}", source_id, compatible_node_ids.join(","));
        let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
        let compat_changed = outbox.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
        let ring_changed = outbox.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
        let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(source_id, compatible_node_ids));
        let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), ring_handle_ids));
        let (first, second) = match (compat_event, ring_event) {
            (Some(first), second) => (first, second),
            (None, Some(first)) => (first, None),
            (None, None) => return,
        };
        let Some(reservation) = outbox.reserve_owned_batch(first, second) else {
            return;
        };
        if compat_changed {
            outbox.link_compat_nodes_emit_key = Some(compat_key);
        }
        if ring_changed {
            outbox.link_target_ring_emit_key = Some(ring_key);
        }
        outbox.publish_event_batch(reservation);
        return;
    }
    let Some(source) = focus.active_link_source_handle_id().map(str::to_string) else {
        outbox.clear_link_gesture_events();
        return;
    };
    let node_ids = link_drag_compatible_target_node_ids(scene, style, &source);
    if node_ids.len() > BOARD_POINTER_ITEM_CAPACITY {
        outbox.event_schema_fault = true;
        return;
    }
    let compat_key = format!("{}|{}", source, node_ids.join(","));
    let (ring_node_id, ring_handle_ids) = link_target_ring_snapshot(scene, focus, style, &source);
    if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
        outbox.event_schema_fault = true;
        return;
    }
    let ring_key = format!("{}|{}|{}", source, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
    let compat_changed = outbox.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
    let ring_changed = outbox.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
    let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(&source, &node_ids));
    let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(&source, ring_node_id.as_deref(), &ring_handle_ids));
    let (first, second) = match (compat_event, ring_event) {
        (Some(first), second) => (first, second),
        (None, Some(first)) => (first, None),
        (None, None) => return,
    };
    let Some(reservation) = outbox.reserve_owned_batch(first, second) else {
        return;
    };
    if compat_changed {
        outbox.link_compat_nodes_emit_key = Some(compat_key);
    }
    if ring_changed {
        outbox.link_target_ring_emit_key = Some(ring_key);
    }
    outbox.publish_event_batch(reservation);
}

pub fn active_link_wire_curve(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState) -> Option<CubicBez> {
    match &focus.interaction {
        Interaction::LinkDragSnap { source_id, target_id, end_world } => link_drag_wire_curve_world(scene, style, source_id.as_str(), target_id.as_deref(), *end_world),
        Interaction::LinkTargetNode { source_id, target_node_id } => link_drag_wire_curve_world(scene, style, source_id.as_str(), None, scene.node_center_world(target_node_id)?),
        Interaction::ExternalLinkPreview { source_id, end_world, .. } => link_drag_wire_curve_world(scene, style, source_id.as_str(), None, *end_world),
        Interaction::DragNodes { proximity_pair: Some((src, tgt)), .. } => link_drag_wire_curve_world(scene, style, src.as_str(), Some(tgt.as_str()), Point::ZERO),
        _ => None,
    }
}

/// 🔗️ Overview LOD: tight world-radius hit on a free handle so link drag can start without enabling broad `resolve_hit_world` handle picks.
pub fn resolve_overview_free_link_handle_pointer_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    if !matches!(current_draw_lod(scene, style), BoardDrawLod::Overview) {
        return None;
    }
    if !focus.selection_options.select_handles {
        return None;
    }
    const MAX_D_WORLD: f64 = 2.25;
    let mut best: Option<(f64, String)> = None;
    for h in scene.handles.values() {
        if !scene.handle_effectively_visible(h.id.as_str()) || scene.handle_has_incident_edge(h.id.as_str()) {
            continue;
        }
        let Some(pos) = handle_world_pos(scene, style, h) else {
            continue;
        };
        let d = distance_between(point, pos);
        if d <= MAX_D_WORLD && best.as_ref().is_none_or(|(bd, _)| d < *bd) {
            best = Some((d, h.id.clone()));
        }
    }
    best.map(|(_, id)| id)
}

pub fn resolve_hover_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    let lod = current_draw_lod(scene, style);
    let zoom = scene.camera.zoom;
    if style.has_ports() && !matches!(lod, BoardDrawLod::Minimap) {
        if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_link_target(scene, focus, style, point) {
                return Some(hid);
            }
            if let Interaction::LinkDragSnap { source_id, .. } = &focus.interaction {
                if let Some(hid) = link_drag_target_ring_hit(scene, focus, style, source_id, point) {
                    return Some(hid);
                }
            }
        }
        if let Some(ring_node_id) = indirect_ring_node_id(scene, focus, style, lod) {
            for h in scene.handles.values().rev() {
                if h.node_id != ring_node_id || !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                if !indirect_ring_handle_eligible(scene, focus, style, h.id.as_str(), ring_node_id.as_str()) {
                    continue;
                }
                let Some(pos) = indirect_handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
            for h in scene.handles.values().rev() {
                if !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                let Some(pos) = handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + effective_handle_radius(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_idle_selected_node(scene, focus, style, point) {
                if scene.handle_selectable(hid.as_str()) {
                    return Some(hid);
                }
            }
        }
    }
    for n in scene.nodes.values().rev() {
        if !scene.node_selectable(n.id.as_str()) {
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
    for w in scene.wires.values().rev() {
        if !scene.wire_selectable(w) {
            continue;
        }
        if let Some(c) = wire_curve(scene, style, w) {
            if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                return Some(w.id.clone());
            }
        }
    }
    for e in scene.edges.values().rev() {
        if !edge_selectable(scene, style, e) {
            continue;
        }
        if let Some(c) = board_edge_curve(scene, style, e) {
            if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                return Some(e.id.clone());
            }
        }
    }
    None
}

pub fn resolve_hit_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    if lod_disables_discrete_pick(scene, style) {
        return None;
    }
    let zoom = scene.camera.zoom;
    let o = &focus.selection_options;
    if style.has_ports() && o.select_handles {
        if matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_link_target(scene, focus, style, point) {
                return Some(hid);
            }
            if let Interaction::LinkDragSnap { source_id, .. } = &focus.interaction {
                if let Some(hid) = link_drag_target_ring_hit(scene, focus, style, source_id, point) {
                    return Some(hid);
                }
            }
        }
        if let Some(ring_node_id) = indirect_ring_node_id(scene, focus, style, current_draw_lod(scene, style)) {
            for h in scene.handles.values().rev() {
                if h.node_id != ring_node_id || !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                if !indirect_ring_handle_eligible(scene, focus, style, h.id.as_str(), ring_node_id.as_str()) {
                    continue;
                }
                let Some(pos) = indirect_handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(current_draw_lod(scene, style), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
            for h in scene.handles.values().rev() {
                if !scene.handle_selectable(h.id.as_str()) {
                    continue;
                }
                let Some(pos) = handle_world_pos(scene, style, h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + effective_handle_radius(scene, style, h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
        }
        if matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
            if let Some(hid) = sole_indirect_handle_hit_idle_selected_node(scene, focus, style, point) {
                return Some(hid);
            }
        }
    }
    if o.select_nodes {
        for n in scene.nodes.values().rev() {
            if !scene.node_selectable(n.id.as_str()) {
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
    }
    if o.select_edges {
        for e in scene.edges.values().rev() {
            if !edge_selectable(scene, style, e) {
                continue;
            }
            if let Some(c) = board_edge_curve(scene, style, e) {
                if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                    return Some(e.id.clone());
                }
            }
        }
    }
    None
}

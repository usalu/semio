//! 🔘️ Indirect ring and link targets of the board focus: the indirect ring node, sole handle hits, link target ring hits and the ring snapshot. See [`super`].

use super::*;

/// 🧭️ Resolves which single node draws the overview/normal indirect handle ring when that node has **more than one** eligible free handles (otherwise the sole handle is implicit).
pub fn indirect_ring_node_id(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, lod: BoardDrawLod) -> Option<String> {
    if !matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
        return None;
    }
    if let Interaction::LinkTargetNode { source_id, target_node_id } = &focus.interaction {
        if link_compatible_handle_count_on_node(scene, style, source_id, target_node_id) > 1 {
            return scene.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
        }
        return None;
    }
    if let Interaction::ExternalLinkPreview { ring_node_id: Some(target_node_id), ring_handle_ids, .. } = &focus.interaction {
        if ring_handle_ids.len() > 1 {
            return scene.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
        }
        return None;
    }
    if let Interaction::LinkDragSnap { source_id, end_world, .. } = &focus.interaction {
        let ring_nid = link_drag_ring_target_node_id(scene, style, source_id, *end_world)?;
        if link_compatible_handle_count_on_node(scene, style, source_id, ring_nid.as_str()) > 1 {
            return Some(ring_nid);
        }
        return None;
    }
    if focus.active_link_source_handle_id().is_some() {
        return None;
    }
    let ring_nid = if focus.selection.len() == 1 {
        focus.selection.iter().next()?.clone()
    } else {
        return None;
    };
    let n = scene.nodes.get(&ring_nid).filter(|n| n.visible)?;
    if scene.eligible_indirect_handle_count_on_node(n.id.as_str()) > 1 {
        Some(ring_nid)
    } else {
        None
    }
}

pub(super) fn sole_indirect_handle_hit_link_target(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    let Interaction::LinkTargetNode { source_id, target_node_id } = &focus.interaction else {
        return None;
    };
    let th = node_sole_free_link_compatible_handle(scene, style, source_id, target_node_id)?;
    let n = scene.nodes.get(target_node_id)?;
    if !n.visible {
        return None;
    }
    if !style.point_in_node_world(n, point) {
        return None;
    }
    Some(th)
}

pub(super) fn sole_indirect_handle_hit_idle_selected_node(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, point: Point) -> Option<String> {
    if !matches!(focus.interaction, Interaction::None) {
        return None;
    }
    if focus.selection.len() != 1 {
        return None;
    }
    let nid = focus.selection.iter().next()?;
    if !scene.nodes.contains_key(nid) {
        return None;
    }
    let sole = scene.sole_eligible_indirect_handle_on_node(nid)?;
    let n = scene.nodes.get(nid)?;
    if !n.visible {
        return None;
    }
    if !style.point_in_node_world(n, point) {
        return None;
    }
    Some(sole)
}

pub(crate) fn indirect_ring_handle_eligible(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, handle_id: &str, ring_node_id: &str) -> bool {
    if scene.handles.get(handle_id).is_none_or(|h| h.node_id != ring_node_id) {
        return false;
    }
    if let Some(source_id) = focus.active_link_source_handle_id() {
        handle_eligible_link_target_ring(scene, style, handle_id, source_id)
    } else {
        scene.handle_eligible_indirect_connect_ring(handle_id)
    }
}

pub(super) fn link_drag_target_ring_hit(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, source_id: &str, point: Point) -> Option<String> {
    if !matches!(current_draw_lod(scene, style), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
        return None;
    }
    if let Interaction::ExternalLinkPreview { source_id: active_source, ring_handle_ids, .. } = &focus.interaction {
        if active_source != source_id {
            return None;
        }
        let zoom = scene.camera.zoom;
        for hid in ring_handle_ids {
            let Some(h) = scene.handles.get(hid) else {
                continue;
            };
            if !handle_eligible_link_target_ring(scene, style, h.id.as_str(), source_id) {
                continue;
            }
            let Some(pos) = indirect_handle_world_pos(scene, style, h) else {
                continue;
            };
            let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
            if distance_between(point, pos) <= tol {
                return Some(h.id.clone());
            }
        }
        return None;
    }
    let node_id = link_drag_ring_target_node_id(scene, style, source_id, point)?;
    if link_compatible_handle_count_on_node(scene, style, source_id, node_id.as_str()) <= 1 {
        return None;
    }
    let zoom = scene.camera.zoom;
    for h in scene.handles.values().rev() {
        if h.node_id != node_id || !handle_eligible_link_target_ring(scene, style, h.id.as_str(), source_id) {
            continue;
        }
        let Some(pos) = indirect_handle_world_pos(scene, style, h) else { continue };
        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h);
        if distance_between(point, pos) <= tol {
            return Some(h.id.clone());
        }
    }
    None
}

pub(super) fn link_target_ring_snapshot(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, source_handle_id: &str) -> (Option<String>, Vec<String>) {
    if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = &focus.interaction {
        if source_id == source_handle_id {
            return (ring_node_id.clone(), ring_handle_ids.clone());
        }
    }
    let node_id = match &focus.interaction {
        Interaction::LinkTargetNode { target_node_id, .. } => Some(target_node_id.clone()),
        Interaction::LinkDragSnap { end_world, .. } => link_drag_ring_target_node_id(scene, style, source_handle_id, *end_world),
        _ => None,
    };
    let Some(nid) = node_id else {
        return (None, Vec::new());
    };
    if link_compatible_handle_count_on_node(scene, style, source_handle_id, nid.as_str()) <= 1 {
        return (None, Vec::new());
    }
    (Some(nid.clone()), link_compatible_handle_ids_on_node(scene, style, source_handle_id, nid.as_str()))
}

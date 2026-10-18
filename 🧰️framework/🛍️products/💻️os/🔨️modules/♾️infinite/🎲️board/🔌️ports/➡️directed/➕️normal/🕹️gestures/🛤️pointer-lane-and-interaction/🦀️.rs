//! 🛤️ Pointer lane and interaction of the board gestures: the event authority emptiness, the pointer lane gates, the interaction JSON and the world pick target resolution. See [`super`].

use super::*;

pub fn event_authority_terminal_is_empty(outbox: &BoardOutboxState, commit: &BoardCommitState) -> bool {
    commit.pending_delete_planning.is_none() && commit.pending_delete_operation.is_none() && outbox.event_overflow.is_none() && outbox.event_batch_overflow.is_none() && outbox.events.terminal_is_empty()
}

/// 🖐️ True from the press that opens a pointer lane — a pending or live area select, or any gesture
/// [`Self::defers_descriptor_sync_from_js`] names — until its release, so a host keeps routing that release to the board
/// even when it lands outside the surface.
pub fn pointer_lane_in_flight(focus: &BoardFocusState, gestures: &BoardGesturesState) -> bool {
    defers_descriptor_sync_from_js(focus, gestures) || matches!(&focus.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. })
}

/// 🎛️ True while the pointer drives a lane [`Self::plan_pointer`] does not model — a gumball transform drag (the
/// rotate ring's `rotate` record), a target-region body drag or grip resize, or anything under the area brush. A host
/// routes those moves and releases through [`Self::pointer_move_screen`] / [`Self::pointer_up_screen`], exactly as
/// React's Board2dHost routes every pointer, and publishes the events they queue.
pub fn pointer_lane_is_direct(focus: &BoardFocusState, gestures: &BoardGesturesState) -> bool {
    focus.active_utility == ActiveUtility::AreaBrush || gestures.region_drag.is_some() || gestures.transform_drag.is_some()
}

/// 🧿️ True during area select, link gestures, node drag, the rotate-ring gesture, or camera pan so JS can defer full `syncDescriptorJson` round-trips.
pub fn defers_descriptor_sync_from_js(focus: &BoardFocusState, gestures: &BoardGesturesState) -> bool {
    gestures.transform_drag.is_some()
        || gestures.region_drag.is_some()
        || gestures.region_paint.is_some()
        || matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. } | Interaction::DragNodes { .. } | Interaction::Pan { .. })
}

/// 🩺️ What the engine is doing right now, as one probe-readable row: the live gesture,
/// the armed utility, the hovered id and the selection/preselect sizes. Published by the React
/// host as `data-board-interaction-json`, the board twin of `World3dHost`'s `data-interaction-json`.
pub fn interaction_json(scene: &BoardSceneState, focus: &BoardFocusState, gestures: &BoardGesturesState) -> String {
    let mode = if gestures.region_paint.is_some() {
        "regionPaint"
    } else if gestures.region_drag.is_some() {
        "regionDrag"
    } else if gestures.transform_drag.is_some() {
        "transformRotate"
    } else {
        match focus.interaction {
            Interaction::None => "none",
            Interaction::Pan { .. } => "pan",
            Interaction::DragNodes { .. } => "dragNodes",
            Interaction::SelectionPending { .. } => "selectionPending",
            Interaction::Selection { .. } => "selection",
            Interaction::LinkAtSourceHandle { .. } => "linkAtSourceHandle",
            Interaction::LinkDragSnap { .. } => "linkDragSnap",
            Interaction::LinkTargetNode { .. } => "linkTargetNode",
            Interaction::ExternalLinkPreview { .. } => "externalLinkPreview",
        }
    };
    let utility = match focus.active_utility {
        ActiveUtility::Select => "select",
        ActiveUtility::Brush => "brush",
        ActiveUtility::AreaBrush => "areaBrush",
    };
    let mut out = String::from("{\"mode\":\"");
    out.push_str(mode);
    out.push_str("\",\"utility\":\"");
    out.push_str(utility);
    out.push_str("\",\"hoveredId\":");
    match focus.hovered_id.as_deref() {
        Some(id) => {
            out.push('"');
            out.push_str(&id.replace('\\', "\\\\").replace('"', "\\\""));
            out.push('"');
        }
        None => out.push_str("null"),
    }
    out.push_str(",\"selectionCount\":");
    out.push_str(&focus.selection.len().to_string());
    out.push_str(",\"preselectCount\":");
    out.push_str(&focus.preselect.len().to_string());
    out.push_str(",\"revision\":");
    out.push_str(&scene.interaction_revision.to_string());
    out.push_str(",\"deferringDescriptorSync\":");
    out.push_str(if defers_descriptor_sync_from_js(focus, gestures) { "true" } else { "false" });
    out.push('}');
    out
}

/// 🎯️ The handle the pointer's own hit test resolves leads — including the indirect-ring handles
/// Overview/Compact draw, which the per-LOD loop below never visits — so a right-click names exactly
/// the handle the hover paints, not the node underneath it.
///
/// 🎯️ Regions are the least specific target on the board — last in the list, so a context
/// menu offers the graph first and the backdrop only when nothing else is under the point.
pub fn resolve_pick_targets_world(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, gestures: &BoardGesturesState, point: Point) -> Vec<BoardPickTargetJson> {
    let mut out = Vec::new();
    let lod = current_draw_lod(scene, style);
    let zoom = scene.camera.zoom;
    if let Some(id) = resolve_hit_world(scene, focus, style, point).filter(|id| scene.handles.contains_key(id.as_str())) {
        push_pick_target(&mut out, "handle", id.clone(), 2, Some(id));
    }
    if style.has_ports() && !matches!(lod, BoardDrawLod::Minimap) && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
        for h in scene.handles.values().rev() {
            if !scene.handle_selectable(h.id.as_str()) {
                continue;
            }
            let Some(pos) = handle_world_pos(scene, style, h) else { continue };
            let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + effective_handle_radius(scene, style, h);
            if distance_between(point, pos) <= tol {
                push_pick_target(&mut out, "handle", h.id.clone(), 2, Some(h.id.clone()));
            }
        }
    }
    for n in scene.nodes.values().rev() {
        if !scene.node_selectable(n.id.as_str()) {
            continue;
        }
        let hit = match n.shape {
            NodeShape::Rectangle => {
                let hw = style.scaled_node_width(n) / 2.0;
                let hh = style.scaled_node_height(n) / 2.0;
                (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
            }
            NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= style.scaled_node_radius(n),
        };
        if hit {
            push_pick_target(&mut out, "node", n.id.clone(), 0, n.text.clone());
        }
    }
    for w in scene.wires.values().rev() {
        if !scene.wire_selectable(w) {
            continue;
        }
        if let Some(c) = wire_curve(scene, style, w) {
            if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                push_pick_target(&mut out, "wire", w.id.clone(), 1, Some(w.id.clone()));
            }
        }
    }
    for e in scene.edges.values().rev() {
        if !edge_selectable(scene, style, e) {
            continue;
        }
        if let Some(c) = board_edge_curve(scene, style, e) {
            if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                push_pick_target(&mut out, "edge", e.id.clone(), 1, Some(e.id.clone()));
            }
        }
    }
    for region in scene.regions.values() {
        if region.hidden {
            continue;
        }
        if region_grip_at(gestures.region_live_bounds(region), zoom, point).is_some() {
            push_pick_target(&mut out, "region", region.id.clone(), 3, region.label.clone().or_else(|| Some(region.id.clone())));
        }
    }
    out
}

//! 🕹️ Normal board brush, gesture, commit and closing components with their gesture and commit functions.
//!
//! Crate `semio-framework-os-infinite-board-normal-gestures`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, Selection};
use semio_framework_os_infinite_board_engine::{ActiveUtility, Edge, EdgeData, HandleData, Interaction, merge_pick_into_selection, NodeData, NodeKindHandleTemplate, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionGrip, rotate_point_about, snap_region_scalar, snap_transform_angle, transform_ring_angle_delta, TransformGumballFlags, WireData};
use semio_framework_os_infinite_board_normal_model::{BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, BoardDrawLod, BoardEventFault, BoardEventKind, BoardGestureMotion, BoardOwnedEvent, BoardPayloadBuilder, BoardPickTargetJson, BoardRegionDrag, BoardRegionPaint, BoardTransformDrag, brush_template_world_pos, BrushCandidatePage, BrushCandidateRef, BrushPreviewSnapshot, DropPreviewSnapshot, EDGE_HIT_TOLERANCE_PX, gesture_release_events, HANDLE_HIT_TOLERANCE_PX, NormalPortError, push_pick_target, region_paint_bounds, WorldContentCache};
use semio_framework_os_infinite_board_normal_scene::{admitted_board_pointer_id, board_edge_curve, BoardDeleteKind, BoardDeletePlan, BoardDeletePlanningOperation, BoardDeletePlanningPhase, BoardPointerPlan, BoardSceneState, BoardStyleState, brush_compatible_candidates, brush_handle_anchor_world, current_draw_lod, edge_selectable, effective_handle_radius, handle_cap_attachment, handle_world_pos, indirect_handle_marker_radius_world, paint_highlighted_node_preview, region_snap_step, snap_region_point, wire_curve};
use semio_framework_os_infinite_board_normal_focus::{BoardFocusState, BoardOutboxState, resolve_hit_world, restore_gesture_selection, set_hovered_id, set_selection_ids_gestured, stage_selection_ids};
use semio_framework_canvas::{Affine, CubicBez, FillRule, OpaqueSceneRetirementStep, OpaqueSceneRetirementToken, Point, Rect, Scene, Stroke, Vec2};
use std::collections::{BTreeSet, HashMap};
use std::cell::Cell;
use graph::{Handle, Node, NodeShape};
use semio_framework_graph_manifest::PropertyValue;
use semio_framework_graph_drawing::routing::{compute_edge_bezier_outward, handle_exterior_cap_peak, handle_outward_at_node_rim};
use geometry::{distance_between, distance_point_to_cubic_bezier};

/// 🔗️ A handle that already carries an edge is not open, so it grows nothing. The pointer
/// path filters those out before it ever enters a slot (`brush_nearest_slot_source`); the
/// direct entry points (`brush_open_slot`, `brush_target_slot`) reach this instead, and the
/// slot must then resolve an EMPTY page rather than a wrong one — that empty page is what a
/// fully fastened document's suggestions popup reads as "no placement available".
pub fn brush_enter_slot(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState, source_handle_id: &str) {
    if brush.brush_slot_source_id.as_deref() == Some(source_handle_id) {
        return;
    }
    if brush.brush_slot_source_id.is_some() {
        brush_finish_slot(scene, focus, style, brush, outbox);
    }
    brush.brush_slot_source_id = Some(source_handle_id.to_string());
    let fastened = scene.handle_has_incident_edge(source_handle_id);
    let Some(source) = scene.handles.get(source_handle_id).filter(|_| !fastened).cloned() else {
        brush.brush_candidates.clear();
        brush.brush_candidate_index = 0;
        brush_rebuild_preview(scene, style, brush, outbox);
        return;
    };
    let Some(candidates) = brush_compatible_candidates(scene, style, &source) else {
        outbox.event_schema_fault = true;
        return;
    };
    brush.brush_candidates = candidates;
    brush.brush_candidate_index = 0;
    brush_rebuild_preview(scene, style, brush, outbox);
}

pub fn brush_rebuild_preview(scene: &mut BoardSceneState, style: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState) {
    let Some(ref source_id) = brush.brush_slot_source_id else {
        brush.brush_preview = None;
        brush_sync_preview_events(brush, outbox);
        return;
    };
    let preview = brush.brush_candidates.get(brush.brush_candidate_index).and_then(|candidate| brush_build_preview(scene, style, brush, source_id, candidate));
    brush.brush_preview = preview;
    if brush.brush_preview.is_some() {
        scene.bump_content_scene_generation();
    }
    brush.brush_preview_emit_key = None;
    brush.brush_candidates_emit_key = None;
    brush_sync_preview_events(brush, outbox);
}

pub fn brush_pointer_move(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState, world: Point) {
    if let Some(slot) = brush_nearest_slot_source(scene, style, brush, world) {
        brush_enter_slot(scene, focus, style, brush, outbox, &slot);
        set_hovered_id(scene, focus, outbox, brush.brush_slot_source_id.clone());
    } else if brush.brush_slot_source_id.is_some() {
        brush_finish_slot(scene, focus, style, brush, outbox);
        set_hovered_id(scene, focus, outbox, None);
    }
}

fn drop_preview_effective_dims(style: &BoardStyleState, brush: &BoardBrushState, preview: &DropPreviewSnapshot) -> (NodeShape, f64, f64, f64) {
    if let Some(kind) = style.node_kinds.get(preview.node_kind_id.as_str()) {
        let radius = brush.brush_node_size * 0.5 * kind.scale;
        let (width, height) = if kind.shape == NodeShape::Rectangle { (brush.brush_node_size * kind.scale, brush.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
        return (kind.shape, radius, width, height);
    }
    (preview.shape, preview.radius, preview.width, preview.height)
}

/// 👻️ Sets or clears the workbench palette snapshot drop ghost node (independent of brush utility).
pub fn set_drop_preview_json(scene: &mut BoardSceneState, brush: &mut BoardBrushState, json: &str) -> Result<(), NormalPortError> {
    if json.trim().is_empty() {
        brush.drop_preview = None;
        scene.bump_content_scene_generation();
        return Ok(());
    }
    let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::DropPreviewJson)?;
    brush.drop_preview = scene.drop_preview_from_json(&v);
    if brush.drop_preview.is_none() {
        return Err(NormalPortError::DropPreviewInvalid);
    }
    scene.bump_content_scene_generation();
    Ok(())
}

pub fn append_drop_preview_paint(scene_state: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
    let Some(ref preview) = brush.drop_preview else {
        return;
    };
    let (shape, radius, width, height) = drop_preview_effective_dims(style, brush, preview);
    let icon_kind = preview.icon_kind.as_deref().filter(|s| !s.is_empty()).or_else(|| style.node_kinds.get(preview.node_kind_id.as_str()).and_then(|k| k.icon.as_deref()));
    paint_highlighted_node_preview(scene_state, style, scene, lod, preview.x, preview.y, shape, radius, width, height, icon_kind, world_space);
}

pub fn append_brush_preview_paint(scene_state: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
    let Some(ref preview) = brush.brush_preview else {
        return;
    };
    let _ = lod;
    paint_highlighted_node_preview(scene_state, style, scene, lod, preview.x, preview.y, preview.shape, preview.radius, preview.width, preview.height, preview.icon_kind.as_deref(), world_space);
    let center = Point::new(preview.x, preview.y);
    let source = match scene_state.handles.get(preview.source_handle_id.as_str()) {
        Some(h) => h,
        None => return,
    };
    let (src_pos, src_out, _) = match handle_cap_attachment(scene_state, style, source) {
        Some(attachment) => attachment,
        None => return,
    };
    let tmpl = match preview.handles.get(preview.target_handle_index) {
        Some(t) => t,
        None => return,
    };
    let tgt_rim = brush_template_world_pos(center, preview.shape, preview.radius, preview.width, preview.height, tmpl.angle);
    let tgt_radius = tmpl.radius.filter(|radius| radius.is_finite() && *radius > 0.0).unwrap_or(ui_styling::radii::HANDLE_DEFAULT);
    let Some(tgt_out) = handle_outward_at_node_rim(tgt_rim, center, preview.shape, preview.radius, preview.width, preview.height) else {
        return;
    };
    let tgt_pos = handle_exterior_cap_peak(tgt_rim, tgt_out, tgt_radius);
    let curve = compute_edge_bezier_outward(src_pos, tgt_pos, src_out, tgt_out);
    let p0 = scene_state.draw_space_point(curve.p0(), world_space);
    let p1 = scene_state.draw_space_point(curve.p1(), world_space);
    let p2 = scene_state.draw_space_point(curve.p2(), world_space);
    let p3 = scene_state.draw_space_point(curve.p3(), world_space);
    let bez = CubicBez::new(p0, p1, p2, p3);
    scene.stroke(&Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT), Affine::IDENTITY, style.canvas_theme.wire_stroke_highlighted, None, &bez);
}

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

impl BoardClosingState {
    pub fn advance_opaque_scene_retirement_step(&self) -> bool {
        let Some(token) = self.opaque_scene_retirement.get() else {
            return true;
        };
        match semio_framework_canvas::advance_opaque_scene_retirement(token, 1, 4096) {
            OpaqueSceneRetirementStep::Blocked | OpaqueSceneRetirementStep::Pending { .. } => false,
            OpaqueSceneRetirementStep::Complete { .. } => {
                self.opaque_scene_retirement.set(None);
                true
            }
            OpaqueSceneRetirementStep::Fault => {
                self.opaque_scene_fault.set(true);
                false
            }
        }
    }

    pub fn publish_opaque_scene_retirement(&self, token: OpaqueSceneRetirementToken, scene: Scene) {
        assert!(self.opaque_scene_retirement.get().is_none(), "board retains at most one exact opaque scene retirement");
        semio_framework_canvas::publish_opaque_scene_retirement(token, scene);
        self.opaque_scene_retirement.set(Some(token));
    }

    pub fn opaque_scene_faulted(&self) -> bool {
        self.opaque_scene_fault.get()
    }

    pub fn push_close_string(&mut self, value: String) {
        let index = usize::from(self.close_string_len);
        assert!(index < self.close_strings.len(), "board close string scratch is schema bounded");
        self.close_strings[index] = Some(value);
        self.close_string_len += 1;
    }

    pub fn push_close_optional_string(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.push_close_string(value);
        }
    }

    pub fn close_scratch_step(&mut self) -> bool {
        if self.close_node_handles.is_some() {
            let handle = self.close_node_handles.as_mut().and_then(Vec::pop);
            if let Some(handle) = handle {
                self.push_close_string(handle.handle_kind);
                return true;
            }
            self.close_node_handles = None;
            return true;
        }
        if self.close_string_len > 0 {
            self.close_string_len -= 1;
            drop(self.close_strings[usize::from(self.close_string_len)].take());
            return true;
        }
        false
    }
}

pub fn quarantine_world_content_step(scene_state: &mut BoardSceneState, closing: &mut BoardClosingState) -> bool {
    if closing.opaque_scene_retirement.get().is_some() {
        closing.advance_opaque_scene_retirement_step();
        return false;
    }
    let mut cache = scene_state.world_content_cache.borrow_mut();
    if cache.is_none() {
        return true;
    }
    let Some(token) = semio_framework_canvas::reserve_opaque_scene_retirement() else {
        closing.opaque_scene_fault.set(true);
        return false;
    };
    let WorldContentCache { scene, .. } = cache.take().expect("world content cache was witnessed occupied");
    closing.publish_opaque_scene_retirement(token, scene);
    false
}

impl BoardBrushState {
    fn brush_slot_hit_radius_world(&self) -> f64 {
        (self.brush_node_size * 0.5).max(1.0)
    }

    fn brush_effective_suggestion_offset(&self) -> f64 {
        if self.brush_alt_pressed || self.brush_slot_suggestions_active {
            self.suggestion_offset
        } else {
            0.0
        }
    }
}

fn brush_slot_center_world(scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, h: &HandleData) -> Option<Point> {
    let hw = brush_handle_anchor_world(scene, style, h)?;
    scene.handle_slot_center_world(h.node_id.as_str(), hw, brush.brush_effective_suggestion_offset())
}

/// 🖌️ World distance from pointer to brush slot when the pointer is on the slot, anchor, or sole-free node body.
fn brush_slot_pointer_hit_distance(scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, world: Point, handle_id: &str, h: &HandleData) -> Option<f64> {
    let slot_center = brush_slot_center_world(scene, style, brush, h)?;
    let zoom = scene.camera.zoom.max(1e-9);
    let slot_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + brush.brush_slot_hit_radius_world();
    let d_slot = distance_between(world, slot_center);
    if d_slot <= slot_hit_r {
        return Some(d_slot);
    }
    let anchor = brush_handle_anchor_world(scene, style, h)?;
    let anchor_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + indirect_handle_marker_radius_world(scene, style, h).max(effective_handle_radius(scene, style, h));
    if distance_between(world, anchor) <= anchor_hit_r {
        return Some(d_slot);
    }
    if scene.sole_eligible_indirect_handle_on_node(&h.node_id).as_deref() == Some(handle_id) {
        let n = scene.nodes.get(&h.node_id)?;
        if style.point_in_node_world(n, world) {
            return Some(d_slot);
        }
    }
    None
}

pub fn brush_nearest_slot_source(scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, world: Point) -> Option<String> {
    let mut best: Option<(f64, String)> = None;
    for (hid, h) in &scene.handles {
        if !scene.handle_effectively_visible(hid.as_str()) || scene.handle_has_incident_edge(hid.as_str()) {
            continue;
        }
        let Some(d) = brush_slot_pointer_hit_distance(scene, style, brush, world, hid.as_str(), h) else {
            continue;
        };
        if best.as_ref().is_none_or(|(bd, _)| d < *bd) {
            best = Some((d, hid.clone()));
        }
    }
    best.map(|(_, id)| id)
}

pub(crate) fn brush_build_preview(scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, source_handle_id: &str, candidate: BrushCandidateRef<'_>) -> Option<BrushPreviewSnapshot> {
    brush_build_preview_with_offset(scene, style, brush, source_handle_id, candidate, brush.brush_effective_suggestion_offset())
}

pub fn brush_build_preview_with_offset(scene: &BoardSceneState, style: &BoardStyleState, brush: &BoardBrushState, source_handle_id: &str, candidate: BrushCandidateRef<'_>, offset: f64) -> Option<BrushPreviewSnapshot> {
    let source = scene.handles.get(source_handle_id)?;
    let kind = style.node_kinds.get(candidate.node_kind_id)?;
    let center = scene.handle_slot_center_world(&source.node_id, brush_handle_anchor_world(scene, style, source)?, offset)?;
    let target_handle_index = candidate.target_handle_index;
    kind.handles.get(target_handle_index)?;
    let node_kind_id = candidate.node_kind_id;
    let radius = brush.brush_node_size * 0.5 * kind.scale;
    let (width, height) = if kind.shape == NodeShape::Rectangle { (brush.brush_node_size * kind.scale, brush.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
    Some(BrushPreviewSnapshot {
        source_handle_id: source_handle_id.to_string(),
        node_kind_id: node_kind_id.to_string(),
        x: center.x,
        y: center.y,
        shape: kind.shape,
        radius,
        width,
        height,
        handles: kind.handles.clone(),
        target_handle_index,
        icon_kind: kind.icon.clone(),
    })
}

pub fn brush_sync_preview_events(brush: &mut BoardBrushState, outbox: &mut BoardOutboxState) {
    if brush.brush_candidates.len() > BOARD_POINTER_ITEM_CAPACITY || brush.brush_preview.as_ref().is_some_and(|preview| preview.handles.len() > BOARD_POINTER_ITEM_CAPACITY) {
        outbox.event_schema_fault = true;
        return;
    }
    let key = brush.brush_preview.as_ref().map(|p| format!("{}|{}|{}|{}|{}", p.source_handle_id, p.node_kind_id, p.target_handle_index, p.x, p.y)).unwrap_or_default();
    let mut candidates_key = format!("{}|", brush.brush_slot_source_id.as_deref().unwrap_or(""));
    for (index, candidate) in brush.brush_candidates.iter().enumerate() {
        if index > 0 {
            candidates_key.push(',');
        }
        candidates_key.push_str(candidate.node_kind_id);
        candidates_key.push('#');
        candidates_key.push_str(&candidate.target_handle_index.to_string());
    }
    candidates_key.push('|');
    candidates_key.push_str(&brush.brush_candidate_index.to_string());
    let preview_changed = brush.brush_preview_emit_key.as_deref() != Some(key.as_str());
    let candidates_changed = brush.brush_candidates_emit_key.as_deref() != Some(candidates_key.as_str());
    let preview_event = preview_changed.then(|| BoardOwnedEvent::brush_preview(brush.brush_preview.as_ref()));
    let candidate_event = candidates_changed.then(|| BoardOwnedEvent::brush_candidates(brush.brush_slot_source_id.as_deref().unwrap_or(""), &brush.brush_candidates, brush.brush_candidate_index, brush.brush_slot_suggestions_active));
    let (first, second) = match (preview_event, candidate_event) {
        (Some(first), second) => (first, second),
        (None, Some(first)) => (first, None),
        (None, None) => return,
    };
    let Some(reservation) = outbox.reserve_owned_batch(first, second) else {
        return;
    };
    if preview_changed {
        brush.brush_preview_emit_key = Some(key);
    }
    if candidates_changed {
        brush.brush_candidates_emit_key = Some(candidates_key);
    }
    outbox.publish_event_batch(reservation);
}

pub fn brush_clear_slot(scene: &mut BoardSceneState, focus: &mut BoardFocusState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState) {
    let had_preview = brush.brush_preview.is_some();
    let clear_hover = brush.brush_slot_suggestions_active;
    brush.brush_slot_suggestions_active = false;
    brush.brush_slot_source_id = None;
    brush.brush_candidates.clear();
    brush.brush_candidate_index = 0;
    brush.brush_preview = None;
    if clear_hover {
        set_hovered_id(scene, focus, outbox, None);
    }
    if had_preview {
        scene.bump_content_scene_generation();
        brush.brush_preview_emit_key = None;
        brush.brush_candidates_emit_key = None;
        brush_sync_preview_events(brush, outbox);
    }
}

pub fn brush_commit_preview(scene: &mut BoardSceneState, style: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState) {
    let Some(preview) = brush.brush_preview.as_ref() else {
        return;
    };
    let serial = brush.brush_placement_serial.wrapping_add(1);
    let node_id = format!("{}{serial}", style.brush_node_prefix());
    let edge_id = format!("{}{serial}", style.brush_edge_prefix());
    let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::brush_place(preview, &node_id, &edge_id)) else {
        return;
    };
    brush.brush_placement_serial = serial;
    let _ = brush.brush_preview.take();
    outbox.publish_event_reservation(reservation);
    scene.bump_content_scene_generation();
    brush.brush_preview_emit_key = None;
}

pub fn brush_finish_slot(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState) {
    if brush.brush_alt_pressed {
        brush_commit_preview(scene, style, brush, outbox);
    }
    brush_clear_slot(scene, focus, brush, outbox);
}

pub fn brush_update_alt(scene: &mut BoardSceneState, style: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState, alt: bool) {
    if brush.brush_alt_pressed == alt {
        return;
    }
    brush.brush_alt_pressed = alt;
    if brush.brush_slot_source_id.is_some() {
        brush.brush_preview_emit_key = None;
        brush_rebuild_preview(scene, style, brush, outbox);
    }
}

impl BoardGesturesState {
    pub fn transform_flags(&self) -> TransformGumballFlags {
        self.transform_flags
    }

    //#region 🎯️TargetRegions
    /// 🖍️ World width/height an area-brush CLICK paints; a non-finite or non-positive axis
    /// keeps the previous extent rather than minting a zero-area region no fill could ever satisfy.
    pub fn set_area_brush_extent(&mut self, width: f64, height: f64) {
        let (mut w, mut h) = self.area_brush_extent;
        if width.is_finite() && width > 0.0 {
            w = width;
        }
        if height.is_finite() && height > 0.0 {
            h = height;
        }
        if (w - self.area_brush_extent.0).abs() < 1e-9 && (h - self.area_brush_extent.1).abs() < 1e-9 {
            return;
        }
        self.area_brush_extent = (w, h);
    }

    pub fn area_brush_extent(&self) -> (f64, f64) {
        self.area_brush_extent
    }

    /// 📐️ Live bounds of one region: the gesture's preview while it is the dragged one, the
    /// document's rectangle otherwise.
    pub(crate) fn region_live_bounds(&self, region: &RegionData) -> [f64; 4] {
        match self.region_drag.as_ref().filter(|drag| drag.id == region.id) {
            Some(drag) => drag.bounds,
            None => region_bounds(region.x, region.y, region.width, region.height),
        }
    }
}

//#region 🕹️TransformGumball
/// 🕹️ Composes which gumball handles the select utility offers. `move` is the native
/// node drag (always available while it is on), `rotate` draws and arms the ring.
pub fn set_transform_flags(scene: &mut BoardSceneState, gestures: &mut BoardGesturesState, move_enabled: bool, rotate_enabled: bool) {
    let next = TransformGumballFlags { move_enabled, rotate_enabled };
    if gestures.transform_flags == next {
        return;
    }
    if !rotate_enabled {
        cancel_transform_drag(scene, gestures);
    }
    gestures.transform_flags = next;
    scene.bump_content_scene_generation();
}

/// 👁️ Re-derives the whole preview from the grab-time snapshot: positions orbit the
/// pivot and every handle angle turns with its node, exactly like the guest reducer the
/// release commits to. Emits nothing — a rotate is ONE document edit, on release.
///
/// 🪞️ A preview frame is expendable: a refused reservation drops the row instead of
/// claiming overflow credit, so a fast drag can never fault the event terminal.
pub fn update_transform_drag(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, gestures: &mut BoardGesturesState, world: Point, snap: bool) {
    let Some(mut drag) = gestures.transform_drag.take() else {
        return;
    };
    drag.radians = snap_transform_angle(transform_ring_angle_delta(drag.pivot, drag.grab, world), snap);
    for (id, (x0, y0)) in &drag.start_positions {
        let rotated = rotate_point_about(drag.pivot, Point::new(*x0, *y0), drag.radians);
        if let Some(node) = scene.nodes.get_mut(id) {
            node.x = rotated.x;
            node.y = rotated.y;
        }
    }
    for (id, angle0) in &drag.start_handle_angles {
        if let Some(handle) = scene.handles.get_mut(id) {
            handle.angle = angle0 + drag.radians;
        }
    }
    let preview = BoardOwnedEvent::transform_preview(drag.start_positions.keys().filter_map(|id| scene.nodes.get(id).map(|node| (id.as_str(), node.x, node.y))));
    gestures.transform_drag = Some(drag);
    scene.bump_content_scene_generation();
    if let Ok(event) = preview {
        if let Ok(reservation) = outbox.events.reserve_event(event) {
            outbox.publish_event_reservation(reservation);
        }
    }
}

/// 🏁️ Ends the gesture with ONE `rotate` gesture record carrying the turned ids, the pivot and the
/// angle — however many preview frames the drag painted. A zero-angle release (a click on the ring)
/// commits nothing.
pub fn commit_transform_drag(outbox: &mut BoardOutboxState, gestures: &mut BoardGesturesState) -> bool {
    let Some(drag) = gestures.transform_drag.take() else {
        return false;
    };
    let record = (drag.radians != 0.0).then(|| BoardOwnedEvent::gesture(&drag.gesture.id, drag.start_positions.keys().map(String::as_str), BoardGestureMotion::Rotate { pivot: drag.pivot, angle: drag.radians }, &[]));
    outbox.publish_gesture_release(&drag.gesture, record);
    true
}

/// ↩️ Restores the exact pre-gesture geometry — Escape, pointer-leave and switching the
/// rotate flag off all abandon a live ring drag without touching the document.
pub fn cancel_transform_drag(scene: &mut BoardSceneState, gestures: &mut BoardGesturesState) -> bool {
    let Some(drag) = gestures.transform_drag.take() else {
        return false;
    };
    for (id, (x0, y0)) in &drag.start_positions {
        if let Some(node) = scene.nodes.get_mut(id) {
            node.x = *x0;
            node.y = *y0;
        }
    }
    for (id, angle0) in &drag.start_handle_angles {
        if let Some(handle) = scene.handles.get_mut(id) {
            handle.angle = *angle0;
        }
    }
    scene.bump_content_scene_generation();
    true
}

/// 🩺️ Every region this board holds, in id order, as the probe reads them out of
/// `data-board-target-regions-json`. Bounds are normalized and reflect a live drag, so the
/// attribute and the pixels never disagree mid-gesture.
pub fn target_regions_json(scene: &BoardSceneState, gestures: &BoardGesturesState) -> String {
    let mut out = String::from("[");
    for (index, region) in scene.regions.values().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let [min_x, min_y, max_x, max_y] = gestures.region_live_bounds(region);
        out.push_str("{\"id\":\"");
        out.push_str(&region.id.replace('\\', "\\\\").replace('"', "\\\""));
        out.push_str("\",\"x\":");
        out.push_str(&min_x.to_string());
        out.push_str(",\"y\":");
        out.push_str(&min_y.to_string());
        out.push_str(",\"width\":");
        out.push_str(&(max_x - min_x).to_string());
        out.push_str(",\"height\":");
        out.push_str(&(max_y - min_y).to_string());
        if let Some(label) = region.label.as_deref() {
            out.push_str(",\"label\":\"");
            out.push_str(&label.replace('\\', "\\\\").replace('"', "\\\""));
            out.push('"');
        }
        out.push_str(",\"hidden\":");
        out.push_str(if region.hidden { "true" } else { "false" });
        out.push_str(",\"locked\":");
        out.push_str(if region.locked { "true" } else { "false" });
        out.push_str(",\"selected\":");
        out.push_str(if region.selected { "true" } else { "false" });
        out.push('}');
    }
    out.push(']');
    out
}

/// 🎯️ The region grip under `world`, asked ONLY after the nodes, handles, edges and the
/// rotate ring have all missed — a region is the board's backdrop, never a lid over its graph.
/// A hidden region is not paint and therefore not a pick target either.
///
/// 🤏️ A real grip always outranks a body: overlapping rectangles must still be resizable
/// at the edge of the one on top, otherwise a region enclosing another would swallow it.
fn region_grip_hit_world(scene: &BoardSceneState, gestures: &BoardGesturesState, world: Point) -> Option<(String, RegionGrip)> {
    let zoom = scene.camera.zoom;
    let mut best: Option<(String, RegionGrip)> = None;
    for region in scene.regions.values() {
        if region.hidden {
            continue;
        }
        let Some(grip) = region_grip_at(gestures.region_live_bounds(region), zoom, world) else { continue };
        let replaces = match best.as_ref() {
            None => true,
            Some((_, previous)) => *previous == RegionGrip::Body || grip != RegionGrip::Body,
        };
        if replaces {
            best = Some((region.id.clone(), grip));
        }
    }
    best
}

/// 🚚️ Grabs a region: the body moves it, a corner/edge grip resizes it. A LOCKED region
/// refuses the grab outright — it still paints and still selects, it simply cannot be dragged.
pub fn try_begin_region_drag_at(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, gestures: &mut BoardGesturesState, world: Point) -> bool {
    let Some((id, grip)) = region_grip_hit_world(scene, gestures, world) else {
        return false;
    };
    let Some((start_bounds, locked)) = scene.regions.get(&id).map(|region| (region_bounds(region.x, region.y, region.width, region.height), region.locked)) else {
        return false;
    };
    let ids: Vec<String> = merge_pick_into_selection(&focus.selection, &id, "replace").into_iter().collect();
    if locked {
        set_selection_ids_gestured(scene, focus, outbox, &ids, None);
        focus.interaction = Interaction::None;
        set_hovered_id(scene, focus, outbox, None);
        return true;
    }
    let mut gesture = outbox.begin_gesture();
    stage_selection_ids(scene, focus, outbox, &mut gesture, &ids, None);
    gestures.region_drag = Some(BoardRegionDrag { gesture, id, grip, grab: world, start_bounds, bounds: start_bounds, delta: Vec2::ZERO });
    set_hovered_id(scene, focus, outbox, None);
    true
}

/// 👁️ Re-derives the whole preview from the grab-time rectangle, so the drag never
/// accumulates float drift. Emits nothing: a region gesture is ONE document edit, on release.
pub fn update_region_drag(scene: &mut BoardSceneState, style: &mut BoardStyleState, gestures: &mut BoardGesturesState, world: Point) {
    let Some(mut drag) = gestures.region_drag.take() else {
        return;
    };
    let step = region_snap_step(scene, style);
    let dx = snap_region_scalar(world.x - drag.grab.x, step);
    let dy = snap_region_scalar(world.y - drag.grab.y, step);
    drag.bounds = region_grip_drag(drag.start_bounds, drag.grip, dx, dy);
    drag.delta = Vec2::new(dx, dy);
    gestures.region_drag = Some(drag);
    scene.bump_content_scene_generation();
}

/// 🏁️ Ends a region gesture as ONE batch with the `select` its press staged: a body drag is a `drag`
/// gesture record by the snapped offset, a grip drag one `regionResize`. A release that moved nothing
/// publishes only the selection.
pub fn commit_region_drag(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, gestures: &mut BoardGesturesState) -> bool {
    let Some(drag) = gestures.region_drag.take() else {
        return false;
    };
    let moved = drag.bounds != drag.start_bounds;
    if moved {
        if let Some(region) = scene.regions.get_mut(&drag.id) {
            region.x = drag.bounds[0];
            region.y = drag.bounds[1];
            region.width = drag.bounds[2] - drag.bounds[0];
            region.height = drag.bounds[3] - drag.bounds[1];
        }
        scene.bump_content_scene_generation();
    }
    match (moved, drag.grip) {
        (false, _) => outbox.publish_gesture_release(&drag.gesture, None),
        (true, RegionGrip::Body) => {
            let motion = BoardGestureMotion::Drag { dx: drag.delta.x, dy: drag.delta.y };
            outbox.publish_gesture_release(&drag.gesture, Some(BoardOwnedEvent::gesture(&drag.gesture.id, [drag.id.as_str()], motion, &[])));
        }
        (true, _) => {
            let [select, _] = gesture_release_events(&drag.gesture, None);
            let resize = BoardOwnedEvent::region_resize(&drag.id, drag.bounds);
            let reservation = match select {
                Some(select) => outbox.reserve_owned_batch(select, Some(resize)),
                None => outbox.reserve_owned_batch(resize, None),
            };
            if let Some(reservation) = reservation {
                outbox.publish_event_batch(reservation);
            }
        }
    }
    true
}

/// ↩️ Abandons a live region gesture with zero trace: the rectangle and the selection its press staged
/// return, and the document is never touched.
pub fn cancel_region_drag(scene: &mut BoardSceneState, focus: &mut BoardFocusState, gestures: &mut BoardGesturesState) -> bool {
    let Some(drag) = gestures.region_drag.take() else {
        return false;
    };
    restore_gesture_selection(scene, focus, drag.gesture);
    scene.bump_content_scene_generation();
    true
}

pub fn begin_region_paint(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState, gestures: &mut BoardGesturesState, world: Point) {
    let anchor = snap_region_point(scene, style, world);
    gestures.region_paint = Some(BoardRegionPaint { anchor, corner: anchor, dragged: false });
    set_hovered_id(scene, focus, outbox, None);
    scene.bump_content_scene_generation();
}

pub fn update_region_paint(scene: &mut BoardSceneState, style: &mut BoardStyleState, gestures: &mut BoardGesturesState, world: Point) {
    let Some(mut paint) = gestures.region_paint.take() else {
        return;
    };
    paint.corner = snap_region_point(scene, style, world);
    paint.dragged = (paint.corner.x - paint.anchor.x).abs() > 1e-9 || (paint.corner.y - paint.anchor.y).abs() > 1e-9;
    gestures.region_paint = Some(paint);
    scene.bump_content_scene_generation();
}

/// 🏁️ Ends an area-brush gesture with exactly ONE `regionCreate` row. A rectangle that
/// collapsed on either axis is refused rather than published — the same floor a resize honours.
pub fn commit_region_paint(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, gestures: &mut BoardGesturesState) -> bool {
    let Some(paint) = gestures.region_paint.take() else {
        return false;
    };
    scene.bump_content_scene_generation();
    let bounds = region_paint_bounds(&paint, gestures.area_brush_extent);
    if bounds[2] - bounds[0] < REGION_MIN_EXTENT_WORLD || bounds[3] - bounds[1] < REGION_MIN_EXTENT_WORLD {
        return true;
    }
    let Some(reservation) = outbox.reserve_owned_event(BoardOwnedEvent::region_create(bounds)) else {
        return true;
    };
    outbox.publish_event_reservation(reservation);
    true
}

pub fn cancel_region_paint(scene: &mut BoardSceneState, gestures: &mut BoardGesturesState) -> bool {
    if gestures.region_paint.take().is_none() {
        return false;
    }
    scene.bump_content_scene_generation();
    true
}

/// 🎯️ Paints every visible region — translucent fill, outline, corner grips on the
/// selected one, and a corner tag for a labelled one. Appended BEFORE the node fill layer, so
/// a region is always the backdrop of the graph and never covers an entity.
///
/// The board engine owns no text primitive, so the `label` is a corner tag here and the string
/// itself reaches the reader through [`Self::target_regions_json`] and the inspector.
pub fn append_target_region_paint(scene_state: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, gestures: &BoardGesturesState, scene: &mut Scene, world_space: bool) {
    let zoom = scene_state.camera.zoom.max(1e-9);
    let grip = REGION_GRIP_PX / zoom;
    let inset = REGION_LABEL_INSET_PX / zoom;
    for region in scene_state.regions.values() {
        if region.hidden {
            continue;
        }
        let [min_x, min_y, max_x, max_y] = gestures.region_live_bounds(region);
        let hovered = focus.hovered_id.as_deref() == Some(region.id.as_str());
        let dragging = gestures.region_drag.as_ref().is_some_and(|drag| drag.id == region.id);
        let stroke_color = if region.selected || dragging {
            style.canvas_theme.node_stroke_selected
        } else if hovered {
            style.canvas_theme.node_stroke_hovered
        } else {
            style.canvas_theme.selection_preview_stroke
        };
        let top_left = scene_state.draw_space_point(Point::new(min_x, min_y), world_space);
        let bottom_right = scene_state.draw_space_point(Point::new(max_x, max_y), world_space);
        let rect = Rect::new(top_left.x, top_left.y, bottom_right.x, bottom_right.y);
        scene.fill(FillRule::NonZero, Affine::IDENTITY, style.canvas_theme.selection_preview_fill, None, &rect);
        let mut outline = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
        if region.locked {
            outline.set_dash_pattern(vec![4.0, 4.0]);
        }
        scene.stroke(&outline, Affine::IDENTITY, stroke_color, None, &rect);
        if region.label.is_some() {
            let tag_x = scene_state.draw_space_point(Point::new(min_x + inset, min_y + inset), world_space);
            let tag_far = scene_state.draw_space_point(Point::new(min_x + inset + grip * 2.0, min_y + inset + grip * 0.6), world_space);
            scene.fill(FillRule::NonZero, Affine::IDENTITY, stroke_color, None, &Rect::new(tag_x.x, tag_x.y, tag_far.x, tag_far.y));
        }
        if (region.selected || dragging) && !region.locked {
            for (cx, cy) in [(min_x, min_y), (max_x, min_y), (min_x, max_y), (max_x, max_y)] {
                let near = scene_state.draw_space_point(Point::new(cx - grip / 2.0, cy - grip / 2.0), world_space);
                let far = scene_state.draw_space_point(Point::new(cx + grip / 2.0, cy + grip / 2.0), world_space);
                scene.fill(FillRule::NonZero, Affine::IDENTITY, stroke_color, None, &Rect::new(near.x, near.y, far.x, far.y));
            }
        }
    }
    if let Some(paint) = gestures.region_paint.as_ref() {
        let [min_x, min_y, max_x, max_y] = region_paint_bounds(paint, gestures.area_brush_extent);
        let near = scene_state.draw_space_point(Point::new(min_x, min_y), world_space);
        let far = scene_state.draw_space_point(Point::new(max_x, max_y), world_space);
        let rect = Rect::new(near.x, near.y, far.x, far.y);
        scene.fill(FillRule::NonZero, Affine::IDENTITY, style.canvas_theme.selection_preview_fill, None, &rect);
        let mut preview = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
        preview.set_dash_pattern(vec![6.0, 5.0]);
        scene.stroke(&preview, Affine::IDENTITY, style.canvas_theme.node_stroke_selected, None, &rect);
    }
}
//#endregion 🎯️TargetRegions

/// 🧩️ BoardHost brush component: brush candidates, previews and suggestion state.
pub struct BoardBrushState {
    pub suggestion_offset: f64,
    pub brush_node_size: f64,
    pub brush_slot_source_id: Option<String>,
    pub brush_candidates: BrushCandidatePage,
    pub brush_candidate_index: usize,
    pub brush_preview: Option<BrushPreviewSnapshot>,
    pub drop_preview: Option<DropPreviewSnapshot>,
    pub brush_candidates_emit_key: Option<String>,
    pub brush_preview_emit_key: Option<String>,
    pub brush_placement_serial: u64,
    pub brush_node_kind_weights: HashMap<String, f64>,
    pub brush_handle_kind_weights: HashMap<String, f64>,
    /// ⌥️ Alt held while brushing — enables suggestion offset and commit-on-leave.
    pub brush_alt_pressed: bool,
    /// ✨️ Suggestions menu opened a slot outside brush utility — use suggestion offset and highlight source handle.
    pub brush_slot_suggestions_active: bool,
}

/// 🧩️ BoardHost gestures component: transform, region and link gesture state.
pub struct BoardGesturesState {
    /// Screen-space polyline preview (CSS px) while dragging a handle link before drop.
    pub link_screen_preview: Option<Vec<Point>>,
    /// 🕹️ Which selection-gumball handles the select utility offers (`setTransformGumballFlag`).
    pub transform_flags: TransformGumballFlags,
    /// 🔄️ The live rotate-ring gesture, `None` whenever the ring is merely drawn.
    pub transform_drag: Option<BoardTransformDrag>,
    /// 🚚️ The live region move/resize gesture (`None` when no region is being dragged).
    pub region_drag: Option<BoardRegionDrag>,
    /// 🖍️ The live area-brush rectangle (`None` unless the areaBrush utility is painting).
    pub region_paint: Option<BoardRegionPaint>,
    /// 🖍️ World width/height an area-brush CLICK (no drag) paints, from the utility's own steppers.
    pub area_brush_extent: (f64, f64),
}

/// 🧩️ BoardHost commit component: pending pointer commit and delete operations.
pub struct BoardCommitState {
    pub pending_delete_planning: Option<BoardDeletePlanningOperation>,
    pub pending_delete_operation: Option<BoardDeleteOperation>,
    pub pending_pointer_commit: Option<BoardPointerCommitOperation>,
    pub queued_pointer_commit: Option<BoardPointerPlan>,
    pub cancelled_pointer_plan_retirement: Option<BoardPointerPlanRetirement>,
    pub pointer_publication: Option<BoardPointerPublication>,
}

/// 🧩️ BoardHost closing component: close and opaque scene retirement state.
pub struct BoardClosingState {
    pub opaque_scene_retirement: Cell<Option<OpaqueSceneRetirementToken>>,
    pub opaque_scene_fault: Cell<bool>,
    pub close_phase: BoardHostClosePhase,
    pub close_entity_retirement: Option<BoardEntityRetirement>,
    pub close_strings: [Option<String>; 16],
    pub close_string_len: u8,
    pub close_node_handles: Option<Vec<NodeKindHandleTemplate>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardHostClosePhase {
    Events,
    Pointer,
    WorldScene,
    Icons,
    Nodes,
    Handles,
    Edges,
    Wires,
    Regions,
    Selection,
    Preselect,
    PreselectRemoved,
    SelectionExit,
    Highlighted,
    Interaction,
    HandleKinds,
    WireKinds,
    NodeKinds,
    EdgeKinds,
    EdgeTips,
    LinkRules,
    Previews,
    Strings,
    Weights,
    Done,
}

pub(crate) struct BoardDeleteOperation {
    pub plan: BoardDeletePlan,
    pub select_event: Option<BoardOwnedEvent>,
    pub mutation_cursor: u16,
    pub publication_cursor: u16,
    pub remaining_claimed_items: u16,
    pub remaining_claimed_bytes: usize,
    pub claimed: bool,
    pub retiring_entity: Option<BoardEntityRetirement>,
    pub cancelling: bool,
}

pub enum BoardRemovedEntity {
    Node(NodeData),
    Edge(EdgeData),
    Handle(HandleData),
    Wire(WireData),
}

enum BoardPropertyContainer {
    Array(Vec<semio_framework_graph_manifest::PropertyValue>),
    Object(semio_framework_graph_manifest::PropertyBag),
}

struct BoardPropertyRetirement {
    pub current: Option<semio_framework_graph_manifest::PropertyValue>,
    pub stack: Box<[Option<BoardPropertyContainer>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub depth: u16,
    pub faulted: bool,
}

pub struct BoardEntityRetirement {
    pub entity: Option<BoardRemovedEntity>,
    pub strings: Box<[Option<String>; 16]>,
    string_len: u8,
    pub properties: Option<BoardPropertyRetirement>,
}

pub struct BoardPointerPublication {
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub len: u16,
}

pub struct BoardPointerCommitOperation {
    pub plan: BoardPointerPlan,
    pub phase: u8,
    pub cursor: u16,
    pub changed: bool,
    pub cancelling: bool,
    pub faulted: bool,
    pub scan_after: Option<String>,
    pub points: Vec<Point>,
    pub screen_points: Vec<Point>,
    pub overlay_points: Vec<Point>,
    pub brush_candidates: BrushCandidatePage,
    pub retiring_ids: BTreeSet<String>,
    pub retiring_points: Vec<Point>,
    pub retiring_screen_points: Vec<Point>,
    pub retiring_overlay_points: Vec<Point>,
    pub retiring_signature_a: Vec<String>,
    pub retiring_signature_b: Vec<String>,
    pub retiring_signature_c: Vec<String>,
    pub retiring_gestures: [Option<String>; 2],
}

impl BoardPointerCommitOperation {
    pub fn new(plan: BoardPointerPlan) -> Self {
        Self {
            plan,
            phase: 0,
            cursor: 0,
            changed: false,
            cancelling: false,
            faulted: false,
            scan_after: None,
            points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
            screen_points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
            overlay_points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
            brush_candidates: BrushCandidatePage::default(),
            retiring_ids: BTreeSet::new(),
            retiring_points: Vec::new(),
            retiring_screen_points: Vec::new(),
            retiring_overlay_points: Vec::new(),
            retiring_signature_a: Vec::new(),
            retiring_signature_b: Vec::new(),
            retiring_signature_c: Vec::new(),
            retiring_gestures: [None, None],
        }
    }

    pub fn retire_one(&mut self) -> bool {
        if let Some(id) = self.retiring_ids.pop_first() {
            drop(id);
            return true;
        }
        if self.retiring_points.pop().is_some() || self.retiring_screen_points.pop().is_some() || self.retiring_overlay_points.pop().is_some() {
            return true;
        }
        for values in [&mut self.retiring_signature_a, &mut self.retiring_signature_b, &mut self.retiring_signature_c] {
            if let Some(value) = values.pop() {
                drop(value);
                return true;
            }
        }
        for gesture in &mut self.retiring_gestures {
            if let Some(value) = gesture.take() {
                drop(value);
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardAuthorityStep {
    Pending,
    Complete,
    Cancelled,
    Fault,
}

impl BoardPropertyRetirement {
    pub fn new(properties: semio_framework_graph_manifest::PropertyBag) -> Self {
        Self { current: Some(semio_framework_graph_manifest::PropertyValue::Object(properties)), stack: Box::new(std::array::from_fn(|_| None)), depth: 0, faulted: false }
    }

    fn push_container(&mut self, container: BoardPropertyContainer) -> Result<(), BoardPropertyContainer> {
        let index = usize::from(self.depth);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(container);
        }
        self.stack[index] = Some(container);
        self.depth += 1;
        Ok(())
    }

    fn pop_container(&mut self) -> Option<BoardPropertyContainer> {
        if self.depth == 0 {
            return None;
        }
        self.depth -= 1;
        self.stack[usize::from(self.depth)].take()
    }

    pub fn step(&mut self) -> Result<bool, ()> {
        if self.faulted {
            return Err(());
        }
        if self.current.is_none() {
            self.current = self.pop_container().map(|container| match container {
                BoardPropertyContainer::Array(values) => semio_framework_graph_manifest::PropertyValue::Array(values),
                BoardPropertyContainer::Object(values) => semio_framework_graph_manifest::PropertyValue::Object(values),
            });
            return Ok(self.current.is_none());
        }
        let current = self.current.take().expect("property retirement current owner");
        match current {
            semio_framework_graph_manifest::PropertyValue::Array(mut values) => {
                let child = values.pop();
                if !values.is_empty() {
                    if let Err(values) = self.push_container(BoardPropertyContainer::Array(values)) {
                        self.current = Some(match values {
                            BoardPropertyContainer::Array(values) => semio_framework_graph_manifest::PropertyValue::Array(values),
                            BoardPropertyContainer::Object(_) => unreachable!(),
                        });
                        self.faulted = true;
                        return Err(());
                    }
                }
                self.current = child;
            }
            semio_framework_graph_manifest::PropertyValue::Object(mut values) => {
                let child = values.pop_last().map(|(key, value)| {
                    drop(key);
                    value
                });
                if !values.is_empty() {
                    if let Err(values) = self.push_container(BoardPropertyContainer::Object(values)) {
                        self.current = Some(match values {
                            BoardPropertyContainer::Object(values) => semio_framework_graph_manifest::PropertyValue::Object(values),
                            BoardPropertyContainer::Array(_) => unreachable!(),
                        });
                        self.faulted = true;
                        return Err(());
                    }
                }
                self.current = child;
            }
            semio_framework_graph_manifest::PropertyValue::String(value) => drop(value),
            semio_framework_graph_manifest::PropertyValue::Null | semio_framework_graph_manifest::PropertyValue::Bool(_) | semio_framework_graph_manifest::PropertyValue::Number(_) => {}
        }
        Ok(false)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.current.is_none() && self.depth == 0 && !self.faulted
    }
}

impl BoardEntityRetirement {
    pub fn new(entity: BoardRemovedEntity) -> Self {
        Self { entity: Some(entity), strings: Box::new(std::array::from_fn(|_| None)), string_len: 0, properties: None }
    }

    fn push_string(&mut self, value: String) {
        let index = usize::from(self.string_len);
        self.strings[index] = Some(value);
        self.string_len += 1;
    }

    fn push_optional_string(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.push_string(value);
        }
    }

    fn split_entity(&mut self, entity: BoardRemovedEntity) {
        let properties = match entity {
            BoardRemovedEntity::Node(NodeData { id, style, text, icon_kind, node_kind, properties, .. }) => {
                self.push_string(id);
                self.push_optional_string(style);
                self.push_optional_string(text);
                self.push_optional_string(icon_kind);
                self.push_string(node_kind);
                properties
            }
            BoardRemovedEntity::Edge(EdgeData { id, source, target, style, edge_kind, source_tip, target_tip, properties, .. }) => {
                self.push_string(id);
                self.push_string(source);
                self.push_string(target);
                self.push_optional_string(style);
                self.push_string(edge_kind);
                self.push_optional_string(source_tip);
                self.push_optional_string(target_tip);
                properties
            }
            BoardRemovedEntity::Handle(HandleData { id, node_id, style, handle_kind, icon_kind, properties, .. }) => {
                self.push_string(id);
                self.push_string(node_id);
                self.push_optional_string(style);
                self.push_string(handle_kind);
                self.push_optional_string(icon_kind);
                properties
            }
            BoardRemovedEntity::Wire(WireData { id, source, target, style, wire_kind, properties, .. }) => {
                self.push_string(id);
                self.push_string(source);
                self.push_optional_string(target);
                self.push_optional_string(style);
                self.push_string(wire_kind);
                properties
            }
        };
        self.properties = Some(BoardPropertyRetirement::new(properties));
    }

    pub fn step(&mut self) -> Result<bool, ()> {
        if let Some(entity) = self.entity.take() {
            self.split_entity(entity);
            return Ok(false);
        }
        if let Some(properties) = self.properties.as_mut() {
            if !properties.step()? {
                return Ok(false);
            }
            debug_assert!(properties.terminal_is_empty());
            self.properties = None;
            return Ok(false);
        }
        if self.string_len > 0 {
            self.string_len -= 1;
            drop(self.strings[usize::from(self.string_len)].take());
            return Ok(false);
        }
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.entity.is_none() && self.string_len == 0 && self.properties.is_none()
    }
}

impl BoardPointerPublication {
    pub fn events_json(&self) -> &str {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("pointer publication is encoded from UTF-8 schema tokens")
    }

    pub fn close_step(&mut self) -> bool {
        self.len = 0;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0
    }
}

pub struct BoardPointerPlanRetirement {
    pub plan: Option<BoardPointerPlan>,
    pub cursor: u16,
}

impl BoardPointerPlanRetirement {
    pub fn new(plan: BoardPointerPlan) -> Self {
        Self { plan: Some(plan), cursor: 0 }
    }

    pub fn close_step(&mut self) -> bool {
        let Some(plan) = self.plan.as_mut() else {
            return true;
        };
        if self.cursor < plan.delta_len {
            plan.deltas[usize::from(self.cursor)] = None;
            self.cursor += 1;
            return false;
        }
        plan.byte_len = 0;
        plan.output_len = 0;
        self.plan = None;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.plan.is_none()
    }
}

impl BoardCommitState {
    pub fn take_pointer_publication(&mut self) -> Option<BoardPointerPublication> {
        self.pointer_publication.take()
    }

    pub fn pointer_publication(&self) -> Option<&BoardPointerPublication> {
        self.pointer_publication.as_ref()
    }

    pub fn pointer_cancel_pending(&self) -> bool {
        self.cancelled_pointer_plan_retirement.is_some() || self.pending_pointer_commit.as_ref().is_some_and(|operation| operation.cancelling)
    }

    pub fn pointer_authority_terminal_is_empty(&self) -> bool {
        self.pending_pointer_commit.is_none() && self.queued_pointer_commit.is_none() && self.cancelled_pointer_plan_retirement.is_none() && self.pointer_publication.is_none()
    }
    // #endregion board_host
}

pub fn step_delete_planning(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, outbox: &mut BoardOutboxState, commit: &mut BoardCommitState) -> BoardAuthorityStep {
    let Some(mut planning) = commit.pending_delete_planning.take() else {
        return BoardAuthorityStep::Complete;
    };
    if let Some(audit) = planning.property_audit.as_mut() {
        if !audit.step() {
            commit.pending_delete_planning = Some(planning);
            return BoardAuthorityStep::Pending;
        }
        let Some((kind, id, properties, nodes, bytes, fault)) = audit.take_result() else {
            planning.fault = Some(BoardEventFault::Schema);
            commit.pending_delete_planning = Some(planning);
            return BoardAuthorityStep::Pending;
        };
        planning.property_audit = None;
        if !scene.restore_delete_properties(kind, &id, properties) {
            planning.fault = Some(BoardEventFault::Schema);
        } else if let Some(fault) = fault {
            planning.fault = Some(fault);
        } else if !planning.cancelling {
            if let Err(fault) = planning.plan.push_admitted_entity(kind, &id, nodes, bytes) {
                planning.fault = Some(fault);
            }
        }
        commit.pending_delete_planning = Some(planning);
        return BoardAuthorityStep::Pending;
    }
    if planning.cancelling {
        return BoardAuthorityStep::Cancelled;
    }
    if planning.fault.is_some() || scene.interaction_revision != planning.plan.revision {
        outbox.event_schema_fault = true;
        return BoardAuthorityStep::Fault;
    }
    match planning.phase {
        BoardDeletePlanningPhase::SelectedEdges => {
            let next = match planning.scan_after.as_ref() {
                Some(after) => focus.selection.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                None => focus.selection.first(),
            };
            let next = next.map(|id| admitted_board_pointer_id(id).map(|owned| (scene.edges.contains_key(id), owned)));
            if let Some(next) = next {
                match next {
                    Ok((is_edge, id)) => {
                        planning.scan_after = Some(id.clone());
                        if is_edge {
                            scene.begin_delete_property_audit(&mut planning, BoardDeleteKind::Edge, &id);
                        }
                    }
                    Err(fault) => planning.fault = Some(fault),
                }
            } else {
                planning.scan_after = None;
                planning.phase = BoardDeletePlanningPhase::Nodes;
            }
        }
        BoardDeletePlanningPhase::Wires => {
            let handle_id = planning.handle_id.as_deref().expect("wire planning owns a handle");
            let next = match planning.relation_after.as_ref() {
                Some(after) => scene.wires.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                None => scene.wires.first_key_value(),
            }
            .map(|(id, wire)| admitted_board_pointer_id(id).map(|owned| (wire.source == handle_id || wire.target.as_deref() == Some(handle_id), owned)));
            if let Some(next) = next {
                match next {
                    Ok((remove, id)) => {
                        planning.relation_after = Some(id.clone());
                        if remove {
                            scene.begin_delete_property_audit(&mut planning, BoardDeleteKind::Wire, &id);
                        }
                    }
                    Err(fault) => planning.fault = Some(fault),
                }
            } else {
                planning.relation_after = None;
                planning.phase = BoardDeletePlanningPhase::Edges;
            }
        }
        BoardDeletePlanningPhase::Handles => {
            let node_id = planning.node_id.as_deref().expect("handle planning owns a node");
            let next = match planning.handle_after.as_ref() {
                Some(after) => scene.handles.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                None => scene.handles.first_key_value(),
            }
            .map(|(id, handle)| admitted_board_pointer_id(id).map(|owned| (handle.node_id == node_id, owned)));
            if let Some(next) = next {
                match next {
                    Ok((belongs, id)) => {
                        planning.handle_after = Some(id.clone());
                        if belongs {
                            planning.handle_id = Some(id);
                            planning.relation_after = None;
                            planning.phase = BoardDeletePlanningPhase::Wires;
                        }
                    }
                    Err(fault) => planning.fault = Some(fault),
                }
            } else {
                planning.handle_after = None;
                planning.phase = BoardDeletePlanningPhase::Node;
            }
        }
        BoardDeletePlanningPhase::Edges => {
            let endpoint_id = planning.handle_id.as_deref().or(planning.node_id.as_deref()).expect("edge planning owns an endpoint");
            let next = match planning.relation_after.as_ref() {
                Some(after) => scene.edges.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                None => scene.edges.first_key_value(),
            }
            .map(|(id, edge)| admitted_board_pointer_id(id).map(|owned| (edge.source == endpoint_id || edge.target == endpoint_id, owned)));
            if let Some(next) = next {
                match next {
                    Ok((remove, id)) => {
                        planning.relation_after = Some(id.clone());
                        if remove {
                            scene.begin_delete_property_audit(&mut planning, BoardDeleteKind::Edge, &id);
                        }
                    }
                    Err(fault) => planning.fault = Some(fault),
                }
            } else {
                planning.relation_after = None;
                planning.phase = if planning.handle_id.is_some() { BoardDeletePlanningPhase::Handle } else { BoardDeletePlanningPhase::Node };
            }
        }
        BoardDeletePlanningPhase::Nodes => {
            let next = match planning.node_after.as_ref() {
                Some(after) => scene.nodes.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                None => scene.nodes.first_key_value(),
            }
            .map(|(id, _)| admitted_board_pointer_id(id).map(|owned| (focus.selection.contains(id), owned)));
            if let Some(next) = next {
                match next {
                    Ok((selected, id)) => {
                        planning.node_after = Some(id.clone());
                        planning.node_id = Some(id);
                        planning.node_relevant = selected;
                        planning.handle_after = None;
                        planning.phase = BoardDeletePlanningPhase::DiscoverNode;
                    }
                    Err(fault) => planning.fault = Some(fault),
                }
            } else {
                planning.node_after = None;
                planning.phase = BoardDeletePlanningPhase::SelectionEvent;
            }
        }
        BoardDeletePlanningPhase::DiscoverNode => {
            if !style.has_ports() {
                if planning.node_relevant {
                    planning.relation_after = None;
                    planning.phase = BoardDeletePlanningPhase::Edges;
                } else {
                    planning.node_id = None;
                    planning.phase = BoardDeletePlanningPhase::Nodes;
                }
            } else {
                let node_id = planning.node_id.as_deref().expect("node discovery owns a node");
                let next = match planning.handle_after.as_ref() {
                    Some(after) => scene.handles.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                    None => scene.handles.first_key_value(),
                }
                .map(|(id, handle)| admitted_board_pointer_id(id).map(|owned| (handle.node_id == node_id && focus.selection.contains(id), owned)));
                if let Some(next) = next {
                    match next {
                        Ok((selected, id)) => {
                            planning.handle_after = Some(id);
                            if selected {
                                planning.node_relevant = true;
                                planning.handle_after = None;
                                planning.phase = BoardDeletePlanningPhase::Handles;
                            }
                        }
                        Err(fault) => planning.fault = Some(fault),
                    }
                } else if planning.node_relevant {
                    planning.handle_after = None;
                    planning.phase = BoardDeletePlanningPhase::Handles;
                } else {
                    planning.handle_after = None;
                    planning.node_id = None;
                    planning.phase = BoardDeletePlanningPhase::Nodes;
                }
            }
        }
        BoardDeletePlanningPhase::Handle => {
            let id = planning.handle_id.take().expect("handle deletion owns an id");
            planning.phase = BoardDeletePlanningPhase::Handles;
            scene.begin_delete_property_audit(&mut planning, BoardDeleteKind::Handle, &id);
        }
        BoardDeletePlanningPhase::Node => {
            let id = planning.node_id.take().expect("node deletion owns an id");
            planning.phase = BoardDeletePlanningPhase::Nodes;
            scene.begin_delete_property_audit(&mut planning, BoardDeleteKind::Node, &id);
        }
        BoardDeletePlanningPhase::SelectionEvent => {
            let next = match planning.scan_after.as_ref() {
                Some(after) => focus.selection.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                None => focus.selection.first(),
            }
            .map(|id| admitted_board_pointer_id(id));
            match next {
                Some(Ok(id)) => {
                    planning.scan_after = Some(id.clone());
                    if !planning.plan.removes_selection_id(&id) {
                        let write = (|| {
                            if !planning.select_first {
                                planning.select_builder.raw(",")?;
                            }
                            planning.select_builder.string(&id)?;
                            Ok::<(), BoardEventFault>(())
                        })();
                        if let Err(fault) = write {
                            planning.fault = Some(fault);
                        }
                        planning.select_first = false;
                    }
                }
                Some(Err(fault)) => planning.fault = Some(fault),
                None => planning.phase = BoardDeletePlanningPhase::Finish,
            }
        }
        BoardDeletePlanningPhase::Finish => {
            if let Err(fault) = planning.select_builder.raw("],\"exitHighlightIds\":[]}") {
                planning.fault = Some(fault);
            }
            if planning.fault.is_some() {
                outbox.event_schema_fault = true;
                return BoardAuthorityStep::Fault;
            }
            let select_event = std::mem::replace(&mut planning.select_builder, BoardPayloadBuilder::new()).finish_owned(BoardEventKind::Select);
            commit.pending_delete_operation = Some(BoardDeleteOperation {
                plan: planning.plan,
                select_event: Some(select_event),
                mutation_cursor: 0,
                publication_cursor: 0,
                remaining_claimed_items: 0,
                remaining_claimed_bytes: 0,
                claimed: false,
                retiring_entity: None,
                cancelling: false,
            });
            return BoardAuthorityStep::Pending;
        }
    }
    commit.pending_delete_planning = Some(planning);
    BoardAuthorityStep::Pending
}

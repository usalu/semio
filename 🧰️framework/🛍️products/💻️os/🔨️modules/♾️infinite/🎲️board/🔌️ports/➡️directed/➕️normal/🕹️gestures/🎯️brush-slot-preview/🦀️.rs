//! 🎯️ Brush slot preview of the board gestures: slot centres and hit distances, the nearest slot source, preview building and sync events, slot clear, commit and finish and the alt update. See [`super`].

use super::*;

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

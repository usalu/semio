//! 🖌️ Brush and drop preview of the board gestures: entering a brush slot, rebuilding and moving the brush preview and the drop and brush preview paint. See [`super`].

use super::*;

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

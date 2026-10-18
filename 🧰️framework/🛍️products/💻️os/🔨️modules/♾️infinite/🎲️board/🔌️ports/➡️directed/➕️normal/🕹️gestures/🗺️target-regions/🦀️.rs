//! 🗺️ Target regions of the board gestures: the target regions JSON, region grip hits, region drag and region paint gestures and the target region paint. See [`super`].

use super::*;

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

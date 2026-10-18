//! 🌀️ Transform drag of the board gestures: the transform flags and the transform drag update, commit and cancel. See [`super`].

use super::*;

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

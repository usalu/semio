//! 🖍️ Selection staging and sync of the board focus: staging and restoring selection ids, cancelling a node drag, background clicks, selection flags, select events and area preselect. See [`super`].

use super::*;

/// 🕹️ Applies a press's selection change to the engine at once and stages its `select` row on
/// `gesture`, keeping the pre-press selection for a cancel. An unchanged selection stages nothing.
pub fn stage_selection_ids(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, gesture: &mut GestureStage, ids: &[String], merge: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    if next == focus.selection {
        return;
    }
    let previous = std::mem::replace(&mut focus.selection, next);
    gesture.restore.get_or_insert(previous);
    gesture.select = Some((sorted_selection_ids(&focus.selection), merge.map(str::to_string)));
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    outbox.last_select_emit_sig = None;
    focus.selection_exit_highlight.clear();
    sync_selection_flags_to_objects(scene, focus);
}

/// ↩️ Restores the selection a cancelled gesture's press staged; nothing is published.
pub fn restore_gesture_selection(scene: &mut BoardSceneState, focus: &mut BoardFocusState, gesture: GestureStage) {
    let Some(restore) = gesture.restore else {
        return;
    };
    focus.selection = restore;
    focus.selection_exit_highlight.clear();
    sync_selection_flags_to_objects(scene, focus);
    scene.bump_content_scene_generation();
}

/// ↩️ Abandons a live node drag with zero trace: every member returns to its grab-time position, the
/// press's staged selection is undone, and nothing reaches the document. The restored positions go out as
/// transient `nodeMove` preview rows only, so sibling panes that mirrored the drag snap back too.
pub fn cancel_node_drag(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, start_positions: BTreeMap<String, (f64, f64)>, gesture: GestureStage) {
    for (id, (x, y)) in &start_positions {
        if let Some(node) = scene.nodes.get_mut(id) {
            node.x = *x;
            node.y = *y;
            if let Ok(reservation) = BoardOwnedEvent::node_move(id, *x, *y).map_err(|_| ()).and_then(|event| outbox.events.reserve_event(event).map_err(|_| ())) {
                outbox.publish_event_reservation(reservation);
            }
        }
    }
    restore_gesture_selection(scene, focus, gesture);
    scene.bump_content_scene_generation();
}

//#endregion 🎬️Gestures

/// 🖱️ Empty selection on background click without exit/highlight chrome or preselect.
pub fn clear_selection_on_background_click(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState) {
    if focus.selection.is_empty() {
        return;
    }
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &[], None, None, None) else {
        return;
    };
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    outbox.last_select_emit_sig = None;
    focus.selection_exit_highlight.clear();
    focus.selection.clear();
    sync_selection_flags_to_objects(scene, focus);
    outbox.publish_event_reservation(reservation);
}

pub fn sync_selection_flags_to_objects(scene: &mut BoardSceneState, focus: &mut BoardFocusState) {
    let chrome = focus.selection_chrome_ids();
    for n in scene.nodes.values_mut() {
        n.selected = chrome.contains(&n.id);
    }
    for h in scene.handles.values_mut() {
        h.selected = chrome.contains(&h.id);
    }
    for e in scene.edges.values_mut() {
        e.selected = chrome.contains(&e.id);
    }
    for w in scene.wires.values_mut() {
        w.selected = chrome.contains(&w.id);
    }
    for r in scene.regions.values_mut() {
        r.selected = chrome.contains(&r.id);
    }
}

pub(crate) fn push_select_event(focus: &mut BoardFocusState, outbox: &mut BoardOutboxState) {
    outbox.last_select_emit_sig = None;
    let mut sorted: Vec<_> = focus.selection.iter().cloned().collect();
    sorted.sort();
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
        return;
    };
    outbox.publish_event_reservation(reservation);
}

pub fn set_selection_ids(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, ids: &[String]) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    if next == focus.selection {
        return;
    }
    let mut sorted: Vec<_> = next.iter().cloned().collect();
    sorted.sort();
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
        return;
    };
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    focus.selection_exit_highlight.clear();
    focus.selection = next;
    sync_selection_flags_to_objects(scene, focus);
    outbox.last_select_emit_sig = None;
    outbox.publish_event_reservation(reservation);
}

/// 🔇️ Updates committed selection without emitting `select` (controlled React sync).
pub fn set_selection_ids_silent(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, ids: &[String]) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    if next == focus.selection {
        return;
    }
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    focus.selection_exit_highlight.clear();
    focus.selection = next;
    sync_selection_flags_to_objects(scene, focus);
}

/// 🔇️ Mirrors area-select preview chrome without emitting `preselect` (shared multi-view sync).
pub fn set_preselect_state_silent(scene: &mut BoardSceneState, focus: &mut BoardFocusState, ids: &[String], removed_ids: &[String]) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let removed: BTreeSet<String> = removed_ids.iter().cloned().collect();
    if focus.preselect == next && focus.preselect_removed == removed {
        return;
    }
    focus.preselect = next;
    focus.preselect_removed = removed;
    sync_selection_flags_to_objects(scene, focus);
}

pub fn set_selection_ids_gestured(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, ids: &[String], gesture: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let mut sorted: Vec<_> = next.iter().cloned().collect();
    sorted.sort();
    let gesture_owned = gesture.map(ToOwned::to_owned);
    let sig = (sorted.clone(), gesture_owned);
    if next == focus.selection && outbox.last_select_emit_sig.as_ref() == Some(&sig) {
        return;
    }
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, gesture) else {
        return;
    };
    outbox.last_select_emit_sig = Some(sig);
    focus.preselect.clear();
    focus.preselect_removed.clear();
    outbox.last_preselect_emit_sig = None;
    if next != focus.selection {
        focus.selection_exit_highlight.clear();
        focus.selection = next;
        sync_selection_flags_to_objects(scene, focus);
    }
    outbox.publish_event_reservation(reservation);
}

/// 👁️ Rectangle/lasso drag preview: `preselect` + `preselect_removed` (anchor \\ preselect); emits `preselect` only.
pub fn board_apply_area_preselect(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, anchor_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let sorted = sorted_selection_ids(&next);
    let removed = sorted_selection_ids(&anchor_ids.difference(&next).cloned().collect());
    let gesture_owned = gesture.map(ToOwned::to_owned);
    let sig = (sorted.clone(), removed.clone(), gesture_owned);
    if focus.preselect == next && outbox.last_preselect_emit_sig.as_ref() == Some(&sig) {
        return;
    }
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Preselect, &sorted, None, Some(&removed), gesture) else {
        return;
    };
    outbox.last_preselect_emit_sig = Some(sig);
    focus.preselect = next;
    focus.preselect_removed = anchor_ids.difference(&focus.preselect).cloned().collect();
    focus.set_hovered_id_silent(None);
    sync_selection_flags_to_objects(scene, focus);
    outbox.publish_event_reservation(reservation);
}

/// 🧿️ Ends a rectangle/lasso cycle: commits `selection`, clears preselect (highlight only lives in preselect).
pub fn commit_area_select_from_initial(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, initial_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
    let next: BTreeSet<String> = ids.iter().cloned().collect();
    let sorted = sorted_selection_ids(&next);
    let anchor = sorted_selection_ids(initial_ids);
    let gesture_owned = gesture.map(ToOwned::to_owned);
    let Some(reservation) = outbox.reserve_selection_event(BoardEventKind::Select, &sorted, Some(&anchor), None, gesture) else {
        return;
    };
    outbox.last_select_emit_sig = None;
    outbox.last_preselect_emit_sig = None;
    focus.preselect.clear();
    focus.preselect_removed.clear();
    focus.selection_exit_highlight.clear();
    focus.selection = next;
    sync_selection_flags_to_objects(scene, focus);
    let _ = gesture_owned;
    outbox.publish_event_reservation(reservation);
}

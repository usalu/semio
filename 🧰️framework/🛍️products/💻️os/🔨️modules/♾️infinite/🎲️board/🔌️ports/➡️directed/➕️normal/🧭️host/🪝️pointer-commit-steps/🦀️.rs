//! 🪝️ Pointer commit steps of the normal board host: the pointer commit, selection preview, selection and brush pointer commit steps. See [`super`].

use super::*;

pub(crate) fn step_pointer_commit_operation(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style_state: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState, operation: &mut BoardPointerCommitOperation) -> bool {
    match operation.plan.kind {
        BoardPointerPlanKind::Idle => true,
        BoardPointerPlanKind::Pan { camera } => {
            scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
            true
        }
        BoardPointerPlanKind::FinishPan { camera } => match operation.phase {
            0 => {
                scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
                operation.phase = 1;
                false
            }
            _ => {
                focus.interaction = Interaction::None;
                true
            }
        },
        BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => {
            if operation.cursor < operation.plan.delta_len {
                let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("pointer commit delta");
                if let Some(node) = scene.nodes.get_mut(operation.plan.id(delta.id)) {
                    operation.changed |= (node.x - delta.x).abs() > 1e-9 || (node.y - delta.y).abs() > 1e-9;
                    node.x = delta.x;
                    node.y = delta.y;
                }
                operation.cursor += 1;
                return false;
            }
            if operation.phase == 0 {
                if operation.changed {
                    scene.bump_content_scene_generation();
                }
                operation.phase = 1;
                return false;
            }
            if matches!(operation.plan.kind, BoardPointerPlanKind::FinishDrag) {
                focus.interaction = Interaction::None;
            }
            true
        }
        BoardPointerPlanKind::SelectionPreview { start, start_screen } => step_selection_preview_commit(scene, focus, outbox, operation, start, start_screen),
        BoardPointerPlanKind::SelectionCommit => step_selection_commit(scene, focus, outbox, operation),
        BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world, activated, start_screen } => match operation.phase {
            0 => {
                if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                    operation.faulted = true;
                    return true;
                }
                operation.phase = 1;
                false
            }
            1 => {
                outbox.link_compat_nodes_emit_key = Some(operation.plan.id(compat_key).to_owned());
                operation.phase = 2;
                false
            }
            2 => {
                outbox.link_target_ring_emit_key = Some(operation.plan.id(ring_key).to_owned());
                operation.phase = 3;
                false
            }
            3 => {
                focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                focus.hovered_kind = None;
                operation.phase = 4;
                false
            }
            4 => {
                let source_id = operation.plan.id(source).to_owned();
                let target_id = target.map(|span| operation.plan.id(span).to_owned());
                focus.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
                operation.phase = 5;
                false
            }
            _ => {
                scene.bump_content_scene_generation();
                true
            }
        },
        BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key } => match operation.phase {
            0 => {
                if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                    operation.faulted = true;
                    return true;
                }
                operation.phase = 1;
                false
            }
            1 => {
                if let (Some(target), Some(edge)) = (target, edge) {
                    let source_id = operation.plan.id(source).to_owned();
                    let target_id = operation.plan.id(target).to_owned();
                    let edge_id = operation.plan.id(edge).to_owned();
                    let Some(source_row) = scene.handles.get(&source_id) else {
                        operation.faulted = true;
                        return true;
                    };
                    let Some(target_row) = scene.handles.get(&target_id) else {
                        operation.faulted = true;
                        return true;
                    };
                    let edge_kind = style_state.default_edge_kind_for_created_link(source_row, target_row);
                    scene.edges.insert(
                        edge_id.clone(),
                        EdgeData { id: edge_id, source: source_id, target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: semio_framework_graph_manifest::PropertyBag::new() },
                    );
                }
                operation.phase = 2;
                false
            }
            2 => {
                outbox.link_compat_nodes_emit_key = compat_key.map(|span| operation.plan.id(span).to_owned());
                operation.phase = 3;
                false
            }
            3 => {
                outbox.link_target_ring_emit_key = ring_key.map(|span| operation.plan.id(span).to_owned());
                operation.phase = 4;
                false
            }
            4 => {
                focus.interaction =
                    if let Some(target_node) = target_node { Interaction::LinkTargetNode { source_id: operation.plan.id(source).to_owned(), target_node_id: operation.plan.id(target_node).to_owned() } } else { Interaction::None };
                operation.phase = 5;
                false
            }
            5 => {
                focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                focus.hovered_kind = None;
                operation.phase = 6;
                false
            }
            _ => {
                scene.bump_content_scene_generation();
                true
            }
        },
        BoardPointerPlanKind::LinkRetain { hover } | BoardPointerPlanKind::Hover { hover } => {
            focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
            focus.hovered_kind = None;
            true
        }
        BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => step_brush_pointer_commit(scene, focus, style_state, brush, operation, source, hover, alt, commit_old),
    }
}

fn step_selection_preview_commit(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, operation: &mut BoardPointerCommitOperation, start: Point, start_screen: Point) -> bool {
    match operation.phase {
        0 => {
            if !matches!(focus.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                operation.faulted = true;
                return true;
            }
            operation.phase = 1;
            false
        }
        1 => {
            if let Some(id) = focus.preselect.pop_first() {
                scene.set_pointer_selection_flag(&id, false);
                return false;
            }
            operation.phase = 2;
            false
        }
        2 => {
            if focus.preselect_removed.pop_first().is_some() {
                return false;
            }
            operation.phase = 3;
            false
        }
        3 => {
            if operation.cursor < operation.plan.delta_len {
                let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection preview id");
                let id = operation.plan.id(delta.id).to_owned();
                focus.preselect.insert(id.clone());
                scene.set_pointer_selection_flag(&id, true);
                operation.cursor += 1;
                return false;
            }
            operation.cursor = 0;
            operation.phase = 4;
            false
        }
        4 => {
            let next = match &focus.interaction {
                Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => match operation.scan_after.as_ref() {
                    Some(after) => initial_ids.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next().cloned(),
                    None => initial_ids.first().cloned(),
                },
                _ => None,
            };
            if let Some(id) = next {
                operation.scan_after = Some(id.clone());
                if !focus.preselect.contains(&id) {
                    focus.preselect_removed.insert(id);
                }
                return false;
            }
            operation.scan_after = None;
            operation.phase = 5;
            false
        }
        5 => {
            if operation.cursor < operation.plan.point_len {
                let index = usize::from(operation.cursor);
                operation.points.push(operation.plan.points[index]);
                operation.screen_points.push(operation.plan.screen_points[index]);
                operation.cursor += 1;
                return false;
            }
            operation.cursor = 0;
            operation.phase = 6;
            false
        }
        6 => {
            if operation.screen_points.len() < 2 {
                operation.phase = 7;
                return false;
            }
            if focus.selection_options.method == "lasso" {
                if usize::from(operation.cursor) < operation.screen_points.len() {
                    operation.overlay_points.push(operation.screen_points[usize::from(operation.cursor)]);
                    operation.cursor += 1;
                    return false;
                }
            } else if operation.cursor < 4 {
                let last = *operation.screen_points.last().unwrap_or(&start_screen);
                let point = match operation.cursor {
                    0 => start_screen,
                    1 => Point::new(last.x, start_screen.y),
                    2 => last,
                    _ => Point::new(start_screen.x, last.y),
                };
                operation.overlay_points.push(point);
                operation.cursor += 1;
                return false;
            }
            operation.phase = 7;
            false
        }
        7 => {
            let previous = std::mem::replace(&mut focus.interaction, Interaction::None);
            let initial_ids = match previous {
                Interaction::SelectionPending { initial_ids, .. } => initial_ids,
                Interaction::Selection { initial_ids, points, screen_points, .. } => {
                    operation.retiring_points = points;
                    operation.retiring_screen_points = screen_points;
                    initial_ids
                }
                _ => {
                    operation.faulted = true;
                    return true;
                }
            };
            focus.selection_preview_crossing = !selection_drag_enclosing(focus.selection_options.method.as_str(), start_screen, &operation.screen_points);
            operation.retiring_overlay_points = focus.selection_screen_preview.take().unwrap_or_default();
            focus.selection_screen_preview = (!operation.overlay_points.is_empty()).then(|| std::mem::take(&mut operation.overlay_points));
            focus.interaction = Interaction::Selection { initial_ids, points: std::mem::take(&mut operation.points), screen_points: std::mem::take(&mut operation.screen_points), start, start_screen };
            if let Some((ids, removed, gesture)) = outbox.last_preselect_emit_sig.take() {
                operation.retiring_signature_b = ids;
                operation.retiring_signature_c = removed;
                operation.retiring_gestures[1] = gesture;
            }
            operation.phase = 8;
            false
        }
        8 => {
            if operation.retire_one() {
                return false;
            }
            true
        }
        _ => true,
    }
}

fn step_selection_commit(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, operation: &mut BoardPointerCommitOperation) -> bool {
    match operation.phase {
        0 => {
            let previous = std::mem::replace(&mut focus.interaction, Interaction::None);
            match previous {
                Interaction::SelectionPending { initial_ids, .. } => operation.retiring_ids = initial_ids,
                Interaction::Selection { initial_ids, points, screen_points, .. } => {
                    operation.retiring_ids = initial_ids;
                    operation.retiring_points = points;
                    operation.retiring_screen_points = screen_points;
                }
                other => {
                    focus.interaction = other;
                    operation.faulted = true;
                    return true;
                }
            }
            operation.phase = 1;
            false
        }
        1 => {
            if let Some(id) = focus.selection.pop_first() {
                scene.set_pointer_selection_flag(&id, false);
                return false;
            }
            operation.phase = 2;
            false
        }
        2 => {
            if let Some(id) = focus.preselect.pop_first() {
                scene.set_pointer_selection_flag(&id, false);
                return false;
            }
            operation.phase = 3;
            false
        }
        3 => {
            if focus.preselect_removed.pop_first().is_some() {
                return false;
            }
            operation.phase = 4;
            false
        }
        4 => {
            if focus.selection_exit_highlight.pop_first().is_some() {
                return false;
            }
            operation.phase = 5;
            false
        }
        5 => {
            if operation.retire_one() {
                return false;
            }
            operation.phase = 6;
            false
        }
        6 => {
            if operation.cursor < operation.plan.delta_len {
                let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection commit id");
                let id = operation.plan.id(delta.id).to_owned();
                focus.selection.insert(id.clone());
                scene.set_pointer_selection_flag(&id, true);
                operation.cursor += 1;
                return false;
            }
            operation.phase = 7;
            false
        }
        7 => {
            if let Some((ids, gesture)) = outbox.last_select_emit_sig.take() {
                operation.retiring_signature_a = ids;
                operation.retiring_gestures[0] = gesture;
            }
            if let Some((ids, removed, gesture)) = outbox.last_preselect_emit_sig.take() {
                operation.retiring_signature_b = ids;
                operation.retiring_signature_c = removed;
                operation.retiring_gestures[1] = gesture;
            }
            operation.retiring_overlay_points = focus.selection_screen_preview.take().unwrap_or_default();
            focus.selection_preview_crossing = false;
            operation.phase = 8;
            false
        }
        8 => {
            if operation.retire_one() {
                return false;
            }
            operation.phase = 9;
            false
        }
        _ => {
            scene.bump_content_scene_generation();
            true
        }
    }
}

fn step_brush_pointer_commit(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style: &mut BoardStyleState, brush: &mut BoardBrushState, operation: &mut BoardPointerCommitOperation, source: Option<BoardPointerSpan>, hover: Option<BoardPointerSpan>, alt: bool, commit_old: bool) -> bool {
    match operation.phase {
        0 => {
            if focus.active_utility != ActiveUtility::Brush || matches!(focus.interaction, Interaction::Pan { .. }) {
                operation.faulted = true;
                return true;
            }
            operation.phase = 1;
            false
        }
        1 => {
            if operation.cursor < operation.plan.delta_len {
                let candidate = operation.plan.deltas[usize::from(operation.cursor)].expect("brush candidate");
                if operation.brush_candidates.push(operation.plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                    operation.faulted = true;
                    return true;
                }
                operation.cursor += 1;
                return false;
            }
            operation.phase = 2;
            false
        }
        2 => {
            if commit_old {
                brush.brush_placement_serial = brush.brush_placement_serial.wrapping_add(1);
            }
            operation.phase = 3;
            false
        }
        3 => {
            brush.brush_alt_pressed = alt;
            brush.brush_slot_suggestions_active = false;
            operation.phase = 4;
            false
        }
        4 => {
            brush.brush_slot_source_id = source.map(|span| operation.plan.id(span).to_owned());
            operation.phase = 5;
            false
        }
        5 => {
            brush.brush_candidates = std::mem::take(&mut operation.brush_candidates);
            brush.brush_candidate_index = 0;
            operation.phase = 6;
            false
        }
        6 => {
            let offset = if alt { brush.suggestion_offset } else { 0.0 };
            brush.brush_preview = brush.brush_slot_source_id.as_deref().and_then(|source_id| brush.brush_candidates.first().and_then(|candidate| brush_build_preview_with_offset(scene, style, brush, source_id, candidate, offset)));
            operation.phase = 7;
            false
        }
        7 => {
            brush.brush_preview_emit_key = None;
            operation.phase = 8;
            false
        }
        8 => {
            brush.brush_candidates_emit_key = None;
            operation.phase = 9;
            false
        }
        9 => {
            focus.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
            focus.hovered_kind = None;
            operation.phase = 10;
            false
        }
        10 => {
            focus.interaction = Interaction::None;
            operation.phase = 11;
            false
        }
        _ => {
            scene.bump_content_scene_generation();
            true
        }
    }
}

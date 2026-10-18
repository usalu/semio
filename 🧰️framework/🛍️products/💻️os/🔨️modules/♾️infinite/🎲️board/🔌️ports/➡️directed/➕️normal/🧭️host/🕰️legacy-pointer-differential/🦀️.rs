//! 🕰️ Legacy pointer differential of the normal board host: the testkit differential that replays a pointer commit against the legacy path. See [`super`].

use super::*;

#[cfg(any(test, feature = "testkit"))]
fn commit_pointer_legacy_differential(scene: &mut BoardSceneState, focus: &mut BoardFocusState, style_state: &mut BoardStyleState, brush: &mut BoardBrushState, outbox: &mut BoardOutboxState, plan: &BoardPointerPlan) -> bool {
    match plan.kind {
        BoardPointerPlanKind::Idle => {}
        BoardPointerPlanKind::Pan { camera } => {
            scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
        }
        BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => unreachable!("drag plans are committed through the retained pointer authority"),
        BoardPointerPlanKind::FinishPan { camera } => {
            scene.camera = Camera { x: camera[0], y: camera[1], zoom: semio_framework_canvas::camera::clamp_zoom(camera[2]) };
            focus.interaction = Interaction::None;
        }
        BoardPointerPlanKind::SelectionPreview { start, start_screen } => {
            let grabbed = std::mem::take(&mut focus.interaction);
            let initial_ids = match grabbed {
                Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => initial_ids,
                other => {
                    focus.interaction = other;
                    return false;
                }
            };
            let next: BTreeSet<String> = plan.selection_ids().map(str::to_string).collect();
            focus.preselect_removed = initial_ids.difference(&next).cloned().collect();
            focus.preselect = next;
            outbox.last_preselect_emit_sig = None;
            sync_selection_flags_to_objects(scene, focus);
            let points = plan.points[..usize::from(plan.point_len)].to_vec();
            let screen_points = plan.screen_points[..usize::from(plan.point_len)].to_vec();
            focus.sync_selection_screen_overlay(start_screen, &screen_points);
            focus.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
        }
        BoardPointerPlanKind::SelectionCommit => {
            if !matches!(focus.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                return false;
            }
            focus.interaction = Interaction::None;
            focus.selection = plan.selection_ids().map(str::to_string).collect();
            focus.preselect.clear();
            focus.preselect_removed.clear();
            focus.selection_exit_highlight.clear();
            outbox.last_select_emit_sig = None;
            outbox.last_preselect_emit_sig = None;
            focus.selection_screen_preview = None;
            focus.selection_preview_crossing = false;
            sync_selection_flags_to_objects(scene, focus);
            scene.bump_content_scene_generation();
        }
        BoardPointerPlanKind::LinkMove { source, target, hover, end_world, activated, start_screen, .. } => {
            if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                return false;
            }
            let source_id = plan.id(source).to_string();
            let target_id = target.map(|target| plan.id(target).to_string());
            let hover_id = hover.map(|hover| plan.id(hover).to_string());
            let node_ids = link_drag_compatible_target_node_ids(scene, style_state, &source_id);
            let ring_node_id = activated.then(|| link_drag_ring_target_node_id(scene, style_state, &source_id, end_world)).flatten().filter(|node_id| link_compatible_handle_count_on_node(scene, style_state, &source_id, node_id) > 1);
            let ring_handle_ids = ring_node_id.as_deref().map(|node_id| link_compatible_handle_ids_on_node(scene, style_state, &source_id, node_id)).unwrap_or_default();
            outbox.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
            outbox.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(",")));
            focus.hovered_id = hover_id;
            focus.hovered_kind = None;
            focus.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
            scene.bump_content_scene_generation();
        }
        BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, .. } => {
            if !matches!(focus.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                return false;
            }
            let source_id = plan.id(source).to_string();
            let target_id = target.map(|target| plan.id(target).to_string());
            let edge_id = edge.map(|edge| plan.id(edge).to_string());
            let target_node_id = target_node.map(|target_node| plan.id(target_node).to_string());
            let hover_id = hover.map(|hover| plan.id(hover).to_string());
            if let (Some(target_id), Some(edge_id)) = (target_id, edge_id) {
                let Some(source_row) = scene.handles.get(&source_id) else {
                    return false;
                };
                let Some(target_row) = scene.handles.get(&target_id) else {
                    return false;
                };
                let edge_kind = style_state.default_edge_kind_for_created_link(source_row, target_row);
                scene.edges.insert(
                    edge_id.clone(),
                    EdgeData { id: edge_id, source: source_id.clone(), target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: semio_framework_graph_manifest::PropertyBag::new() },
                );
            }
            if let Some(target_node_id) = target_node_id {
                let node_ids = link_drag_compatible_target_node_ids(scene, style_state, &source_id);
                let ring_handle_ids = link_compatible_handle_ids_on_node(scene, style_state, &source_id, &target_node_id);
                outbox.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
                outbox.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
                focus.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
            } else {
                outbox.link_compat_nodes_emit_key = None;
                outbox.link_target_ring_emit_key = None;
                focus.interaction = Interaction::None;
            }
            focus.hovered_id = hover_id;
            focus.hovered_kind = None;
            scene.bump_content_scene_generation();
        }
        BoardPointerPlanKind::LinkRetain { hover } => {
            if !matches!(focus.interaction, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
                return false;
            }
            focus.hovered_id = hover.map(|hover| plan.id(hover).to_string());
            focus.hovered_kind = None;
        }
        BoardPointerPlanKind::Hover { hover } => {
            if !matches!(focus.interaction, Interaction::None) {
                return false;
            }
            focus.hovered_id = hover.map(|hover| plan.id(hover).to_string());
            focus.hovered_kind = None;
        }
        BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => {
            if focus.active_utility != ActiveUtility::Brush || matches!(focus.interaction, Interaction::Pan { .. }) {
                return false;
            }
            let source_id = source.map(|source| plan.id(source).to_string());
            let mut candidates = BrushCandidatePage::default();
            for candidate in plan.deltas[..usize::from(plan.delta_len)].iter().flatten() {
                if candidates.push(plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                    return false;
                }
            }
            if commit_old {
                brush.brush_placement_serial = brush.brush_placement_serial.wrapping_add(1);
            }
            brush.brush_alt_pressed = alt;
            brush.brush_slot_suggestions_active = false;
            brush.brush_slot_source_id = source_id.clone();
            brush.brush_candidates = candidates;
            brush.brush_candidate_index = 0;
            let offset = if alt { brush.suggestion_offset } else { 0.0 };
            brush.brush_preview = source_id.as_deref().and_then(|source_id| brush.brush_candidates.first().and_then(|candidate| brush_build_preview_with_offset(scene, style_state, brush, source_id, candidate, offset)));
            brush.brush_preview_emit_key = None;
            brush.brush_candidates_emit_key = None;
            focus.hovered_id = hover.map(|hover| plan.id(hover).to_string());
            focus.hovered_kind = None;
            focus.interaction = Interaction::None;
            scene.bump_content_scene_generation();
        }
    }
    scene.interaction_revision = plan.revision.wrapping_add(1);
    true
}

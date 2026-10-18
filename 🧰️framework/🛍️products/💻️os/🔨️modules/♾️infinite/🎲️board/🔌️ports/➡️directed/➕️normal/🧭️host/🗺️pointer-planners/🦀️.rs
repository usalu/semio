//! 🗺️ Pointer planners of the normal board host: the selection, link, hover, brush and drag pointer planners and the pointer commit and cancel begin entry points. See [`super`].

use super::*;

pub(super) fn plan_selection_pending_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, intent: BoardPointerIntent, initial_ids: &BTreeSet<String>, start: Point, start_screen: Point, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    match intent.phase {
        BoardPointerPhase::Move if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX => Ok(BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle)),
        BoardPointerPhase::Move => {
            let points = [start, world];
            let screen_points = [start_screen, screen];
            let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, focus.selection_options.mode.as_str());
            let next = resolve_area_selection_with_initial(scene, focus, style, initial_ids, start, &points, merge_mode.as_str());
            let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
            scene.build_selection_pointer_plan(BoardPointerPlanKind::SelectionPreview { start, start_screen }, &next, initial_ids, gesture, &points, &screen_points)
        }
        BoardPointerPhase::Up => {
            let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, focus.selection_options.mode.as_str());
            let next = if intent.ctrl_or_meta || intent.shift { resolve_area_selection_with_initial(scene, focus, style, initial_ids, start, &[start], merge_mode.as_str()) } else { BTreeSet::new() };
            let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
            scene.build_selection_pointer_plan(BoardPointerPlanKind::SelectionCommit, &next, initial_ids, gesture, &[], &[])
        }
        BoardPointerPhase::Leave => Ok(BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle)),
    }
}

pub(super) fn plan_selection_pointer(
    scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState,
    intent: BoardPointerIntent,
    initial_ids: &BTreeSet<String>,
    points: &[Point],
    screen_points: &[Point],
    start: Point,
    start_screen: Point,
    screen: Point,
    world: Point,
) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    if intent.phase == BoardPointerPhase::Leave {
        return Ok(BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle));
    }
    if points.len() != screen_points.len() || points.len() >= BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let mut next_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
    let mut next_screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
    next_points[..points.len()].copy_from_slice(points);
    next_screen_points[..screen_points.len()].copy_from_slice(screen_points);
    let mut len = points.len();
    let last_screen = screen_points.last().copied().unwrap_or(start_screen);
    let add_point = intent.phase == BoardPointerPhase::Up || focus.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
    if add_point {
        next_points[len] = world;
        next_screen_points[len] = screen;
        len += 1;
    } else if len > 0 {
        next_points[len - 1] = world;
        next_screen_points[len - 1] = screen;
    }
    let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, focus.selection_options.mode.as_str());
    let end_screen = next_screen_points[len.saturating_sub(1)];
    let click_only = intent.phase == BoardPointerPhase::Up && distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
    let next = if click_only { BTreeSet::new() } else { resolve_area_selection_with_initial(scene, focus, style, initial_ids, start, &next_points[..len], merge_mode.as_str()) };
    let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
    let kind = if intent.phase == BoardPointerPhase::Up { BoardPointerPlanKind::SelectionCommit } else { BoardPointerPlanKind::SelectionPreview { start, start_screen } };
    scene.build_selection_pointer_plan(kind, &next, initial_ids, gesture, &next_points[..len], &next_screen_points[..len])
}

pub(super) fn plan_link_move_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, outbox: &BoardOutboxState, source_id: &str, start_screen: Option<Point>, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let activated = start_screen.is_none_or(|start_screen| distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX);
    let target_id = activated.then(|| nearest_link_snap_handle_world(scene, style, source_id, world)).flatten();
    let hover_id = target_id.clone().or_else(|| resolve_hover_world(scene, focus, style, world));
    let node_ids = link_drag_compatible_target_node_ids(scene, style, source_id);
    let ring_node_id = activated.then(|| link_drag_ring_target_node_id(scene, style, source_id, world)).flatten().filter(|node_id| link_compatible_handle_count_on_node(scene, style, source_id, node_id) > 1);
    let ring_handle_ids = ring_node_id.as_deref().map(|node_id| link_compatible_handle_ids_on_node(scene, style, source_id, node_id)).unwrap_or_default();
    if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let compat_key = format!("{}|{}", source_id, node_ids.join(","));
    let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let events = [
        (outbox.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str())).then(|| BoardOwnedEvent::link_compatible(source_id, &node_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        (outbox.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str())).then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), &ring_handle_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
    ];
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = plan.push_id(source_id)?;
    let target = target_id.as_deref().map(|target_id| plan.push_id(target_id)).transpose()?;
    let hover = hover_id.as_deref().map(|hover_id| plan.push_id(hover_id)).transpose()?;
    let compat_key = plan.push_id(&compat_key)?;
    let ring_key = plan.push_id(&ring_key)?;
    plan.kind = BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world: world, activated, start_screen: start_screen.unwrap_or(screen) };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

pub(super) fn plan_link_finish_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, outbox: &BoardOutboxState, source_id: &str, target_id: Option<&str>, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let direct_target = target_id.filter(|target_id| link_snap_commit_proximity_ok(scene, style, target_id, world));
    let mut target_node_id = None;
    let mut edge_target = direct_target.map(str::to_string);
    let mut also_emit = direct_target.map(|_| BoardEventKind::ProximityConnect);
    if edge_target.is_none() {
        if let Some(node_id) = resolve_node_hit_world(scene, style, world).filter(|node_id| scene.handles.get(source_id).is_some_and(|source| source.node_id.as_str() != node_id.as_str())) {
            if let Some(sole_target) = node_sole_free_link_compatible_handle(scene, style, source_id, &node_id) {
                edge_target = Some(sole_target);
                also_emit = Some(BoardEventKind::IndirectConnect);
            } else if node_has_any_free_link_compatible_handle(scene, style, source_id, &node_id) {
                target_node_id = Some(node_id);
            }
        }
    }
    let edge_id = edge_target.as_deref().and_then(|target| planned_link_edge_id(scene, style, source_id, target));
    if edge_id.is_none() {
        edge_target = None;
        also_emit = None;
    }
    let hover_id = target_node_id.clone().or_else(|| resolve_hover_world(scene, focus, style, world));
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let mut events: [Option<BoardOwnedEvent>; 5] = std::array::from_fn(|_| None);
    let mut compat_key_owned = None;
    let mut ring_key_owned = None;
    if let (Some(edge_id), Some(edge_target)) = (edge_id.as_deref(), edge_target.as_deref()) {
        events[0] = Some(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        if let Some(also_emit) = also_emit {
            events[1] = Some(BoardOwnedEvent::edge(also_emit, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        }
    }
    if let Some(target_node_id) = target_node_id.as_deref() {
        let node_ids = link_drag_compatible_target_node_ids(scene, style, source_id);
        let ring_handle_ids = link_compatible_handle_ids_on_node(scene, style, source_id, target_node_id);
        compat_key_owned = Some(format!("{}|{}", source_id, node_ids.join(",")));
        ring_key_owned = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
        events[2] = Some(BoardOwnedEvent::link_compatible(source_id, &node_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
        events[3] = Some(BoardOwnedEvent::link_ring(source_id, Some(target_node_id), &ring_handle_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
    } else {
        let empty: &[String] = &[];
        events[2] = outbox.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
        events[3] = outbox.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
    }
    if focus.hovered_id != hover_id || focus.hovered_kind.is_some() {
        events[4] = Some(BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
    }
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = plan.push_id(source_id)?;
    let target = edge_target.as_deref().map(|target| plan.push_id(target)).transpose()?;
    let edge = edge_id.as_deref().map(|edge| plan.push_id(edge)).transpose()?;
    let target_node = target_node_id.as_deref().map(|target_node| plan.push_id(target_node)).transpose()?;
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    let compat_key = compat_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
    let ring_key = ring_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
    plan.kind = BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

pub(super) fn plan_link_retain_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let hover_id = resolve_hover_world(scene, focus, style, world);
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let event = (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
        .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
        .transpose()
        .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    plan.kind = BoardPointerPlanKind::LinkRetain { hover };
    plan.seal_optional_events(&[event])?;
    Ok(plan)
}

pub(super) fn plan_link_clear_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, outbox: &BoardOutboxState, source_id: &str, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let hover_id = resolve_hover_world(scene, focus, style, world);
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let empty: &[String] = &[];
    let events = [
        outbox.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        outbox.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
    ];
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = plan.push_id(source_id)?;
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    plan.kind = BoardPointerPlanKind::LinkFinish { source, target: None, edge: None, target_node: None, hover, compat_key: None, ring_key: None };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

pub(super) fn plan_hover_pointer(scene: &BoardSceneState, focus: &BoardFocusState, hover_id: Option<&str>) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let hover_kind = hover_id.and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let event = (focus.hovered_id.as_deref() != hover_id || focus.hovered_kind.is_some())
        .then(|| BoardOwnedEvent::hover(hover_id, hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
        .transpose()
        .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let hover = hover_id.map(|hover| plan.push_id(hover)).transpose()?;
    plan.kind = BoardPointerPlanKind::Hover { hover };
    plan.seal_optional_events(&[event])?;
    Ok(plan)
}

pub(super) fn plan_brush_pointer(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState, brush: &BoardBrushState, intent: BoardPointerIntent, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let next_source = if intent.phase == BoardPointerPhase::Move { brush_nearest_slot_source(scene, style, brush, world) } else { None };
    let offset = if intent.alt || brush.brush_slot_suggestions_active { brush.suggestion_offset } else { 0.0 };
    let next_candidates = if let Some(source_id) = next_source.as_deref() {
        if brush.brush_slot_source_id.as_deref() == Some(source_id) {
            brush.brush_candidates.clone()
        } else {
            let source = scene.handles.get(source_id).ok_or(BoardPointerPlanFault::Unsupported)?;
            brush_compatible_candidates(scene, style, source).ok_or(BoardPointerPlanFault::ItemCredits)?
        }
    } else {
        BrushCandidatePage::default()
    };
    if next_candidates.len() > BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let next_index = if brush.brush_slot_source_id == next_source { brush.brush_candidate_index.min(next_candidates.len().saturating_sub(1)) } else { 0 };
    let next_preview = next_source.as_deref().and_then(|source_id| next_candidates.get(next_index).and_then(|candidate| brush_build_preview_with_offset(scene, style, brush, source_id, candidate, offset)));
    let leaving_old = brush.brush_slot_source_id.is_some() && brush.brush_slot_source_id != next_source;
    let commit_old = intent.alt && leaving_old && brush.brush_preview.is_some();
    let old_preview = brush.brush_slot_source_id.as_deref().and_then(|source_id| brush.brush_candidates.get(brush.brush_candidate_index).and_then(|candidate| brush_build_preview_with_offset(scene, style, brush, source_id, candidate, offset)));
    let hover_id = next_source.clone();
    let hover_kind = hover_id.as_ref().and_then(|hover_id| scene.resolve_element_kind_hover(hover_id));
    let serial = brush.brush_placement_serial.wrapping_add(1);
    let node_id = format!("{}{serial}", style.brush_node_prefix());
    let edge_id = format!("{}{serial}", style.brush_edge_prefix());
    let events = [
        commit_old.then(|| old_preview.as_ref().ok_or(BoardEventFault::Schema).and_then(|preview| BoardOwnedEvent::brush_place(preview, &node_id, &edge_id))).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
        Some(BoardOwnedEvent::brush_preview(next_preview.as_ref()).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
        Some(BoardOwnedEvent::brush_candidates(next_source.as_deref().unwrap_or(""), &next_candidates, next_index, false).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
        (focus.hovered_id != hover_id || focus.hovered_kind.is_some())
            .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
            .transpose()
            .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
    ];
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, BoardPointerPlanKind::Idle);
    let source = next_source.as_deref().map(|source| plan.push_id(source)).transpose()?;
    let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
    for candidate in next_candidates.iter() {
        plan.push_delta(candidate.node_kind_id, candidate.target_handle_index as f64, 0.0)?;
    }
    plan.kind = BoardPointerPlanKind::Brush { source, hover, alt: intent.alt, commit_old };
    plan.seal_optional_events(&events)?;
    Ok(plan)
}

/// 🧲️ A drag frame (`DragMove`) or its release (`FinishDrag`) as a retained plan: every member's resolved
/// position, and for the release the staged `select` plus the ONE gesture record, sealed as one page.
pub(super) fn plan_drag_pointer(scene: &BoardSceneState, style: &BoardStyleState, world: Point, drag: (&str, Vec2, &BTreeMap<String, (f64, f64)>), release: Option<(&GestureStage, Option<&(String, String)>)>) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
    let (primary_id, offset, start_positions) = drag;
    if start_positions.len() > BOARD_POINTER_ITEM_CAPACITY {
        return Err(BoardPointerPlanFault::ItemCredits);
    }
    let delta = drag_delta(scene, style, world, primary_id, offset, start_positions).ok_or(BoardPointerPlanFault::Unsupported)?;
    let mut plan = BoardPointerPlan::empty(scene.interaction_revision, if release.is_some() { BoardPointerPlanKind::FinishDrag } else { BoardPointerPlanKind::DragMove });
    for (id, (x, y)) in start_positions {
        plan.push_delta(id, x + delta.x, y + delta.y)?;
    }
    if let Some((gesture, proximity_pair)) = release {
        let [select, record] = gesture_release_events(gesture, drag_gesture_record(gesture, start_positions, delta, proximity_pair));
        plan.seal_optional_events(&[select.transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?, record.transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?])?;
    }
    Ok(plan)
}

pub fn begin_pointer_commit(scene: &mut BoardSceneState, commit: &mut BoardCommitState, plan: BoardPointerPlan) -> Result<(), BoardPointerPlan> {
    if commit.pointer_publication.is_some() || scene.interaction_revision != plan.revision || !plan.requires_retained_commit() {
        return Err(plan);
    }
    if let Some(active) = commit.pending_pointer_commit.as_ref() {
        if active.plan.revision != plan.revision || !matches!(active.plan.kind, BoardPointerPlanKind::DragMove) || !matches!(plan.kind, BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag) {
            return Err(plan);
        }
        match commit.queued_pointer_commit.as_ref().map(|queued| queued.kind) {
            None => {
                commit.queued_pointer_commit = Some(plan);
                return Ok(());
            }
            Some(BoardPointerPlanKind::DragMove) => {
                commit.queued_pointer_commit = Some(plan);
                return Ok(());
            }
            Some(_) => return Err(plan),
        }
    }
    commit.pending_pointer_commit = Some(BoardPointerCommitOperation::new(plan));
    Ok(())
}

pub fn begin_pointer_cancel(scene: &mut BoardSceneState, commit: &mut BoardCommitState) {
    let revision = scene.interaction_revision.wrapping_add(1);
    scene.interaction_revision = revision;
    if let Some(operation) = commit.pending_pointer_commit.as_mut() {
        operation.cancelling = true;
        operation.plan.revision = revision;
    }
    if commit.cancelled_pointer_plan_retirement.is_none() {
        commit.cancelled_pointer_plan_retirement = commit.queued_pointer_commit.take().map(BoardPointerPlanRetirement::new);
    }
}

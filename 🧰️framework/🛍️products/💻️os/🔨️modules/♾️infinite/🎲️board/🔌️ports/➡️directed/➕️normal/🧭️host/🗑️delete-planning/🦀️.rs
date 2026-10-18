//! 🗑️ Delete planning of the normal board host: the delete selection plan, the pending delete step and the board delete selection entry point. See [`super`].

use super::*;

#[cfg(any(test, feature = "testkit"))]
pub fn plan_delete_selection(scene: &BoardSceneState, focus: &BoardFocusState, style: &BoardStyleState) -> Result<BoardDeletePlan, BoardEventFault> {
    let mut plan = BoardDeletePlan::new(scene.interaction_revision);
    for id in focus.selection.iter().filter(|id| scene.edges.contains_key(*id)) {
        let edge = scene.edges.get(id).expect("selected edge exists");
        plan.push_entity(BoardDeleteKind::Edge, id, &edge.properties)?;
    }
    if style.has_ports() {
        for (node_id, node) in scene.nodes.iter().filter(|(node_id, _)| focus.selection.contains(*node_id) || scene.handles.values().any(|handle| handle.node_id == **node_id && focus.selection.contains(&handle.id))) {
            for (handle_id, handle) in scene.handles.iter().filter(|(_, handle)| handle.node_id == *node_id) {
                for (wire_id, wire) in scene.wires.iter().filter(|(_, wire)| wire.source == *handle_id || wire.target.as_ref() == Some(handle_id)) {
                    plan.push_entity(BoardDeleteKind::Wire, wire_id, &wire.properties)?;
                }
                for (edge_id, edge) in scene.edges.iter().filter(|(_, edge)| edge.source == *handle_id || edge.target == *handle_id) {
                    plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
                }
                plan.push_entity(BoardDeleteKind::Handle, handle_id, &handle.properties)?;
            }
            plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
        }
    } else {
        for (node_id, node) in scene.nodes.iter().filter(|(node_id, _)| focus.selection.contains(*node_id)) {
            for (edge_id, edge) in scene.edges.iter().filter(|(_, edge)| edge.source == *node_id || edge.target == *node_id) {
                plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
            }
            plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
        }
    }
    Ok(plan)
}

pub(crate) fn step_pending_delete_operation(scene: &mut BoardSceneState, focus: &mut BoardFocusState, outbox: &mut BoardOutboxState, commit: &mut BoardCommitState) {
    let Some(mut operation) = commit.pending_delete_operation.take() else {
        return;
    };
    if let Some(retiring) = operation.retiring_entity.as_mut() {
        match retiring.step() {
            Ok(true) => {
                debug_assert!(retiring.terminal_is_empty());
                operation.retiring_entity = None;
            }
            Ok(false) => {}
            Err(()) => outbox.event_schema_fault = true,
        }
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if !operation.claimed {
        if scene.interaction_revision != operation.plan.revision {
            outbox.event_schema_fault = true;
            return;
        }
        let event_count = operation.plan.emitted_event_count() + 1;
        let Some(event_bytes) = operation.plan.emitted_event_bytes().and_then(|bytes| bytes.checked_add(operation.select_event.as_ref()?.owned_bytes())) else {
            outbox.event_schema_fault = true;
            return;
        };
        if outbox.events.claim(event_count, event_bytes).is_err() {
            commit.pending_delete_operation = Some(operation);
            return;
        }
        operation.claimed = true;
        operation.remaining_claimed_items = event_count as u16;
        operation.remaining_claimed_bytes = event_bytes;
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if operation.mutation_cursor < operation.plan.len {
        let entry = operation.plan.entries[usize::from(operation.mutation_cursor)].expect("delete plan entry");
        let id = operation.plan.id(entry.id);
        let retiring = match entry.kind {
            BoardDeleteKind::Edge => {
                let removed = scene.edges.remove(id).map(BoardRemovedEntity::Edge);
                focus.selection.remove(id);
                removed
            }
            BoardDeleteKind::Node => {
                let removed = scene.nodes.remove(id).map(BoardRemovedEntity::Node);
                focus.selection.remove(id);
                removed
            }
            BoardDeleteKind::Handle => {
                let removed = scene.handles.remove(id).map(BoardRemovedEntity::Handle);
                focus.selection.remove(id);
                removed
            }
            BoardDeleteKind::Wire => {
                let removed = scene.wires.remove(id).map(BoardRemovedEntity::Wire);
                focus.selection.remove(id);
                removed
            }
        };
        operation.mutation_cursor += 1;
        operation.retiring_entity = retiring.map(BoardEntityRetirement::new);
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if operation.publication_cursor < operation.plan.len {
        let entry = operation.plan.entries[usize::from(operation.publication_cursor)].expect("delete plan entry");
        operation.publication_cursor += 1;
        let id = operation.plan.id(entry.id);
        let event = match entry.kind {
            BoardDeleteKind::Edge => Some(BoardOwnedEvent::id(BoardEventKind::EdgeDelete, id).expect("delete edge payload was exactly preflighted")),
            BoardDeleteKind::Node => Some(BoardOwnedEvent::id(BoardEventKind::NodeDelete, id).expect("delete node payload was exactly preflighted")),
            BoardDeleteKind::Handle | BoardDeleteKind::Wire => None,
        };
        if let Some(event) = event {
            operation.remaining_claimed_items -= 1;
            operation.remaining_claimed_bytes -= event.owned_bytes();
            outbox.events.push_claimed(event).expect("delete event owns claimed queue credits");
        }
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if let Some(event) = operation.select_event.take() {
        operation.remaining_claimed_items -= 1;
        operation.remaining_claimed_bytes -= event.owned_bytes();
        outbox.events.push_claimed(event).expect("delete selection owns claimed queue credits");
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if let Some(id) = focus.selection_exit_highlight.pop_first() {
        drop(id);
        commit.pending_delete_operation = Some(operation);
        return;
    }
    if operation.remaining_claimed_items != 0 || operation.remaining_claimed_bytes != 0 {
        let _ = outbox.events.release_claim(usize::from(operation.remaining_claimed_items), operation.remaining_claimed_bytes);
        outbox.event_schema_fault = true;
    }
    scene.bump_content_scene_generation();
    scene.interaction_revision = operation.plan.revision.wrapping_add(1);
}

pub fn board_delete_selection(scene: &mut BoardSceneState, outbox: &mut BoardOutboxState, commit: &mut BoardCommitState) {
    if commit.pending_delete_planning.is_some() || commit.pending_delete_operation.is_some() {
        return;
    }
    let mut select_builder = BoardPayloadBuilder::new();
    if select_builder.raw("{\"ids\":[").is_err() {
        outbox.event_schema_fault = true;
        return;
    }
    commit.pending_delete_planning = Some(BoardDeletePlanningOperation {
        plan: BoardDeletePlan::new(scene.interaction_revision),
        phase: BoardDeletePlanningPhase::SelectedEdges,
        scan_after: None,
        node_after: None,
        handle_after: None,
        relation_after: None,
        node_id: None,
        handle_id: None,
        node_relevant: false,
        property_audit: None,
        select_builder,
        select_first: true,
        fault: None,
        cancelling: false,
    });
}

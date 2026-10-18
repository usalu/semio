//! 🗑️ Delete planning step of the board gestures: the resumable delete planning step. See [`super`].

use super::*;

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

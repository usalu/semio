//! 🧮️ Cursor steps of the Flow VCS: the scan, shift, mutation, document replacement and rollback steps of the action cursor. See [`super`].

use super::*;

//#region 🌊️RetainedVcs
//#region 🌊️RetainedActionCursor
pub(super) fn flow_vcs_step_scan(host_snapshot: &FlowHostSnapshot, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let index = operation.cursor.scan;
    let action = operation.action.as_ref().ok_or(FlowVcsFault::InvalidMutation)?;
    match action {
        FlowVcsAction::InsertWidget { index: target, item } => {
            if *target > host_snapshot.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if index == host_snapshot.widgets.len() {
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
            if widget_id_for(&host_snapshot.widgets[index]) == widget_id_for(item) {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsAction::RemoveWidgetAt { index: target } => {
            if *target >= host_snapshot.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            operation.cursor.origin = *target;
            operation.cursor.current = *target;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            return Ok(());
        }
        FlowVcsAction::RemoveWidget { id } | FlowVcsAction::MoveWidget { id, .. } | FlowVcsAction::PatchWidget { id, .. } => {
            if index == host_snapshot.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if widget_id_for(&host_snapshot.widgets[index]) == id {
                operation.cursor.origin = index;
                operation.cursor.current = index;
                operation.cursor.phase = if matches!(action, FlowVcsAction::PatchWidget { .. }) { FlowVcsCursorPhase::Mutate } else { FlowVcsCursorPhase::Shift };
                return Ok(());
            }
        }
        FlowVcsAction::InsertSynapse { index: target, item } => {
            if *target > host_snapshot.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if index == host_snapshot.synapses.len() {
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
            if host_snapshot.synapses[index].id == item.id {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsAction::RemoveSynapseAt { index: target } => {
            if *target >= host_snapshot.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            operation.cursor.origin = *target;
            operation.cursor.current = *target;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            return Ok(());
        }
        FlowVcsAction::RemoveSynapse { id } | FlowVcsAction::MoveSynapse { id, .. } | FlowVcsAction::PatchSynapse { id, .. } => {
            if index == host_snapshot.synapses.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if host_snapshot.synapses[index].id == *id {
                operation.cursor.origin = index;
                operation.cursor.current = index;
                operation.cursor.phase = if matches!(action, FlowVcsAction::PatchSynapse { .. }) { FlowVcsCursorPhase::Mutate } else { FlowVcsCursorPhase::Shift };
                return Ok(());
            }
        }
        FlowVcsAction::SetLayout(entry) => {
            if index == host_snapshot.widgets.len() {
                return Err(FlowVcsFault::InvalidMutation);
            }
            if widget_id_for(&host_snapshot.widgets[index]) == entry.id {
                operation.cursor.origin = index;
                operation.cursor.phase = FlowVcsCursorPhase::Mutate;
                return Ok(());
            }
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    operation.cursor.scan += 1;
    Ok(())
}

pub(super) fn flow_vcs_step_shift(host_snapshot: &mut FlowHostSnapshot, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let cursor = &mut operation.cursor;
    match cursor.kind {
        FlowVcsCursorKind::InsertWidget | FlowVcsCursorKind::InsertSynapse => {
            if cursor.current > cursor.target {
                if cursor.kind == FlowVcsCursorKind::InsertWidget {
                    host_snapshot.widgets.swap(cursor.current, cursor.current - 1);
                } else {
                    host_snapshot.synapses.swap(cursor.current, cursor.current - 1);
                }
                cursor.current -= 1;
                return Ok(());
            }
        }
        FlowVcsCursorKind::RemoveWidget | FlowVcsCursorKind::RemoveSynapse => {
            let length = if cursor.kind == FlowVcsCursorKind::RemoveWidget { host_snapshot.widgets.len() } else { host_snapshot.synapses.len() };
            if cursor.current + 1 < length {
                if cursor.kind == FlowVcsCursorKind::RemoveWidget {
                    host_snapshot.widgets.swap(cursor.current, cursor.current + 1);
                } else {
                    host_snapshot.synapses.swap(cursor.current, cursor.current + 1);
                }
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
        }
        FlowVcsCursorKind::MoveWidget | FlowVcsCursorKind::MoveSynapse => {
            if cursor.current < cursor.target {
                if cursor.kind == FlowVcsCursorKind::MoveWidget {
                    host_snapshot.widgets.swap(cursor.current, cursor.current + 1);
                } else {
                    host_snapshot.synapses.swap(cursor.current, cursor.current + 1);
                }
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
            if cursor.current > cursor.target {
                if cursor.kind == FlowVcsCursorKind::MoveWidget {
                    host_snapshot.widgets.swap(cursor.current, cursor.current - 1);
                } else {
                    host_snapshot.synapses.swap(cursor.current, cursor.current - 1);
                }
                cursor.current -= 1;
                cursor.mutated = true;
                return Ok(());
            }
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    cursor.phase = if matches!(cursor.kind, FlowVcsCursorKind::InsertWidget | FlowVcsCursorKind::InsertSynapse) { FlowVcsCursorPhase::TransferHistory } else { FlowVcsCursorPhase::Mutate };
    Ok(())
}

pub(super) fn flow_vcs_step_mutation(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
    if let Some(update) = operation.layout_update.as_mut() {
        let capacity = update.next_capacity_byte_demand().map_err(|_| FlowVcsFault::Limit)?;
        update.advance(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: grant.bytes, maximum_capacity_bytes: capacity, maximum_release_bytes: 0, maximum_depth: update.next_depth_demand() }).map_err(|error| if matches!(error.kind, semio_framework_value::ValueRefusalKind::DepthLimit) { FlowVcsFault::Depth } else { FlowVcsFault::Limit })?;
        if let Some(layout) = update.take_result() {
            let previous = std::mem::replace(&mut document.host_snapshot_mut().layout, layout);
            operation.action = Some(FlowVcsAction::LayoutRoot(previous));
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
        }
        return Ok(());
    }
    let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
    let host_snapshot = document.host_snapshot_mut();
    operation.action = Some(match action {
        FlowVcsAction::InsertWidget { index, item } => {
            host_snapshot.widgets.push(item);
            operation.cursor.current = host_snapshot.widgets.len() - 1;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveWidgetAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveWidget { .. } | FlowVcsAction::RemoveWidgetAt { .. }) => {
            let item = host_snapshot.widgets.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertWidget { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveWidget { id, .. } => FlowVcsAction::MoveWidget { id, index: operation.cursor.origin },
        FlowVcsAction::PatchWidget { id, mut item } => {
            std::mem::swap(&mut host_snapshot.widgets[operation.cursor.origin], &mut item);
            operation.cursor.mutated = true;
            FlowVcsAction::PatchWidget { id, item }
        }
        FlowVcsAction::InsertSynapse { index, item } => {
            host_snapshot.synapses.push(item);
            operation.cursor.current = host_snapshot.synapses.len() - 1;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveSynapseAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveSynapse { .. } | FlowVcsAction::RemoveSynapseAt { .. }) => {
            let item = host_snapshot.synapses.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertSynapse { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveSynapse { id, .. } => FlowVcsAction::MoveSynapse { id, index: operation.cursor.origin },
        FlowVcsAction::PatchSynapse { id, mut item } => {
            std::mem::swap(&mut host_snapshot.synapses[operation.cursor.origin], &mut item);
            operation.cursor.mutated = true;
            FlowVcsAction::PatchSynapse { id, item }
        }
        FlowVcsAction::SetLayout(entry) => {
            operation.layout_update = Some(match entry.layout {
                Some(layout) => host_snapshot.layout.begin_set(entry.id, layout),
                None => host_snapshot.layout.begin_remove(entry.id),
            });
            return Ok(());
        }
        FlowVcsAction::LayoutRoot(layout) => {
            operation.cursor.mutated = true;
            FlowVcsAction::LayoutRoot(std::mem::replace(&mut host_snapshot.layout, layout))
        }
        FlowVcsAction::Checkpoint => FlowVcsAction::Checkpoint,
        action => {
            operation.action = Some(action);
            return Err(FlowVcsFault::InvalidMutation);
        }
    });
    operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
    Ok(())
}

pub(super) fn flow_vcs_step_document_replacement(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let action = operation.action.as_mut().ok_or(FlowVcsFault::InvalidMutation)?;
    let source = match action {
        FlowVcsAction::ReplaceDocument(source) => source,
        _ => return Err(FlowVcsFault::InvalidMutation),
    };
    match operation.cursor.phase {
        FlowVcsCursorPhase::ReserveReplacement => {
            if document.versions.is_full() {
                return Err(FlowVcsFault::Full);
            }
            let empty = FlowHostSnapshot { schema: String::new(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 0.0 }, widgets: Vec::new(), synapses: Vec::new(), layout: OrderedMap::new() };
            document.versions.push(empty).map_err(|_| FlowVcsFault::Full)?;
            operation.cursor.target = document.versions.len() - 1;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceSchema;
        }
        FlowVcsCursorPhase::ReplaceSchema => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").schema = std::mem::take(&mut source.schema);
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraX;
        }
        FlowVcsCursorPhase::ReplaceCameraX => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.x = source.camera.x;
            source.camera.x = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraY;
        }
        FlowVcsCursorPhase::ReplaceCameraY => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.y = source.camera.y;
            source.camera.y = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceCameraZoom;
        }
        FlowVcsCursorPhase::ReplaceCameraZoom => {
            document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").camera.zoom = source.camera.zoom;
            source.camera.zoom = 0.0;
            operation.cursor.phase = FlowVcsCursorPhase::ReplaceWidgets;
        }
        FlowVcsCursorPhase::ReplaceWidgets => {
            if let Some(widget) = source.widgets.pop() {
                document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").widgets.push(widget);
            } else {
                operation.cursor.scan = 0;
                operation.cursor.phase = FlowVcsCursorPhase::ReverseWidgets;
            }
        }
        FlowVcsCursorPhase::ReverseWidgets => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            if operation.cursor.scan < target.widgets.len() / 2 {
                let opposite = target.widgets.len() - operation.cursor.scan - 1;
                target.widgets.swap(operation.cursor.scan, opposite);
                operation.cursor.scan += 1;
            } else {
                operation.cursor.phase = FlowVcsCursorPhase::ReplaceSynapses;
            }
        }
        FlowVcsCursorPhase::ReplaceSynapses => {
            if let Some(synapse) = source.synapses.pop() {
                document.versions.get_mut(operation.cursor.target).expect("retained replacement slot").synapses.push(synapse);
            } else {
                operation.cursor.scan = 0;
                operation.cursor.phase = FlowVcsCursorPhase::ReverseSynapses;
            }
        }
        FlowVcsCursorPhase::ReverseSynapses => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            if operation.cursor.scan < target.synapses.len() / 2 {
                let opposite = target.synapses.len() - operation.cursor.scan - 1;
                target.synapses.swap(operation.cursor.scan, opposite);
                operation.cursor.scan += 1;
            } else {
                operation.cursor.phase = FlowVcsCursorPhase::ReplaceLayout;
            }
        }
        FlowVcsCursorPhase::ReplaceLayout => {
            let target = document.versions.get_mut(operation.cursor.target).expect("retained replacement slot");
            std::mem::swap(&mut target.layout, &mut source.layout);
            operation.cursor.origin = document.active;
            document.active = operation.cursor.target;
            operation.action = Some(FlowVcsAction::Checkpoint);
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::TransferHistory;
        }
        _ => return Err(FlowVcsFault::InvalidMutation),
    }
    Ok(())
}

pub(super) fn flow_vcs_step_rollback(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation) -> Result<bool, FlowVcsFault> {
    if !operation.cursor.mutated {
        return Ok(true);
    }
    let cursor = &mut operation.cursor;
    match cursor.kind {
        FlowVcsCursorKind::InsertWidget => {
            let host_snapshot = document.host_snapshot_mut();
            if cursor.current + 1 < host_snapshot.widgets.len() {
                host_snapshot.widgets.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            let item = host_snapshot.widgets.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.action = Some(FlowVcsAction::InsertWidget { index: cursor.target, item });
        }
        FlowVcsCursorKind::InsertSynapse => {
            let host_snapshot = document.host_snapshot_mut();
            if cursor.current + 1 < host_snapshot.synapses.len() {
                host_snapshot.synapses.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            let item = host_snapshot.synapses.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            operation.action = Some(FlowVcsAction::InsertSynapse { index: cursor.target, item });
        }
        FlowVcsCursorKind::RemoveWidget => {
            let host_snapshot = document.host_snapshot_mut();
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertWidget { .. })) {
                let FlowVcsAction::InsertWidget { item, .. } = operation.action.take().expect("retained widget inverse") else { unreachable!() };
                host_snapshot.widgets.push(item);
                cursor.current = host_snapshot.widgets.len() - 1;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                host_snapshot.widgets.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::RemoveSynapse => {
            let host_snapshot = document.host_snapshot_mut();
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertSynapse { .. })) {
                let FlowVcsAction::InsertSynapse { item, .. } = operation.action.take().expect("retained synapse inverse") else { unreachable!() };
                host_snapshot.synapses.push(item);
                cursor.current = host_snapshot.synapses.len() - 1;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                host_snapshot.synapses.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::MoveWidget => {
            let host_snapshot = document.host_snapshot_mut();
            if cursor.current < cursor.origin {
                host_snapshot.widgets.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                host_snapshot.widgets.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::MoveWidget { id, .. } = action {
                operation.action = Some(FlowVcsAction::MoveWidget { id, index: cursor.target });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::MoveSynapse => {
            let host_snapshot = document.host_snapshot_mut();
            if cursor.current < cursor.origin {
                host_snapshot.synapses.swap(cursor.current, cursor.current + 1);
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                host_snapshot.synapses.swap(cursor.current, cursor.current - 1);
                cursor.current -= 1;
                return Ok(false);
            }
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::MoveSynapse { id, .. } = action {
                operation.action = Some(FlowVcsAction::MoveSynapse { id, index: cursor.target });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::PatchWidget => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::PatchWidget { id, mut item } = action {
                std::mem::swap(&mut document.host_snapshot_mut().widgets[cursor.origin], &mut item);
                operation.action = Some(FlowVcsAction::PatchWidget { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::PatchSynapse => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::PatchSynapse { id, mut item } = action {
                std::mem::swap(&mut document.host_snapshot_mut().synapses[cursor.origin], &mut item);
                operation.action = Some(FlowVcsAction::PatchSynapse { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::Layout => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            match action {
                FlowVcsAction::LayoutRoot(layout) => {
                    operation.action = Some(FlowVcsAction::LayoutRoot(std::mem::replace(&mut document.host_snapshot_mut().layout, layout)));
                }
                _ => return Err(FlowVcsFault::InvalidMutation),
            }
        }
        FlowVcsCursorKind::ReplaceDocument => {
            if document.active == cursor.target {
                document.active = cursor.origin;
                return Ok(false);
            }
            if cursor.target + 1 != document.versions.len() {
                return Err(FlowVcsFault::ClosePending);
            }
            let candidate = document.versions.pop().ok_or(FlowVcsFault::InvalidMutation)?;
            retain_owner(&mut operation.retirement,FlowVcsClosingOwner::Snapshot(candidate));
        }
        FlowVcsCursorKind::None => {}
    }
    if let Some(action) = operation.rollback_owner.take() {
        operation.action = Some(action);
    }
    cursor.mutated = false;
    Ok(true)
}
//#endregion 🌊️RetainedActionCursor
//#endregion 🌊️RetainedVcs

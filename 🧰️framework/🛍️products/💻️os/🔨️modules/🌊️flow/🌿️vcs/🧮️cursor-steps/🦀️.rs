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

pub(super) /// 🎯️ The one write lane of the retained stepper. The concrete mutation's diff is built from reads of the active version and turned into a
/// successor ONLY by [`protocol::apply_diff_cold`]; the successor replaces the displaced active version, which retires cold. Returns the mutation's
/// own concrete inverse rows (built from the pre-state), which the caller keeps as its history inverse or retires.
fn flow_vcs_apply_inverted(document: &mut FlowVcsDocument, mutation: FlowMutation) -> Result<Vec<FlowMutation>, FlowVcsFault> {
    let active = document.host_snapshot();
    let inverse = crate::os_spr::Mutation::inverse(&mutation, active).map_err(|_| FlowVcsFault::InvalidMutation);
    let (diff, messages) = crate::os_spr::Mutation::diff(&mutation, active).into_parts();
    let applied = if messages.is_empty() { protocol::apply_diff_cold(&diff, active).ok() } else { None };
    <FlowDiff as protocol::MutationDiff<FlowHostSnapshot>>::retire_cold(diff);
    retire_flow_mutation(mutation);
    match (applied, inverse) {
        (Some(next), Ok(inverse)) => {
            document.install_applied(next).retire_cold();
            Ok(inverse)
        }
        (next, inverse) => {
            next.into_iter().for_each(FlowHostSnapshot::retire_cold);
            inverse.into_iter().flatten().for_each(retire_flow_mutation);
            Err(FlowVcsFault::InvalidMutation)
        }
    }
}

/// ▶️ [`flow_vcs_apply_inverted`] when the step keeps no inverse.
fn flow_vcs_apply(document: &mut FlowVcsDocument, mutation: FlowMutation) -> Result<(), FlowVcsFault> {
    flow_vcs_apply_inverted(document, mutation)?.into_iter().for_each(retire_flow_mutation);
    Ok(())
}

fn flow_vcs_wire_index(index: usize) -> Result<u32, FlowVcsFault> {
    u32::try_from(index).map_err(|_| FlowVcsFault::Limit)
}

/// 🧲️ The widget a `remove-widget` inverse (`add-widget`) carries, any other row retired.
fn flow_vcs_inverse_widget(inverse: Vec<FlowMutation>) -> Result<Widget, FlowVcsFault> {
    let mut rows = inverse.into_iter();
    match rows.next() {
        Some(FlowMutation::AddWidget(AddWidget { widget, .. })) => {
            rows.for_each(retire_flow_mutation);
            Ok(widget)
        }
        other => {
            other.into_iter().chain(rows).for_each(retire_flow_mutation);
            Err(FlowVcsFault::InvalidMutation)
        }
    }
}

/// 🧲️ The synapse a `remove-synapse` inverse (`add-synapse`) carries, any other row retired.
fn flow_vcs_inverse_synapse(inverse: Vec<FlowMutation>) -> Result<SynapseSpec, FlowVcsFault> {
    let mut rows = inverse.into_iter();
    match rows.next() {
        Some(FlowMutation::AddSynapse(AddSynapse { synapse, .. })) => {
            rows.for_each(retire_flow_mutation);
            Ok(synapse)
        }
        other => {
            other.into_iter().chain(rows).for_each(retire_flow_mutation);
            Err(FlowVcsFault::InvalidMutation)
        }
    }
}

/// 🧲️ The previous widget (under its own id) a `change-widget` inverse carries, any other row retired.
fn flow_vcs_inverse_changed_widget(inverse: Vec<FlowMutation>) -> Result<(String, Widget), FlowVcsFault> {
    let mut rows = inverse.into_iter();
    match rows.next() {
        Some(FlowMutation::ChangeWidget(ChangeWidget { id, widget })) => {
            rows.for_each(retire_flow_mutation);
            Ok((id, widget))
        }
        other => {
            other.into_iter().chain(rows).for_each(retire_flow_mutation);
            Err(FlowVcsFault::InvalidMutation)
        }
    }
}

/// 🧲️ The previous synapse (under its own id) a `change-synapse` inverse carries, any other row retired.
fn flow_vcs_inverse_changed_synapse(inverse: Vec<FlowMutation>) -> Result<(String, SynapseSpec), FlowVcsFault> {
    let mut rows = inverse.into_iter();
    match rows.next() {
        Some(FlowMutation::ChangeSynapse(ChangeSynapse { id, synapse })) => {
            rows.for_each(retire_flow_mutation);
            Ok((id, synapse))
        }
        other => {
            other.into_iter().chain(rows).for_each(retire_flow_mutation);
            Err(FlowVcsFault::InvalidMutation)
        }
    }
}

/// 🧲️ The previous layout entry a one-entry `change-layout` inverse carries, any other row retired.
fn flow_vcs_inverse_layout_entry(inverse: Vec<FlowMutation>) -> Result<FlowLayoutEntry, FlowVcsFault> {
    let mut rows = inverse.into_iter();
    match rows.next() {
        Some(FlowMutation::ChangeLayout(ChangeLayout { mut entries })) if entries.len() == 1 => {
            rows.for_each(retire_flow_mutation);
            entries.pop().ok_or(FlowVcsFault::InvalidMutation)
        }
        other => {
            other.into_iter().chain(rows).for_each(retire_flow_mutation);
            Err(FlowVcsFault::InvalidMutation)
        }
    }
}

/// 🔀️ One adjacent reorder of the active version. The step is the concrete `move-widget`/`move-synapse` kind: its diff names a single
/// `{id, from, to}` row read from the active version, and only [`protocol::apply_diff_cold`] turns that diff into the successor version.
fn flow_vcs_move_adjacent(document: &mut FlowVcsDocument, widgets: bool, from: usize, to: usize) -> Result<(), FlowVcsFault> {
    let active = document.host_snapshot();
    let id = if widgets { active.widgets.get(from).map(|widget| widget_id_for(widget).to_owned()) } else { active.synapses.get(from).map(|synapse| synapse.id.as_str().to_owned()) }.ok_or(FlowVcsFault::InvalidMutation)?;
    let to_index = flow_vcs_wire_index(to)?;
    flow_vcs_apply(document, if widgets { FlowMutation::MoveWidget(MoveWidget { id, to_index }) } else { FlowMutation::MoveSynapse(MoveSynapse { id, to_index }) })
}

pub(crate) fn flow_vcs_step_shift(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation) -> Result<(), FlowVcsFault> {
    let cursor = &mut operation.cursor;
    match cursor.kind {
        FlowVcsCursorKind::InsertWidget | FlowVcsCursorKind::InsertSynapse => {
            if cursor.current > cursor.target {
                flow_vcs_move_adjacent(document, cursor.kind == FlowVcsCursorKind::InsertWidget, cursor.current, cursor.current - 1)?;
                cursor.current -= 1;
                return Ok(());
            }
        }
        FlowVcsCursorKind::RemoveWidget | FlowVcsCursorKind::RemoveSynapse => {
            let widgets = cursor.kind == FlowVcsCursorKind::RemoveWidget;
            let length = if widgets { document.host_snapshot().widgets.len() } else { document.host_snapshot().synapses.len() };
            if cursor.current + 1 < length {
                flow_vcs_move_adjacent(document, widgets, cursor.current, cursor.current + 1)?;
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
        }
        FlowVcsCursorKind::MoveWidget | FlowVcsCursorKind::MoveSynapse => {
                        let widgets = cursor.kind == FlowVcsCursorKind::MoveWidget;
if cursor.current < cursor.target {
                flow_vcs_move_adjacent(document, widgets, cursor.current, cursor.current + 1)?;
                cursor.current += 1;
                cursor.mutated = true;
                return Ok(());
            }
            if cursor.current > cursor.target {
                flow_vcs_move_adjacent(document, widgets, cursor.current, cursor.current - 1)?;
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

pub(super) fn flow_vcs_step_mutation(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation, _grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
    let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
    
    operation.action = Some(match action {
        FlowVcsAction::InsertWidget { index, item } => {
            let length = document.host_snapshot().widgets.len();
            flow_vcs_apply(document, FlowMutation::AddWidget(AddWidget { index: flow_vcs_wire_index(length)?, widget: item }))?;
            operation.cursor.current = length;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveWidgetAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveWidget { .. } | FlowVcsAction::RemoveWidgetAt { .. }) => {
            let id = document.host_snapshot().widgets.last().map(|widget| widget_id_for(widget).to_owned()).ok_or(FlowVcsFault::InvalidMutation)?;
            let item = flow_vcs_inverse_widget(flow_vcs_apply_inverted(document, FlowMutation::RemoveWidget(RemoveWidget { id }))?)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertWidget { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveWidget { id, .. } => FlowVcsAction::MoveWidget { id, index: operation.cursor.origin },
        FlowVcsAction::PatchWidget { id, item } => {
            let (id, item) = flow_vcs_inverse_changed_widget(flow_vcs_apply_inverted(document, FlowMutation::ChangeWidget(ChangeWidget { id, widget: item }))?)?;
            operation.cursor.mutated = true;
            FlowVcsAction::PatchWidget { id, item }
        }
        FlowVcsAction::InsertSynapse { index, item } => {
            let length = document.host_snapshot().synapses.len();
            flow_vcs_apply(document, FlowMutation::AddSynapse(AddSynapse { index: flow_vcs_wire_index(length)?, synapse: item }))?;
            operation.cursor.current = length;
            operation.cursor.target = index;
            operation.cursor.mutated = true;
            operation.cursor.phase = FlowVcsCursorPhase::Shift;
            operation.action = Some(FlowVcsAction::RemoveSynapseAt { index });
            return Ok(());
        }
        action @ (FlowVcsAction::RemoveSynapse { .. } | FlowVcsAction::RemoveSynapseAt { .. }) => {
            let id = document.host_snapshot().synapses.last().map(|synapse| synapse.id.as_str().to_owned()).ok_or(FlowVcsFault::InvalidMutation)?;
            let item = flow_vcs_inverse_synapse(flow_vcs_apply_inverted(document, FlowMutation::RemoveSynapse(RemoveSynapse { id }))?)?;
            operation.rollback_owner = Some(action);
            operation.cursor.mutated = true;
            FlowVcsAction::InsertSynapse { index: operation.cursor.origin, item }
        }
        FlowVcsAction::MoveSynapse { id, .. } => FlowVcsAction::MoveSynapse { id, index: operation.cursor.origin },
        FlowVcsAction::PatchSynapse { id, item } => {
            let (id, item) = flow_vcs_inverse_changed_synapse(flow_vcs_apply_inverted(document, FlowMutation::ChangeSynapse(ChangeSynapse { id, synapse: item }))?)?;
            operation.cursor.mutated = true;
            FlowVcsAction::PatchSynapse { id, item }
        }
        FlowVcsAction::SetLayout(entry) | FlowVcsAction::LayoutEntry(entry) => {
            let previous = flow_vcs_inverse_layout_entry(flow_vcs_apply_inverted(document, FlowMutation::ChangeLayout(ChangeLayout { entries: vec![entry] }))?)?;
            operation.cursor.mutated = true;
            FlowVcsAction::LayoutEntry(previous)
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
            if cursor.current + 1 < document.host_snapshot().widgets.len() {
                flow_vcs_move_adjacent(document, true, cursor.current, cursor.current + 1)?;
                cursor.current += 1;
                return Ok(false);
            }
            let id = document.host_snapshot().widgets.last().map(|widget| widget_id_for(widget).to_owned()).ok_or(FlowVcsFault::InvalidMutation)?;
            let item = flow_vcs_inverse_widget(flow_vcs_apply_inverted(document, FlowMutation::RemoveWidget(RemoveWidget { id }))?)?;
            operation.action = Some(FlowVcsAction::InsertWidget { index: cursor.target, item });
        }
        FlowVcsCursorKind::InsertSynapse => {
            if cursor.current + 1 < document.host_snapshot().synapses.len() {
                flow_vcs_move_adjacent(document, false, cursor.current, cursor.current + 1)?;
                cursor.current += 1;
                return Ok(false);
            }
            let id = document.host_snapshot().synapses.last().map(|synapse| synapse.id.as_str().to_owned()).ok_or(FlowVcsFault::InvalidMutation)?;
            let item = flow_vcs_inverse_synapse(flow_vcs_apply_inverted(document, FlowMutation::RemoveSynapse(RemoveSynapse { id }))?)?;
            operation.action = Some(FlowVcsAction::InsertSynapse { index: cursor.target, item });
        }
        FlowVcsCursorKind::RemoveWidget => {
            
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertWidget { .. })) {
                let FlowVcsAction::InsertWidget { item, .. } = operation.action.take().expect("retained widget inverse") else { unreachable!() };
                let length = document.host_snapshot().widgets.len();
                flow_vcs_apply(document, FlowMutation::AddWidget(AddWidget { index: flow_vcs_wire_index(length)?, widget: item }))?;
                cursor.current = length;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                flow_vcs_move_adjacent(document, true, cursor.current, cursor.current - 1)?;
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::RemoveSynapse => {
            
            if matches!(operation.action.as_ref(), Some(FlowVcsAction::InsertSynapse { .. })) {
                let FlowVcsAction::InsertSynapse { item, .. } = operation.action.take().expect("retained synapse inverse") else { unreachable!() };
                let length = document.host_snapshot().synapses.len();
                flow_vcs_apply(document, FlowMutation::AddSynapse(AddSynapse { index: flow_vcs_wire_index(length)?, synapse: item }))?;
                cursor.current = length;
                operation.action = Some(FlowVcsAction::Checkpoint);
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                flow_vcs_move_adjacent(document, false, cursor.current, cursor.current - 1)?;
                cursor.current -= 1;
                return Ok(false);
            }
        }
        FlowVcsCursorKind::MoveWidget => {
            
            if cursor.current < cursor.origin {
                flow_vcs_move_adjacent(document, true, cursor.current, cursor.current + 1)?;
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                flow_vcs_move_adjacent(document, true, cursor.current, cursor.current - 1)?;
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
            
            if cursor.current < cursor.origin {
                flow_vcs_move_adjacent(document, false, cursor.current, cursor.current + 1)?;
                cursor.current += 1;
                return Ok(false);
            }
            if cursor.current > cursor.origin {
                flow_vcs_move_adjacent(document, false, cursor.current, cursor.current - 1)?;
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
            if let FlowVcsAction::PatchWidget { id, item } = action {
                let (id, item) = flow_vcs_inverse_changed_widget(flow_vcs_apply_inverted(document, FlowMutation::ChangeWidget(ChangeWidget { id, widget: item }))?)?;
                operation.action = Some(FlowVcsAction::PatchWidget { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::PatchSynapse => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            if let FlowVcsAction::PatchSynapse { id, item } = action {
                let (id, item) = flow_vcs_inverse_changed_synapse(flow_vcs_apply_inverted(document, FlowMutation::ChangeSynapse(ChangeSynapse { id, synapse: item }))?)?;
                operation.action = Some(FlowVcsAction::PatchSynapse { id, item });
            } else {
                return Err(FlowVcsFault::InvalidMutation);
            }
        }
        FlowVcsCursorKind::Layout => {
            let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
            match action {
                FlowVcsAction::LayoutEntry(entry) => {
                    let previous = flow_vcs_inverse_layout_entry(flow_vcs_apply_inverted(document, FlowMutation::ChangeLayout(ChangeLayout { entries: vec![entry] }))?)?;
                    operation.action = Some(FlowVcsAction::LayoutEntry(previous));
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

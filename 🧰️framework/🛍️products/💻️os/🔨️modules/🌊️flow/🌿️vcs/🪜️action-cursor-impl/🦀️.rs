//! 🪜️ Action cursor impl of the Flow VCS: stepping the action cursor through the history load, transfer, visibility and page publication phases and the cursor helper fns. See [`super`].

use super::*;

//#region 🌊️RetainedVcs
//#region 🌊️RetainedActionCursor
impl FlowRetainedVcs {
    pub(super) fn step_action_cursor(&mut self, slot: usize, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
        let document = self.document.as_ref().ok_or(FlowVcsFault::Closed)?;
        if document.edit_owner.is_some_and(|owner| owner != operation.handle.operation) {
            return Err(FlowVcsFault::Full);
        }
        let authority_matches = if operation.cursor.visibility_published {
            operation.authority.base_revision.checked_add(1) == Some(document.revision) && document.parent_revision == operation.authority.base_revision
        } else {
            operation.authority.base_revision == document.revision && operation.authority.parent_revision == document.parent_revision
        };
        if operation.authority.session_generation != self.session_generation || !authority_matches {
            return Err(FlowVcsFault::StaleAuthority);
        }
        if !operation.cursor.surface_transferred && document.surface.is_some() && self.retired_surfaces.is_full() {
            return Err(FlowVcsFault::Full);
        }
        let history_mode = operation.cursor.history_mode;
        let history_capacity = match history_mode {
            0 => self.undo.remaining() >= 1 && self.retired_actions.remaining() >= self.redo.len(),
            1 => self.redo.remaining() >= 1,
            2 => self.undo.remaining() >= 1,
            _ => true,
        };
        if !operation.cursor.history_transferred && !history_capacity {
            return Err(FlowVcsFault::Full);
        }
        let phase = operation.cursor.phase;
        if flow_vcs_cursor_requires_edit(phase) && !operation.cursor.owns_edit {
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if document.edit_owner.is_some() {
                return Err(FlowVcsFault::Full);
            }
            document.edit_owner = Some(operation.handle.operation);
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.owns_edit = true;
        }
        if phase == FlowVcsCursorPhase::ReserveReplacement && (!self.undo.is_empty() || !self.redo.is_empty()) {
            return Err(FlowVcsFault::InvalidMutation);
        }
        if phase == FlowVcsCursorPhase::LoadHistory {
            self.load_history_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::RetireRedo {
            if let Some(action) = self.redo.pop() {
                self.retired_actions.push(action).map_err(|_| FlowVcsFault::Full)?;
                self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.redo_retired += 1;
                return Ok(self.cursor_progress(slot));
            }
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.phase = FlowVcsCursorPhase::TransferHistory;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::TransferHistory {
            self.transfer_history_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::TransferSurface {
            self.transfer_surface_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::PublishVisibility {
            self.publish_visibility_cursor(slot)?;
            return Ok(self.cursor_progress(slot));
        }
        if phase == FlowVcsCursorPhase::PublishPage {
            return self.publish_page_cursor(slot, grant);
        }
        let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        flow_vcs_step_cursor(document, operation, grant)?;
        if operation.cursor.phase == FlowVcsCursorPhase::TransferHistory && operation.cursor.history_mode == 0 && !self.redo.is_empty() {
            operation.cursor.phase = FlowVcsCursorPhase::RetireRedo;
        }
        Ok(self.cursor_progress(slot))
    }

    fn load_history_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        let command = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
        let (action, history_mode) = match command {
            FlowVcsAction::Undo => (self.undo.pop().ok_or(FlowVcsFault::InvalidMutation)?, 1),
            FlowVcsAction::Redo => (self.redo.pop().ok_or(FlowVcsFault::InvalidMutation)?, 2),
            _ => return Err(FlowVcsFault::InvalidMutation),
        };
        let mut cursor = FlowVcsCursor::new(&action);
        cursor.history_mode = history_mode;
        cursor.history_loaded = true;
        cursor.owns_edit = true;
        operation.action = Some(action);
        operation.cursor = cursor;
        Ok(())
    }

    fn cursor_progress(&self, slot: usize) -> FlowVcsPoll {
        let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
        FlowVcsPoll::Progress { completed: u32::try_from(operation.cursor.scan.saturating_add(2)).unwrap_or(u32::MAX), total: u32::try_from(operation.source.items.saturating_add(6)).unwrap_or(u32::MAX) }
    }

    fn transfer_history_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let history_mode = self.operations[slot].as_ref().expect("validated Flow VCS operation").cursor.history_mode;
        if history_mode == 0 && self.undo.is_full() || history_mode == 1 && self.redo.is_full() || history_mode == 2 && self.undo.is_full() {
            return Err(FlowVcsFault::Full);
        }
        let inverse = self.operations[slot].as_mut().expect("validated Flow VCS operation").action.take().ok_or(FlowVcsFault::InvalidMutation)?;
        match history_mode {
            0 => self.undo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            1 => self.redo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            2 => self.undo.push(inverse).map_err(|_| FlowVcsFault::Full)?,
            _ => drop(inverse),
        }
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.history_transferred = history_mode != 3;
        operation.cursor.phase = FlowVcsCursorPhase::TransferSurface;
        Ok(())
    }

    fn transfer_surface_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let document = self.document.as_mut().expect("open Flow VCS document");
        if let Some(surface) = document.surface.take() {
            self.retired_surfaces.push(surface).map_err(|surface| {
                document.surface = Some(surface);
                FlowVcsFault::Full
            })?;
            self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.surface_transferred = true;
        }
        self.operations[slot].as_mut().expect("validated Flow VCS operation").cursor.phase = FlowVcsCursorPhase::PublishVisibility;
        Ok(())
    }

    fn publish_visibility_cursor(&mut self, slot: usize) -> Result<(), FlowVcsFault> {
        let document = self.document.as_mut().expect("open Flow VCS document");
        let revision = document.revision.checked_add(1).ok_or(FlowVcsFault::Limit)?;
        let generation = document.generation.checked_add(1).ok_or(FlowVcsFault::Limit)?;
        let widget_count = u32::try_from(document.host_snapshot().widgets.len()).unwrap_or(u32::MAX);
        let synapse_count = u32::try_from(document.host_snapshot().synapses.len()).unwrap_or(u32::MAX);
        let layout_count = u32::try_from(document.host_snapshot().layout.len()).unwrap_or(u32::MAX);
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.prior_generation = document.generation;
        operation.cursor.prior_digest = document.committed_digest;
        operation.cursor.visibility_published = true;
        document.parent_revision = document.revision;
        document.revision = revision;
        document.generation = generation;
        document.committed_digest =
            document.committed_digest.rotate_left(13) ^ revision ^ u64::from(widget_count).rotate_left(7) ^ u64::from(synapse_count).rotate_left(17) ^ u64::from(layout_count).rotate_left(29) ^ u64::try_from(document.active).unwrap_or(u64::MAX);
        operation.cursor.phase = FlowVcsCursorPhase::PublishPage;
        Ok(())
    }

    fn publish_page_cursor(&mut self, slot: usize, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        if grant.outputs == 0 || grant.events == 0 || grant.bytes < size_of::<FlowVcsPage>() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let document = self.document.as_ref().ok_or(FlowVcsFault::Closed)?;
        let widget_count = u32::try_from(document.host_snapshot().widgets.len()).unwrap_or(u32::MAX);
        let synapse_count = u32::try_from(document.host_snapshot().synapses.len()).unwrap_or(u32::MAX);
        let layout_count = u32::try_from(document.host_snapshot().layout.len()).unwrap_or(u32::MAX);
        let page = FlowVcsPage {
            sequence: self.next_page,
            operation: self.operations[slot].as_ref().expect("validated Flow VCS operation").handle.operation,
            session_generation: self.session_generation,
            revision: document.revision,
            parent_revision: document.parent_revision,
            document_generation: document.generation,
            widget_count,
            synapse_count,
            layout_count,
            semantic_digest: document.committed_digest,
        };
        let operation = self.operations[slot].as_mut().expect("validated Flow VCS operation");
        operation.cursor.owns_edit = false;
        operation.page = Some(page);
        operation.stage = FlowVcsStage::PageReady;
        self.document.as_mut().expect("open Flow VCS document").edit_owner = None;
        self.next_page += 1;
        Ok(FlowVcsPoll::Preview { widgets: page.widget_count, synapses: page.synapse_count, layout: page.layout_count })
    }
}

pub(super) fn flow_vcs_rediscovered_handle(operation: Option<&FlowVcsOperation>, expected_operation: u64, generation: u32) -> Option<FlowVcsHandle> {
    let handle = operation?.handle;
    (handle.operation == expected_operation && handle.generation == generation).then_some(handle)
}

fn flow_vcs_cursor_requires_edit(phase: FlowVcsCursorPhase) -> bool {
    matches!(
        phase,
        FlowVcsCursorPhase::LoadHistory
            | FlowVcsCursorPhase::Mutate
            | FlowVcsCursorPhase::Shift
            | FlowVcsCursorPhase::ReserveReplacement
            | FlowVcsCursorPhase::ReplaceSchema
            | FlowVcsCursorPhase::ReplaceCameraX
            | FlowVcsCursorPhase::ReplaceCameraY
            | FlowVcsCursorPhase::ReplaceCameraZoom
            | FlowVcsCursorPhase::ReplaceWidgets
            | FlowVcsCursorPhase::ReverseWidgets
            | FlowVcsCursorPhase::ReplaceSynapses
            | FlowVcsCursorPhase::ReverseSynapses
            | FlowVcsCursorPhase::ReplaceLayout
            | FlowVcsCursorPhase::RetireRedo
            | FlowVcsCursorPhase::TransferHistory
            | FlowVcsCursorPhase::TransferSurface
            | FlowVcsCursorPhase::PublishVisibility
            | FlowVcsCursorPhase::PublishPage
    )
}

fn flow_vcs_step_cursor(document: &mut FlowVcsDocument, operation: &mut FlowVcsOperation, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
    match operation.cursor.phase {
        FlowVcsCursorPhase::Scan => flow_vcs_step_scan(document.host_snapshot(), operation),
        FlowVcsCursorPhase::Mutate => flow_vcs_step_mutation(document, operation, grant),
        FlowVcsCursorPhase::Shift => flow_vcs_step_shift(document.host_snapshot_mut(), operation),
        FlowVcsCursorPhase::ReserveReplacement
        | FlowVcsCursorPhase::ReplaceSchema
        | FlowVcsCursorPhase::ReplaceCameraX
        | FlowVcsCursorPhase::ReplaceCameraY
        | FlowVcsCursorPhase::ReplaceCameraZoom
        | FlowVcsCursorPhase::ReplaceWidgets
        | FlowVcsCursorPhase::ReverseWidgets
        | FlowVcsCursorPhase::ReplaceSynapses
        | FlowVcsCursorPhase::ReverseSynapses
        | FlowVcsCursorPhase::ReplaceLayout => flow_vcs_step_document_replacement(document, operation),
        _ => Err(FlowVcsFault::InvalidMutation),
    }
}
//#endregion 🌊️RetainedActionCursor
//#endregion 🌊️RetainedVcs

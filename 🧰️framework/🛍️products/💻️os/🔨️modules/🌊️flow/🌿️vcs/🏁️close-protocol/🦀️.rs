//! 🏁️ Close protocol of the Flow VCS: the operation and retired owner close demands and steps, the close phase and the terminal state. See [`super`].

use super::*;

//#region 🌊️RetainedVcs
impl FlowRetainedVcs {
    /// 📏️ Quotes the exact original operation owner without moving or allocating source data.
    pub fn next_close_operation_demands(&self,handle:FlowVcsHandle,copy:usize)->Result<FlowVcsCloseDemands,FlowVcsCloseFailure>{
        let slot=self.slot(handle)?;let operation=self.operations[slot].as_ref().unwrap();
        if operation.retirement.is_some(){return owner_demands(&operation.retirement,copy)}
        let mut demands=FlowVcsCloseDemands{depth:1,..Default::default()};
        if operation.layout_update.is_none()&&operation.cursor.phase==FlowVcsCursorPhase::Rollback&&operation.cursor.mutated&&!operation.cursor.visibility_published&&!operation.cursor.surface_transferred&&!operation.cursor.history_transferred&&operation.cursor.redo_retired==0{
            demands.copy_bytes=match operation.cursor.kind{
                FlowVcsCursorKind::MoveWidget|FlowVcsCursorKind::RemoveWidget if operation.cursor.current!=operation.cursor.origin=>2*size_of::<Widget>(),
                FlowVcsCursorKind::MoveSynapse|FlowVcsCursorKind::RemoveSynapse if operation.cursor.current!=operation.cursor.origin=>2*size_of::<SynapseSpec>(),
                FlowVcsCursorKind::PatchWidget=>2*size_of::<Widget>(),
                FlowVcsCursorKind::PatchSynapse=>2*size_of::<SynapseSpec>(),
                _=>0,
            };
        }
        Ok(demands)
    }

    /// 🧹️ Advances one original operation close under its exact caller wallet.
    pub fn close_operation_step(&mut self,handle:FlowVcsHandle,grant:FlowVcsGrant)->Result<RetainedCloneStep,FlowVcsCloseFailure>{
        if grant.controls==0||!grant.permits_work(){return Err(FlowVcsFault::InsufficientGrant.into())}
        let slot=self.slot(handle)?;let stage=self.operations[slot].as_ref().unwrap().stage;
        if !matches!(stage,FlowVcsStage::Complete|FlowVcsStage::Cancelled|FlowVcsStage::Faulted|FlowVcsStage::Closing){return Err(FlowVcsFault::ClosePending.into())}
        let demands=self.next_close_operation_demands(handle,grant.retained.maximum_copy_bytes)?;
        if self.operations[slot].as_ref().unwrap().retirement.is_some(){return owner_step(&mut self.operations[slot].as_mut().unwrap().retirement,grant.retained)}
        if !admit_turn(grant.retained,demands)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demands.copy_bytes,..Default::default()};
        let operation=self.operations[slot].as_mut().unwrap();
        if let Some(update)=operation.layout_update.take(){retain_owner(&mut operation.retirement,FlowVcsClosingOwner::LayoutUpdate(update));return Ok(RetainedCloneStep::Progress(progress))}
        if operation.cursor.phase == FlowVcsCursorPhase::Rollback {
            if operation.cursor.visibility_published {
                let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
                document.revision = operation.authority.base_revision;
                document.parent_revision = operation.authority.parent_revision;
                document.generation = operation.cursor.prior_generation;
                document.committed_digest = operation.cursor.prior_digest;
                operation.cursor.visibility_published = false;
                return Ok(RetainedCloneStep::Progress(progress));
            }
            if operation.cursor.surface_transferred {
                let surface = self.retired_surfaces.pop().ok_or(FlowVcsFault::InvalidMutation)?;
                self.document.as_mut().ok_or(FlowVcsFault::Closed)?.surface = Some(surface);
                operation.cursor.surface_transferred = false;
                return Ok(RetainedCloneStep::Progress(progress));
            }
            if operation.cursor.history_transferred {
                let action = match operation.cursor.history_mode {
                    0 | 2 => self.undo.pop(),
                    1 => self.redo.pop(),
                    _ => None,
                }
                .ok_or(FlowVcsFault::InvalidMutation)?;
                operation.action = Some(action);
                operation.cursor.history_transferred = false;
                return Ok(RetainedCloneStep::Progress(progress));
            }
            if operation.cursor.redo_retired > 0 {
                let action = self.retired_actions.pop().ok_or(FlowVcsFault::InvalidMutation)?;
                self.redo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.redo_retired -= 1;
                return Ok(RetainedCloneStep::Progress(progress));
            }
            let document = self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if !flow_vcs_step_rollback(document, operation)? {
                return Ok(RetainedCloneStep::Progress(progress));
            }
            if operation.cursor.owns_edit {
                document.edit_owner = None;
                operation.cursor.owns_edit = false;
            }
            if operation.cursor.history_loaded && operation.cursor.history_mode == 1 {
                let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
                self.undo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.history_mode = 0;
                operation.cursor.history_loaded = false;
            } else if operation.cursor.history_loaded && operation.cursor.history_mode == 2 {
                let action = operation.action.take().ok_or(FlowVcsFault::InvalidMutation)?;
                self.redo.push(action).map_err(|_| FlowVcsFault::Full)?;
                operation.cursor.history_mode = 0;
                operation.cursor.history_loaded = false;
            }
            operation.cursor.phase = FlowVcsCursorPhase::Scan;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        match operation.close_phase{
            0=>{operation.page=None;operation.page_leased=false;operation.close_phase=1;Ok(RetainedCloneStep::Progress(progress))}
            1=>{
                if operation.action.is_some()&&self.retired_actions.is_full(){return Err(FlowVcsFault::Full.into())}
                if let Some(action)=operation.action.take(){self.retired_actions.push(action).map_err(|action|{operation.action=Some(action);FlowVcsFault::Full})?}
                operation.close_phase=2;Ok(RetainedCloneStep::Progress(progress))
            }
            _=>{
                if let Some(action)=operation.rollback_owner.take(){retain_owner(&mut operation.retirement,FlowVcsClosingOwner::Action(action));return Ok(RetainedCloneStep::Progress(progress))}
                let generation=self.slot_generations[slot].checked_add(1).ok_or(FlowVcsFault::Limit)?;
                let operation=self.operations[slot].take().unwrap();self.hand_back(operation.source,operation.delivery_held);self.slot_generations[slot]=generation;
                Ok(RetainedCloneStep::Complete(progress))
            }
        }
    }

    /// 📏️ Quotes the original retained action, layout or document frontier without effects.
    pub fn next_close_retired_demands(&self,copy:usize)->Result<FlowVcsCloseDemands,FlowVcsCloseFailure>{
        if self.terminal_is_empty(){return Ok(Default::default())}
        if self.retirement.is_some(){return owner_demands(&self.retirement,copy)}
        Ok(FlowVcsCloseDemands{depth:1,..Default::default()})
    }

    /// 🌿️ Physically closes one original retained frontier and returns every actual effect.
    pub fn close_retired_step(&mut self,grant:FlowVcsGrant)->Result<RetainedCloneStep,FlowVcsCloseFailure>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if grant.controls==0||!grant.permits_work(){return Err(FlowVcsFault::InsufficientGrant.into())}
        if self.credits.operations>0{return Err(FlowVcsFault::ClosePending.into())}
        if self.retirement.is_some(){return owner_step(&mut self.retirement,grant.retained)}
        let demands=self.next_close_retired_demands(grant.retained.maximum_copy_bytes)?;
        if !admit_turn(grant.retained,demands)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        if let Some(surface)=self.retired_surfaces.last_mut(){if surface.close_one(){self.retired_surfaces.pop();}return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(action)=self.retired_actions.pop(){retain_owner(&mut self.retirement,FlowVcsClosingOwner::Action(action));return Ok(RetainedCloneStep::Progress(progress))}
        if self.closing{if let Some(action)=self.undo.pop().or_else(||self.redo.pop()){retain_owner(&mut self.retirement,FlowVcsClosingOwner::Action(action));return Ok(RetainedCloneStep::Progress(progress))}}
        if self.closing{
            let document=self.document.as_mut().ok_or(FlowVcsFault::Closed)?;
            if document.surface.is_some()&&self.retired_surfaces.is_full(){return Err(FlowVcsFault::Full.into())}
            if let Some(surface)=document.surface.take(){self.retired_surfaces.push(surface).map_err(|surface|{document.surface=Some(surface);FlowVcsFault::Full})?;return Ok(RetainedCloneStep::Progress(progress))}
            if let Some(snapshot)=document.versions.pop(){retain_owner(&mut self.retirement,FlowVcsClosingOwner::Snapshot(snapshot));return Ok(RetainedCloneStep::Progress(progress))}
            self.document=None;
        }
        Ok(RetainedCloneStep::Complete(progress))
    }

    /// 🪜️ Names the original custody frontier that the next full-wallet turn visits.
    pub fn close_phase(&self) -> FlowVcsClosePhase {
        if self.terminal_is_empty() {
            return FlowVcsClosePhase::Complete;
        }
        if self.credits.operations > 0 {
            return FlowVcsClosePhase::Operations;
        }
        if self.retirement.is_some() {
            return FlowVcsClosePhase::Backing;
        }
        if !self.retired_surfaces.is_empty() {
            return FlowVcsClosePhase::RetiredSurface;
        }
        if !self.retired_actions.is_empty() {
            return FlowVcsClosePhase::RetiredAction;
        }
        if self.closing && !(self.undo.is_empty() && self.redo.is_empty()) {
            return FlowVcsClosePhase::History;
        }
        if !self.closing {
            return FlowVcsClosePhase::Complete;
        }
        match self.document.as_ref() {
            Some(document) if document.surface.is_some() => FlowVcsClosePhase::DocumentSurface,
            Some(document) if !document.versions.is_empty() => FlowVcsClosePhase::DocumentVersion,
            Some(_) => FlowVcsClosePhase::Document,
            None => FlowVcsClosePhase::Complete,
        }
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.document.is_none() && self.credits.operations == 0 && self.credits == FlowVcsCredits::default() && self.undo.is_empty() && self.redo.is_empty() && self.retired_actions.is_empty() && self.retired_surfaces.is_empty() && self.retirement.is_none() && self.operations.iter().all(Option::is_none)
    }
}
//#endregion 🌊️RetainedVcs

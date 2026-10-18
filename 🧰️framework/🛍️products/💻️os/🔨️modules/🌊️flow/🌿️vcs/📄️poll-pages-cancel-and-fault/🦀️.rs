//! 📄️ Poll, pages, cancel and fault of the Flow VCS: polling, page take, resume, retry and acknowledge, cancellation, fault injection and handle rediscovery. See [`super`].

use super::*;

//#region 🌊️RetainedVcs
impl FlowRetainedVcs {
    pub fn poll(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<FlowVcsPoll, FlowVcsFault> {
        if !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let slot = self.slot(handle)?;
        match self.operations[slot].as_ref().expect("validated Flow VCS operation").stage {
            FlowVcsStage::Admitted => {
                self.operations[slot].as_mut().expect("validated Flow VCS operation").stage = FlowVcsStage::Ready;
                Ok(FlowVcsPoll::Progress { completed: 1, total: 3 })
            }
            FlowVcsStage::Ready => {
                self.operations[slot].as_mut().expect("validated Flow VCS operation").stage = FlowVcsStage::PublishReady;
                let operation = self.operations[slot].as_ref().expect("validated Flow VCS operation");
                Ok(FlowVcsPoll::Checkpoint { operation: operation.handle.operation, revision: operation.authority.base_revision })
            }
            FlowVcsStage::PublishReady => self.step_action_cursor(slot, grant),
            FlowVcsStage::PageReady => Ok(FlowVcsPoll::PageReady { sequence: self.operations[slot].as_ref().and_then(|operation| operation.page.as_ref()).ok_or(FlowVcsFault::OutputNotReady)?.sequence }),
            FlowVcsStage::Complete => Ok(FlowVcsPoll::Terminal),
            _ => Err(FlowVcsFault::ClosePending),
        }
    }

    pub fn take_page(&mut self, handle: FlowVcsHandle) -> Result<FlowVcsPage, FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.stage != FlowVcsStage::PageReady {
            return Err(FlowVcsFault::OutputNotReady);
        }
        if operation.page_leased {
            return Err(FlowVcsFault::OutputAlreadyLeased);
        }
        let page = *operation.page.as_ref().ok_or(FlowVcsFault::OutputNotReady)?;
        operation.page_leased = true;
        Ok(page)
    }

    pub fn resume_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<(), FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
            return Err(FlowVcsFault::WrongPage);
        }
        if !operation.page_leased {
            return Err(FlowVcsFault::OutputNotLeased);
        }
        operation.page_leased = false;
        Ok(())
    }

    pub fn retry_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<FlowVcsPage, FlowVcsFault> {
        let operation = self.operation_mut(handle)?;
        if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
            return Err(FlowVcsFault::WrongPage);
        }
        if operation.page_leased {
            return Err(FlowVcsFault::OutputAlreadyLeased);
        }
        operation.page_leased = true;
        Ok(*operation.page.as_ref().expect("validated Flow VCS page"))
    }

    pub fn acknowledge_page(&mut self, handle: FlowVcsHandle, sequence: u64) -> Result<(), FlowVcsFault> {
        {
            let operation = self.operation_mut(handle)?;
            if operation.page.as_ref().map(|page| page.sequence) != Some(sequence) {
                return Err(FlowVcsFault::WrongPage);
            }
            if !operation.page_leased {
                return Err(FlowVcsFault::OutputNotLeased);
            }
            operation.page = None;
            operation.page_leased = false;
            operation.stage = FlowVcsStage::Complete;
            operation.delivery_held = false;
        }
        self.credits.pages -= 1;
        self.credits.outputs -= 1;
        self.credits.events -= 3;
        Ok(())
    }

    pub fn cancel(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let operation = self.operation_mut(handle)?;
        if !matches!(operation.stage, FlowVcsStage::Admitted | FlowVcsStage::Ready | FlowVcsStage::PublishReady) {
            return Err(if matches!(operation.stage, FlowVcsStage::PageReady | FlowVcsStage::Complete) { FlowVcsFault::Published } else { FlowVcsFault::DuplicateControl });
        }
        operation.stage = FlowVcsStage::Cancelled;
        if operation.cursor.mutated
            || operation.cursor.owns_edit
            || operation.cursor.redo_retired > 0
            || operation.cursor.history_transferred
            || operation.cursor.surface_transferred
            || operation.cursor.visibility_published
            || operation.cursor.history_loaded
        {
            operation.cursor.phase = FlowVcsCursorPhase::Rollback;
        }
        Ok(())
    }

    pub fn fault(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        if grant.controls == 0 || !grant.permits_work() {
            return Err(FlowVcsFault::InsufficientGrant);
        }
        let operation = self.operation_mut(handle)?;
        if !matches!(operation.stage, FlowVcsStage::Admitted | FlowVcsStage::Ready | FlowVcsStage::PublishReady) {
            return Err(FlowVcsFault::DuplicateControl);
        }
        operation.stage = FlowVcsStage::Faulted;
        if operation.cursor.mutated
            || operation.cursor.owns_edit
            || operation.cursor.redo_retired > 0
            || operation.cursor.history_transferred
            || operation.cursor.surface_transferred
            || operation.cursor.visibility_published
            || operation.cursor.history_loaded
        {
            operation.cursor.phase = FlowVcsCursorPhase::Rollback;
        }
        Ok(())
    }

    pub fn panic_fault(&mut self, handle: FlowVcsHandle, grant: FlowVcsGrant) -> Result<(), FlowVcsFault> {
        self.fault(handle, grant)
    }

    pub fn rediscover(&self, operation: u64, generation: u32) -> Result<FlowVcsHandle, FlowVcsFault> {
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[0].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[1].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[2].as_ref(), operation, generation) {
            return Ok(handle);
        }
        if let Some(handle) = flow_vcs_rediscovered_handle(self.operations[3].as_ref(), operation, generation) {
            return Ok(handle);
        }
        Err(FlowVcsFault::StaleHandle)
    }
}
//#endregion 🌊️RetainedVcs

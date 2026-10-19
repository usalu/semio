//! 🎟️ Admission and slots of the Flow VCS: preflight, admission, credit charge and hand back and the operation slot lookup. See [`super`].

use super::*;

//#region 🌊️RetainedVcs
impl FlowRetainedVcs {
    pub fn preflight(&self, census: FlowVcsCensus) -> Result<(), FlowVcsFault> {
        if self.closing || self.document.is_none() {
            return Err(FlowVcsFault::Closed);
        }
        if census.depth > FLOW_VCS_MAX_DEPTH {
            return Err(FlowVcsFault::Depth);
        }
        if census.items > FLOW_VCS_MAX_ITEMS || census.bytes > FLOW_VCS_MAX_BYTES {
            return Err(FlowVcsFault::Limit);
        }
        if self.credits.operations == FLOW_VCS_MAX_OPERATIONS || !self.can_charge(census) {
            return Err(FlowVcsFault::Full);
        }
        if self.next_operation == u64::MAX || self.next_page == u64::MAX {
            return Err(FlowVcsFault::Limit);
        }
        Ok(())
    }

    pub fn admit(&mut self, authority: FlowVcsAuthority, source: FlowVcsCensus, action: FlowVcsAction) -> Result<FlowVcsHandle, FlowVcsFault> {
        let slot = if self.operations[0].is_none() {
            0
        } else if self.operations[1].is_none() {
            1
        } else if self.operations[2].is_none() {
            2
        } else if self.operations[3].is_none() {
            3
        } else {
            return Err(FlowVcsFault::Full);
        };
        let handle = FlowVcsHandle { operation: self.next_operation, slot: slot as u8, generation: self.slot_generations[slot] };
        self.charge(source);
        let cursor = FlowVcsCursor::new(&action);
        self.operations[slot] = Some(FlowVcsOperation { handle, authority, source, action: Some(action), rollback_owner: None, retirement: None, cursor, page: None, page_leased: false, delivery_held: true, stage: FlowVcsStage::Admitted, close_phase: 0 });
        self.next_operation += 1;
        Ok(handle)
    }

    fn can_charge(&self, source: FlowVcsCensus) -> bool {
        self.credits.operations < FLOW_VCS_MAX_OPERATIONS
            && self.credits.pages < FLOW_VCS_MAX_PAGES
            && self.credits.items.checked_add(source.items).is_some_and(|value| value <= FLOW_VCS_MAX_ITEMS)
            && self.credits.bytes.checked_add(source.bytes).is_some_and(|value| value <= FLOW_VCS_MAX_BYTES)
            && self.credits.outputs < FLOW_VCS_MAX_OUTPUTS
            && self.credits.events.checked_add(3).is_some_and(|value| value <= FLOW_VCS_MAX_EVENTS)
            && self.credits.controls < FLOW_VCS_MAX_CONTROLS
    }

    fn charge(&mut self, source: FlowVcsCensus) {
        self.credits.operations += 1;
        self.credits.pages += 1;
        self.credits.items += source.items;
        self.credits.bytes += source.bytes;
        self.credits.outputs += 1;
        self.credits.events += 3;
        self.credits.controls += 1;
    }

    pub(super) fn hand_back(&mut self, source: FlowVcsCensus, delivery: bool) {
        self.credits.operations -= 1;
        self.credits.items -= source.items;
        self.credits.bytes -= source.bytes;
        self.credits.controls -= 1;
        if delivery {
            self.credits.pages -= 1;
            self.credits.outputs -= 1;
            self.credits.events -= 3;
        }
    }

    pub fn slot(&self, handle: FlowVcsHandle) -> Result<usize, FlowVcsFault> {
        let slot = usize::from(handle.slot);
        let operation = self.operations.get(slot).and_then(Option::as_ref).ok_or(FlowVcsFault::StaleHandle)?;
        if operation.handle.operation != handle.operation {
            return Err(FlowVcsFault::WrongHandle);
        }
        if operation.handle.generation != handle.generation {
            return Err(FlowVcsFault::StaleHandle);
        }
        Ok(slot)
    }

    pub(super) fn operation_mut(&mut self, handle: FlowVcsHandle) -> Result<&mut FlowVcsOperation, FlowVcsFault> {
        let slot = self.slot(handle)?;
        Ok(self.operations[slot].as_mut().expect("validated Flow VCS operation"))
    }
}
//#endregion 🌊️RetainedVcs

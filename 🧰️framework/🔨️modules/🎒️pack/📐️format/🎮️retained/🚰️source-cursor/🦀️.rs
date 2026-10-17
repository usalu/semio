//! 🚰️ Source cursor of the retained Pack source: the implementation that admits canonical bytes page by page and its drop guard. See [`super`].

use super::*;

//#region 🔖️RetainedCanonicalSource
impl RetainedPackSourceCursor {
    pub fn try_new(maximum_pages: usize, maximum_payload_bytes: usize, maximum_allocation_bytes: usize) -> Result<Self, RetainedPackSourceFault> {
        if maximum_pages==0||maximum_payload_bytes==0{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.zero-credits"));}
        if maximum_allocation_bytes==0{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.zero-credits"));}
        if maximum_pages > RETAINED_PACK_MAXIMUM_PAGES {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.page-credits"));
        }
        if maximum_payload_bytes > maximum_pages.checked_mul(RETAINED_PACK_PAGE_BYTES).ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.credit-overflow"))? {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.byte-credits"));
        }
        if maximum_allocation_bytes > isize::MAX as usize {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.allocation-credits"));
        }
        Ok(Self {
            pages: RetainedPackPages::default(),
            maximum_pages,
            maximum_payload_bytes,
            maximum_allocation_bytes,
            admitted_bytes: 0,
            page: 0,
            byte: 0,
            consumed: 0,
            sealed: false,
            cancelled: false,
            completed: false,
            allocation_fault: None,
            closed: false,
        })
    }

    pub fn next_allocation_bytes(&self) -> Result<usize, RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed||self.sealed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.source-closed"));}
        if self.pages.len() == self.maximum_pages {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.page-credits"));
        }
        let requested = self.pages.next_allocation_bytes().map_err(RetainedPackSourceFault::from_paged_refusal)?;
        self.pages
            .allocated_bytes()
            .checked_add(requested)
            .filter(|total| *total <= self.maximum_allocation_bytes)
            .ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.allocation-credits"))?;
        Ok(requested)
    }

    pub fn reserve_page(&mut self, maximum_bytes: usize) -> Result<RetainedPackSourceAllocationStep, RetainedPackSourceAllocationError> {
        let rejected=|fault|RetainedPackSourceAllocationError{allocated_bytes:0,fault};
        let requested = self.next_allocation_bytes().map_err(rejected)?;
        let remaining = self.maximum_allocation_bytes - self.pages.allocated_bytes();
        let step = match self.pages.reserve_one(maximum_bytes.min(remaining)) {
            Ok(step) => step,
            Err(error) => {
                if error.allocated_bytes != 0 {
                    self.allocation_fault=Some(RetainedPackSourceFault::from_paged_allocation(error));
                }
                return Err(RetainedPackSourceAllocationError{allocated_bytes:error.allocated_bytes,fault:RetainedPackSourceFault::from_paged_allocation(error)});
            }
        };
        if self.pages.allocated_bytes() > self.maximum_allocation_bytes {
            let fault=RetainedPackSourceFault::allocation(ValueRefusalKind::OwnershipLimit,"retained-pack actual allocation exceeds physical credits; owner retained",step.allocated_bytes);
            self.allocation_fault=Some(fault);
            return Err(RetainedPackSourceAllocationError {
                allocated_bytes: step.allocated_bytes,
                fault,
            });
        }
        debug_assert!(step.allocated_bytes == 0 || step.allocated_bytes >= requested);
        Ok(RetainedPackSourceAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
    }

    pub fn has_reserved_page(&self) -> bool {
        self.allocation_fault.is_none() && self.pages.has_reserved_slot() && self.pages.len() < self.maximum_pages
    }

    pub fn allocated_bytes(&self) -> usize {
        self.pages.allocated_bytes()
    }

    /// 📏️ Observes the original page movement or whole backing release before close.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        retained_page_close_demands(&self.pages)
    }

    pub fn next_release_allocation_bytes(&self) -> Result<usize, PagedListError> {
        self.pages.next_release_allocation_bytes()
    }

    pub fn preflight_page(&self, len: usize) -> Result<(), RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed||self.sealed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.source-closed"));}
        let pages = self.pages.len().checked_add(1).ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.page-overflow"))?;
        let bytes = self.admitted_bytes.checked_add(len).ok_or(RetainedPackSourceFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.byte-overflow"))?;
        if len==0||len>RETAINED_PACK_PAGE_BYTES{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.producer-handback"));}
        if pages>self.maximum_pages||bytes>self.maximum_payload_bytes{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.producer-handback"));}
        if !self.has_reserved_page() {
            return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.page-allocation-required"));
        }
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "A full retained source returns the exact page owner without allocating beyond admitted capacity.")]
    pub fn admit_page(&mut self, page: RetainedPackPage) -> Result<(), RetainedPackPage> {
        if self.preflight_page(page.len()).is_err() {
            return Err(page);
        }
        let len = page.len();
        match self.pages.push_reserved(page) {
            Ok(()) => {
                self.admitted_bytes += len;
                Ok(())
            }
            Err(page) => Err(page),
        }
    }

    pub fn seal(&mut self) -> Result<(), RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.source-closed"));}
        self.sealed = true;
        Ok(())
    }

    pub fn request_cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn progress(&self) -> RetainedPackSourceProgress {
        RetainedPackSourceProgress {
            admitted_pages: self.pages.len(),
            admitted_bytes: self.admitted_bytes,
            reserved_pages: self.pages.capacity(),
            allocated_bytes: self.pages.allocated_bytes(),
            consumed_bytes: self.consumed,
            sealed: self.sealed,
        }
    }

    pub fn grant(&mut self) -> Result<Option<RetainedPackSourceEvent>, RetainedPackSourceFault> {
        if let Some(fault) = self.allocation_fault {
            return Err(fault);
        }
        if self.closed{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-closed"));}
        if self.cancelled{return Err(RetainedPackSourceFault::refusal(ValueRefusalKind::Canceled,"retained-pack.cancelled"));}
        if !self.sealed {
            return Ok(None);
        }
        if self.completed {
            return Ok(Some(RetainedPackSourceEvent::Complete { bytes: self.consumed, pages: self.pages.len() }));
        }
        while self.page < self.pages.len() && self.byte == self.pages.get(self.page).expect("admitted retained Pack page").len {
            self.page += 1;
            self.byte = 0;
        }
        if self.page == self.pages.len() {
            self.completed = true;
            return Ok(Some(RetainedPackSourceEvent::Complete { bytes: self.consumed, pages: self.pages.len() }));
        }
        let value = self.pages.get(self.page).expect("admitted retained Pack page").bytes[self.byte];
        let offset = self.consumed;
        self.byte += 1;
        self.consumed += 1;
        Ok(Some(RetainedPackSourceEvent::Byte { offset, value }))
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedPackCloseStep, PagedListError> {
        if maximum_items == 0 && maximum_bytes == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.cancelled = true;
        if !self.pages.is_empty() && maximum_items == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(page) = self.pages.pop() {
            self.admitted_bytes -= page.len();
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.pages.terminal_is_empty() {
            let step = self.pages.release_empty_page(maximum_bytes)?;
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: step.released_allocation_bytes });
        }
        self.closed = true;
        Ok(RetainedPackCloseStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.admitted_bytes == 0 && self.pages.terminal_is_empty()
    }

    #[cfg(test)]
    pub(super) fn retained_page_ptr(&self, index: usize) -> Option<*const RetainedPackPage> {
        self.pages.backing_ptr(index)
    }

    #[cfg(test)]
    pub(super) fn initialized_pages(&self) -> usize {
        self.pages.initialized_len()
    }
}

impl Drop for RetainedPackSourceCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained canonical pack source reached Drop before terminal-empty close");
    }
}
//#endregion 🔖️RetainedCanonicalSource

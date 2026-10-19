//! 🗃️ The retained symbol table: paged symbol spans and scalars with checked allocation, handback and controlled close.

use semio_framework_value::{ValueRefusalKind, RetirementDemand, ValueError};
use semio_framework_value_persistent::{list::PagedListError, list::PagedListAllocationError, list::PagedListRefusalKind};
use std::mem::size_of;
use semio_framework_pack_format_retained::{RetainedPackCatalogCause, RetainedPackCatalogFault, RetainedPackCloseStep, RetainedPackSegmentHeader, retained_page_close_demands};
use super::{RetainedPackCatalogAllocationError, RetainedPackCatalogAllocationStep, RetainedPackChunkEntry, RetainedPackSymbolSpan, original_symbol_storage};

pub(super) const RETAINED_PACK_MAXIMUM_SYMBOL_SPANS: usize = isize::MAX as usize / size_of::<RetainedPackSymbolSpan>();

pub(super) const RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS: usize = isize::MAX as usize / size_of::<char>();

pub(super) const RETAINED_PACK_MAXIMUM_CHUNK_ENTRIES: usize = isize::MAX as usize / size_of::<RetainedPackChunkEntry>();

pub(super) const RETAINED_PACK_MAXIMUM_OBSERVED_CHUNKS: usize = isize::MAX as usize / size_of::<RetainedPackSegmentHeader>();

pub(super) type RetainedPackSymbolSpans = semio_framework_value_persistent::list::PagedList<RetainedPackSymbolSpan, RETAINED_PACK_MAXIMUM_SYMBOL_SPANS>;

pub(super) type RetainedPackSymbolScalars = semio_framework_value_persistent::list::PagedList<char, RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS>;

pub(super) type RetainedPackChunkEntries = semio_framework_value_persistent::list::PagedList<RetainedPackChunkEntry, RETAINED_PACK_MAXIMUM_CHUNK_ENTRIES>;

pub(super) type RetainedPackObservedChunks = semio_framework_value_persistent::list::PagedList<RetainedPackSegmentHeader, RETAINED_PACK_MAXIMUM_OBSERVED_CHUNKS>;

pub struct RetainedPackSymbolTable {
    pub(super) maximum_symbols: usize,
    pub(super) maximum_utf8_bytes: usize,
    pub(super) maximum_scalars: usize,
    pub(super) maximum_allocation_bytes: usize,
    pub(super) symbols: RetainedPackSymbolSpans,
    pub(super) scalars: RetainedPackSymbolScalars,
    pub(super) published_scalars: usize,
    pub(super) pending_utf8_bytes: usize,
    pub(super) utf8_bytes: usize,
    pub(super) fault: Option<RetainedPackCatalogFault>,
    pub(super) closing: bool,
    pub(super) closed: bool,
    pub(super) storage_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<original_symbol_storage::SymbolStorage>>,
    pub(super) storage_retiring_bytes: usize,
}

impl RetainedPackSymbolTable {
    /// 📏️ Quotes the next original scalar, symbol or semantic page close action.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if !self.scalars.is_empty() { return retained_page_close_demands(&self.scalars); }
        if !self.symbols.is_empty() { return retained_page_close_demands(&self.symbols); }
        if !self.scalars.terminal_is_empty() { return retained_page_close_demands(&self.scalars); }
        if !self.symbols.terminal_is_empty() { return retained_page_close_demands(&self.symbols); }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }
    pub fn try_new(maximum_symbols: usize, maximum_utf8_bytes: usize, maximum_scalars: usize, maximum_allocation_bytes: usize) -> Result<Self, RetainedPackCatalogFault> {
        if maximum_symbols > RETAINED_PACK_MAXIMUM_SYMBOL_SPANS
            || maximum_scalars > RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS
            || maximum_allocation_bytes > isize::MAX as usize
        {
            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-credits",0));
        }
        if maximum_scalars>maximum_utf8_bytes{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.symbol-credits",0));}
        Ok(Self {
            maximum_symbols,
            maximum_utf8_bytes,
            maximum_scalars,
            maximum_allocation_bytes,
            symbols: RetainedPackSymbolSpans::default(),
            scalars: RetainedPackSymbolScalars::default(),
            published_scalars: 0,
            pending_utf8_bytes: 0,
            utf8_bytes: 0,
            fault: None,
            closing: false,
            closed: false,
            storage_retirement: None,
            storage_retiring_bytes: 0,
        })
    }

    pub(super) fn remember<T>(&mut self, result: Result<T, RetainedPackCatalogFault>) -> Result<T, RetainedPackCatalogFault> {
        match result {
            Ok(value) => Ok(value),
            Err(fault) => {
                let first = *self.fault.get_or_insert(fault);
                Err(first)
            }
        }
    }

    pub(super) fn allocated_bytes_checked(&self) -> Option<usize> {
        self.symbols.allocated_bytes().checked_add(self.scalars.allocated_bytes())?.checked_add(self.storage_retiring_bytes)
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated_bytes_checked().unwrap_or(usize::MAX)
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn scalar_len(&self) -> usize {
        self.scalars.len()
    }

    pub fn symbol_capacity(&self) -> usize {
        self.symbols.capacity()
    }

    pub fn scalar_capacity(&self) -> usize {
        self.scalars.capacity()
    }

    pub fn maximum_symbols(&self) -> usize {
        self.maximum_symbols
    }

    pub fn maximum_utf8_bytes(&self) -> usize {
        self.maximum_utf8_bytes
    }

    pub fn maximum_scalars(&self) -> usize {
        self.maximum_scalars
    }

    /// 🪟️ Pure original source authority quotes the next native reserve without installing a refusal.
    pub fn source_reserve_demands(&self,symbol:bool,target:usize,offset:u64)->Result<RetirementDemand,RetainedPackCatalogFault>{
        if let Some(fault)=self.fault{return Err(fault)}
        if self.closed||self.closing{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.source-reserve",offset))}
        if target>if symbol{self.maximum_symbols}else{self.maximum_scalars}{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.source-reserve",offset))}
        let error=|error|RetainedPackCatalogFault::from_paged_refusal(error,"retained-pack.source-reserve",offset);
        let capacity=if symbol{self.symbols.next_capacity_allocation_bytes(target)}else{self.scalars.next_capacity_allocation_bytes(if target>self.scalars.capacity(){self.maximum_scalars}else{target})}.map_err(error)?.unwrap_or(0);
        if self.allocated_bytes_checked().and_then(|owned|owned.checked_add(capacity)).is_none_or(|next|next>self.maximum_allocation_bytes){return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.source-reserve",offset))}
        let depth=if symbol{self.symbols.next_reserve_depth_demand()}else{self.scalars.next_reserve_depth_demand()}.map_err(error)?;
        Ok(RetirementDemand{capacity_bytes:capacity,depth,..Default::default()})
    }
    /// ✍️ Native original char writes consume payload copy while symbol spans move semantic metadata.
    pub fn source_write_demands(&self,symbol:bool)->Result<RetirementDemand,ValueError>{
        let depth=if symbol{self.symbols.next_push_depth_demand()}else{self.scalars.next_push_depth_demand()}.map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))?;
        Ok(RetirementDemand{copy_bytes:if symbol{0}else{size_of::<char>()},depth,..Default::default()})
    }

    pub fn next_symbol_allocation_bytes(&mut self, target: usize, offset: u64) -> Result<Option<usize>, RetainedPackCatalogFault> {
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.closed||self.closing{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-count",offset)));}
        if target>self.maximum_symbols{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.symbol-count",offset)));}
        let requested = match self.symbols.next_capacity_allocation_bytes(target) {
            Ok(Some(requested)) => requested,
            Ok(None) => return Ok(None),
            Err(error) => return self.remember(Err(RetainedPackCatalogFault::from_paged_refusal(error,"retained-pack.symbol-span-allocation",offset))),
        };
        let result = self
            .allocated_bytes_checked()
            .and_then(|allocated| allocated.checked_add(requested))
            .filter(|total| *total <= self.maximum_allocation_bytes)
            .map(|_| Some(requested))
            .ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-allocation-credits",offset));
        self.remember(result)
    }

    pub fn next_scalar_allocation_bytes(&mut self, target: usize, offset: u64) -> Result<Option<usize>, RetainedPackCatalogFault> {
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.closed||self.closing{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-scalars",offset)));}
        if target>self.maximum_scalars{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.symbol-scalars",offset)));}
        let requested = match self.scalars.next_capacity_allocation_bytes(if target>self.scalars.capacity(){self.maximum_scalars}else{target}) {
            Ok(Some(requested)) => requested,
            Ok(None) => return Ok(None),
            Err(error) => return self.remember(Err(RetainedPackCatalogFault::from_paged_refusal(error,"retained-pack.symbol-scalar-allocation",offset))),
        };
        let result = self
            .allocated_bytes_checked()
            .and_then(|allocated| allocated.checked_add(requested))
            .filter(|total| *total <= self.maximum_allocation_bytes)
            .map(|_| Some(requested))
            .ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-allocation-credits",offset));
        self.remember(result)
    }

    pub fn reserve_symbol_capacity(&mut self, target: usize, maximum_bytes: usize, offset: u64) -> Result<RetainedPackCatalogAllocationStep, RetainedPackCatalogAllocationError> {
        let requested = self.next_symbol_allocation_bytes(target, offset).map_err(|fault| RetainedPackCatalogAllocationError { allocated_bytes: 0, fault })?;
        self.reserve(target, maximum_bytes, offset, requested, true)
    }

    pub fn reserve_scalar_capacity(&mut self, target: usize, maximum_bytes: usize, offset: u64) -> Result<RetainedPackCatalogAllocationStep, RetainedPackCatalogAllocationError> {
        let requested = self.next_scalar_allocation_bytes(target, offset).map_err(|fault| RetainedPackCatalogAllocationError { allocated_bytes: 0, fault })?;
        self.reserve(if target>self.scalars.capacity(){self.maximum_scalars}else{target}, maximum_bytes, offset, requested, false)
    }

    fn reserve(
        &mut self,
        target: usize,
        maximum_bytes: usize,
        offset: u64,
        requested: Option<usize>,
        symbol: bool,
    ) -> Result<RetainedPackCatalogAllocationStep, RetainedPackCatalogAllocationError> {
        let Some(requested) = requested else { return Ok(RetainedPackCatalogAllocationStep::default()) };
        if maximum_bytes < requested {
            return Ok(RetainedPackCatalogAllocationStep::default());
        }
        let allocated = self.allocated_bytes_checked().ok_or(RetainedPackCatalogAllocationError {
            allocated_bytes: 0,
            fault: RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-allocation-overflow",offset),
        })?;
        let remaining = self.maximum_allocation_bytes.saturating_sub(allocated);
        let result = if symbol {
            self.symbols.reserve_capacity_one(target, maximum_bytes.min(remaining))
        } else {
            self.scalars.reserve_capacity_one(target, maximum_bytes.min(remaining))
        };
        let step = match result {
            Ok(step) => step,
            Err(error) => {
                let fault = RetainedPackCatalogFault::from_paged_allocation(error,if error.allocated_bytes==0{"retained-pack.symbol-allocation"}else{"retained-pack.symbol-allocation-overgrant"},offset);
                let first = *self.fault.get_or_insert(fault);
                return Err(RetainedPackCatalogAllocationError { allocated_bytes: error.allocated_bytes, fault: first });
            }
        };
        if self.allocated_bytes() > self.maximum_allocation_bytes {
            let fault = RetainedPackCatalogFault{cause:RetainedPackCatalogCause::Allocation(PagedListAllocationError{kind:PagedListRefusalKind::OwnershipLimit,reason:"retained-pack.symbol-allocation-overgrant",allocated_bytes:step.allocated_bytes}),code:"retained-pack.symbol-allocation-overgrant",offset};
            self.fault.get_or_insert(fault);
            return Err(RetainedPackCatalogAllocationError { allocated_bytes: step.allocated_bytes, fault });
        }
        Ok(RetainedPackCatalogAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
    }

    pub fn symbol_char(&self, symbol: u64, character: usize) -> Result<Option<char>, RetainedPackCatalogFault> {
        let symbol=usize::try_from(symbol).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-reference",symbol))?;
        let span=self.symbols.get(symbol).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.symbol-reference",symbol as u64))?;
        let scalar_start=usize::try_from(span.scalar_start).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",symbol as u64))?;
        let scalar_len=usize::try_from(span.scalar_len).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",symbol as u64))?;
        if character >= scalar_len {
            return Ok(None);
        }
        let index=scalar_start.checked_add(character).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",symbol as u64))?;
        Ok(self.scalars.get(index).copied())
    }

    pub fn symbol_chars(&self, symbol: u64) -> Result<usize, RetainedPackCatalogFault> {
        let index=usize::try_from(symbol).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-reference",symbol))?;
        let span=self.symbols.get(index).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.symbol-reference",symbol))?;
        usize::try_from(span.scalar_len).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",symbol))
    }

    pub fn symbol_span(&self, symbol: u64) -> Result<RetainedPackSymbolSpan, RetainedPackCatalogFault> {
        let index=usize::try_from(symbol).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-reference",symbol))?;
        let span=self.symbols.get(index).copied().ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.symbol-reference",symbol))?;
        let start=usize::try_from(span.scalar_start).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",symbol))?;
        let len=usize::try_from(span.scalar_len).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",symbol))?;
        let end=start.checked_add(len).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",symbol))?;
        if end>self.scalars.len(){return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-span",symbol));}Ok(span)
    }

    pub fn push_symbol_reserved(&mut self, span: RetainedPackSymbolSpan, offset: u64) -> Result<u64, RetainedPackCatalogFault> {
        if let Some(fault)=self.fault{return Err(fault);}
        if self.closed||self.closing{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-count",offset)));}
        if self.symbols.len()==self.maximum_symbols{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.symbol-count",offset)));}
        let scalar_start = match usize::try_from(span.scalar_start) {
            Ok(value) => value,
            Err(_)=>return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",offset))),
        };
        let scalar_len = match usize::try_from(span.scalar_len) {
            Ok(value) => value,
            Err(_)=>return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-span",offset))),
        };
        let utf8_len = match usize::try_from(span.utf8_len) {
            Ok(value) => value,
            Err(_)=>return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.symbol-bytes",offset))),
        };
        if !self.utf8_bytes.checked_add(utf8_len).is_some_and(|total|total<=self.maximum_utf8_bytes){return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.symbol-span",offset)));}
        let valid = scalar_start == self.published_scalars
            && scalar_start.checked_add(scalar_len) == Some(self.scalars.len())
            && utf8_len == self.pending_utf8_bytes;
        if !valid {
            return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-span",offset)));
        }
        if self.symbols.push_reserved(span).is_err(){
            return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-span-allocation",offset)));
        }
        self.published_scalars = self.scalars.len();
        self.utf8_bytes += utf8_len;
        self.pending_utf8_bytes = 0;
        Ok(self.symbols.len() as u64 - 1)
    }

    pub fn push_scalar_reserved(&mut self, value: char, offset: u64) -> Result<(), RetainedPackCatalogFault> {
        if let Some(fault)=self.fault{return Err(fault);}
        if self.closed||self.closing{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-scalars",offset)));}
        if self.scalars.len()==self.maximum_scalars{return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.symbol-scalars",offset)));}
        let pending = match self.pending_utf8_bytes.checked_add(value.len_utf8()) {
            Some(pending) if self.utf8_bytes.checked_add(pending).is_some_and(|total| total <= self.maximum_utf8_bytes) => pending,
            _=>return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.symbol-bytes",offset))),
        };
        if self.scalars.push_reserved(value).is_err(){
            return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.symbol-scalar-allocation",offset)));
        }
        self.pending_utf8_bytes = pending;
        Ok(())
    }

    pub fn next_release_allocation_bytes(&self) -> Result<Option<usize>, PagedListError> {
        if !self.scalars.is_empty() || !self.symbols.is_empty() {
            return Ok(None);
        }
        if !self.scalars.terminal_is_empty() {
            return self.scalars.next_release_allocation_bytes().map(Some);
        }
        if !self.symbols.terminal_is_empty() {
            return self.symbols.next_release_allocation_bytes().map(Some);
        }
        Ok(None)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedPackCloseStep, PagedListError> {
        if !self.closed && maximum_items == 0 && maximum_bytes == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if maximum_items == 0 && (!self.scalars.is_empty() || !self.symbols.is_empty()) {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.closing = true;
        if maximum_items != 0 && (self.scalars.pop().is_some() || self.symbols.pop().is_some()) {
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.scalars.is_empty() || !self.symbols.is_empty() {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.published_scalars = 0;
        self.pending_utf8_bytes = 0;
        self.utf8_bytes = 0;
        for owner in [false, true] {
            let (terminal, step) = if owner {
                (self.symbols.terminal_is_empty(), self.symbols.release_empty_page(maximum_bytes))
            } else {
                (self.scalars.terminal_is_empty(), self.scalars.release_empty_page(maximum_bytes))
            };
            if !terminal {
                let step = step?;
                return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: step.released_allocation_bytes });
            }
        }
        self.closed = true;
        Ok(RetainedPackCloseStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed
            && self.storage_retirement.is_none()
            && self.storage_retiring_bytes == 0
            && self.scalars.terminal_is_empty()
            && self.symbols.terminal_is_empty()
            && self.published_scalars == 0
            && self.pending_utf8_bytes == 0
            && self.utf8_bytes == 0
    }

    #[cfg(test)]
    pub(super) fn scalar_ptr(&self) -> Option<*const char> {
        self.scalars.backing_ptr(0)
    }
}

impl Drop for RetainedPackSymbolTable {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained symbol table reached Drop before terminal-empty close");
    }
}

//! 📜️ Record body cursor of the retained record decoder: the body token and phase enums, the resumable record body cursor, its allocation records and its drop guard. See [`super`].

use super::*;

//#region 🔖️RetainedValue
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedRecordBodyToken {
    Catalog { symbols: u64 },
    SymbolChar { symbol: u64, character: char },
    CatalogComplete,
    Value(RetainedValueToken),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RetainedRecordBodyPhase {
    SymbolCount(RetainedVarint),
    SymbolLength { symbol: u64, cursor: RetainedVarint },
    SymbolText { symbol: u64, scalar_start: usize, utf8_len: usize, remaining: u64, cursor: RetainedUtf8 },
    Value,
    Closing,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RetainedRecordBodyClosePhase {
    ValueLogical,
    SymbolsLogical,
    ValuePhysical,
    SymbolsPhysical,
}

/// 🧵️ Retained cursor for the canonical container-less record-body wire used by mutations.
/// It owns a fixed symbol registry and forwards the record value without constructing a
/// schema-erased record.
pub struct RetainedRecordBodyCursor {
    pub(super) limits: PackLimits,
    pub(super) intrinsic_field: bool,
    pub(super) phase: RetainedRecordBodyPhase,
    pub(super) symbols: std::mem::ManuallyDrop<semio_framework_pack_format_catalog::RetainedPackSymbolTable>,
    pub(super) maximum_allocation_bytes: usize,
    pub(super) expected_symbols: u64,
    pub(super) symbol_utf8_bytes: usize,
    pub(super) offset: u64,
    pub(super) value_offset: u64,
    pub(super) pending: Option<(u64, u8)>,
    pub(super) catalog_complete_pending: bool,
    pub(super) sealed: bool,
    value_sealed: bool,
    pub(super) value: std::mem::ManuallyDrop<Option<RetainedValueCursor>>,
    pub(super) fault: Option<PackRefusal>,
    pub(super) closing: bool,
    pub(super) close_phase: RetainedRecordBodyClosePhase,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedRecordBodyAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedRecordBodyAllocationError {
    pub allocated_bytes: usize,
    pub fault: PackRefusal,
}

#[derive(Clone, Copy)]
enum RetainedRecordBodyAllocationOwner {
    SymbolSpans { target: usize, offset: u64 },
    SymbolScalars { target: usize, offset: u64 },
    Value,
}

impl RetainedRecordBodyCursor {
    pub fn try_new(
        limits: PackLimits,
        maximum_symbols: usize,
        maximum_symbol_utf8_bytes: usize,
        maximum_symbol_scalars: usize,
        maximum_allocation_bytes: usize,
    ) -> Result<Self, PackRefusal> {
        if limits.max_symbols == 0 || limits.max_items == 0 || limits.max_depth == 0 {
            return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body credits" });
        }
        if maximum_symbols > limits.max_symbols as usize || maximum_symbol_utf8_bytes as u64 > limits.max_file_len { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body physical credits" }); }
        if maximum_symbol_scalars > maximum_symbol_utf8_bytes { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvalidValue, limit: "retained record-body physical credits" }); }
        if maximum_allocation_bytes == 0 || maximum_allocation_bytes > isize::MAX as usize { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body physical credits" }); }
        Ok(Self {
            limits,
            intrinsic_field:false,
            phase: RetainedRecordBodyPhase::SymbolCount(RetainedVarint::default()),
            symbols: std::mem::ManuallyDrop::new(
                semio_framework_pack_format_catalog::RetainedPackSymbolTable::try_new(maximum_symbols, maximum_symbol_utf8_bytes, maximum_symbol_scalars, maximum_allocation_bytes)
                    .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?,
            ),
            maximum_allocation_bytes,
            expected_symbols: 0,
            symbol_utf8_bytes: 0,
            offset: 0,
            value_offset: 0,
            pending: None,
            catalog_complete_pending: false,
            sealed: false,
            value_sealed: false,
            value: std::mem::ManuallyDrop::new(None),
            fault: None,
            closing: false,
            close_phase: RetainedRecordBodyClosePhase::ValueLogical,
        })
    }

    /// 🎞️ Validates intrinsic logical depth independently from the single-field Body envelope.
    pub fn try_new_intrinsic(limits:PackLimits,maximum_symbols:usize,maximum_symbol_utf8_bytes:usize,maximum_symbol_scalars:usize,maximum_allocation_bytes:usize)->Result<Self,PackRefusal>{
        let mut cursor=Self::try_new(limits,maximum_symbols,maximum_symbol_utf8_bytes,maximum_symbol_scalars,maximum_allocation_bytes)?;cursor.intrinsic_field=true;Ok(cursor)
    }

    pub fn admit_byte(&mut self, offset: u64, value: u8) -> Result<(), (u64, u8)> {
        if self.sealed
            || self.closing
            || self.fault.is_some()
            || self.pending.is_some()
            || self.catalog_complete_pending
            || offset != self.offset
            || matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed)
        {
            return Err((offset, value));
        }
        self.pending = Some((offset, value));
        Ok(())
    }

    pub fn seal(&mut self, bytes: u64) -> Result<(), PackRefusal> {
        if self.closing || self.fault.is_some() || self.pending.is_some() || bytes != self.offset {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "seal position mismatch" });
        }
        self.sealed = true;
        Ok(())
    }

    fn allocated_bytes_checked(&self) -> Option<usize> {
        self.symbols.allocated_bytes().checked_add(self.value.as_ref().map_or(0, RetainedValueCursor::allocated_bytes))
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated_bytes_checked().unwrap_or(usize::MAX)
    }

    fn allocation_need(&self) -> Result<Option<(RetainedRecordBodyAllocationOwner, usize)>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        if self.closing || matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "allocation after close" });
        }
        if self.phase == RetainedRecordBodyPhase::Value {
            let value = self.value.as_ref().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" })?;
            return value.next_allocation_bytes().map(|need| need.map(|bytes| (RetainedRecordBodyAllocationOwner::Value, bytes)));
        }
        let Some((offset, byte)) = self.pending else { return Ok(None) };
        let need = match self.phase {
            RetainedRecordBodyPhase::SymbolCount(cursor) => match cursor.preview(byte, offset)? {
                None => None,
                Some(count) => {
                    let target = usize::try_from(count)
                        .ok()
                        .filter(|target| *target <= self.symbols.maximum_symbols())
                        .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol count" })?;
                    {let bytes=self.symbols.source_reserve_demands(true,target,offset).map_err(|fault|fault.into_pack_refusal("retained-record-body-symbol-allocation"))?.capacity_bytes;(bytes!=0).then_some((RetainedRecordBodyAllocationOwner::SymbolSpans{target,offset},bytes))}
                }
            },
            RetainedRecordBodyPhase::SymbolLength { cursor, .. } => {
                if let Some(length) = cursor.preview(byte, offset)? {
                    let length = usize::try_from(length).map_err(|_| PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol bytes" })?;
                    self.symbol_utf8_bytes
                        .checked_add(length)
                        .filter(|total| *total <= self.symbols.maximum_utf8_bytes())
                        .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body cumulative symbol bytes" })?;
                }
                None
            }
            RetainedRecordBodyPhase::SymbolText { cursor, .. } => match cursor.preview(byte, offset)? {
                None => None,
                Some(_) => {
                    let target = self.symbols.scalar_len().checked_add(1).filter(|target| *target <= self.symbols.maximum_scalars()).ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol scalars" })?;
                    {let bytes=self.symbols.source_reserve_demands(false,target,offset).map_err(|fault|fault.into_pack_refusal("retained-record-body-symbol-allocation"))?.capacity_bytes;(bytes!=0).then_some((RetainedRecordBodyAllocationOwner::SymbolScalars{target,offset},bytes))}
                }
            },
            RetainedRecordBodyPhase::Value | RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed => None,
        };
        Ok(need)
    }

    pub fn next_allocation_bytes(&mut self) -> Result<Option<usize>, PackRefusal> {
        let need = match self.allocation_need() {
            Ok(need) => need,
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                return Err(self.fault.clone().expect("record body first fault"));
            }
        };
        let Some((_, requested)) = need else { return Ok(None) };
        let result = self
            .allocated_bytes_checked()
            .and_then(|allocated| allocated.checked_add(requested))
            .filter(|total| *total <= self.maximum_allocation_bytes)
            .map(|_| Some(requested))
            .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body allocation credits" });
        if let Err(fault) = &result {
            self.fault.get_or_insert_with(|| fault.clone());
        }
        result
    }

    /// 🔍️ Quotes the original next physical allocation or scalar write without moving pending source.
    pub fn source_step_demands(&self)->Result<semio_framework_value::RetirementDemand,PackRefusal>{
        use semio_framework_value::RetirementDemand;
        let mut demand=if let Some((owner,bytes))=self.allocation_need()?{match owner{
            RetainedRecordBodyAllocationOwner::SymbolSpans{target,offset}=>self.symbols.source_reserve_demands(true,target,offset).map_err(|error|error.into_pack_refusal("retained-record-body-source"))?,
            RetainedRecordBodyAllocationOwner::SymbolScalars{target,offset}=>self.symbols.source_reserve_demands(false,target,offset).map_err(|error|error.into_pack_refusal("retained-record-body-source"))?,
            RetainedRecordBodyAllocationOwner::Value=>RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()},
        }}else{match(self.phase,self.pending){
            (RetainedRecordBodyPhase::SymbolText{remaining,cursor,..},Some((offset,byte)))if cursor.preview(byte,offset)?.is_some()=>{let mut next=self.symbols.source_write_demands(false)?;if remaining==1{next.depth=next.depth.max(self.symbols.source_write_demands(true)?.depth)}next},
            (RetainedRecordBodyPhase::SymbolLength{cursor,..},Some((offset,byte)))if cursor.preview(byte,offset)?==Some(0)=>self.symbols.source_write_demands(true)?,
            _=>RetirementDemand{depth:1,..Default::default()},
        }};
        demand.depth=demand.depth.checked_add(1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"record-body source physical depth"})?;Ok(demand)
    }
    /// 🧹️ Scalar symbol and native index removal has no payload copy; actual backing release stays distinct.
    pub fn source_close_demands(&self)->Result<semio_framework_value::RetirementDemand,PackRefusal>{
        use semio_framework_value::RetirementDemand;
        if self.terminal_is_empty(){return Ok(Default::default())}
        let mut demand=if self.pending.is_some()||self.catalog_complete_pending{RetirementDemand{depth:1,..Default::default()}}
        else{match self.close_phase{
            RetainedRecordBodyClosePhase::ValueLogical=>RetirementDemand{depth:if self.value.is_some(){2}else{1},..Default::default()},
            RetainedRecordBodyClosePhase::SymbolsLogical=>{let mut next=self.symbols.retirement_demands()?;next.copy_bytes=0;next.release_bytes=0;next},
            RetainedRecordBodyClosePhase::SymbolsPhysical=>{let mut next=self.symbols.retirement_demands()?;next.copy_bytes=0;next},
            RetainedRecordBodyClosePhase::ValuePhysical=>RetirementDemand{release_bytes:self.value.as_ref().and_then(RetainedValueCursor::next_release_allocation_bytes).unwrap_or(0),depth:2,..Default::default()},
        }};
        demand.depth=demand.depth.checked_add(1).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"record-body source physical depth"})?;Ok(demand)
    }

    pub fn reserve_allocation(&mut self, maximum_bytes: usize) -> Result<RetainedRecordBodyAllocationStep, RetainedRecordBodyAllocationError> {
        let need = match self.allocation_need() {
            Ok(need) => need,
            Err(fault) => {
                let first = self.fault.get_or_insert_with(|| fault.clone()).clone();
                return Err(RetainedRecordBodyAllocationError { allocated_bytes: 0, fault: first });
            }
        };
        let Some((owner, requested)) = need else { return Ok(RetainedRecordBodyAllocationStep::default()) };
        if maximum_bytes < requested {
            return Ok(RetainedRecordBodyAllocationStep::default());
        }
        let remaining = self.maximum_allocation_bytes.saturating_sub(self.allocated_bytes());
        let step = match owner {
            RetainedRecordBodyAllocationOwner::SymbolSpans { target, offset } => self
                .symbols
                .reserve_symbol_capacity(target, maximum_bytes.min(remaining), offset)
                .map(|step| RetainedRecordBodyAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                .map_err(|error| RetainedRecordBodyAllocationError {
                    allocated_bytes: error.allocated_bytes,
                    fault: error.fault.into_pack_refusal("retained-record-body-symbol-allocation"),
                }),
            RetainedRecordBodyAllocationOwner::SymbolScalars { target, offset } => self
                .symbols
                .reserve_scalar_capacity(target, maximum_bytes.min(remaining), offset)
                .map(|step| RetainedRecordBodyAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                .map_err(|error| RetainedRecordBodyAllocationError {
                    allocated_bytes: error.allocated_bytes,
                    fault: error.fault.into_pack_refusal("retained-record-body-symbol-allocation"),
                }),
            RetainedRecordBodyAllocationOwner::Value => self
                .value
                .as_mut()
                .ok_or_else(|| RetainedRecordBodyAllocationError {
                    allocated_bytes: 0,
                    fault: PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" },
                })?
                .reserve_allocation(maximum_bytes.min(remaining))
                .map(|step| RetainedRecordBodyAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
                .map_err(|error| RetainedRecordBodyAllocationError { allocated_bytes: error.allocated_bytes, fault: error.fault }),
        };
        match step {
            Ok(step) if self.allocated_bytes() <= self.maximum_allocation_bytes => Ok(step),
            Ok(step) => {
                let fault = PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body allocation overgrant" };
                self.fault.get_or_insert_with(|| fault.clone());
                Err(RetainedRecordBodyAllocationError { allocated_bytes: step.allocated_bytes, fault })
            }
            Err(error) => {
                let first = self.fault.get_or_insert_with(|| error.fault.clone()).clone();
                Err(RetainedRecordBodyAllocationError { allocated_bytes: error.allocated_bytes, fault: first })
            }
        }
    }

    pub fn symbol_chars(&self, symbol: u64) -> Result<usize, PackRefusal> {
        self.symbols.symbol_chars(symbol).map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))
    }

    /// 🧮️ Borrows the validated exact UTF-8 size before any symbol copy is allocated.
    pub fn symbol_utf8_bytes(&self, symbol:u64)->Result<usize,PackRefusal>{
        let span=self.symbols.symbol_span(symbol).map_err(|fault|fault.into_pack_refusal("retained-record-body-symbol"))?;
        usize::try_from(span.utf8_len).map_err(|_|PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained symbol UTF-8 size"})
    }

    /// 📍️ Counts consumed wire bytes without counting any still-pending ingress byte.
    pub fn consumed_bytes(&self)->u64{
        self.offset-self.value_offset+self.value.as_ref().map_or(0,|value|value.offset)
    }

    pub fn symbol_char(&self, symbol: u64, index: usize) -> Result<Option<char>, PackRefusal> {
        self.symbols.symbol_char(symbol, index).map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))
    }

    fn begin_value(&mut self) -> Result<(), PackRefusal> {
        *self.value = Some(if self.intrinsic_field{RetainedValueCursor::try_new_intrinsic(self.limits.clone(),self.maximum_allocation_bytes)?}else{RetainedValueCursor::try_new(self.limits.clone(), self.maximum_allocation_bytes)?});
        self.phase = RetainedRecordBodyPhase::Value;
        Ok(())
    }

    pub(super) fn grant_inner(&mut self) -> Result<Option<RetainedRecordBodyToken>, PackRefusal> {
        if matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "grant after close" });
        }
        if self.catalog_complete_pending {
            self.catalog_complete_pending = false;
            return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
        }
        if self.phase == RetainedRecordBodyPhase::Value {
            if let Some((offset, byte)) = self.pending.take() {
                let value = self.value.as_mut().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset, detail: "value owner missing" })?;
                if !value.ingress_ready() {
                    self.pending = Some((offset, byte));
                    return Ok(None);
                }
                value.admit_byte(self.value_offset, byte).map_err(|_| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated,
                    what: "retained-record-body",
                    offset,
                    detail: "value producer handback",
                })?;
                self.offset += 1;
                self.value_offset += 1;
            } else if self.sealed && !self.value_sealed {
                self.value.as_mut().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" })?.seal(self.value_offset)?;
                self.value_sealed = true;
            }
            return self.value.as_mut().ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body", offset: self.offset, detail: "value owner missing" })?.grant().map(|token| token.map(RetainedRecordBodyToken::Value));
        }
        let Some((offset, byte)) = self.pending.take() else {
            return if self.sealed { Err(PackRefusal::Truncated(self.offset)) } else { Ok(None) };
        };
        self.offset += 1;
        match self.phase {
            RetainedRecordBodyPhase::SymbolCount(mut cursor) => match cursor.admit(byte, offset)? {
                None => self.phase = RetainedRecordBodyPhase::SymbolCount(cursor),
                Some(symbols) => {
                    if symbols > self.symbols.maximum_symbols() as u64 {
                        return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body symbol count" });
                    }
                    self.expected_symbols = symbols;
                    if symbols == 0 {
                        self.begin_value()?;
                        return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
                    }
                    self.phase = RetainedRecordBodyPhase::SymbolLength { symbol: 0, cursor: RetainedVarint::default() };
                    return Ok(Some(RetainedRecordBodyToken::Catalog { symbols }));
                }
            },
            RetainedRecordBodyPhase::SymbolLength { symbol, mut cursor } => match cursor.admit(byte, offset)? {
                None => self.phase = RetainedRecordBodyPhase::SymbolLength { symbol, cursor },
                Some(length) => {
                    if length > self.limits.max_segment_len { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained record-body symbol length" }); }
                    if length > usize::MAX as u64 { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body symbol length" }); }
                    let length = length as usize;
                    self.symbol_utf8_bytes = self
                        .symbol_utf8_bytes
                        .checked_add(length)
                        .filter(|total| *total <= self.symbols.maximum_utf8_bytes())
                        .ok_or(PackRefusal::LimitExceeded { kind: ValueRefusalKind::OwnershipLimit, limit: "retained record-body cumulative symbol bytes" })?;
                    if length == 0 {
                        let scalar_start = self.symbols.scalar_len() as u64;
                        self.symbols
                            .push_symbol_reserved(semio_framework_pack_format_catalog::RetainedPackSymbolSpan { scalar_start, scalar_len: 0, utf8_len: 0 }, offset)
                            .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?;
                        if symbol + 1 == self.expected_symbols {
                            self.begin_value()?;
                            return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
                        }
                        self.phase = RetainedRecordBodyPhase::SymbolLength { symbol: symbol + 1, cursor: RetainedVarint::default() };
                    } else {
                        self.phase = RetainedRecordBodyPhase::SymbolText {
                            symbol,
                            scalar_start: self.symbols.scalar_len(),
                            utf8_len: length,
                            remaining: length as u64,
                            cursor: RetainedUtf8::default(),
                        };
                    }
                }
            },
            RetainedRecordBodyPhase::SymbolText { symbol, scalar_start, utf8_len, remaining, mut cursor } => {
                let character = cursor.admit(byte, offset)?;
                let next = remaining - 1;
                if let Some(character) = character {
                    self.symbols
                        .push_scalar_reserved(character, offset)
                        .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?;
                }
                if next == 0 {
                    if cursor.remaining != 0 {
                        return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-record-body-symbol", offset, detail: "truncated UTF-8 scalar" });
                    }
                    let scalar_len = self
                        .symbols
                        .scalar_len()
                        .checked_sub(scalar_start)
                        .ok_or_else(|| PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvariantViolated, what: "retained-record-body-symbol", offset, detail: "scalar span underflow" })?;
                    self.symbols
                        .push_symbol_reserved(
                            semio_framework_pack_format_catalog::RetainedPackSymbolSpan { scalar_start: scalar_start as u64, scalar_len: scalar_len as u64, utf8_len: utf8_len as u64 },
                            offset,
                        )
                        .map_err(|fault| fault.into_pack_refusal("retained-record-body-symbol"))?;
                    if symbol + 1 == self.expected_symbols {
                        self.begin_value()?;
                        self.catalog_complete_pending = character.is_some();
                    } else {
                        self.phase = RetainedRecordBodyPhase::SymbolLength { symbol: symbol + 1, cursor: RetainedVarint::default() };
                    }
                } else {
                    self.phase = RetainedRecordBodyPhase::SymbolText { symbol, scalar_start, utf8_len, remaining: next, cursor };
                }
                if let Some(character) = character {
                    return Ok(Some(RetainedRecordBodyToken::SymbolChar { symbol, character }));
                }
                if self.phase == RetainedRecordBodyPhase::Value {
                    return Ok(Some(RetainedRecordBodyToken::CatalogComplete));
                }
            }
            RetainedRecordBodyPhase::Value | RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed => unreachable!(),
        }
        Ok(None)
    }

    pub fn grant(&mut self) -> Result<Option<RetainedRecordBodyToken>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        match self.next_allocation_bytes() {
            Ok(Some(_)) => return Ok(None),
            Ok(None) => {}
            Err(fault) => return Err(fault),
        }
        match self.grant_inner() {
            Ok(value) => Ok(value),
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                Err(self.fault.clone().expect("record body first fault"))
            }
        }
    }

    pub fn next_release_allocation_bytes(&self) -> Result<Option<usize>, PackRefusal> {
        if self.pending.is_some() || !self.closing {
            return Ok(None);
        }
        match self.close_phase {
            RetainedRecordBodyClosePhase::ValuePhysical => {
                Ok(self.value.as_ref().and_then(RetainedValueCursor::next_release_allocation_bytes))
            }
            RetainedRecordBodyClosePhase::SymbolsPhysical => self.symbols.next_release_allocation_bytes().map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset)),
            RetainedRecordBodyClosePhase::ValueLogical | RetainedRecordBodyClosePhase::SymbolsLogical => Ok(None),
        }
    }

    pub fn close_step(
        &mut self,
        maximum_items: usize,
        maximum_bytes: usize,
    ) -> Result<semio_framework_pack_format_retained::RetainedPackCloseStep, PackRefusal> {
        use semio_framework_pack_format_retained::RetainedPackCloseStep;
        if self.phase == RetainedRecordBodyPhase::Closed {
            return Ok(RetainedPackCloseStep::Complete);
        }
        if maximum_items == 0 && maximum_bytes == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.closing = true;
        if maximum_items != 0 && self.pending.take().is_some() {
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if maximum_items != 0 && self.catalog_complete_pending {
            self.catalog_complete_pending = false;
            return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        loop {
            match self.close_phase {
                RetainedRecordBodyClosePhase::ValueLogical => {
                    if let Some(value) = self.value.as_mut() {
                        match value.close_step(maximum_items, 0)? {
                            RetainedPackCloseStep::Pending { released_items, released_bytes } if released_items != 0 || released_bytes != 0 => {
                                return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                            }
                            RetainedPackCloseStep::Pending { .. } if value.next_release_allocation_bytes().is_none() && !value.terminal_is_empty() => {
                                return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                            }
                            RetainedPackCloseStep::Pending { .. } | RetainedPackCloseStep::Complete => {}
                        }
                    }
                    if maximum_items == 0 {
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    self.phase = RetainedRecordBodyPhase::Closing;
                    self.expected_symbols = 0;
                    self.symbol_utf8_bytes = 0;
                    self.close_phase = RetainedRecordBodyClosePhase::SymbolsLogical;
                    return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
                }
                RetainedRecordBodyClosePhase::SymbolsLogical => match self.symbols.close_step(maximum_items, 0).map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset))? {
                    RetainedPackCloseStep::Pending { released_items, released_bytes } if released_items != 0 || released_bytes != 0 => {
                        return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                    }
                    RetainedPackCloseStep::Pending { .. } if self.symbols.next_release_allocation_bytes().map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset))?.is_none() && !self.symbols.terminal_is_empty() => {
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    RetainedPackCloseStep::Pending { .. } | RetainedPackCloseStep::Complete => {
                        if maximum_items == 0 {
                            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                        }
                        self.close_phase = RetainedRecordBodyClosePhase::ValuePhysical;
                        return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                },
                RetainedRecordBodyClosePhase::ValuePhysical => {
                    if let Some(value) = self.value.as_mut() {
                        let owner_items = if value.next_release_allocation_bytes().is_none() { maximum_items } else { 0 };
                        match value.close_step(owner_items, maximum_bytes)? {
                            RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                                return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                            }
                            RetainedPackCloseStep::Complete => {
                                if maximum_items == 0 {
                                    return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
                                }
                                drop(self.value.take());
                                self.close_phase = RetainedRecordBodyClosePhase::SymbolsPhysical;
                                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
                            }
                        }
                    }
                    self.close_phase = RetainedRecordBodyClosePhase::SymbolsPhysical;
                }
                RetainedRecordBodyClosePhase::SymbolsPhysical => match self.symbols.close_step(maximum_items, maximum_bytes).map_err(|error|PackRefusal::from_paged_refusal(error,"retained-record-body-symbol-release",self.offset))? {
                    RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                        return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                    }
                    RetainedPackCloseStep::Complete => {
                        self.phase = RetainedRecordBodyPhase::Closed;
                        return Ok(RetainedPackCloseStep::Complete);
                    }
                },
            }
        }
    }

    /// 🚦️ Whether [`Self::admit_byte`] would accept another byte AND be able to forward it on the
    /// next [`Self::grant`]. Once the value producer is live every admitted byte is handed straight
    /// to it, so this cursor's own empty ingress slot is not sufficient: a producer still chewing on
    /// the previous byte rejects the handoff as `retained-record-body/value producer handback`.
    pub fn ingress_ready(&self) -> bool {
        !self.sealed
            && !self.closing
            && self.fault.is_none()
            && self.pending.is_none()
            && !self.catalog_complete_pending
            && !matches!(self.phase, RetainedRecordBodyPhase::Closing | RetainedRecordBodyPhase::Closed)
            && self.value.as_ref().is_none_or(RetainedValueCursor::ingress_ready)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase == RetainedRecordBodyPhase::Closed
            && self.value.is_none()
            && self.symbols.terminal_is_empty()
            && self.symbols.allocated_bytes() == 0
            && self.pending.is_none()
            && !self.catalog_complete_pending
    }
}

impl Drop for RetainedRecordBodyCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained record-body cursor reached Drop before terminal-empty close");
    }
}
//#endregion 🔖️RetainedValue

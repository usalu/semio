//! 🧭️ Value cursor of the retained record decoder: the resumable cursor that admits one wire value per granted step, its allocation and progress records and its drop guard. See [`super`].

use super::*;

//#region 🔖️RetainedValue
pub struct RetainedValueCursor {
    pub(super) limits: PackLimits,
    pub(super) intrinsic_field: bool,
    pub(super) stack: std::mem::ManuallyDrop<Vec<Expect>>,
    pub(super) maximum_frames: usize,
    pub(super) maximum_allocation_bytes: usize,
    pub(super) initialized_roots: u8,
    pub(super) pending: Option<(u64, u8)>,
    pub(super) offset: u64,
    pub(super) sealed: bool,
    pub(super) closing: bool,
    pub(super) fault: Option<PackRefusal>,
    pub(super) closed: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedValueAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RetainedValueAllocationError {
    pub allocated_bytes: usize,
    pub fault: PackRefusal,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedValueProgress {
    pub frames: usize,
    pub frame_capacity: usize,
    pub allocated_bytes: usize,
    pub initialized_roots: u8,
    pub pending_input: bool,
    pub closed: bool,
}

impl RetainedValueCursor {
    /// 📏️ Quotes the original scalar frame or whole stack allocation before close.
    pub fn retirement_demands(&self) -> semio_framework_value::RetirementDemand {
        use semio_framework_value::RetirementDemand;
        if self.terminal_is_empty() { return Default::default(); }
        let copy_bytes = if self.pending.is_some() { size_of::<(u64, u8)>() } else if !self.stack.is_empty() { size_of::<Expect>() } else { 0 };
        RetirementDemand { copy_bytes, release_bytes: if copy_bytes == 0 { self.stack_allocation_bytes() } else { 0 }, depth: 1, ..Default::default() }
    }
    pub fn try_new(limits: PackLimits, maximum_allocation_bytes: usize) -> Result<Self, PackRefusal> {
        if limits.max_depth == 0{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"retained value depth credits"});}
        if limits.max_items == 0{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"retained value item credits"});}
        let maximum_frames = usize::from(limits.max_depth).checked_mul(8).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack"})?;
        let requested = maximum_frames.checked_mul(size_of::<Expect>()).ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack allocation"})?;
        if maximum_allocation_bytes == 0 || maximum_allocation_bytes > isize::MAX as usize || requested > maximum_allocation_bytes {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack allocation credits"});
        }
        Ok(Self {
            limits,
            intrinsic_field:false,
            stack: std::mem::ManuallyDrop::new(Vec::new()),
            maximum_frames,
            maximum_allocation_bytes,
            initialized_roots: 0,
            pending: None,
            offset: 0,
            sealed: false,
            closing: false,
            fault: None,
            closed: false,
        })
    }

    /// 🎞️ Counts intrinsic value depth from its declared Value field rather than physical framing.
    pub fn try_new_intrinsic(limits:PackLimits,maximum_allocation_bytes:usize)->Result<Self,PackRefusal>{
        let mut cursor=Self::try_new(limits,maximum_allocation_bytes)?;cursor.intrinsic_field=true;Ok(cursor)
    }

    pub(super) fn stack_allocation_bytes(&self) -> usize {
        self.stack.capacity().saturating_mul(size_of::<Expect>())
    }

    pub fn allocated_bytes(&self) -> usize {
        self.stack_allocation_bytes()
    }

    pub fn next_allocation_bytes(&self) -> Result<Option<usize>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        if self.closed || self.closing {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated,what: "retained-value", offset: self.offset, detail: "allocation after close" });
        }
        if self.stack.capacity() >= self.maximum_frames {
            return Ok(None);
        }
        self.maximum_frames
            .checked_mul(size_of::<Expect>())
            .filter(|requested| *requested <= self.maximum_allocation_bytes)
            .map(Some)
            .ok_or(PackRefusal::LimitExceeded{kind:ValueRefusalKind::OwnershipLimit,limit:"retained value stack allocation credits"})
    }

    pub fn reserve_allocation(&mut self, maximum_bytes: usize) -> Result<RetainedValueAllocationStep, RetainedValueAllocationError> {
        let requested = self.next_allocation_bytes().map_err(|fault| RetainedValueAllocationError { allocated_bytes: 0, fault })?;
        let Some(requested) = requested else { return Ok(RetainedValueAllocationStep::default()) };
        if maximum_bytes < requested {
            return Ok(RetainedValueAllocationStep::default());
        }
        if self.stack.try_reserve_exact(self.maximum_frames).is_err() {
            let fault = PackRefusal::RetainedAllocation{kind:ValueRefusalKind::AllocationFailed,allocated_bytes:0,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation"};
            let first = self.fault.get_or_insert_with(|| fault.clone()).clone();
            return Err(RetainedValueAllocationError { allocated_bytes: 0, fault: first });
        }
        let allocated_bytes = self.stack_allocation_bytes();
        if allocated_bytes > maximum_bytes || allocated_bytes > self.maximum_allocation_bytes {
            let fault = PackRefusal::RetainedAllocation{kind:ValueRefusalKind::OwnershipLimit,allocated_bytes,what:"retained-value-stack",offset:self.offset,detail:"retained value stack allocation overgrant"};
            self.fault.get_or_insert_with(|| fault.clone());
            return Err(RetainedValueAllocationError { allocated_bytes, fault });
        }
        Ok(RetainedValueAllocationStep { progressed: true, allocated_bytes })
    }

    fn initialize_root(&mut self) -> Result<bool, PackRefusal> {
        if self.initialized_roots == 2 {
            return Ok(false);
        }
        if self.stack.capacity() < self.maximum_frames {
            return Ok(false);
        }
        let frame = if self.initialized_roots == 0 { Expect::Finish } else { Expect::Varint(RetainedVarint::default(), AfterVarint::Record(0)) };
        self.stack.push(frame);
        self.initialized_roots += 1;
        Ok(true)
    }

    pub fn admit_byte(&mut self, offset: u64, byte: u8) -> Result<(), (u64, u8)> {
        if self.closed || self.closing || self.fault.is_some() || self.sealed || self.initialized_roots != 2 || self.stack.is_empty() || self.pending.is_some() || offset != self.offset {
            return Err((offset, byte));
        }
        self.pending = Some((offset, byte));
        Ok(())
    }

    pub fn seal(&mut self, bytes: u64) -> Result<(), PackRefusal> {
        if self.closed || self.closing || self.fault.is_some() || self.initialized_roots != 2 || self.pending.is_some() || bytes != self.offset {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated,what: "retained-value", offset: self.offset, detail: "seal position mismatch" });
        }
        self.sealed = true;
        Ok(())
    }

    pub(super) fn push(&mut self, value: Expect) -> Result<(), PackRefusal> {
        if self.stack.len() == self.maximum_frames { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained value owner stack" }); }
        if self.stack.len() == self.stack.capacity() { return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::InvariantViolated, limit: "retained value owner stack" }); }
        self.stack.push(value);
        Ok(())
    }

    pub(super) fn count(&self, count: u64) -> Result<(), PackRefusal> {
        if count > self.limits.max_items {
            Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"retained value item count"})
        } else {
            Ok(())
        }
    }

    fn string(&mut self) -> Result<(), PackRefusal> {
        self.push(Expect::StringTag)
    }

    fn control(&mut self, value: Expect) -> Result<Option<RetainedValueToken>, PackRefusal> {
        match value {
            Expect::End(kind) => Ok(Some(RetainedValueToken::End(kind))),
            Expect::Finish if self.sealed => Ok(Some(RetainedValueToken::Complete { bytes: self.offset })),
            Expect::Finish => {
                self.push(Expect::Finish)?;
                Ok(None)
            }
            Expect::Record(0, _) | Expect::Values(0, _, _) | Expect::Varints(0, _) | Expect::F64s(0) | Expect::Statements(0, _) | Expect::Map(0, _, _) | Expect::TableColumns(0, _, _) | Expect::Utf8(0, _) | Expect::Bytes(0) => Ok(None),
            Expect::TableBitmap(0, _, rows, present, columns, depth) => {
                self.push(Expect::TableElem(present, rows, columns, depth))?;
                Ok(None)
            }
            Expect::Record(count, depth) => {
                self.push(Expect::Record(count - 1, depth))?;
                self.push(Expect::Value(depth + 1, RetainedContext::Field))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Field))?;
                Ok(None)
            }
            Expect::Values(count, depth, context) => {
                self.push(Expect::Values(count - 1, depth, context))?;
                self.push(Expect::Value(depth + 1, context))?;
                Ok(None)
            }
            Expect::Varints(count, role) => {
                self.push(Expect::Varints(count - 1, role))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(role)))?;
                Ok(None)
            }
            Expect::F64s(count) => {
                self.push(Expect::F64s(count - 1))?;
                self.push(Expect::F64([0; 8], 0))?;
                Ok(None)
            }
            Expect::Statements(count, depth) => {
                self.push(Expect::Statements(count - 1, depth))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Record(depth + 1)))?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Symbol)))?;
                Ok(None)
            }
            Expect::Map(count, depth, context) => {
                self.push(Expect::Map(count - 1, depth, context))?;
                self.push(Expect::Value(depth + 1, context))?;
                self.string()?;
                Ok(None)
            }
            Expect::TableColumns(count, rows, depth) => {
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::TableField(rows, count - 1, depth)))?;
                Ok(None)
            }
            other => {
                self.push(other)?;
                Ok(None)
            }
        }
    }

    pub(super) fn after(&mut self, value: u64, after: AfterVarint) -> Result<RetainedValueToken, PackRefusal> {
        match after {
            AfterVarint::Record(depth) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Record))?;
                self.push(Expect::Record(value, depth))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Record, count: value })
            }
            AfterVarint::Field => {
                if value > u16::MAX as u64 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "field-id", offset: self.offset, detail: "exceeds u16" });
                }
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::FieldId, value })
            }
            AfterVarint::Scalar(role) => {
                if role == RetainedValueRole::Integer {
                    Ok(RetainedValueToken::Signed(((value >> 1) as i64) ^ -((value & 1) as i64)))
                } else {
                    Ok(RetainedValueToken::Unsigned { role, value })
                }
            }
            AfterVarint::Sequence(kind, depth, context) => {
                self.count(value)?;
                self.push(Expect::End(kind))?;
                self.push(Expect::Values(value, depth, context))?;
                Ok(RetainedValueToken::Begin { kind, count: value })
            }
            AfterVarint::Statements(depth) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Statements))?;
                self.push(Expect::Statements(value, depth))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Statements, count: value })
            }
            AfterVarint::Map(depth, context) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Map))?;
                self.push(Expect::Map(value, depth, context))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Map, count: value })
            }
            AfterVarint::String => {
                if value > self.limits.max_segment_len {
                    return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained string length" });
                }
                self.push(Expect::Utf8(value, RetainedUtf8::default()))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::StringLength, value })
            }
            AfterVarint::Bytes => {
                if value > self.limits.max_segment_len {
                    return Err(PackRefusal::LimitExceeded { kind: ValueRefusalKind::WorkLimit, limit: "retained bytes length" });
                }
                self.push(Expect::Bytes(value))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::BytesLength, value })
            }
            AfterVarint::Chunks => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::ChunkedBytes))?;
                self.push(Expect::Varints(value, RetainedValueRole::Chunk))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::ChunkedBytes, count: value })
            }
            AfterVarint::TableRows(depth) => {
                self.count(value)?;
                self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::TableColumns(value, depth)))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::TableRows, value })
            }
            AfterVarint::TableColumns(rows, depth) => {
                self.count(value)?;
                self.push(Expect::End(RetainedValueContainer::Table))?;
                self.push(Expect::TableColumns(value, rows, depth))?;
                Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Table, count: rows })
            }
            AfterVarint::TableField(rows, columns, depth) => {
                if value > u16::MAX as u64 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "table-field", offset: self.offset, detail: "exceeds u16" });
                }
                self.push(Expect::TablePresence(columns, rows, depth))?;
                Ok(RetainedValueToken::Unsigned { role: RetainedValueRole::TableField, value })
            }
            AfterVarint::Packed(kind) => {
                self.count(value)?;
                self.push(Expect::End(kind))?;
                self.push(match kind {
                    RetainedValueContainer::PackedF64 => Expect::F64s(value),
                    RetainedValueContainer::PackedVarint => Expect::Varints(value, RetainedValueRole::Integer),
                    _ => unreachable!(),
                })?;
                Ok(RetainedValueToken::Begin { kind, count: value })
            }
        }
    }

    pub(super) fn tag(&mut self, offset: u64, tag: u8, depth: u16, context: RetainedContext) -> Result<RetainedValueToken, PackRefusal> {
        if depth > self.limits.max_depth {
            return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::DepthLimit,limit:"retained value depth"});
        }
        if context == RetainedContext::Dsl && !matches!(tag, TAG_FALSE | TAG_TRUE | TAG_INT | TAG_UINT | TAG_F64 | TAG_STR | TAG_STR_INLINE | TAG_BYTES | TAG_BYTES_CHUNKED | TAG_LIST | TAG_MAP | TAG_NULL) {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "dsl-value", offset, detail: "field-only tag" });
        }
        if context == RetainedContext::Field && tag == TAG_NULL {
            return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "field-value", offset, detail: "DSL-only null" });
        }
        match tag {
            TAG_ABSENT | TAG_FALSE | TAG_TRUE | TAG_NULL => {}
            TAG_INT => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Integer)))?,
            TAG_UINT => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Unsigned)))?,
            TAG_F64 => self.push(Expect::F64([0; 8], 0))?,
            TAG_STR => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Symbol)))?,
            TAG_STR_INLINE => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::String))?,
            TAG_BYTES => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Bytes))?,
            TAG_BYTES_CHUNKED => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Chunks))?,
            TAG_ENUM => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Enum)))?,
            TAG_TUPLE | TAG_LIST => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Sequence(if tag == TAG_TUPLE { RetainedValueContainer::Tuple } else { RetainedValueContainer::List }, depth, context)))?,
            TAG_RECORD => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Record(depth + 1)))?,
            TAG_BLOCK => self.push(Expect::Value(depth + 1, context))?,
            TAG_STATEMENTS => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Statements(depth)))?,
            TAG_MAP => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Map(depth, context)))?,
            TAG_VALUE => self.push(Expect::Value(if self.intrinsic_field&&depth==1{0}else{depth+1}, RetainedContext::Dsl))?,
            TAG_WIRE => {
                self.push(Expect::End(RetainedValueContainer::Wire))?;
                self.push(Expect::Wire(depth))?;
            }
            TAG_TABLE_SOA => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::TableRows(depth)))?,
            TAG_PACKED_F64 | TAG_PACKED_VARINT => self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Packed(if tag == TAG_PACKED_F64 { RetainedValueContainer::PackedF64 } else { RetainedValueContainer::PackedVarint })))?,
            TAG_EXPR => self.string()?,
            _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-tag", offset, detail: "unknown tag" }),
        }
        if tag == TAG_WIRE {
            Ok(RetainedValueToken::Begin { kind: RetainedValueContainer::Wire, count: 1 })
        } else {
            Ok(RetainedValueToken::Tag { offset, value: tag })
        }
    }

    pub(super) fn grant_inner(&mut self) -> Result<Option<RetainedValueToken>, PackRefusal> {
        let Some(expectation) = self.stack.pop() else { return Ok(None) };
        let control = matches!(
            expectation,
            Expect::Finish
                | Expect::End(..)
                | Expect::Record(..)
                | Expect::Values(..)
                | Expect::Varints(..)
                | Expect::F64s(..)
                | Expect::Statements(..)
                | Expect::Map(..)
                | Expect::TableColumns(..)
                | Expect::Utf8(0, _)
                | Expect::Bytes(0)
                | Expect::TableBitmap(0, ..)
        );
        if control {
            return self.control(expectation);
        }
        let Some((offset, byte)) = self.pending.take() else {
            self.stack.push(expectation);
            return if self.sealed { Err(PackRefusal::Truncated(self.offset)) } else { Ok(None) };
        };
        self.offset += 1;
        match expectation {
            Expect::Value(depth, context) => self.tag(offset, byte, depth, context).map(Some),
            Expect::StringTag => match byte {
                TAG_STR => {
                    self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::Scalar(RetainedValueRole::Symbol)))?;
                    Ok(Some(RetainedValueToken::Tag { offset, value: byte }))
                }
                TAG_STR_INLINE => {
                    self.push(Expect::Varint(RetainedVarint::default(), AfterVarint::String))?;
                    Ok(Some(RetainedValueToken::Tag { offset, value: byte }))
                }
                _ => Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-string", offset, detail: "expected string tag" }),
            },
            Expect::Varint(mut cursor, after) => match cursor.admit(byte, offset)? {
                Some(value) => self.after(value, after).map(Some),
                None => {
                    self.push(Expect::Varint(cursor, after))?;
                    Ok(None)
                }
            },
            Expect::F64(mut bytes, index) => {
                bytes[index] = byte;
                if index == 7 {
                    Ok(Some(RetainedValueToken::F64(u64::from_le_bytes(bytes))))
                } else {
                    self.push(Expect::F64(bytes, index + 1))?;
                    Ok(None)
                }
            }
            Expect::Utf8(remaining, mut cursor) => {
                let token = cursor.admit(byte, offset)?.map(RetainedValueToken::StringChar);
                if remaining > 1 {
                    self.push(Expect::Utf8(remaining - 1, cursor))?;
                } else if cursor.remaining != 0 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "retained-utf8", offset, detail: "truncated scalar" });
                }
                Ok(token)
            }
            Expect::Bytes(remaining) => {
                if remaining > 1 {
                    self.push(Expect::Bytes(remaining - 1))?;
                }
                Ok(Some(RetainedValueToken::Byte(byte)))
            }
            Expect::Wire(depth) => {
                if byte & !7 != 0 || (byte & 2 != 0 && byte & 1 == 0) {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "wire-presence", offset, detail: "invalid bits" });
                }
                self.push(Expect::Value(depth + 1, RetainedContext::Dsl))?;
                if byte & 4 != 0 {
                    self.push(Expect::WireLabel)?;
                }
                if byte & 1 != 0 {
                    self.push(Expect::WireNode)?;
                }
                self.push(Expect::WireNode)?;
                Ok(Some(RetainedValueToken::WirePresence(byte)))
            }
            Expect::WireNode => {
                if byte & !3 != 0 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "wire-node", offset, detail: "invalid bits" });
                }
                if byte & 2 != 0 {
                    self.string()?;
                }
                if byte & 1 != 0 {
                    self.string()?;
                }
                self.string()?;
                Ok(Some(RetainedValueToken::WireNodePresence(byte)))
            }
            Expect::WireLabel => {
                if byte & !3 != 0 {
                    return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "wire-label", offset, detail: "invalid bits" });
                }
                if byte & 2 != 0 {
                    self.string()?;
                }
                if byte & 1 != 0 {
                    self.string()?;
                }
                Ok(Some(RetainedValueToken::WireLabelPresence(byte)))
            }
            Expect::TablePresence(columns, rows, depth) => {
                match byte {
                    0 => self.push(Expect::TableElem(rows, rows, columns, depth))?,
                    1 => self.push(Expect::TableBitmap(rows.div_ceil(8), 0, rows, 0, columns, depth))?,
                    _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "table-presence", offset, detail: "expected 0 or 1" }),
                }
                Ok(Some(RetainedValueToken::TablePresence { rows, value: byte }))
            }
            Expect::TableBitmap(bytes, row, rows, mut present, columns, depth) => {
                let used = (rows - row).min(8) as u8;
                let mask = if used == 8 { u8::MAX } else { ((1u16 << used) - 1) as u8 };
                if byte & !mask != 0 {
                    return Err(PackRefusal::NonCanonical("table bitmap padding bits"));
                }
                present += (byte & mask).count_ones() as u64;
                if bytes > 1 {
                    self.push(Expect::TableBitmap(bytes - 1, row + 8, rows, present, columns, depth))?;
                } else {
                    self.push(Expect::TableElem(present, rows, columns, depth))?;
                }
                Ok(Some(RetainedValueToken::TableBitmap { first_row: row, value: byte }))
            }
            Expect::TableElem(present, rows, columns, depth) => {
                self.push(Expect::TableColumns(columns, rows, depth))?;
                self.push(match byte {
                    ELEM_FALLBACK => Expect::Values(present, depth, RetainedContext::Field),
                    ELEM_BOOL => Expect::Bytes(rows.div_ceil(8)),
                    ELEM_INT => Expect::Varints(present, RetainedValueRole::Integer),
                    ELEM_UINT => Expect::Varints(present, RetainedValueRole::Unsigned),
                    ELEM_F64 => Expect::F64s(present),
                    ELEM_STR => Expect::Varints(present, RetainedValueRole::Symbol),
                    ELEM_ENUM => Expect::Varints(present, RetainedValueRole::Enum),
                    _ => return Err(PackRefusal::RetainedMalformed { kind: ValueRefusalKind::InvalidValue, what: "table-element", offset, detail: "unknown tag" }),
                })?;
                Ok(Some(RetainedValueToken::Tag { offset, value: byte }))
            }
            _ => unreachable!(),
        }
    }

    pub fn grant(&mut self) -> Result<Option<RetainedValueToken>, PackRefusal> {
        if let Some(fault) = self.fault.clone() {
            return Err(fault);
        }
        if self.closed || self.closing {
            return Err(PackRefusal::RetainedMalformed { kind:ValueRefusalKind::InvariantViolated,what: "retained-value", offset: self.offset, detail: "grant after close" });
        }
        match self.initialize_root() {
            Ok(true) => return Ok(None),
            Ok(false) => {}
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                return Err(self.fault.clone().expect("retained value first fault"));
            }
        }
        if self.initialized_roots != 2 {
            return Ok(None);
        }
        match self.grant_inner() {
            Ok(value) => Ok(value),
            Err(fault) => {
                self.fault.get_or_insert_with(|| fault.clone());
                Err(self.fault.clone().expect("retained value first fault"))
            }
        }
    }

    pub fn next_release_allocation_bytes(&self) -> Option<usize> {
        if self.pending.is_some() || !self.stack.is_empty() {
            return None;
        }
        let allocated = self.stack_allocation_bytes();
        (allocated != 0).then_some(allocated)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_pack_format_retained::RetainedPackCloseStep, PackRefusal> {
        if !self.closed && maximum_items == 0 && maximum_bytes == 0 {
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if maximum_items == 0 && (self.pending.is_some() || !self.stack.is_empty()) {
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.closing = true;
        if maximum_items != 0 {
            if self.pending.take().is_some() {
                return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.stack.pop().is_some() {
                if self.stack.is_empty() {
                    self.initialized_roots = 0;
                }
                return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        if self.pending.is_some() || !self.stack.is_empty() {
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.initialized_roots = 0;
        let allocated_bytes = self.stack_allocation_bytes();
        if allocated_bytes != 0 {
            if maximum_bytes < allocated_bytes {
                return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            *self.stack = Vec::new();
            return Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Pending { released_items: 0, released_bytes: allocated_bytes });
        }
        self.closed = true;
        Ok(semio_framework_pack_format_retained::RetainedPackCloseStep::Complete)
    }

    /// 🚦️ Whether [`Self::admit_byte`] would accept another byte right now. One admitted byte can
    /// take SEVERAL [`Self::grant`] turns to be consumed — every control frame on the expectation
    /// stack (`Expect::Record`, `Expect::Map`, `Expect::End`, …) is popped without touching the
    /// pending slot — so a producer that admits on a fixed cadence rather than on this predicate
    /// hands back a byte the cursor is still holding.
    pub fn ingress_ready(&self) -> bool {
        !self.closed && !self.closing && self.fault.is_none() && !self.sealed && self.initialized_roots == 2 && !self.stack.is_empty() && self.pending.is_none()
    }

    /// ⏳️ Reports validated bytes independently of admitted pending ingress.
    pub fn consumed_bytes(&self)->u64{self.offset}

    pub fn progress(&self) -> RetainedValueProgress {
        RetainedValueProgress {
            frames: self.stack.len(),
            frame_capacity: self.stack.capacity(),
            allocated_bytes: self.stack_allocation_bytes(),
            initialized_roots: self.initialized_roots,
            pending_input: self.pending.is_some(),
            closed: self.closed,
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.stack.is_empty() && self.stack.capacity() == 0 && self.stack_allocation_bytes() == 0 && self.initialized_roots == 0 && self.pending.is_none()
    }
}

impl Drop for RetainedValueCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained value cursor reached Drop before terminal-empty close");
    }
}
//#endregion 🔖️RetainedValue

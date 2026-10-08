//! 🎒️ Retained canonical record symbols and ordinal emission from an immutable original source.
use super::*;
use semio_framework_dsl_record::{BorrowedRecordSpec, BorrowedShape as B};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as V};
use semio_framework_value::{list::PagedList, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};

const SYMBOL_LIMIT: usize = isize::MAX as usize;

#[derive(Clone, Copy)]
struct Symbol { path: [usize; 64], depth: usize, occurrences: usize, selected: bool, forced: bool }
impl Symbol {
    fn text<'a>(&self, source: &'a dyn FieldProjectionSource) -> Result<&'a str, ValueError> {
        match source.projection_view(&self.path[..self.depth])? { V::Text(text) | V::IntrinsicText(text) => Ok(text), _ => Err(invalid("Pack symbol source changed")) }
    }
}

#[derive(Clone, Copy, Default)]
struct Frame { phase: u8, index: usize, count: usize, ordinal: usize, previous: u32, candidate: u32, flags: u8, packed: u8, length: usize, offset: usize, rank: usize, shape: Option<B>, record: Option<BorrowedRecordSpec>, table_row: bool, force_text: bool }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Discover, Find, Reserve, Insert, Select, SymbolsCount, Symbols, SymbolBytes, Body, Complete, Closing, Closed }

/// 📏️ Exact caller-funded bytes and one bounded structural event from the retained codec.
#[derive(Default, Debug)]
pub struct BorrowedProjectedPackProgress { pub progress: RetainedCloneProgress, pub written_bytes: usize, pub complete: bool }

/// 🧭️ Locators own no text; symbol pages remain retained until their whole physical close grant.
pub struct BorrowedProjectedPackCursor {
    identity: Option<usize>, root_spec: Option<BorrowedRecordSpec>, intrinsic_field: Option<u16>, phase: Phase, symbols: PagedList<Symbol, SYMBOL_LIMIT>,
    frames: [Frame; 64], path: [usize; 64], depth: usize,
    note: Option<Symbol>, search: usize, compare_offset: usize, insertion: usize,
    selected: usize, symbol_index: usize, symbol_offset: usize,
    pending: [u8; 24], pending_length: usize, pending_offset: usize,
}

impl Default for BorrowedProjectedPackCursor {
    fn default() -> Self { Self { identity: None, root_spec: None, intrinsic_field: None, phase: Phase::Discover, symbols: PagedList::empty(), frames: [Frame::default(); 64], path: [0; 64], depth: 0, note: None, search: 0, compare_offset: 0, insertion: 0, selected: 0, symbol_index: 0, symbol_offset: 0, pending: [0; 24], pending_length: 0, pending_offset: 0 } }
}

fn invalid(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }
fn page_error(error: semio_framework_value::list::PagedListError) -> ValueError { ValueError::new(match error.kind { semio_framework_value::list::PagedListRefusalKind::AllocationFailed => ValueRefusalKind::AllocationFailed, semio_framework_value::list::PagedListRefusalKind::OwnershipLimit => ValueRefusalKind::OwnershipLimit, _ => ValueRefusalKind::InvariantViolated }, error.reason) }

impl BorrowedProjectedPackCursor {
    pub fn is_complete(&self) -> bool { self.phase == Phase::Complete && self.pending_offset == self.pending_length }
    pub fn terminal_is_empty(&self) -> bool { self.phase == Phase::Closed && self.symbols.terminal_is_empty() }
    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.phase == Phase::Reserve { self.symbols.next_allocation_bytes().map_err(page_error) } else { Ok(0) } }
    pub fn next_minimum_copy_bytes(&self) -> usize {
        if self.pending_offset < self.pending_length { return 1; }
        match self.phase { Phase::Find => 2, Phase::SymbolBytes => 1, Phase::Body => if matches!(self.frames[self.depth].phase, 7 | 18) { 2 } else { 1 }, _ => 0 }
    }
    pub fn next_close_byte_demand(&self) -> Result<usize, ValueError> { if !self.symbols.is_empty() { Ok(0) } else { self.symbols.next_release_allocation_bytes().map_err(page_error) } }

    fn queue(&mut self, bytes: &[u8]) { self.pending[..bytes.len()].copy_from_slice(bytes); self.pending_length = bytes.len(); self.pending_offset = 0; }
    fn queue_number(&mut self, tag: Option<u8>, value: u64) {
        self.pending_length = 0; self.pending_offset = 0;
        if let Some(tag) = tag { self.pending[0] = tag; self.pending_length = 1; }
        self.append_number(value);
    }
    fn append_number(&mut self, mut value: u64) {
        loop { let byte = (value & 127) as u8; value >>= 7; self.pending[self.pending_length] = byte | if value == 0 { 0 } else { 128 }; self.pending_length += 1; if value == 0 { break; } }
    }
    fn queue_float(&mut self, value: f64, tag: bool) { let mut bytes = [0; 9]; let start = usize::from(tag); if tag { bytes[0] = TAG_F64; } bytes[start..start + 8].copy_from_slice(&value.to_le_bytes()); self.queue(&bytes[..start + 8]); }
    fn push(&mut self, ordinal: usize) -> Result<(), ValueError> {
        let parent = self.frames[self.depth];
        let (shape, table_row) = if let Some(B::Table(make)) = parent.shape { (Some(B::Record(make)), true) }
        else if let Some(record) = parent.record { (Some(record.fields.get(ordinal).ok_or_else(|| invalid("Pack source ordinal exceeds declared fields"))?.shape), false) }
        else { (match parent.shape { Some(B::List(make) | B::Tuple(make, _) | B::Block(make)) => Some(make()), Some(B::Coord(_) | B::Dim(_) | B::Dir | B::Range) => Some(B::Float), _ => None }, false) };
        if self.depth + 1 == 64 { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "borrowed Pack exceeds inline depth")); }
        self.path[self.depth] = ordinal; self.depth += 1;
        self.frames[self.depth] = Frame { shape, table_row, force_text: parent.table_row && matches!(shape, Some(B::Text | B::Ref(_))), ..Default::default() }; Ok(())
    }
    fn table_cell<'a>(&mut self, source: &'a dyn FieldProjectionSource, row: usize, column: usize) -> Result<V<'a>, ValueError> {
        if self.depth + 2 >= 64 { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "Pack Table exceeds inline depth")); }
        self.path[self.depth] = row;
        let V::Record(ids) = source.projection_view(&self.path[..self.depth + 1])? else { return Err(invalid("Pack Table row changed declared Record shape")); };
        let spec = self.frames[self.depth].record.ok_or_else(|| invalid("Pack Table lost declared columns"))?;
        if ids.len() != spec.fields.len() || ids.get(column).copied() != spec.fields.get(column).map(|field|field.id) { return Err(invalid("Pack Table column ordinal changed declared identity")); }
        self.path[self.depth + 1] = column;
        source.projection_view(&self.path[..self.depth + 2])
    }
    fn table_tag(shape: B) -> u8 { match shape { B::Bool=>ELEM_BOOL,B::Int=>ELEM_INT,B::UInt|B::Count=>ELEM_UINT,B::Float|B::Quantity(_)|B::Angle(_)=>ELEM_F64,B::Text|B::Ref(_)=>ELEM_STR,B::Enum(_)=>ELEM_ENUM,_=>ELEM_FALLBACK } }
    fn pop(&mut self) { if self.depth == 0 { self.phase = if self.phase == Phase::Discover { Phase::Select } else { Phase::Complete }; self.symbol_index = 0; } else { self.depth -= 1; } }

    fn compare(&mut self, left: &[u8], right: &[u8], maximum: usize) -> Option<(Option<Ordering>, usize)> {
        let remaining = left.len().min(right.len()).saturating_sub(self.compare_offset);
        if remaining == 0 { self.compare_offset = 0; return Some((Some(left.len().cmp(&right.len())), 0)); }
        let count = (maximum / 2).min(32).min(remaining);
        if count == 0 { return None; }
        let offset = self.compare_offset;
        let order = left[offset..offset + count].cmp(&right[offset..offset + count]);
        self.compare_offset += count;
        let finished = order != Ordering::Equal || self.compare_offset == left.len().min(right.len());
        if finished { self.compare_offset = 0; Some((Some(if order == Ordering::Equal { left.len().cmp(&right.len()) } else { order }), count * 2)) }
        else { Some((None, count * 2)) }
    }

    fn declare(&mut self, view: &V<'_>) -> Result<(), ValueError> {
        if self.intrinsic_field.is_some() {
            return if matches!(view,V::IntrinsicNull|V::IntrinsicBool(_)|V::IntrinsicNumber(_)|V::IntrinsicText(_)|V::IntrinsicBytes(_)|V::IntrinsicArray(_)|V::IntrinsicObject(_)) { Ok(()) } else { Err(invalid("intrinsic Pack source changed its declared value authority")) };
        }
        let frame = self.frames[self.depth];
        if let V::Record(ids) = view {
            let record = if self.depth == 0 { self.root_spec } else { match frame.shape { Some(B::Record(make)) => Some(make()), _ => None } }.ok_or_else(|| invalid("Pack Record lacks declared Record authority"))?;
            if ids.len() != record.fields.len() || ids.len() > 256 { return Err(invalid("Pack Record source field count disagrees with declared authority")); }
            self.frames[self.depth].record = Some(record);
            return Ok(());
        }
        if let (Some(B::Table(make)), V::List(_)) = (frame.shape, view) {
            let record=make(); if record.fields.len()>256 { return Err(invalid("Pack Table exceeds declared column bound")); }
            self.frames[self.depth].record=Some(record); return Ok(());
        }
        let valid = matches!((frame.shape, view),
            (_, V::Absent) | (Some(B::Bool), V::Bool(_)) | (Some(B::Int), V::Int(_)) |
            (Some(B::UInt | B::Count), V::UInt(_)) | (Some(B::Float | B::Quantity(_) | B::Angle(_)), V::Float(_)) |
            (Some(B::Text | B::Ref(_)), V::Text(_)) | (Some(B::Bytes64), V::Bytes(_)) | (Some(B::Enum(_)), V::Enum(_)) |
            (Some(B::Block(_)), V::Block) | (Some(B::List(_)), V::List(_)) | (Some(B::Tuple(_, _) | B::Coord(_) | B::Dim(_) | B::Dir | B::Range), V::Tuple(_)));
        if valid { Ok(()) } else { Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "borrowed Pack requires its exact literal Record/scalar/sequence shape authority")) }
    }

    fn discover(&mut self, source: &dyn FieldProjectionSource) -> Result<(), ValueError> {
        let frame = self.frames[self.depth];
        let view = source.projection_view(&self.path[..self.depth])?;
        if frame.phase == 0 {
            self.declare(&view)?;
            match view {
                V::Text(_) | V::IntrinsicText(_) => { self.note = Some(Symbol { path: self.path, depth: self.depth, occurrences: 1, selected: false, forced: frame.force_text }); self.frames[self.depth].phase=1; self.pop(); self.phase = Phase::Find; self.search = 0; self.compare_offset = 0; }
                V::Block => { self.frames[self.depth].phase=1;self.frames[self.depth].length=1; }
                V::Record(ids) => { self.frames[self.depth].phase = 1; self.frames[self.depth].length = ids.len(); }
                V::List(length) | V::Tuple(length) | V::IntrinsicArray(length) | V::IntrinsicObject(length) => { self.frames[self.depth].phase = 1; self.frames[self.depth].length = length; }
                V::Absent | V::Bool(_) | V::Int(_) | V::UInt(_) | V::Float(_) | V::Enum(_) | V::Bytes(_) | V::IntrinsicNull | V::IntrinsicBool(_) | V::IntrinsicNumber(_) | V::IntrinsicBytes(_) => self.pop(),
                _ => return Err(invalid("borrowed Pack cursor has no declared projection authority for this shape")),
            }
        } else if frame.index < frame.length { self.frames[self.depth].index += 1; self.push(frame.index)?; }
        else { self.pop(); }
        Ok(())
    }

    /// ✍️ Performs one structural event, one physical birth, or at most sixty-four source/output bytes.
    pub fn advance(&mut self, source: &dyn FieldProjectionSource, spec: BorrowedRecordSpec, output: &mut [u8], grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, ValueError> {
        self.advance_root(source,Some(spec),None,output,grant)
    }

    /// 🌱️ Emits one declared intrinsic Body field, preserving original member order and duplicates.
    pub fn advance_intrinsic(&mut self, source: &dyn FieldProjectionSource, field_id: u16, output: &mut [u8], grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, ValueError> {
        self.advance_root(source,None,Some(field_id),output,grant)
    }

    fn advance_root(&mut self, source: &dyn FieldProjectionSource, spec: Option<BorrowedRecordSpec>, field_id: Option<u16>, output: &mut [u8], grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, ValueError> {
        if matches!(self.phase, Phase::Closing | Phase::Closed) { return Err(invalid("borrowed Pack cursor is closing")); }
        let identity = source as *const dyn FieldProjectionSource as *const () as usize;
        if self.identity.is_some_and(|expected| expected != identity) { return Err(invalid("borrowed Pack original source identity changed")); }
        if self.identity.is_some() && (self.intrinsic_field != field_id || self.root_spec.is_some() != spec.is_some()) { return Err(invalid("borrowed Pack root framing authority changed")); }
        if self.root_spec.is_some_and(|original| spec.is_none_or(|spec|original.fields.as_ptr() != spec.fields.as_ptr() || original.fields.len() != spec.fields.len())) { return Err(invalid("borrowed Pack declared root fields changed")); }
        if self.is_complete() { return Ok(BorrowedProjectedPackProgress { complete: true, ..Default::default() }); }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < self.next_minimum_copy_bytes() { return Ok(Default::default()); }
        if grant.maximum_depth < 64 { return Err(ValueError::new(ValueRefusalKind::DepthLimit, "borrowed Pack requires its declared inline depth")); }
        self.identity = Some(identity); self.root_spec = spec; self.intrinsic_field = field_id;
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let maximum = grant.maximum_copy_bytes.min(64);
        if self.pending_offset < self.pending_length {
            let count = output.len().min(maximum).min(self.pending_length - self.pending_offset);
            if count == 0 { return Ok(Default::default()); }
            output[..count].copy_from_slice(&self.pending[self.pending_offset..self.pending_offset + count]); self.pending_offset += count; progress.copied_bytes = count;
            return Ok(BorrowedProjectedPackProgress { progress, written_bytes: count, complete: self.is_complete() });
        }
        match self.phase {
            Phase::Discover => self.discover(source)?,
            Phase::Find => {
                let note = self.note.ok_or_else(|| invalid("borrowed Pack lost its pending text locator"))?;
                if self.search == self.symbols.len() { self.insertion = self.search; self.phase = Phase::Reserve; }
                else {
                    let left = note.text(source)?.as_bytes(); let right = self.symbols[self.search].text(source)?.as_bytes();
                    let Some((order, bytes)) = self.compare(left, right, maximum) else { return Ok(Default::default()); };
                    progress.copied_bytes = bytes;
                    if let Some(order) = order { match order { Ordering::Less => { self.insertion = self.search; self.phase = Phase::Reserve; }, Ordering::Greater => self.search += 1, Ordering::Equal => { self.symbols[self.search].occurrences = self.symbols[self.search].occurrences.checked_add(1).ok_or_else(|| invalid("Pack symbol occurrence overflow"))?; self.symbols[self.search].forced |= note.forced; self.note = None; self.phase = Phase::Discover; } } }
                }
            }
            Phase::Reserve => {
                if !self.symbols.has_reserved_slot() {
                    let required = self.symbols.next_allocation_bytes().map_err(page_error)?;
                    if required > grant.maximum_capacity_bytes { return Ok(Default::default()); }
                    let step = self.symbols.reserve_one(required).map_err(|error| page_error(error.refusal()))?;
                    progress.retained_capacity_bytes = step.allocated_bytes;
                } else { self.symbols.push_reserved(self.note.ok_or_else(|| invalid("Pack insertion lost text locator"))?).map_err(|_| invalid("Pack symbol reserved slot rejected"))?; self.search = self.symbols.len() - 1; self.phase = Phase::Insert; }
            }
            Phase::Insert => {
                if self.search > self.insertion { let previous = self.symbols[self.search - 1]; self.symbols[self.search - 1] = self.symbols[self.search]; self.symbols[self.search] = previous; self.search -= 1; }
                else { self.note = None; self.phase = Phase::Discover; }
            }
            Phase::Select => {
                if self.symbol_index < self.symbols.len() { let selected = self.symbols[self.symbol_index].forced || self.symbols[self.symbol_index].text(source)?.len() <= 128 || self.symbols[self.symbol_index].occurrences >= 2; self.symbols[self.symbol_index].selected = selected; self.selected += usize::from(selected); self.symbol_index += 1; }
                else { self.phase = Phase::SymbolsCount; }
            }
            Phase::SymbolsCount => { self.queue_number(None, self.selected as u64); self.symbol_index = 0; self.phase = Phase::Symbols; }
            Phase::Symbols => {
                if self.symbol_index == self.symbols.len() {
                    self.frames = [Frame::default(); 64]; self.depth = 0; self.phase = Phase::Body;
                    if let Some(field)=self.intrinsic_field { self.queue_number(None,1);self.append_number(u64::from(field));self.pending[self.pending_length]=TAG_VALUE;self.pending_length+=1; }
                }
                else if self.symbols[self.symbol_index].selected { self.queue_number(None, self.symbols[self.symbol_index].text(source)?.len() as u64); self.symbol_offset = 0; self.phase = Phase::SymbolBytes; }
                else { self.symbol_index += 1; }
            }
            Phase::SymbolBytes => {
                let bytes = self.symbols[self.symbol_index].text(source)?.as_bytes();
                let count = output.len().min(maximum).min(bytes.len() - self.symbol_offset);
                if count == 0 && self.symbol_offset < bytes.len() { return Ok(Default::default()); }
                output[..count].copy_from_slice(&bytes[self.symbol_offset..self.symbol_offset + count]); self.symbol_offset += count; progress.copied_bytes = count;
                if self.symbol_offset == bytes.len() { self.symbol_index += 1; self.phase = Phase::Symbols; }
                return Ok(BorrowedProjectedPackProgress { progress, written_bytes: count, complete: false });
            }
            Phase::Body => { return self.body(source, output, maximum, progress); }
            _ => unreachable!(),
        }
        Ok(BorrowedProjectedPackProgress { progress, written_bytes: 0, complete: self.is_complete() })
    }

    fn body(&mut self, source: &dyn FieldProjectionSource, output: &mut [u8], maximum: usize, mut progress: RetainedCloneProgress) -> Result<BorrowedProjectedPackProgress, ValueError> {
        let frame = self.frames[self.depth];
        let view = source.projection_view(&self.path[..self.depth])?;
        if frame.phase == 0 {
            self.declare(&view)?;
            match view {
                V::Block => { self.queue(&[TAG_BLOCK]);self.frames[self.depth].phase=20; }
                V::Record(_) => { if self.depth != 0 { self.queue(&[TAG_RECORD]); } self.frames[self.depth].phase = 1; }
                V::List(length) if matches!(frame.shape, Some(B::Table(_))) => { self.queue_number(Some(TAG_TABLE_SOA),length as u64); self.frames[self.depth].phase=10; self.frames[self.depth].length=length; }
                V::List(length) | V::Tuple(length) => { self.frames[self.depth].phase = 4; self.frames[self.depth].length = length; self.frames[self.depth].flags = if length == 0 { 0 } else { 3 }; }
                V::IntrinsicArray(length) => { self.queue_number(Some(TAG_LIST),length as u64);self.frames[self.depth].phase=5;self.frames[self.depth].length=length; }
                V::IntrinsicObject(length) => { self.queue_number(Some(TAG_MAP),length as u64);self.frames[self.depth].phase=21;self.frames[self.depth].length=length; }
                V::Text(_) | V::IntrinsicText(_) => { self.frames[self.depth].phase = 7; self.compare_offset = 0; }
                V::Bytes(bytes) | V::IntrinsicBytes(bytes) => { self.queue_number(Some(TAG_BYTES), bytes.len() as u64); self.frames[self.depth].phase = 8; }
                V::Bool(value) | V::IntrinsicBool(value) => { self.queue(&[if value { TAG_TRUE } else { TAG_FALSE }]); self.pop(); }
                V::IntrinsicNull => { self.queue(&[TAG_NULL]);self.pop(); }
                V::Absent => { self.queue(&[TAG_ABSENT]); self.pop(); }
                V::Float(value) => { self.queue_float(value, true); self.pop(); }
                V::UInt(value) => { self.queue_number(Some(TAG_UINT), value); self.pop(); }
                V::Int(value) => { self.queue_number(Some(TAG_INT), ((value as u64) << 1) ^ ((value >> 63) as u64)); self.pop(); }
                V::Enum(value) => { self.queue_number(Some(TAG_ENUM), u64::from(value)); self.pop(); }
                V::IntrinsicNumber(value) => { match value { Number::UInt(value)=>self.queue_number(Some(TAG_UINT),value),Number::Int(value)=>self.queue_number(Some(TAG_INT),((value as u64)<<1)^((value>>63)as u64)),Number::Float(value)=>self.queue_float(value,true) };self.pop(); }
                _ => return Err(invalid("borrowed Pack body shape has no codec authority")),
            }
        } else { match frame.phase {
            1 => {
                let V::Record(ids) = view else { return Err(invalid("Pack record changed shape")); };
                if frame.index < ids.len() { self.path[self.depth] = frame.index; if !matches!(source.projection_view(&self.path[..self.depth + 1])?, V::Absent) { self.frames[self.depth].count += 1; } self.frames[self.depth].index += 1; }
                else { self.queue_number(None, frame.count as u64); self.frames[self.depth].phase = 2; self.frames[self.depth].index = 0; self.frames[self.depth].previous = 0; self.frames[self.depth].candidate = u32::MAX; }
            }
            2 => {
                let V::Record(ids) = view else { return Err(invalid("Pack record changed shape")); };
                if frame.ordinal == frame.count { self.pop(); }
                else if frame.index < ids.len() {
                    let id = u32::from(ids[frame.index]);
                    if frame.record.ok_or_else(|| invalid("Pack Record schema disappeared"))?.fields[frame.index].id != ids[frame.index] { return Err(invalid("Pack source field ordinal changed declared identity")); }
                    self.path[self.depth] = frame.index;
                    if (frame.ordinal == 0 || id > frame.previous) && id < frame.candidate && !matches!(source.projection_view(&self.path[..self.depth + 1])?, V::Absent) { self.frames[self.depth].candidate = id; self.frames[self.depth].offset = frame.index; }
                    self.frames[self.depth].index += 1;
                } else {
                    if frame.candidate == u32::MAX { return Err(invalid("Pack record has duplicate field IDs")); }
                    self.queue_number(None, u64::from(frame.candidate)); self.frames[self.depth].phase = 3;
                }
            }
            3 => { self.frames[self.depth].ordinal += 1; self.frames[self.depth].previous = frame.candidate; self.frames[self.depth].candidate = u32::MAX; self.frames[self.depth].index = 0; self.frames[self.depth].phase = 2; self.push(frame.offset)?; }
            4 => {
                if frame.index < frame.length { self.path[self.depth] = frame.index; let child = source.projection_view(&self.path[..self.depth + 1])?; if !matches!(child, V::Float(_)) { self.frames[self.depth].flags &= !1; } if !matches!(child, V::Int(_) | V::Enum(_)) && !matches!(child, V::UInt(value) if value <= i64::MAX as u64) { self.frames[self.depth].flags &= !2; } self.frames[self.depth].index += 1; }
                else { let packed = if frame.flags & 1 != 0 { 1 } else if frame.flags & 2 != 0 { 2 } else { 0 }; let tag = match packed { 1 => TAG_PACKED_F64, 2 => TAG_PACKED_VARINT, _ => if matches!(view, V::Tuple(_)) { TAG_TUPLE } else { TAG_LIST } }; self.queue_number(Some(tag), frame.length as u64); self.frames[self.depth].packed = packed; self.frames[self.depth].phase = 5; self.frames[self.depth].index = 0; }
            }
            5 => {
                if frame.index == frame.length { self.pop(); }
                else { self.frames[self.depth].index += 1; if frame.packed == 0 { self.push(frame.index)?; } else { self.path[self.depth] = frame.index; match source.projection_view(&self.path[..self.depth + 1])? { V::Float(value) if frame.packed == 1 => self.queue_float(value, false), V::Int(value) => self.queue_number(None, ((value as u64) << 1) ^ ((value >> 63) as u64)), V::UInt(value) => self.queue_number(None, value << 1), V::Enum(value) => self.queue_number(None, u64::from(value) << 1), _ => return Err(invalid("Pack numeric list changed its original field kind")) } } }
            }
            7 => {
                let text = match view {V::Text(text)|V::IntrinsicText(text)=>text,_=>return Err(invalid("Pack text changed shape"))};
                if frame.index == self.symbols.len() { self.queue_number(Some(TAG_STR_INLINE), text.len() as u64); self.frames[self.depth].phase = 8; self.frames[self.depth].offset = 0; }
                else {
                    let right = self.symbols[frame.index].text(source)?.as_bytes();
                    let Some((order, bytes)) = self.compare(text.as_bytes(), right, maximum) else { return Ok(Default::default()); };
                    progress.copied_bytes = bytes;
                    if let Some(order) = order {
                        if order == Ordering::Equal && self.symbols[frame.index].selected { self.queue_number(Some(TAG_STR), frame.rank as u64); self.pop(); }
                        else if order == Ordering::Equal { self.queue_number(Some(TAG_STR_INLINE), text.len() as u64); self.frames[self.depth].phase = 8; self.frames[self.depth].offset = 0; }
                        else { self.frames[self.depth].index += 1; self.frames[self.depth].rank += usize::from(self.symbols[frame.index].selected); }
                    }
                }
            }
            8 => {
                let bytes = match view { V::Text(text) | V::IntrinsicText(text) => text.as_bytes(), V::Bytes(bytes) | V::IntrinsicBytes(bytes) => bytes, _ => return Err(invalid("Pack payload changed its original field kind")) };
                let count = output.len().min(maximum).min(bytes.len() - frame.offset);
                if count == 0 && frame.offset < bytes.len() { return Ok(Default::default()); }
                output[..count].copy_from_slice(&bytes[frame.offset..frame.offset + count]); self.frames[self.depth].offset += count; progress.copied_bytes = count;
                if self.frames[self.depth].offset == bytes.len() { self.pop(); }
                return Ok(BorrowedProjectedPackProgress { progress, written_bytes: count, complete: self.is_complete() });
            }
            10 => { self.queue_number(None,frame.record.ok_or_else(||invalid("Pack Table columns disappeared"))?.fields.len() as u64); self.frames[self.depth].phase=11; self.frames[self.depth].candidate=u32::MAX; }
            11 => {
                let fields=frame.record.ok_or_else(||invalid("Pack Table columns disappeared"))?.fields;
                if frame.ordinal==fields.len() { self.pop(); }
                else if frame.index<fields.len() { let id=u32::from(fields[frame.index].id); if (frame.ordinal==0 || id>frame.previous) && id<frame.candidate { self.frames[self.depth].candidate=id; self.frames[self.depth].offset=frame.index; } self.frames[self.depth].index+=1; }
                else { if frame.candidate==u32::MAX { return Err(invalid("Pack Table has duplicate column IDs")); } self.frames[self.depth].phase=12;self.frames[self.depth].index=0;self.frames[self.depth].flags=1; }
            }
            12 => { if frame.index<frame.length { if matches!(self.table_cell(source,frame.index,frame.offset)?,V::Absent) { self.frames[self.depth].flags=0; } self.frames[self.depth].index+=1; } else { self.queue_number(None,u64::from(frame.candidate));self.frames[self.depth].phase=13; } }
            13 => { self.queue(&[u8::from(frame.flags==0)]);self.frames[self.depth].phase=if frame.flags==0 {14}else{16};self.frames[self.depth].index=0;self.frames[self.depth].flags=0; }
            14 => {
                if frame.index==frame.length { self.frames[self.depth].phase=16; }
                else { let mut bits=frame.flags;if !matches!(self.table_cell(source,frame.index,frame.offset)?,V::Absent) {bits|=1<<(frame.index%8);}self.frames[self.depth].index+=1;
                    if frame.index%8==7 || frame.index+1==frame.length { self.queue(&[bits]);self.frames[self.depth].flags=0; } else { self.frames[self.depth].flags=bits; }
                }
            }
            16 => { let tag=Self::table_tag(frame.record.ok_or_else(||invalid("Pack Table columns disappeared"))?.fields[frame.offset].shape);self.queue(&[tag]);self.frames[self.depth].packed=tag;self.frames[self.depth].phase=17;self.frames[self.depth].index=0;self.frames[self.depth].flags=0; }
            17 => {
                if frame.index==frame.length { self.frames[self.depth].ordinal+=1;self.frames[self.depth].previous=frame.candidate;self.frames[self.depth].candidate=u32::MAX;self.frames[self.depth].index=0;self.frames[self.depth].phase=11; }
                else {
                    let cell=self.table_cell(source,frame.index,frame.offset)?;
                    if frame.packed==ELEM_BOOL { let bits=frame.flags | if matches!(cell,V::Bool(true)) {1<<(frame.index%8)}else{0};self.frames[self.depth].index+=1;if frame.index%8==7 || frame.index+1==frame.length {self.queue(&[bits]);self.frames[self.depth].flags=0;}else{self.frames[self.depth].flags=bits;} }
                    else if matches!(cell,V::Absent) {self.frames[self.depth].index+=1;}
                    else { match (frame.packed,cell) {
                        (ELEM_F64,V::Float(value))=>self.queue_float(value,false),
                        (ELEM_INT,V::Int(value))=>self.queue_number(None,((value as u64)<<1)^((value>>63)as u64)),
                        (ELEM_UINT,V::UInt(value))=>self.queue_number(None,value),
                        (ELEM_ENUM,V::Enum(value))=>self.queue_number(None,u64::from(value)),
                        (ELEM_STR,V::Text(_))=>{self.frames[self.depth].phase=18;self.frames[self.depth].count=0;self.frames[self.depth].rank=0;self.compare_offset=0;return Ok(BorrowedProjectedPackProgress{progress,..Default::default()});},
                        (ELEM_FALLBACK,_)=>{self.frames[self.depth].index+=1;self.push(frame.index)?;self.declare(&source.projection_view(&self.path[..self.depth])?)?;self.frames[self.depth].phase=19;self.push(frame.offset)?;return Ok(BorrowedProjectedPackProgress{progress,..Default::default()});},
                        _=>return Err(invalid("Pack Table cell changed declared scalar shape")),
                    } self.frames[self.depth].index+=1; }
                }
            }
            18 => {
                let V::Text(text)=self.table_cell(source,frame.index,frame.offset)? else {return Err(invalid("Pack Table text changed shape"));};
                if frame.count==self.symbols.len() {return Err(invalid("Pack Table text lost forced symbol"));}
                let right=self.symbols[frame.count].text(source)?.as_bytes();let Some((order,bytes))=self.compare(text.as_bytes(),right,maximum) else {return Ok(Default::default());};progress.copied_bytes=bytes;
                if let Some(order)=order { if order==Ordering::Equal {if !self.symbols[frame.count].selected{return Err(invalid("Pack Table symbol was not selected"));}self.queue_number(None,frame.rank as u64);self.frames[self.depth].phase=17;self.frames[self.depth].index+=1;}else{self.frames[self.depth].rank+=usize::from(self.symbols[frame.count].selected);self.frames[self.depth].count+=1;} }
            }
            19 => self.pop(),
            20 => { self.frames[self.depth].phase=19;self.push(0)?; },
            21 => {
                if frame.index==frame.length {self.pop();}
                else {let key=source.projection_key(&self.path[..self.depth],frame.index)?;self.queue_number(Some(TAG_STR_INLINE),key.len()as u64);self.frames[self.depth].phase=22;self.frames[self.depth].offset=0;}
            }
            22 => {
                let bytes=source.projection_key(&self.path[..self.depth],frame.index)?.as_bytes();let count=output.len().min(maximum).min(bytes.len()-frame.offset);
                if count==0&&frame.offset<bytes.len(){return Ok(Default::default());}
                output[..count].copy_from_slice(&bytes[frame.offset..frame.offset+count]);self.frames[self.depth].offset+=count;progress.copied_bytes=count;
                if self.frames[self.depth].offset==bytes.len(){self.frames[self.depth].phase=23;}
                return Ok(BorrowedProjectedPackProgress{progress,written_bytes:count,complete:false});
            }
            23 => {self.frames[self.depth].phase=21;self.frames[self.depth].index+=1;self.push(frame.index)?;},
            _ => return Err(invalid("Pack cursor phase changed")),
        } }
        Ok(BorrowedProjectedPackProgress { progress, written_bytes: 0, complete: self.phase == Phase::Complete && self.pending_offset == self.pending_length })
    }

    /// 📏️ Quotes the original symbol frontier with separate release and structural depth.
    pub fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.phase == Phase::Closed { return Ok(Default::default()); }
        let depth = if !self.symbols.is_empty() { self.symbols.next_pop_depth_demand() } else { self.symbols.next_release_depth_demand() }.map_err(page_error)?;
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.next_close_byte_demand()?, depth: depth.checked_add(1).ok_or_else(|| invalid("Pack retirement depth overflow"))?, ..Default::default() })
    }

    /// 🍂️ Retires one inline locator or one whole paid symbol allocation without source disposal.
    pub fn close(&mut self, grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, ValueError> {
        if self.phase == Phase::Closed { return Ok(BorrowedProjectedPackProgress { complete: true, ..Default::default() }); }
        let demand = self.retirement_demands()?;
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(Default::default()); }
        self.phase = Phase::Closing; self.note = None;
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if !self.symbols.is_empty() { self.symbols.pop(); }
        else if !self.symbols.terminal_is_empty() { let required = self.symbols.next_release_allocation_bytes().map_err(page_error)?; if required > grant.maximum_release_bytes { return Ok(Default::default()); } let step = self.symbols.release_empty_page(required).map_err(page_error)?; progress.released_bytes = step.released_allocation_bytes; }
        else { self.identity = None; self.phase = Phase::Closed; }
        Ok(BorrowedProjectedPackProgress { progress, written_bytes: 0, complete: self.phase == Phase::Closed })
    }
}

impl Drop for BorrowedProjectedPackCursor {
    fn drop(&mut self) { assert!(self.symbols.terminal_is_empty(), "borrowed Pack symbol backing requires its exact terminal-empty close witness"); }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

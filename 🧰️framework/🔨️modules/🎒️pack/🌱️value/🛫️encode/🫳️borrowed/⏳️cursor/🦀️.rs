//! 🎒️ Retained canonical record symbols and ordinal emission from an immutable original source.
use semio_framework_value::{Number, ValueError, ValueRefusalKind};
use std::cmp::Ordering;
use std::mem::size_of;
use semio_framework_pack_record_schema::{ELEM_BOOL, ELEM_ENUM, ELEM_F64, ELEM_FALLBACK, ELEM_INT, ELEM_STR, ELEM_UINT, TAG_ABSENT, TAG_BLOCK, TAG_BYTES, TAG_BYTES_CHUNKED, TAG_ENUM, TAG_EXPR, TAG_F64, TAG_FALSE, TAG_INT, TAG_LIST, TAG_MAP, TAG_NULL, TAG_PACKED_F64, TAG_PACKED_VARINT, TAG_RECORD, TAG_STATEMENTS, TAG_STR, TAG_STR_INLINE, TAG_TABLE_SOA, TAG_TRUE, TAG_TUPLE, TAG_UINT, TAG_VALUE, TAG_WIRE};
#[cfg(test)]
use semio_framework_value::{DslValue, native_encoding::NativeEncodeControl};
use semio_framework_dsl_record::{BorrowedRecordSpec, BorrowedShape as B};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as V};
use semio_framework_dsl_record::native_encoding::RetainedExpressionCursor;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value_persistent::list::PagedList;
use semio_framework_value_paged_text::paged_text::TextReadView;

#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

//#region 🎒️BorrowedDocumentCursor
#[cfg(feature = "deflate")]
#[path = "../🎒️document/⏳️cursor/🦀️.rs"]
mod borrowed_document;
#[cfg(feature = "deflate")]
pub use borrowed_document::{BorrowedPackDocumentCursor, BorrowedPackDocumentFailure, BorrowedPackDocumentProgress};
//#endregion 🎒️BorrowedDocumentCursor

//#region 📨️OwnedWireCursor
#[path = "../📨️wire/🦀️.rs"]
mod owned_wire;
pub use owned_wire::{OwnedProjectedPackWireCursor, OwnedProjectedPackWireReady, OwnedProjectedPackWireStep};
//#endregion 📨️OwnedWireCursor

const SYMBOL_LIMIT: usize = isize::MAX as usize;

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Symbol { path: [usize; 64], depth: usize, occurrences: usize, selected: bool, forced: bool, key: Option<usize> }
#[derive(Clone, Copy, Debug)]
struct ChunkLocator { path: [usize; 64], depth: usize, offset: usize, length: usize }
impl semio_framework_value::retirement::RetireOwned for ChunkLocator{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl semio_framework_value::retirement::RetireOwned for Symbol{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl Symbol {
    fn text<'a>(&self, source: &'a dyn FieldProjectionSource) -> Result<TextReadView<'a>, ValueError> {
        if let Some(key)=self.key{source.projection_key_view(&self.path[..self.depth],key)}else{text_view(source.projection_view(&self.path[..self.depth])?)}
    }
}

#[derive(Clone, Copy, Default)]
struct Frame { phase: u8, index: usize, count: usize, ordinal: usize, previous: u32, candidate: u32, flags: u8, packed: u8, length: usize, offset: usize, rank: usize, shape: Option<B>, record: Option<BorrowedRecordSpec>, table_row: bool, force_text: bool, intrinsic:bool,bare_record:bool }
impl semio_framework_value::retirement::RetireOwned for Frame{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::leaf(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(semio_framework_value::retirement::leaf_birth_bytes::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Initialize, Discover, Find, Reserve, Insert, Select, SymbolsCount, Symbols, SymbolBytes, Reset, Body, Expression, ChunkReserve, Complete, Closing, Closed }

#[derive(Default)]
struct ExpressionState{mode:u8,length:usize,index:usize,offset:usize,rank:usize,different:bool}

/// 🧩️ Canonical document boundaries retain their distinct raw hashing and compression authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum BorrowedPackSection { Discovery, Symbols, Body, Complete, Closing, Closed }

/// 📏️ Exact caller-funded bytes and one bounded structural event from the retained codec.
#[derive(Default, Debug)]
pub struct BorrowedProjectedPackProgress { pub progress: RetainedCloneProgress, pub written_bytes: usize, pub complete: bool }

/// ⚠️ The original Value refusal retains the sole physical receipt and initialized output prefix.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct BorrowedProjectedPackFailure { pub reason: ValueError, pub written_bytes: usize }

impl From<ValueError> for BorrowedProjectedPackFailure {
    fn from(reason: ValueError) -> Self { Self { reason, written_bytes: 0 } }
}

fn failed_turn(reason: ValueError, progress: RetainedCloneProgress) -> BorrowedProjectedPackFailure {
    let performed = progress.checked_add(reason.retained_progress()).expect("one addressable borrowed Pack turn has representable physical progress");
    BorrowedProjectedPackFailure { reason: reason.with_retained_progress(performed), written_bytes: 0 }
}


/// 🧭️ Locators own no text; symbol pages remain retained until their whole physical close grant.
pub struct BorrowedProjectedPackCursor {
    identity: Option<usize>, root_spec: Option<BorrowedRecordSpec>, intrinsic_field: Option<u16>, phase: Phase, symbols: PagedList<Symbol, SYMBOL_LIMIT>,
    frames: PagedList<Frame,64>, path: [usize; 64], depth: usize, reset:usize,
    note: Option<Symbol>, search: usize, compare_offset: usize, compare_left: Option<u8>, insertion: usize,
    selected: usize, symbol_index: usize, symbol_offset: usize,
    pending: [u8; 24], pending_length: usize, pending_offset: usize,
    chunks:PagedList<ChunkLocator,{isize::MAX as usize}>,chunk_note:Option<ChunkLocator>,chunk_index:usize,chunking:bool,replaying:bool,
    maximum_source_depth:usize,
    expression:RetainedExpressionCursor,expression_state:ExpressionState,
}

impl Default for BorrowedProjectedPackCursor {
    fn default() -> Self { Self { identity: None, root_spec: None, intrinsic_field: None, phase: Phase::Initialize, symbols: PagedList::empty(), frames: PagedList::empty(), path: [0; 64], depth: 0, reset:0, note: None, search: 0, compare_offset: 0, compare_left: None, insertion: 0, selected: 0, symbol_index: 0, symbol_offset: 0, pending: [0; 24], pending_length: 0, pending_offset: 0,chunks:PagedList::empty(),chunk_note:None,chunk_index:0,chunking:false,replaying:false,maximum_source_depth:63,expression:Default::default(),expression_state:Default::default() } }
}

fn invalid(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }
fn text_view(view:V<'_>)->Result<TextReadView<'_>,ValueError>{match view{V::Text(text)|V::IntrinsicText(text)=>Ok(TextReadView::from_str(text)),V::TextSource(text)|V::IntrinsicTextSource(text)=>Ok(text),_=>Err(invalid("Pack text source changed"))}}
fn page_error(error: semio_framework_value_persistent::list::PagedListError) -> ValueError { ValueError::literal(match error.kind { semio_framework_value_persistent::list::PagedListRefusalKind::AllocationFailed => ValueRefusalKind::AllocationFailed, semio_framework_value_persistent::list::PagedListRefusalKind::OwnershipLimit => ValueRefusalKind::OwnershipLimit, _ => ValueRefusalKind::InvariantViolated }, error.reason) }

impl BorrowedProjectedPackCursor {
    pub fn set_maximum_source_depth(&mut self,maximum:usize){self.maximum_source_depth=maximum.min(63);}
    pub fn for_document()->Self{let mut cursor=Self::default();cursor.chunking=true;cursor}
    pub fn chunk_count(&self)->usize{self.chunks.len()}
    pub fn chunk_len(&self,index:usize)->Result<usize,ValueError>{self.chunks.get(index).map(|chunk|chunk.length).ok_or_else(||invalid("Pack chunk ordinal lost original source"))}
    pub fn chunk_depth(&self,index:usize)->Result<usize,ValueError>{Ok(self.chunks.get(index).ok_or_else(||invalid("Pack chunk ordinal lost original source"))?.depth.max(self.chunks.next_pop_depth_demand().map_err(page_error)?)+1)}
    pub fn chunk_byte(&self,source:&dyn FieldProjectionSource,index:usize,offset:usize)->Result<u8,ValueError>{let chunk=self.chunks.get(index).ok_or_else(||invalid("Pack chunk ordinal lost original source"))?;if offset>=chunk.length{return Err(invalid("Pack chunk byte exceeds original range"));}match source.projection_view(&chunk.path[..chunk.depth])?{V::Bytes(bytes)|V::IntrinsicBytes(bytes)=>bytes.get(chunk.offset+offset).copied().ok_or_else(||invalid("Pack chunk byte disappeared")),V::BytesSource(bytes)|V::IntrinsicBytesSource(bytes)=>bytes.byte_at(chunk.offset+offset).ok_or_else(||invalid("Pack chunk byte disappeared")),_=>Err(invalid("Pack chunk original shape changed"))}}
    pub fn section(&self)->BorrowedPackSection { match self.phase {Phase::SymbolsCount|Phase::Symbols|Phase::SymbolBytes|Phase::Reset=>BorrowedPackSection::Symbols,Phase::Body|Phase::Expression|Phase::ChunkReserve=>BorrowedPackSection::Body,Phase::Complete if !self.is_complete()=>BorrowedPackSection::Body,Phase::Complete=>BorrowedPackSection::Complete,Phase::Closing=>BorrowedPackSection::Closing,Phase::Closed=>BorrowedPackSection::Closed,_=>BorrowedPackSection::Discovery} }
    pub fn symbol_count(&self)->Option<usize>{(!matches!(self.section(),BorrowedPackSection::Discovery|BorrowedPackSection::Closing|BorrowedPackSection::Closed)).then_some(self.selected)}
    pub fn field_count(&self)->Option<usize>{self.is_complete().then_some(if self.intrinsic_field.is_some(){1}else{self.frames[0].count})}
    /// 🔁️ Replays immutable source bytes from retained canonical symbol locators without another birth.
    pub fn restart_emission(&mut self)->Result<(),ValueError>{if !self.is_complete(){return Err(invalid("Pack replay requires complete raw output custody"));}self.phase=Phase::SymbolsCount;self.symbol_index=0;self.symbol_offset=0;self.pending_length=0;self.pending_offset=0;self.depth=0;self.chunk_index=0;self.replaying=true;Ok(())}
    pub fn is_complete(&self) -> bool { self.phase == Phase::Complete && self.pending_offset == self.pending_length }
    pub fn terminal_is_empty(&self) -> bool { self.phase == Phase::Closed && self.symbols.terminal_is_empty() && self.frames.terminal_is_empty()&&self.chunks.terminal_is_empty()&&self.expression.terminal_is_empty() }
    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.phase==Phase::Expression&&self.expression_state.mode!=1{return self.expression.next_capacity_byte_demand();}if self.phase==Phase::Initialize&&!self.frames.has_reserved_slot(){self.frames.next_allocation_bytes().map_err(page_error)}else if self.phase==Phase::ChunkReserve&&!self.replaying&&!self.chunks.has_reserved_slot(){self.chunks.next_allocation_bytes().map_err(page_error)}else if self.phase == Phase::Reserve { self.symbols.next_allocation_bytes().map_err(page_error) } else { Ok(0) } }
    /// 🪜️ Quotes the actual next original source path or symbol backing without pricing authority.
    pub fn next_advance_depth_demand(&self) -> Result<usize,ValueError> {
        let source_depth=self.depth.checked_add(1).ok_or_else(||invalid("Pack source depth overflow"))?;
        let symbol_depth=|index:usize|->Result<usize,ValueError>{
            let original=self.symbols.get(index).ok_or_else(||invalid("Pack symbol depth lost original locator"))?;
            let source=original.depth.checked_add(1).ok_or_else(||invalid("Pack symbol source depth overflow"))?;
            let backing=self.symbols.next_pop_depth_demand().map_err(page_error)?.checked_add(1).ok_or_else(||invalid("Pack symbol backing depth overflow"))?;
            Ok(source.max(backing))
        };
        if self.pending_offset<self.pending_length{return Ok(1);}
        let source=match self.phase {
            Phase::Initialize=>if self.frames.has_reserved_slot(){self.frames.next_pop_depth_demand().map_err(page_error)?+1}else{self.frames.next_reserve_depth_demand().map_err(page_error)?+1},
            Phase::Reset=>self.frames.next_pop_depth_demand().map_err(page_error)?+1,
            Phase::ChunkReserve=>if self.replaying||self.chunks.has_reserved_slot(){self.chunks.next_pop_depth_demand().map_err(page_error)?+1}else{self.chunks.next_reserve_depth_demand().map_err(page_error)?+1},
            Phase::Discover=>source_depth+usize::from(self.frames[self.depth].phase!=0&&self.frames[self.depth].index<self.frames[self.depth].length),
            Phase::Find if self.search<self.symbols.len()=>symbol_depth(self.search)?.max(self.note.ok_or_else(||invalid("Pack depth lost pending text locator"))?.depth+1),
            Phase::Reserve=>self.symbols.next_reserve_depth_demand().map_err(page_error)?.checked_add(1).ok_or_else(||invalid("Pack symbol reservation depth overflow"))?,
            Phase::Insert=>self.symbols.next_pop_depth_demand().map_err(page_error)?.checked_add(1).ok_or_else(||invalid("Pack symbol insertion depth overflow"))?,
            Phase::Select|Phase::Symbols|Phase::SymbolBytes if self.symbol_index<self.symbols.len()=>symbol_depth(self.symbol_index)?,
            Phase::Expression=>{let child=source_depth+self.expression.next_depth_demand()?;if self.expression_state.mode!=0&&self.expression_state.index<self.symbols.len(){child.max(symbol_depth(self.expression_state.index)?)}else{child}},
            Phase::Body=>{
                let frame=self.frames[self.depth];
                let child=match frame.phase {10..=18|30..=59=>source_depth+2,1..=5|20|23=>source_depth+1,_=>source_depth};
                let symbol=match frame.phase {7 if frame.index<self.symbols.len()=>symbol_depth(frame.index)?,18 if frame.count<self.symbols.len()=>symbol_depth(frame.count)?,_=>0};
                child.max(symbol)
            },
            Phase::Complete|Phase::Closed=>0,
            _=>1,
        };Ok(source.max(self.frames.next_pop_depth_demand().map_err(page_error)?+1))
    }

    pub fn next_minimum_copy_bytes(&self) -> usize {
        if self.pending_offset < self.pending_length { return 1; }
        match self.phase { Phase::Expression=>if self.expression_state.mode==1{1}else{self.expression.next_copy_byte_demand().unwrap_or(usize::MAX).saturating_add(1)},Phase::Initialize if !self.frames.has_reserved_slot()=>self.frames.next_reserve_copy_byte_demand().unwrap_or(usize::MAX),Phase::Initialize|Phase::Reset=>size_of::<Frame>(),Phase::Reserve if !self.symbols.has_reserved_slot()=>self.symbols.next_reserve_copy_byte_demand().unwrap_or(usize::MAX),Phase::Reserve=>size_of::<Symbol>(),Phase::Insert if self.search>self.insertion=>3*size_of::<Symbol>(),Phase::Discover if self.frames[self.depth].phase==31&&matches!(self.frames[self.depth].shape,Some(B::Statements(_)))=>1,Phase::ChunkReserve if !self.replaying&&!self.chunks.has_reserved_slot()=>self.chunks.next_reserve_copy_byte_demand().unwrap_or(usize::MAX),Phase::ChunkReserve=>size_of::<ChunkLocator>(),Phase::Body if self.frames[self.depth].phase==60=>size_of::<ChunkLocator>(),Phase::Find | Phase::SymbolBytes | Phase::Body => 1, _ => 0 }
    }
    pub fn next_close_byte_demand(&self) -> Result<usize, ValueError> { if !self.expression.terminal_is_empty(){return Ok(self.expression.retirement_demands()?.release_bytes);}if !self.symbols.terminal_is_empty(){if !self.symbols.is_empty(){Ok(0)}else{self.symbols.next_release_allocation_bytes().map_err(page_error)}}else if !self.frames.is_empty(){Ok(0)}else if !self.frames.terminal_is_empty(){self.frames.next_release_allocation_bytes().map_err(page_error)}else if !self.chunks.is_empty(){Ok(0)}else{self.chunks.next_release_allocation_bytes().map_err(page_error)} }

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
        else if let Some(record) = parent.record.filter(|_|!matches!(parent.shape,Some(B::Statements(_)))) { (Some(record.fields.get(ordinal).ok_or_else(|| invalid("Pack source ordinal exceeds declared fields"))?.shape), false) }
        else { (match parent.shape { Some(B::List(make) | B::Tuple(make, _) | B::Block(make)|B::Map(make)) => Some(make()), Some(B::Coord(_) | B::Dim(_) | B::Dir | B::Range) => Some(B::Float),Some(B::Wire)if ordinal<8=>Some(B::Text), _ => None }, false) };
        if self.depth + 1 == 64||self.depth+1>self.maximum_source_depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "borrowed Pack exceeds caller logical source depth")); }
        self.path[self.depth] = ordinal; self.depth += 1;
        self.frames[self.depth] = Frame { shape, table_row, force_text: parent.table_row && matches!(shape, Some(B::Text | B::Ref(_))), intrinsic:parent.intrinsic||matches!(shape,Some(B::Value))||matches!(parent.shape,Some(B::Wire))&&ordinal==8,record:if matches!(parent.shape,Some(B::Statements(_))){parent.record}else{None},bare_record:matches!(parent.shape,Some(B::Statements(_))), ..Default::default() }; Ok(())
    }
    fn table_cell<'a>(&mut self, source: &'a dyn FieldProjectionSource, row: usize, column: usize) -> Result<V<'a>, ValueError> {
        if self.depth + 2 >= 64||self.depth+2>self.maximum_source_depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Pack Table exceeds caller logical source depth")); }
        self.path[self.depth] = row;
        let V::Record(ids) = source.projection_view(&self.path[..self.depth + 1])? else { return Err(invalid("Pack Table row changed declared Record shape")); };
        let spec = self.frames[self.depth].record.ok_or_else(|| invalid("Pack Table lost declared columns"))?;
        if ids.len() != spec.fields.len() || ids.get(column).copied() != spec.fields.get(column).map(|field|field.id) { return Err(invalid("Pack Table column ordinal changed declared identity")); }
        self.path[self.depth + 1] = column;
        source.projection_view(&self.path[..self.depth + 2])
    }
    fn table_tag(shape: B) -> u8 { match shape { B::Bool=>ELEM_BOOL,B::Int=>ELEM_INT,B::UInt|B::Count=>ELEM_UINT,B::Float|B::Quantity(_)|B::Angle(_)=>ELEM_F64,B::Text|B::Ref(_)=>ELEM_STR,B::Enum(_)=>ELEM_ENUM,_=>ELEM_FALLBACK } }
    fn pop(&mut self) { if self.depth == 0 { self.phase = if self.phase == Phase::Discover { Phase::Select } else { Phase::Complete }; self.symbol_index = 0; } else { self.depth -= 1; } }

    fn compare(&mut self, left: TextReadView<'_>, right: TextReadView<'_>, maximum: usize) -> Result<Option<(Option<Ordering>, usize)>,ValueError> {
        let limit = left.len().min(right.len());
        if self.compare_offset == limit { self.compare_offset = 0; self.compare_left = None; return Ok(Some((Some(left.len().cmp(&right.len())), 0))); }
        let mut copied = 0;
        while copied < maximum {
            let Some(first) = self.compare_left.take() else {
                self.compare_left = Some(left.byte_at(self.compare_offset).ok_or_else(||invalid("Pack original text byte disappeared"))?);
                copied += 1;
                continue;
            };
            let order = first.cmp(&right.byte_at(self.compare_offset).ok_or_else(||invalid("Pack original symbol byte disappeared"))?);
            copied += 1;
            self.compare_offset += 1;
            if order != Ordering::Equal || self.compare_offset == limit {
                self.compare_offset = 0;
                return Ok(Some((Some(if order == Ordering::Equal { left.len().cmp(&right.len()) } else { order }), copied)));
            }
        }
        Ok((copied != 0).then_some((None, copied)))
    }

    fn declare(&mut self, view: &V<'_>) -> Result<(), ValueError> {
        if matches!(view,V::Absent){return Ok(());}
        if self.intrinsic_field.is_some() || self.frames[self.depth].intrinsic {
            return if matches!(view,V::IntrinsicNull|V::IntrinsicBool(_)|V::IntrinsicNumber(_)|V::IntrinsicText(_)|V::IntrinsicTextSource(_)|V::IntrinsicBytes(_)|V::IntrinsicBytesSource(_)|V::IntrinsicArray(_)|V::IntrinsicObject(_)) { Ok(()) } else { Err(invalid("intrinsic Pack source changed its declared value authority")) };
        }
        let frame = self.frames[self.depth];
        if let V::Record(ids) = view {
            let record = if self.depth == 0 { self.root_spec } else { frame.record.or_else(||match frame.shape { Some(B::Record(make)) => Some(make()), _ => None }) }.ok_or_else(|| invalid("Pack Record lacks declared Record authority"))?;
            if ids.len() != record.fields.len() { return Err(invalid("Pack Record source field count disagrees with declared authority")); }
            self.frames[self.depth].record = Some(record);
            return Ok(());
        }
        if let (Some(B::Table(make)), V::List(_)) = (frame.shape, view) {
            let record=make();
            self.frames[self.depth].record=Some(record); return Ok(());
        }
        let valid = matches!((frame.shape, view),
            (Some(B::Expr),V::Expr(_))|
            (_, V::Absent) | (Some(B::Bool), V::Bool(_)) | (Some(B::Int), V::Int(_)) |
            (Some(B::UInt | B::Count), V::UInt(_)) | (Some(B::Float | B::Quantity(_) | B::Angle(_)), V::Float(_)) |
            (Some(B::Text | B::Ref(_)|B::Embed(_)|B::EmbedFrom(_)), V::Text(_) | V::TextSource(_)) | (Some(B::Bytes64), V::Bytes(_) | V::BytesSource(_)) | (Some(B::Enum(_)), V::Enum(_)) |
            (Some(B::Map(_)),V::Map(_))|(Some(B::Statements(_)),V::Statements(_))|(Some(B::Wire),V::Wire(_))|
            (Some(B::Block(_)), V::Block) | (Some(B::List(_)), V::List(_)) | (Some(B::Tuple(_, _) | B::Coord(_) | B::Dim(_) | B::Dir | B::Range), V::Tuple(_)));
        if valid {if let V::Tuple(length)=view{let expected=match frame.shape{Some(B::Tuple(_,arity))=>arity,Some(B::Coord(width)|B::Dim(width))=>Some(usize::from(width)),Some(B::Dir)=>Some(3),Some(B::Range)=>Some(2),_=>None};if expected.is_some_and(|expected|expected!=*length){return Err(invalid("Pack tuple source changed declared arity"));}}Ok(())} else { Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "borrowed Pack requires its exact literal Record/scalar/sequence shape authority")) }
    }

    fn discover(&mut self, source: &dyn FieldProjectionSource,maximum:usize,progress:&mut RetainedCloneProgress) -> Result<(), ValueError> {
        let frame = self.frames[self.depth];
        let view = source.projection_view(&self.path[..self.depth])?;
        if frame.phase == 0 {
            self.declare(&view)?;
            match view {
                V::Text(_) | V::IntrinsicText(_) | V::TextSource(_) | V::IntrinsicTextSource(_) => { self.note = Some(Symbol { path: self.path, depth: self.depth, occurrences: 1, selected: false, forced: frame.force_text,key:None }); self.frames[self.depth].phase=1; self.pop(); self.phase = Phase::Find; self.search = 0; self.compare_offset = 0; self.compare_left = None; }
                V::Block => { self.frames[self.depth].phase=1;self.frames[self.depth].length=1; }
                V::Record(ids) => { self.frames[self.depth].phase = 1; self.frames[self.depth].length = ids.len(); }
                V::List(length) | V::Tuple(length) | V::IntrinsicArray(length) | V::IntrinsicObject(length) => { self.frames[self.depth].phase = 1; self.frames[self.depth].length = length; }
                V::Map(length)|V::Statements(length)=>{self.frames[self.depth].phase=30;self.frames[self.depth].length=length;},
                V::Wire(_)=>{self.frames[self.depth].phase=1;self.frames[self.depth].length=9;},
                V::Expr(_)=>self.pop(),
                V::Absent | V::Bool(_) | V::Int(_) | V::UInt(_) | V::Float(_) | V::Enum(_) | V::Bytes(_) | V::BytesSource(_) | V::IntrinsicNull | V::IntrinsicBool(_) | V::IntrinsicNumber(_) | V::IntrinsicBytes(_) | V::IntrinsicBytesSource(_) => self.pop(),
                _ => return Err(invalid("borrowed Pack cursor has no declared projection authority for this shape")),
            }
        } else if frame.phase==30 {
            if frame.index==frame.length{self.pop();}
            else{self.path[self.depth]=frame.index;if matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){self.frames[self.depth].index+=1;}
            else{let forced=matches!(frame.shape,Some(B::Statements(_)));self.note=Some(Symbol{path:self.path,depth:self.depth,occurrences:1,selected:false,forced,key:Some(frame.index)});self.frames[self.depth].phase=31;self.frames[self.depth].count=0;self.phase=Phase::Find;self.search=0;self.compare_offset=0;self.compare_left=None;}}
        } else if frame.phase==31&&matches!(frame.shape,Some(B::Statements(_))){
            let Some(B::Statements(variants))=frame.shape else{unreachable!()};let Some((keyword,make))=variants.get(frame.count)else{return Err(invalid("Pack statement keyword lacks authored variant"));};let key=source.projection_key_view(&self.path[..self.depth],frame.index)?;
            if let Some((order,copied))=self.compare(key,TextReadView::from_str(keyword),maximum)?{progress.copied_bytes=copied;if let Some(order)=order{if order==Ordering::Equal{self.frames[self.depth].record=Some(make());self.frames[self.depth].phase=30;self.frames[self.depth].index+=1;self.push(frame.index)?;}else{self.frames[self.depth].count+=1;}}}
        } else if frame.phase==31{self.frames[self.depth].phase=30;self.frames[self.depth].index+=1;self.push(frame.index)?;}
        else if frame.index < frame.length { self.frames[self.depth].index += 1; self.push(frame.index)?; }
        else { self.pop(); }
        Ok(())
    }

    /// ✍️ Performs one structural event, one physical birth, or at most sixty-four source/output bytes.
    pub fn advance(&mut self, source: &dyn FieldProjectionSource, spec: BorrowedRecordSpec, output: &mut [u8], grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, BorrowedProjectedPackFailure> {
        self.advance_root(source,Some(spec),None,output,grant)
    }

    /// 🌱️ Emits one declared intrinsic Body field, preserving original member order and duplicates.
    pub fn advance_intrinsic(&mut self, source: &dyn FieldProjectionSource, field_id: u16, output: &mut [u8], grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, BorrowedProjectedPackFailure> {
        self.advance_root(source,None,Some(field_id),output,grant)
    }

    fn advance_root(&mut self, source: &dyn FieldProjectionSource, spec: Option<BorrowedRecordSpec>, field_id: Option<u16>, output: &mut [u8], grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, BorrowedProjectedPackFailure> {
        let mut progress = RetainedCloneProgress::default();
        let result = (|| -> Result<BorrowedProjectedPackProgress, ValueError> {
        if matches!(self.phase, Phase::Closing | Phase::Closed) { return Err(invalid("borrowed Pack cursor is closing")); }
        let identity = source.projection_identity();
        if self.identity.is_some_and(|expected| expected != identity) { return Err(invalid("borrowed Pack original source identity changed")); }
        if self.identity.is_some() && (self.intrinsic_field != field_id || self.root_spec.is_some() != spec.is_some()) { return Err(invalid("borrowed Pack root framing authority changed")); }
        if self.root_spec.is_some_and(|original| spec.is_none_or(|spec|original.fields.as_ptr() != spec.fields.as_ptr() || original.fields.len() != spec.fields.len())) { return Err(invalid("borrowed Pack declared root fields changed")); }
        if self.is_complete() { return Ok(BorrowedProjectedPackProgress { complete: true, ..Default::default() }); }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < self.next_minimum_copy_bytes() { return Ok(Default::default()); }
        if grant.maximum_depth < self.next_advance_depth_demand()? { return Ok(Default::default()); }
        self.identity = Some(identity); self.root_spec = spec; self.intrinsic_field = field_id;
        progress.copied_items = 1;
        let maximum = grant.maximum_copy_bytes.min(64);
        if self.pending_offset < self.pending_length {
            let count = output.len().min(maximum).min(self.pending_length - self.pending_offset);
            if count == 0 { return Ok(Default::default()); }
            output[..count].copy_from_slice(&self.pending[self.pending_offset..self.pending_offset + count]); self.pending_offset += count; progress.copied_bytes = count;
            return Ok(BorrowedProjectedPackProgress { progress, written_bytes: count, complete: self.is_complete() });
        }
        match self.phase {
            Phase::Initialize=>{if !self.frames.has_reserved_slot(){progress=self.frames.reserve_one_funded(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(|error|{progress.retained_capacity_bytes=error.allocated_bytes;page_error(error.refusal())})?;}else{self.frames.push_reserved(Frame::default()).map_err(|_|invalid("Pack frame has no admitted initialization slot"))?;progress.copied_bytes=size_of::<Frame>();if self.frames.len()==64{self.phase=Phase::Discover;}}},
            Phase::Discover => self.discover(source,maximum,&mut progress)?,
            Phase::Find => {
                let note = self.note.ok_or_else(|| invalid("borrowed Pack lost its pending text locator"))?;
                if self.search == self.symbols.len() { self.insertion = self.search; self.phase = Phase::Reserve; }
                else {
                    let left = note.text(source)?; let right = self.symbols[self.search].text(source)?;
                    let Some((order, bytes)) = self.compare(left, right, maximum)? else { return Ok(Default::default()); };
                    progress.copied_bytes = bytes;
                    if let Some(order) = order { match order { Ordering::Less => { self.insertion = self.search; self.phase = Phase::Reserve; }, Ordering::Greater => self.search += 1, Ordering::Equal => { self.symbols[self.search].occurrences = self.symbols[self.search].occurrences.checked_add(1).ok_or_else(|| invalid("Pack symbol occurrence overflow"))?; self.symbols[self.search].forced |= note.forced; self.note = None; self.phase = Phase::Discover; } } }
                }
            }
            Phase::Reserve => {
                if !self.symbols.has_reserved_slot() {
                    let required = self.symbols.next_allocation_bytes().map_err(page_error)?;
                    if required > grant.maximum_capacity_bytes { return Ok(Default::default()); }
                    progress=self.symbols.reserve_one_funded(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(|error| { progress.retained_capacity_bytes = error.allocated_bytes; page_error(error.refusal()) })?;
                } else { self.symbols.push_reserved(self.note.ok_or_else(|| invalid("Pack insertion lost text locator"))?).map_err(|_| invalid("Pack symbol reserved slot rejected"))?;progress.copied_bytes=size_of::<Symbol>(); self.search = self.symbols.len() - 1; self.phase = Phase::Insert; }
            }
            Phase::Insert => {
                if self.search > self.insertion { self.symbols.swap(self.search-1,self.search);progress.copied_bytes=3*size_of::<Symbol>();self.search -= 1; }
                else { self.note = None; self.phase = Phase::Discover; }
            }
            Phase::Select => {
                if self.symbol_index < self.symbols.len() { let selected = self.symbols[self.symbol_index].forced || self.symbols[self.symbol_index].text(source)?.len() <= 128 || self.symbols[self.symbol_index].occurrences >= 2; self.symbols[self.symbol_index].selected = selected; self.selected += usize::from(selected); self.symbol_index += 1; }
                else { self.phase = Phase::SymbolsCount; }
            }
            Phase::SymbolsCount => { self.queue_number(None, self.selected as u64); self.symbol_index = 0; self.phase = Phase::Symbols; }
            Phase::Symbols => {
                if self.symbol_index == self.symbols.len() {
                    self.reset=0;self.depth=0;self.phase=Phase::Reset;
                }
                else if self.symbols[self.symbol_index].selected { self.queue_number(None, self.symbols[self.symbol_index].text(source)?.len() as u64); self.symbol_offset = 0; self.phase = Phase::SymbolBytes; }
                else { self.symbol_index += 1; }
            }
            Phase::SymbolBytes => {
                let bytes = self.symbols[self.symbol_index].text(source)?;
                let count = output.len().min(maximum).min(1).min(bytes.len() - self.symbol_offset);
                if count == 0 && self.symbol_offset < bytes.len() { return Ok(Default::default()); }
                if count!=0{output[0]=bytes.byte_at(self.symbol_offset).ok_or_else(||invalid("Pack original symbol byte disappeared"))?;} self.symbol_offset += count; progress.copied_bytes = count;
                if self.symbol_offset == bytes.len() { self.symbol_index += 1; self.phase = Phase::Symbols; }
                return Ok(BorrowedProjectedPackProgress { progress, written_bytes: count, complete: false });
            }
            Phase::Reset=>{self.frames[self.reset]=Frame::default();self.reset+=1;progress.copied_bytes=size_of::<Frame>();if self.reset==64{self.phase=Phase::Body;if let Some(field)=self.intrinsic_field{self.queue_number(None,1);self.append_number(u64::from(field));self.pending[self.pending_length]=TAG_VALUE;self.pending_length+=1;}}},
            Phase::ChunkReserve=>{
                if !self.replaying&&!self.chunks.has_reserved_slot(){progress=self.chunks.reserve_one_funded(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(|error|{progress.retained_capacity_bytes=error.allocated_bytes;page_error(error.refusal())})?;}
                else{let chunk=self.chunk_note.ok_or_else(||invalid("Pack chunk reservation lost original locator"))?;let id=if self.replaying{let original=self.chunks.get(self.chunk_index).ok_or_else(||invalid("Pack chunk replay lost original ordinal"))?;if original.depth!=chunk.depth||original.offset!=chunk.offset||original.length!=chunk.length||original.path[..original.depth]!=chunk.path[..chunk.depth]{return Err(invalid("Pack chunk replay original range changed"));}let id=self.chunk_index;self.chunk_index+=1;id}else{let id=self.chunks.len();if id>u32::MAX as usize{return Err(invalid("Pack chunk ID exceeds canonical width"));}self.chunks.push_reserved(chunk).map_err(|_|invalid("Pack chunk has no admitted slot"))?;id};self.frames[self.depth].offset+=chunk.length;self.queue_number(None,id as u64);self.chunk_note=None;self.phase=Phase::Body;progress.copied_bytes=size_of::<ChunkLocator>();}
            },
            Phase::Body => { return self.body(source, output, maximum, &mut progress); }
            Phase::Expression=>{progress=Default::default();return self.expression_turn(source,output,grant);},
            _ => unreachable!(),
        }
        Ok(BorrowedProjectedPackProgress { progress, written_bytes: 0, complete: self.is_complete() })
        })();
        result.map_err(|reason| failed_turn(reason, progress))
    }

    fn body(&mut self, source: &dyn FieldProjectionSource, output: &mut [u8], maximum: usize, progress: &mut RetainedCloneProgress) -> Result<BorrowedProjectedPackProgress, ValueError> {
        let frame = self.frames[self.depth];
        let view = source.projection_view(&self.path[..self.depth])?;
        if frame.phase == 0 {
            self.declare(&view)?;
            if matches!(frame.shape,Some(B::Value))&&frame.flags&128==0&&!matches!(view,V::Absent){self.queue(&[TAG_VALUE]);self.frames[self.depth].flags|=128;return Ok(BorrowedProjectedPackProgress{progress:*progress,..Default::default()});}
            match view {
                V::Block => { self.queue(&[TAG_BLOCK]);self.frames[self.depth].phase=20; }
                V::Record(_) => { if self.depth != 0&&!frame.bare_record { self.queue(&[TAG_RECORD]); } self.frames[self.depth].phase = 1; }
                V::List(length) if matches!(frame.shape, Some(B::Table(_))) => { self.queue_number(Some(TAG_TABLE_SOA),length as u64); self.frames[self.depth].phase=10; self.frames[self.depth].length=length; }
                V::List(length) | V::Tuple(length) => { self.frames[self.depth].phase = 4; self.frames[self.depth].length = length; self.frames[self.depth].flags = if length == 0 { 0 } else { 3 }; }
                V::IntrinsicArray(length) => { self.queue_number(Some(TAG_LIST),length as u64);self.frames[self.depth].phase=5;self.frames[self.depth].length=length; }
                V::IntrinsicObject(length) => { self.queue_number(Some(TAG_MAP),length as u64);self.frames[self.depth].phase=21;self.frames[self.depth].length=length; }
                V::Map(length)=>{self.frames[self.depth].phase=30;self.frames[self.depth].length=length;self.frames[self.depth].previous=u32::MAX;self.frames[self.depth].offset=usize::MAX;},
                V::Statements(length)=>{self.queue_number(Some(TAG_STATEMENTS),length as u64);self.frames[self.depth].phase=50;self.frames[self.depth].length=length;},
                V::Wire(directed)=>{self.queue(&[TAG_WIRE]);self.frames[self.depth].phase=40;self.frames[self.depth].flags=u8::from(directed.is_some())|u8::from(directed==Some(true))<<1;},
                V::Expr(_)=>{self.expression.restart();self.expression.set_maximum_depth(self.maximum_source_depth.saturating_sub(self.depth));self.expression_state=Default::default();self.phase=Phase::Expression;},
                V::Text(_) | V::IntrinsicText(_) | V::TextSource(_) | V::IntrinsicTextSource(_) => { self.frames[self.depth].phase = 7; self.compare_offset = 0; self.compare_left = None; }
                V::Bytes(bytes) | V::IntrinsicBytes(bytes) => {if self.chunking&&bytes.len()>=262144{self.queue_number(Some(TAG_BYTES_CHUNKED),bytes.len().div_ceil(1048576)as u64);self.frames[self.depth].phase=60;self.frames[self.depth].length=bytes.len();self.frames[self.depth].offset=0;}else{self.queue_number(Some(TAG_BYTES), bytes.len() as u64); self.frames[self.depth].phase = 8;} }
                V::BytesSource(bytes) | V::IntrinsicBytesSource(bytes) => {let length=bytes.byte_len();if self.chunking&&length>=262144{self.queue_number(Some(TAG_BYTES_CHUNKED),length.div_ceil(1048576)as u64);self.frames[self.depth].phase=60;self.frames[self.depth].length=length;self.frames[self.depth].offset=0;}else{self.queue_number(Some(TAG_BYTES),length as u64);self.frames[self.depth].phase=8;}}
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
                let text = text_view(view)?;
                if frame.index == self.symbols.len() { self.queue_number(Some(TAG_STR_INLINE), text.len() as u64); self.frames[self.depth].phase = 8; self.frames[self.depth].offset = 0; }
                else {
                    let right = self.symbols[frame.index].text(source)?;
                    let Some((order, bytes)) = self.compare(text, right, maximum)? else { return Ok(Default::default()); };
                    progress.copied_bytes = bytes;
                    if let Some(order) = order {
                        if order == Ordering::Equal && self.symbols[frame.index].selected { self.queue_number(Some(TAG_STR), frame.rank as u64); self.pop(); }
                        else if order == Ordering::Equal { self.queue_number(Some(TAG_STR_INLINE), text.len() as u64); self.frames[self.depth].phase = 8; self.frames[self.depth].offset = 0; }
                        else { self.frames[self.depth].index += 1; self.frames[self.depth].rank += usize::from(self.symbols[frame.index].selected); }
                    }
                }
            }
            8 => {
                let length=match &view{V::Bytes(bytes)|V::IntrinsicBytes(bytes)=>bytes.len(),V::BytesSource(bytes)|V::IntrinsicBytesSource(bytes)=>bytes.byte_len(),V::Text(text)|V::IntrinsicText(text)=>text.len(),V::TextSource(text)|V::IntrinsicTextSource(text)=>text.len(),_=>return Err(invalid("Pack original payload shape changed"))};
                let count=output.len().min(maximum).min(length-frame.offset);
                if count==0&&frame.offset<length{return Ok(Default::default());}
                let copied=match view{V::Bytes(bytes)|V::IntrinsicBytes(bytes)=>{output[..count].copy_from_slice(&bytes[frame.offset..frame.offset+count]);count},V::BytesSource(bytes)|V::IntrinsicBytesSource(bytes)=>bytes.read_bytes(frame.offset,&mut output[..count]),view=>{let text=text_view(view)?;let mut copied=0;while copied<count{output[copied]=text.byte_at(frame.offset+copied).ok_or_else(||invalid("Pack original payload byte disappeared"))?;copied+=1;}copied}};
                if copied!=count{return Err(invalid("Pack original payload byte disappeared"))}self.frames[self.depth].offset+=count;progress.copied_bytes=count;
                if self.frames[self.depth].offset == length { self.pop(); }
                return Ok(BorrowedProjectedPackProgress { progress: *progress, written_bytes: count, complete: self.is_complete() });
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
                        (ELEM_STR,V::Text(_)|V::TextSource(_))=>{self.frames[self.depth].phase=18;self.frames[self.depth].count=0;self.frames[self.depth].rank=0;self.compare_offset=0;self.compare_left=None;return Ok(BorrowedProjectedPackProgress{progress: *progress,..Default::default()});},
                        (ELEM_FALLBACK,_)=>{self.frames[self.depth].index+=1;self.push(frame.index)?;self.declare(&source.projection_view(&self.path[..self.depth])?)?;self.frames[self.depth].phase=19;self.push(frame.offset)?;return Ok(BorrowedProjectedPackProgress{progress: *progress,..Default::default()});},
                        _=>return Err(invalid("Pack Table cell changed declared scalar shape")),
                    } self.frames[self.depth].index+=1; }
                }
            }
            18 => {
                let text=text_view(self.table_cell(source,frame.index,frame.offset)?)?;
                if frame.count==self.symbols.len() {return Err(invalid("Pack Table text lost forced symbol"));}
                let right=self.symbols[frame.count].text(source)?;let Some((order,bytes))=self.compare(text,right,maximum)? else {return Ok(Default::default());};progress.copied_bytes=bytes;
                if let Some(order)=order { if order==Ordering::Equal {if !self.symbols[frame.count].selected{return Err(invalid("Pack Table symbol was not selected"));}self.queue_number(None,frame.rank as u64);self.frames[self.depth].phase=17;self.frames[self.depth].index+=1;}else{self.frames[self.depth].rank+=usize::from(self.symbols[frame.count].selected);self.frames[self.depth].count+=1;} }
            }
            19 => self.pop(),
            20 => { self.frames[self.depth].phase=19;self.push(0)?; },
            21 => {
                if frame.index==frame.length {self.pop();}
                else {let key=source.projection_key_view(&self.path[..self.depth],frame.index)?;self.queue_number(Some(TAG_STR_INLINE),key.len()as u64);self.frames[self.depth].phase=22;self.frames[self.depth].offset=0;}
            }
            22 => {
                let bytes=source.projection_key_view(&self.path[..self.depth],frame.index)?;let count=output.len().min(maximum).min(1).min(bytes.len()-frame.offset);
                if count==0&&frame.offset<bytes.len(){return Ok(Default::default());}
                if count!=0{output[0]=bytes.byte_at(frame.offset).ok_or_else(||invalid("Pack intrinsic key original byte disappeared"))?;}self.frames[self.depth].offset+=count;progress.copied_bytes=count;
                if self.frames[self.depth].offset==bytes.len(){self.frames[self.depth].phase=23;}
                return Ok(BorrowedProjectedPackProgress{progress: *progress,written_bytes:count,complete:false});
            }
            23 => {self.frames[self.depth].phase=21;self.frames[self.depth].index+=1;self.push(frame.index)?;},
            30=>{
                if frame.index<frame.length{self.path[self.depth]=frame.index;if !matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){self.frames[self.depth].count+=1;}self.frames[self.depth].index+=1;}
                else{self.queue_number(Some(TAG_MAP),frame.count as u64);self.frames[self.depth].candidate=u32::try_from(frame.count).map_err(|_|invalid("Pack Map count exceeds ordinal width"))?;self.frames[self.depth].phase=31;self.frames[self.depth].index=0;self.frames[self.depth].offset=usize::MAX;}
            },
            31=>{
                if frame.ordinal==frame.candidate as usize{self.pop();}
                else if frame.index==frame.length{if frame.offset==usize::MAX{return Err(invalid("Pack Map lost original sorted key"));}self.frames[self.depth].phase=34;self.frames[self.depth].count=0;self.frames[self.depth].rank=0;self.compare_offset=0;self.compare_left=None;}
                else{self.path[self.depth]=frame.index;if matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){self.frames[self.depth].index+=1;}
                else{self.frames[self.depth].phase=if frame.previous==u32::MAX{33}else{32};self.compare_offset=0;self.compare_left=None;}}
            },
            32=>{
                let key=source.projection_key_view(&self.path[..self.depth],frame.index)?;let last=source.projection_key_view(&self.path[..self.depth],frame.previous as usize)?;
                if let Some((order,bytes))=self.compare(key,last,maximum)?{progress.copied_bytes=bytes;if let Some(order)=order{if order==Ordering::Greater||order==Ordering::Equal&&frame.index>frame.previous as usize{self.frames[self.depth].phase=33;}else{self.frames[self.depth].phase=31;self.frames[self.depth].index+=1;}}}
            },
            33=>{
                if frame.offset==usize::MAX{self.frames[self.depth].offset=frame.index;self.frames[self.depth].index+=1;self.frames[self.depth].phase=31;}
                else{let key=source.projection_key_view(&self.path[..self.depth],frame.index)?;let best=source.projection_key_view(&self.path[..self.depth],frame.offset)?;
                if let Some((order,bytes))=self.compare(key,best,maximum)?{progress.copied_bytes=bytes;if let Some(order)=order{if order==Ordering::Less||order==Ordering::Equal&&frame.index<frame.offset{self.frames[self.depth].offset=frame.index;}self.frames[self.depth].index+=1;self.frames[self.depth].phase=31;}}}
            },
            34=>{
                let key=source.projection_key_view(&self.path[..self.depth],frame.offset)?;
                if frame.count==self.symbols.len(){self.queue_number(Some(TAG_STR_INLINE),key.len()as u64);self.frames[self.depth].phase=35;self.frames[self.depth].count=0;}
                else{let right=self.symbols[frame.count].text(source)?;if let Some((order,bytes))=self.compare(key,right,maximum)?{progress.copied_bytes=bytes;if let Some(order)=order{if order==Ordering::Equal&&self.symbols[frame.count].selected{self.queue_number(Some(TAG_STR),frame.rank as u64);self.frames[self.depth].phase=37;}else if order==Ordering::Equal{self.queue_number(Some(TAG_STR_INLINE),key.len()as u64);self.frames[self.depth].phase=35;self.frames[self.depth].count=0;}else{self.frames[self.depth].rank+=usize::from(self.symbols[frame.count].selected);self.frames[self.depth].count+=1;}}}}
            },
            35=>{
                let key=source.projection_key_view(&self.path[..self.depth],frame.offset)?;let count=output.len().min(maximum).min(1).min(key.len()-frame.count);if count==0&&frame.count<key.len(){return Ok(Default::default());}if count!=0{output[0]=key.byte_at(frame.count).ok_or_else(||invalid("Pack original Map key byte disappeared"))?;}self.frames[self.depth].count+=count;progress.copied_bytes=count;if self.frames[self.depth].count==key.len(){self.frames[self.depth].phase=37;}return Ok(BorrowedProjectedPackProgress{progress:*progress,written_bytes:count,complete:false});
            },
            37=>{self.frames[self.depth].phase=38;self.push(frame.offset)?;},
            38=>{self.frames[self.depth].previous=u32::try_from(frame.offset).map_err(|_|invalid("Pack Map original ordinal exceeds width"))?;self.frames[self.depth].ordinal+=1;self.frames[self.depth].index=0;self.frames[self.depth].offset=usize::MAX;self.frames[self.depth].phase=31;},
            40=>{let mut flags=frame.flags;for index in 6..8{self.path[self.depth]=index;if !matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){flags|=4;}}self.queue(&[flags]);self.frames[self.depth].flags=flags;self.frames[self.depth].offset=0;self.frames[self.depth].phase=41;},
            41=>{let mut flags=0;for relative in 1..3{self.path[self.depth]=frame.offset+relative;if !matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){flags|=1<<(relative-1);}}self.path[self.depth]=frame.offset;if matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){return Err(invalid("Pack Wire node lost required id"));}self.queue(&[flags]);self.frames[self.depth].index=frame.offset;self.frames[self.depth].phase=42;},
            42=>{if frame.index==frame.offset+3{if frame.offset==0&&frame.flags&1!=0{self.frames[self.depth].offset=3;self.frames[self.depth].phase=41;}else{self.frames[self.depth].phase=44;}}else{self.path[self.depth]=frame.index;self.frames[self.depth].index+=1;if !matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){self.push(frame.index)?;}}},
            44=>{if frame.flags&4==0{self.frames[self.depth].phase=46;}else{let mut flags=0;for index in 6..8{self.path[self.depth]=index;if !matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){flags|=1<<(index-6);}}self.queue(&[flags]);self.frames[self.depth].index=6;self.frames[self.depth].phase=45;}},
            45=>{if frame.index==8{self.frames[self.depth].phase=46;}else{self.path[self.depth]=frame.index;self.frames[self.depth].index+=1;if !matches!(source.projection_view(&self.path[..self.depth+1])?,V::Absent){self.push(frame.index)?;}}},
            46=>{self.frames[self.depth].phase=19;self.push(8)?;},
            50=>{
                if frame.index==frame.length{self.pop();}else if frame.count==self.symbols.len(){return Err(invalid("Pack statement keyword lost forced symbol"));}else{let key=source.projection_key_view(&self.path[..self.depth],frame.index)?;let right=self.symbols[frame.count].text(source)?;if let Some((order,bytes))=self.compare(key,right,maximum)?{progress.copied_bytes=bytes;if let Some(order)=order{if order==Ordering::Equal{if !self.symbols[frame.count].selected{return Err(invalid("Pack statement forced symbol not selected"));}self.queue_number(None,frame.rank as u64);self.frames[self.depth].phase=51;self.frames[self.depth].count=0;}else{self.frames[self.depth].rank+=usize::from(self.symbols[frame.count].selected);self.frames[self.depth].count+=1;}}}}
            },
            51=>{
                let Some(B::Statements(variants))=frame.shape else{return Err(invalid("Pack statements lost variant authority"));};let Some((keyword,make))=variants.get(frame.count)else{return Err(invalid("Pack statement keyword lacks authored variant"));};let key=source.projection_key_view(&self.path[..self.depth],frame.index)?;
                if let Some((order,bytes))=self.compare(key,TextReadView::from_str(keyword),maximum)?{progress.copied_bytes=bytes;if let Some(order)=order{if order==Ordering::Equal{self.frames[self.depth].record=Some(make());self.frames[self.depth].phase=52;}else{self.frames[self.depth].count+=1;}}}
            },
            52=>{self.frames[self.depth].phase=50;self.frames[self.depth].index+=1;self.frames[self.depth].count=0;self.frames[self.depth].rank=0;self.push(frame.index)?;},
            60=>{if frame.offset==frame.length{self.pop();}else{self.chunk_note=Some(ChunkLocator{path:self.path,depth:self.depth,offset:frame.offset,length:(frame.length-frame.offset).min(1048576)});self.phase=Phase::ChunkReserve;progress.copied_bytes=size_of::<ChunkLocator>();}},
            _ => return Err(invalid("Pack cursor phase changed")),
        } }
        Ok(BorrowedProjectedPackProgress { progress: *progress, written_bytes: 0, complete: self.phase == Phase::Complete && self.pending_offset == self.pending_length })
    }

    /// 🔣️ Quotes the original symbol frontier with separate release and structural depth.
    fn expression_turn(&mut self,source:&dyn FieldProjectionSource,output:&mut[u8],grant:RetainedCloneGrant)->Result<BorrowedProjectedPackProgress,ValueError>{
        if self.expression_state.mode==3&&output.is_empty(){return Ok(Default::default());}
        let V::Expr(original)=source.projection_view(&self.path[..self.depth])?else{return Err(invalid("original Pack expression field changed"));};
        if self.expression_state.mode==1{
            if self.expression_state.index==self.symbols.len(){self.expression.restart();self.expression_state.mode=3;self.queue(&[TAG_EXPR,TAG_STR_INLINE]);self.append_number(self.expression_state.length as u64);}
            else{let symbol=&self.symbols[self.expression_state.index];if !symbol.selected||symbol.text(source)?.len()!=self.expression_state.length{self.expression_state.rank+=usize::from(symbol.selected);self.expression_state.index+=1;}else{self.expression.restart();self.expression_state.offset=0;self.expression_state.different=false;self.expression_state.mode=2;}}
            return Ok(BorrowedProjectedPackProgress{progress:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()});
        }
        let mut byte=[0;1];let step=self.expression.advance(original,&mut byte,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,maximum_depth:grant.maximum_depth-self.depth-1,..grant})?;let mut progress=step.progress;let mut written_bytes=0;
        match self.expression_state.mode{
            0=>{self.expression_state.length=self.expression_state.length.checked_add(step.written_bytes).ok_or_else(||invalid("original expression text length overflow"))?;if step.complete{self.expression_state.mode=1;}},
            2=>{if step.written_bytes!=0{let original=self.symbols[self.expression_state.index].text(source)?.byte_at(self.expression_state.offset).ok_or_else(||invalid("original expression matching symbol changed"))?;progress.copied_bytes+=1;self.expression_state.offset+=1;self.expression_state.different|=original!=byte[0];}if self.expression_state.different{self.expression_state.rank+=1;self.expression_state.index+=1;self.expression_state.mode=1;}else if step.complete{self.queue(&[TAG_EXPR,TAG_STR]);self.append_number(self.expression_state.rank as u64);self.pop();self.phase=Phase::Body;}},
            3=>{if step.written_bytes!=0{if output.is_empty(){return Err(invalid("original expression lost final byte output custody"));}output[0]=byte[0];progress.copied_bytes+=1;written_bytes=1;}if step.complete{self.pop();self.phase=Phase::Body;}},_=>return Err(invalid("original expression retained mode changed"))
        }
        Ok(BorrowedProjectedPackProgress{progress,written_bytes,complete:false})
    }

    /// ♻️ Prices every original child owner before any expression, symbol or frame backing closes.
    pub fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.phase == Phase::Closed { return Ok(Default::default()); }if !self.expression.terminal_is_empty(){let demand=self.expression.retirement_demands()?;return Ok(semio_framework_value::RetirementDemand{depth:demand.depth+1,..demand});}
        let depth = if !self.symbols.terminal_is_empty(){if !self.symbols.is_empty(){self.symbols.next_pop_depth_demand()}else{self.symbols.next_release_depth_demand()}}else if !self.frames.is_empty(){self.frames.next_pop_depth_demand()}else if !self.frames.terminal_is_empty(){self.frames.next_release_depth_demand()}else if !self.chunks.is_empty(){self.chunks.next_pop_depth_demand()}else{self.chunks.next_release_depth_demand()}.map_err(page_error)?;
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.next_close_byte_demand()?, depth: depth.checked_add(1).ok_or_else(|| invalid("Pack retirement depth overflow"))?, ..Default::default() })
    }

    /// 🍂️ Retires one inline locator or one whole paid symbol allocation without source disposal.
    pub fn close(&mut self, grant: RetainedCloneGrant) -> Result<BorrowedProjectedPackProgress, BorrowedProjectedPackFailure> {
        let mut progress = RetainedCloneProgress::default();
        let result = (|| -> Result<BorrowedProjectedPackProgress, ValueError> {
        if self.phase == Phase::Closed { return Ok(BorrowedProjectedPackProgress { complete: true, ..Default::default() }); }
        let demand = self.retirement_demands()?;
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return Ok(Default::default()); }
        self.phase = Phase::Closing; self.note = None; self.compare_left = None;
        progress.copied_items = 1;
        if !self.expression.terminal_is_empty(){let step=self.expression.close(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;progress=step.progress;}else if !self.symbols.is_empty() { self.symbols.pop(); }
        else if !self.symbols.terminal_is_empty() { let required = self.symbols.next_release_allocation_bytes().map_err(page_error)?; if required > grant.maximum_release_bytes { return Ok(Default::default()); } let step = self.symbols.release_empty_page(required).map_err(page_error)?; progress.released_bytes = step.released_allocation_bytes; }
        else if !self.frames.is_empty(){self.frames.pop();}
        else if !self.frames.terminal_is_empty(){let step=self.frames.release_empty_page(demand.release_bytes).map_err(page_error)?;progress.released_bytes=step.released_allocation_bytes;}
        else if !self.chunks.is_empty(){self.chunks.pop();}
        else if !self.chunks.terminal_is_empty(){let step=self.chunks.release_empty_page(demand.release_bytes).map_err(page_error)?;progress.released_bytes=step.released_allocation_bytes;}
        else { self.identity = None; self.phase = Phase::Closed; }
        Ok(BorrowedProjectedPackProgress { progress, written_bytes: 0, complete: self.phase == Phase::Closed })
        })();
        result.map_err(|reason| failed_turn(reason, progress))
    }

}

impl Drop for BorrowedProjectedPackCursor {
    fn drop(&mut self) { assert!(std::thread::panicking()||self.symbols.terminal_is_empty()&&self.frames.terminal_is_empty()&&self.chunks.terminal_is_empty(), "borrowed Pack symbol/frame backing requires its exact terminal-empty close witness"); }
}

/// 📃️ Every paged owner of an interrupted cursor retires in one physical sequence: expression frames, symbols, frames and chunks.
#[derive(semio_framework_value::RetireOwned)]
struct PhysicalBorrowedPack{expression:RetainedExpressionCursor,symbols:PagedList<Symbol,SYMBOL_LIMIT>,frames:PagedList<Frame,64>,chunks:PagedList<ChunkLocator,{isize::MAX as usize}>}

struct OriginalProjectedPackRetirement(std::mem::ManuallyDrop<BorrowedProjectedPackCursor>);
impl semio_framework_value::retirement::RetirementCursor for OriginalProjectedPackRetirement{
    fn next_demand(&self,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.0.retirement_demands()}
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::RetirementStep as S;match self.0.close(grant){Err(failure)=>S::Failure(failure.reason),Ok(step) if step.complete&&step.progress==Default::default()=>S::Complete,Ok(step)=>S::Progress(step.progress)}}
    fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands()?.copy_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands()?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for OriginalProjectedPackRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.0.terminal_is_empty(),"original Pack retirement must close every symbol, frame, chunk and expression owner");if self.0.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.0);}}}}
impl semio_framework_value::retirement::RetireOwned for BorrowedProjectedPackCursor{
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(OriginalProjectedPackRetirement(std::mem::ManuallyDrop::new(self)))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalProjectedPackRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

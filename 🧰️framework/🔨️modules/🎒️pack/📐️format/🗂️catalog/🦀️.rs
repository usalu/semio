//! 🗂️ Retained canonical Pack catalog: bounded, cancellable cursors for the manifest, the symbol table, the chunk table and the inline symbols.
//! They consume the segment events admitted by `semio-framework-pack-format-retained`.

use semio_framework_replication_codec::{ByteRange, PackLimits};
use semio_framework_value::{ValueRefusalKind, RetirementDemand, ValueError};
use semio_framework_value::list::{PagedListError, PagedListAllocationError, PagedListRefusalKind};
use std::mem::size_of;
use semio_framework_pack_format::{RawManifest, Superblock};
use semio_framework_pack_format_retained::{RetainedPackCatalogCause, RetainedPackCatalogFault, RetainedPackCloseStep, RetainedPackSegmentEvent, RetainedPackSegmentHeader, RetainedVarintCursor, RetainedVarintStep, retained_page_close_demands};
#[cfg(test)]
use semio_framework_pack_error::PackRefusal;
#[cfg(test)]
use semio_framework_replication_codec::{ChunkId, CodecId};
#[cfg(test)]
use semio_framework_pack_source::PackSink;
#[cfg(test)]
use semio_framework_pack_format_retained::{RETAINED_PACK_PAGE_BYTES, RetainedPackAnchorCursor, RetainedPackPage, RetainedPackSegmentCursor, RetainedPackSourceCursor, RetainedPackSourceEvent};

#[path = "../🏷️symbols/🎟️storage/🦀️.rs"]
mod original_symbol_storage;

#[path = "../🏷️symbols/🎮️decode/🦀️.rs"]
pub mod retained_inline_symbols;

//#region 🔖️RetainedCanonicalSource
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackChunkEntry {
    pub offset: u64,
    pub stored_len: u64,
    pub raw_len: u64,
    pub crc32: u32,
    pub blake3: [u8; 32],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackSymbolSpan {
    pub scalar_start: u64,
    pub scalar_len: u64,
    pub utf8_len: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackManifest {
    pub schema_symbol: Option<u64>,
    pub schema_hash: [u8; 32],
    pub doc_span: ByteRange,
    pub doc_frame_count: u64,
    pub symbols_span: ByteRange,
    pub chunk_table_span: ByteRange,
    pub field_index_span: ByteRange,
    pub uncompressed_body_len: u64,
    pub field_count: u64,
    pub chunk_count: u64,
    pub symbol_count: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackCatalog {
    pub manifest: RetainedPackManifest,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedPackCatalogAllocationStep {
    pub progressed: bool,
    pub allocated_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedPackCatalogAllocationError {
    pub allocated_bytes: usize,
    pub fault: RetainedPackCatalogFault,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedPackCatalogProgress {
    pub symbols: usize,
    pub symbol_capacity: usize,
    pub symbol_utf8_bytes: usize,
    pub symbol_scalars: usize,
    pub symbol_scalar_capacity: usize,
    pub chunks: usize,
    pub chunk_capacity: usize,
    pub observed_chunks: usize,
    pub observed_chunk_capacity: usize,
    pub allocated_bytes: usize,
    pub partial_symbol_bytes: usize,
    pub pending_input: bool,
    pub complete: bool,
    pub handed_back: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedPackCatalogEvent {
    DocumentByte { frame: u64, index: u64, value: u8 },
    SchemaByte { index: u64, value: u8 },
    FieldIndexByte { index: u64, value: u8 },
    Item { kind: u8, index: u64 },
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedManifestPhase {
    Varint(usize, RetainedVarintCursor),
    Hash(usize),
    Complete,
}

struct RetainedManifestCursor {
    values: [u64; 14],
    hash: [u8; 32],
    phase: RetainedManifestPhase,
}

impl RetainedManifestCursor {
    fn new() -> Self {
        Self { values: [0; 14], hash: [0; 32], phase: RetainedManifestPhase::Varint(0, RetainedVarintCursor::default()) }
    }

    fn admit(&mut self, byte: u8, offset: u64) -> Result<bool, RetainedPackCatalogFault> {
        match self.phase {
            RetainedManifestPhase::Varint(field, mut cursor) => match cursor.admit(byte, offset).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-manifest"))? {
                RetainedVarintStep::Pending => self.phase = RetainedManifestPhase::Varint(field, cursor),
                RetainedVarintStep::Complete(value) => {
                    self.values[field] = value;
                    self.phase = if field == 0 {
                        RetainedManifestPhase::Hash(0)
                    } else if field == 13 {
                        RetainedManifestPhase::Complete
                    } else {
                        RetainedManifestPhase::Varint(field + 1, RetainedVarintCursor::default())
                    };
                }
            },
            RetainedManifestPhase::Hash(index) => {
                self.hash[index] = byte;
                self.phase = if index + 1 == self.hash.len() { RetainedManifestPhase::Varint(1, RetainedVarintCursor::default()) } else { RetainedManifestPhase::Hash(index + 1) };
            }
            RetainedManifestPhase::Complete => {}
        }
        Ok(matches!(self.phase, RetainedManifestPhase::Complete))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// 🔤️ Validates UTF8 scalars incrementally without retaining a text allocation.
pub struct RetainedUtf8Cursor {
    value: u32,
    minimum: u32,
    remaining: u8,
}

impl RetainedUtf8Cursor {
    pub fn admit(&mut self,byte:u8,offset:u64)->Result<Option<char>,RetainedPackCatalogFault>{
        if self.remaining == 0 {
            match byte {
                0x00..=0x7f => return Ok(Some(byte as char)),
                0xc2..=0xdf => {
                    self.value = (byte & 0x1f) as u32;
                    self.minimum = 0x80;
                    self.remaining = 1;
                }
                0xe0..=0xef => {
                    self.value = (byte & 0x0f) as u32;
                    self.minimum = 0x800;
                    self.remaining = 2;
                }
                0xf0..=0xf4 => {
                    self.value = (byte & 0x07) as u32;
                    self.minimum = 0x10000;
                    self.remaining = 3;
                }
                _ => return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-utf8-leading",offset)),
            }
            return Ok(None);
        }
        if byte & 0xc0 != 0x80 {
            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-utf8-continuation",offset));
        }
        self.value = (self.value << 6) | (byte & 0x3f) as u32;
        self.remaining -= 1;
        if self.remaining != 0 {
            return Ok(None);
        }
        let value = self.value;
        if value < self.minimum || (0xd800..=0xdfff).contains(&value) || value > 0x10ffff {
            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-utf8-scalar",offset));
        }
        char::from_u32(value).map(Some).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-utf8-scalar",offset))
    }

    pub fn complete(&self) -> bool {
        self.remaining == 0
    }

    fn preview(self,byte:u8,offset:u64)->Result<Option<char>,RetainedPackCatalogFault>{
        let mut cursor = self;
        cursor.admit(byte,offset)
    }
}

#[derive(Clone, Copy, Debug)]
enum RetainedSymbolsPhase {
    Count(RetainedVarintCursor),
    Length(RetainedVarintCursor),
    Text { scalar_start: usize, utf8_len: usize, remaining: usize },
    Complete,
}

struct RetainedSymbolsCursor {
    phase: RetainedSymbolsPhase,
    expected: usize,
    maximum: usize,
    maximum_utf8_bytes: usize,
    maximum_scalars: usize,
    total_utf8_bytes: usize,
    utf8: RetainedUtf8Cursor,
    closed: bool,
}

impl RetainedSymbolsCursor {
    fn new(maximum: usize, maximum_utf8_bytes: usize, maximum_scalars: usize) -> Self {
        Self { phase: RetainedSymbolsPhase::Count(RetainedVarintCursor::default()), expected: 0, maximum, maximum_utf8_bytes, maximum_scalars, total_utf8_bytes: 0, utf8: RetainedUtf8Cursor::default(), closed: false }
    }

    fn admit(&mut self,byte:u8,offset:u64,symbols:&mut RetainedPackSymbolTable)->Result<Option<u64>,RetainedPackCatalogFault>{
        match self.phase {
            RetainedSymbolsPhase::Count(mut cursor) => match cursor.admit(byte, offset).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-symbol-count-varint"))? {
                RetainedVarintStep::Pending => self.phase = RetainedSymbolsPhase::Count(cursor),
                RetainedVarintStep::Complete(count) => {
                    self.expected=usize::try_from(count).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-symbol-count",offset))?;
                    if self.expected > self.maximum {
                        return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-symbol-count",offset));
                    }
                    self.phase = if self.expected == 0 { RetainedSymbolsPhase::Complete } else { RetainedSymbolsPhase::Length(RetainedVarintCursor::default()) };
                }
            },
            RetainedSymbolsPhase::Length(mut cursor) => match cursor.admit(byte, offset).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-symbol-length-varint"))? {
                RetainedVarintStep::Pending => self.phase = RetainedSymbolsPhase::Length(cursor),
                RetainedVarintStep::Complete(len) => {
                    let len=usize::try_from(len).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-symbol-bytes",offset))?;
                    self.total_utf8_bytes=self.total_utf8_bytes.checked_add(len).filter(|total|*total<=self.maximum_utf8_bytes).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-symbol-bytes",offset))?;
                    let scalar_start = symbols.scalar_len();
                    if len == 0 {
                        let index = symbols
                            .push_symbol_reserved(RetainedPackSymbolSpan { scalar_start: scalar_start as u64, scalar_len: 0, utf8_len: 0 }, offset)
                            ?;
                        self.phase = if symbols.len() == self.expected { RetainedSymbolsPhase::Complete } else { RetainedSymbolsPhase::Length(RetainedVarintCursor::default()) };
                        return Ok(Some(index));
                    }
                    self.phase = RetainedSymbolsPhase::Text { scalar_start, utf8_len: len, remaining: len };
                }
            },
            RetainedSymbolsPhase::Text { scalar_start, utf8_len, remaining } => {
                if remaining == 0 {
                    return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-symbol-state",offset));
                }
                if let Some(character)=self.utf8.admit(byte,offset)?{
                    if symbols.scalar_len() == self.maximum_scalars {
                        return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-symbol-scalars",offset));
                    }
                    symbols.push_scalar_reserved(character,offset)?;
                }
                let remaining = remaining - 1;
                if remaining == 0 {
                    if !self.utf8.complete() {
                        return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-utf8-truncated",offset));
                    }
                    let scalar_len=symbols.scalar_len().checked_sub(scalar_start).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-symbol-span",offset))?;
                    let index = symbols
                        .push_symbol_reserved(
                            RetainedPackSymbolSpan { scalar_start: scalar_start as u64, scalar_len: scalar_len as u64, utf8_len: utf8_len as u64 },
                            offset,
                        )
                        ?;
                    self.phase = if symbols.len() == self.expected { RetainedSymbolsPhase::Complete } else { RetainedSymbolsPhase::Length(RetainedVarintCursor::default()) };
                    return Ok(Some(index));
                }
                self.phase = RetainedSymbolsPhase::Text { scalar_start, utf8_len, remaining };
            }
            RetainedSymbolsPhase::Complete=>return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-symbol-trailing",offset)),
        }
        Ok(None)
    }

    fn partial_utf8_bytes(&self) -> usize {
        match self.phase {
            RetainedSymbolsPhase::Text { utf8_len, remaining, .. } => utf8_len - remaining,
            _ => 0,
        }
    }

    fn close(&mut self) {
        self.phase = RetainedSymbolsPhase::Complete;
        self.expected = 0;
        self.total_utf8_bytes = 0;
        self.utf8 = RetainedUtf8Cursor::default();
        self.closed = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RetainedChunksPhase {
    Count(RetainedVarintCursor),
    Varint(usize, RetainedVarintCursor),
    Crc(usize),
    Hash(usize),
    Complete,
}

struct RetainedChunksCursor {
    phase: RetainedChunksPhase,
    expected: usize,
    maximum: usize,
    values: [u64; 3],
    crc: [u8; 4],
    hash: [u8; 32],
}

impl RetainedChunksCursor {
    fn new(maximum: usize) -> Self {
        Self { phase: RetainedChunksPhase::Count(RetainedVarintCursor::default()), expected: 0, maximum, values: [0; 3], crc: [0; 4], hash: [0; 32] }
    }

    fn admit<const N:usize>(&mut self,byte:u8,offset:u64,limits:&PackLimits,chunks:&mut semio_framework_value::list::PagedList<RetainedPackChunkEntry,N>)->Result<Option<u64>,RetainedPackCatalogFault>{
        match self.phase {
            RetainedChunksPhase::Count(mut cursor)=>match cursor.admit(byte,offset).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-chunk-count-varint"))?{
                RetainedVarintStep::Pending => self.phase = RetainedChunksPhase::Count(cursor),
                RetainedVarintStep::Complete(count) => {
                    self.expected=usize::try_from(count).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-chunk-count",offset))?;
                    if self.expected > self.maximum {
                        return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-chunk-count",offset));
                    }
                    self.phase = if self.expected == 0 { RetainedChunksPhase::Complete } else { RetainedChunksPhase::Varint(0, RetainedVarintCursor::default()) };
                }
            },
            RetainedChunksPhase::Varint(field,mut cursor)=>match cursor.admit(byte,offset).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-chunk-varint"))?{
                RetainedVarintStep::Pending => self.phase = RetainedChunksPhase::Varint(field, cursor),
                RetainedVarintStep::Complete(value) => {
                    self.values[field] = value;
                    if field == 2 {
                        if self.values[1] > limits.max_segment_len || self.values[2] > limits.max_segment_len {
                            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-chunk-length",offset));
                        }
                        self.phase = RetainedChunksPhase::Crc(0);
                    } else {
                        self.phase = RetainedChunksPhase::Varint(field + 1, RetainedVarintCursor::default());
                    }
                }
            },
            RetainedChunksPhase::Crc(index) => {
                self.crc[index] = byte;
                self.phase = if index + 1 == 4 { RetainedChunksPhase::Hash(0) } else { RetainedChunksPhase::Crc(index + 1) };
            }
            RetainedChunksPhase::Hash(index) => {
                self.hash[index] = byte;
                if index + 1 == 32 {
                    chunks.push_reserved(RetainedPackChunkEntry{offset:self.values[0],stored_len:self.values[1],raw_len:self.values[2],crc32:u32::from_le_bytes(self.crc),blake3:self.hash}).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-chunk-allocation",offset))?;
                    let item = chunks.len() as u64 - 1;
                    self.phase = if chunks.len() == self.expected { RetainedChunksPhase::Complete } else { RetainedChunksPhase::Varint(0, RetainedVarintCursor::default()) };
                    return Ok(Some(item));
                }
                self.phase = RetainedChunksPhase::Hash(index + 1);
            }
            RetainedChunksPhase::Complete=>return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-chunk-trailing",offset)),
        }
        Ok(None)
    }
}

const RETAINED_PACK_MAXIMUM_SYMBOL_SPANS: usize = isize::MAX as usize / size_of::<RetainedPackSymbolSpan>();

const RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS: usize = isize::MAX as usize / size_of::<char>();

const RETAINED_PACK_MAXIMUM_CHUNK_ENTRIES: usize = isize::MAX as usize / size_of::<RetainedPackChunkEntry>();

const RETAINED_PACK_MAXIMUM_OBSERVED_CHUNKS: usize = isize::MAX as usize / size_of::<RetainedPackSegmentHeader>();

type RetainedPackSymbolSpans = semio_framework_value::list::PagedList<RetainedPackSymbolSpan, RETAINED_PACK_MAXIMUM_SYMBOL_SPANS>;

type RetainedPackSymbolScalars = semio_framework_value::list::PagedList<char, RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS>;

type RetainedPackChunkEntries = semio_framework_value::list::PagedList<RetainedPackChunkEntry, RETAINED_PACK_MAXIMUM_CHUNK_ENTRIES>;

type RetainedPackObservedChunks = semio_framework_value::list::PagedList<RetainedPackSegmentHeader, RETAINED_PACK_MAXIMUM_OBSERVED_CHUNKS>;

pub struct RetainedPackSymbolTable {
    maximum_symbols: usize,
    maximum_utf8_bytes: usize,
    maximum_scalars: usize,
    maximum_allocation_bytes: usize,
    symbols: RetainedPackSymbolSpans,
    scalars: RetainedPackSymbolScalars,
    published_scalars: usize,
    pending_utf8_bytes: usize,
    utf8_bytes: usize,
    fault: Option<RetainedPackCatalogFault>,
    closing: bool,
    closed: bool,
    storage_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<original_symbol_storage::SymbolStorage>>,
    storage_retiring_bytes: usize,
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

    fn remember<T>(&mut self, result: Result<T, RetainedPackCatalogFault>) -> Result<T, RetainedPackCatalogFault> {
        match result {
            Ok(value) => Ok(value),
            Err(fault) => {
                let first = *self.fault.get_or_insert(fault);
                Err(first)
            }
        }
    }

    fn allocated_bytes_checked(&self) -> Option<usize> {
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
    fn scalar_ptr(&self) -> Option<*const char> {
        self.scalars.backing_ptr(0)
    }
}

impl Drop for RetainedPackSymbolTable {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained symbol table reached Drop before terminal-empty close");
    }
}

#[derive(Clone, Copy)]
enum RetainedPackCatalogAllocationOwner {
    SymbolSpans,
    SymbolScalars,
    Chunks,
    ObservedChunks,
}

pub struct RetainedPackCatalogCursor {
    limits: PackLimits,
    maximum_symbols: usize,
    maximum_symbol_utf8_bytes: usize,
    maximum_symbol_scalars: usize,
    maximum_chunks: usize,
    maximum_allocation_bytes: usize,
    symbols: RetainedPackSymbolTable,
    chunks: RetainedPackChunkEntries,
    observed_chunks: RetainedPackObservedChunks,
    manifest: RetainedManifestCursor,
    symbol_parser: RetainedSymbolsCursor,
    chunk_parser: RetainedChunksCursor,
    pending: Option<RetainedPackSegmentEvent>,
    active: Option<RetainedPackSegmentHeader>,
    manifest_span: Option<ByteRange>,
    symbols_span: Option<ByteRange>,
    chunks_span: Option<ByteRange>,
    document_frames: u64,
    document_bytes: u64,
    document_span: Option<ByteRange>,
    document_hash: semio_framework_hash::Hasher,
    schema_bytes: u64,
    field_index_bytes: u64,
    complete: bool,
    handed_back: bool,
    closing: bool,
    fault: Option<RetainedPackCatalogFault>,
    closed: bool,
}

impl RetainedPackCatalogCursor {
    /// 📏️ Borrows the exact currently retained catalog frame or page path for close.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        let frame = |copy_bytes| Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
        if self.pending.is_some() { return frame(size_of::<RetainedPackSegmentEvent>()); }
        if self.active.is_some() { return frame(size_of::<RetainedPackSegmentHeader>()); }
        if !self.symbol_parser.closed { return frame(size_of::<RetainedSymbolsCursor>()); }
        if !self.symbols.scalars.is_empty() { return retained_page_close_demands(&self.symbols.scalars); }
        if !self.symbols.symbols.is_empty() { return retained_page_close_demands(&self.symbols.symbols); }
        if !self.chunks.is_empty() { return retained_page_close_demands(&self.chunks); }
        if !self.observed_chunks.is_empty() { return retained_page_close_demands(&self.observed_chunks); }
        if !self.symbols.terminal_is_empty() { return self.symbols.retirement_demands(); }
        if !self.chunks.terminal_is_empty() { return retained_page_close_demands(&self.chunks); }
        if !self.observed_chunks.terminal_is_empty() { return retained_page_close_demands(&self.observed_chunks); }
        frame(0)
    }
    pub fn try_new(
        limits: PackLimits,
        maximum_symbols: usize,
        maximum_symbol_utf8_bytes: usize,
        maximum_symbol_scalars: usize,
        maximum_chunks: usize,
        maximum_allocation_bytes: usize,
    ) -> Result<Self, RetainedPackCatalogFault> {
        let physical = maximum_symbols > RETAINED_PACK_MAXIMUM_SYMBOL_SPANS
            || maximum_symbol_scalars > RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS
            || maximum_chunks > RETAINED_PACK_MAXIMUM_CHUNK_ENTRIES.min(RETAINED_PACK_MAXIMUM_OBSERVED_CHUNKS)
            || maximum_allocation_bytes > isize::MAX as usize;
        if physical{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-credits",0));}
        if maximum_symbols as u64>u64::from(limits.max_symbols)||maximum_chunks as u64>limits.max_items||maximum_symbol_utf8_bytes as u64>limits.max_file_len{
            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-credits",0));
        }
        if maximum_symbol_scalars>maximum_symbol_utf8_bytes{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-credits",0));}
        Ok(Self {
            limits,
            maximum_symbols,
            maximum_symbol_utf8_bytes,
            maximum_symbol_scalars,
            maximum_chunks,
            maximum_allocation_bytes,
            symbols: RetainedPackSymbolTable::try_new(maximum_symbols, maximum_symbol_utf8_bytes, maximum_symbol_scalars, maximum_allocation_bytes)?,
            chunks: RetainedPackChunkEntries::default(),
            observed_chunks: RetainedPackObservedChunks::default(),
            manifest: RetainedManifestCursor::new(),
            symbol_parser: RetainedSymbolsCursor::new(maximum_symbols, maximum_symbol_utf8_bytes, maximum_symbol_scalars),
            chunk_parser: RetainedChunksCursor::new(maximum_chunks),
            pending: None,
            active: None,
            manifest_span: None,
            symbols_span: None,
            chunks_span: None,
            document_frames: 0,
            document_bytes: 0,
            document_span: None,
            document_hash: semio_framework_hash::Hasher::new(),
            schema_bytes: 0,
            field_index_bytes: 0,
            complete: false,
            handed_back: false,
            closing: false,
            fault: None,
            closed: false,
        })
    }

    pub fn admit(&mut self, event: RetainedPackSegmentEvent) -> Result<(), RetainedPackSegmentEvent> {
        if self.closed || self.closing || self.complete || self.fault.is_some() || self.pending.is_some() {
            return Err(event);
        }
        self.pending = Some(event);
        Ok(())
    }

    fn allocated_bytes_checked(&self) -> Option<usize> {
        self.symbols
            .allocated_bytes()
            .checked_add(self.chunks.allocated_bytes())?
            .checked_add(self.observed_chunks.allocated_bytes())
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated_bytes_checked().unwrap_or(usize::MAX)
    }

    fn allocation_need(&self) -> Result<Option<(RetainedPackCatalogAllocationOwner, usize, u64)>, RetainedPackCatalogFault> {
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.closed || self.closing {
            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-closed",0));
        }
        let Some(event) = self.pending else { return Ok(None) };
        let need = match event {
            RetainedPackSegmentEvent::Begin(segment) if self.active.is_none() && segment.kind == semio_framework_replication_codec::KIND_CHUNK => {
                let target=self.observed_chunks.len().checked_add(1).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-observed-count",segment.offset))?;
                if target > self.maximum_chunks {
                    return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-observed-count",segment.offset));
                }
                (target > self.observed_chunks.capacity()).then_some((RetainedPackCatalogAllocationOwner::ObservedChunks, target, segment.offset))
            }
            RetainedPackSegmentEvent::RawByte { segment, index, value } if self.active == Some(segment) && segment.kind == semio_framework_replication_codec::KIND_SYMBOLS => match self.symbol_parser.phase {
                RetainedSymbolsPhase::Count(cursor)=>match cursor.preview(value,segment.payload_offset+index).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-symbol-count-varint"))?{
                    RetainedVarintStep::Pending => None,
                    RetainedVarintStep::Complete(count) => {
                        let target=usize::try_from(count).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-symbol-count",segment.payload_offset+index))?;
                        if target>self.maximum_symbols{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-symbol-count",segment.payload_offset+index));}
                        (target > self.symbols.symbol_capacity()).then_some((RetainedPackCatalogAllocationOwner::SymbolSpans, target, segment.payload_offset + index))
                    }
                },
                RetainedSymbolsPhase::Length(cursor) => {
                    if let RetainedVarintStep::Complete(len)=cursor.preview(value,segment.payload_offset+index).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-symbol-length-varint"))?{
                        let len=usize::try_from(len).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-symbol-bytes",segment.payload_offset+index))?;
                        let total=self.symbol_parser.total_utf8_bytes.checked_add(len).filter(|total|*total<=self.maximum_symbol_utf8_bytes);
                        if total.is_none() {
                            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-symbol-bytes",segment.payload_offset+index));
                        }
                    }
                    None
                }
                RetainedSymbolsPhase::Text { remaining, .. } => {
                    if remaining == 0 {
                        return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-symbol-state",segment.payload_offset+index));
                    }
                    match self.symbol_parser.utf8.preview(value,segment.payload_offset+index)?{
                        Some(_) => {
                            let target=self.symbols.scalar_len().checked_add(1).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-symbol-scalars",segment.payload_offset+index))?;
                            if target>self.maximum_symbol_scalars{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-symbol-scalars",segment.payload_offset+index));}
                            (target > self.symbols.scalar_capacity()).then_some((RetainedPackCatalogAllocationOwner::SymbolScalars, target, segment.payload_offset + index))
                        }
                        None => None,
                    }
                }
                RetainedSymbolsPhase::Complete => None,
            },
            RetainedPackSegmentEvent::RawByte { segment, index, value } if self.active == Some(segment) && segment.kind == semio_framework_replication_codec::KIND_CHUNK_TABLE => match self.chunk_parser.phase {
                RetainedChunksPhase::Count(cursor)=>match cursor.preview(value,segment.payload_offset+index).map_err(|fault|fault.into_catalog_fault("retained-pack.catalog-chunk-count-varint"))?{
                    RetainedVarintStep::Pending => None,
                    RetainedVarintStep::Complete(count) => {
                        let target=usize::try_from(count).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-chunk-count",segment.payload_offset+index))?;
                        if target>self.maximum_chunks{return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::WorkLimit,"retained-pack.catalog-chunk-count",segment.payload_offset+index));}
                        (target > self.chunks.capacity()).then_some((RetainedPackCatalogAllocationOwner::Chunks, target, segment.payload_offset + index))
                    }
                },
                _ => None,
            },
            _ => None,
        };
        Ok(need)
    }

    fn requested_allocation_bytes(&self, owner: RetainedPackCatalogAllocationOwner, target: usize, offset: u64) -> Result<usize, RetainedPackCatalogFault> {
        let result = match owner {
            RetainedPackCatalogAllocationOwner::SymbolSpans => self.symbols.symbols.next_capacity_allocation_bytes(target),
            RetainedPackCatalogAllocationOwner::SymbolScalars => self.symbols.scalars.next_capacity_allocation_bytes(target),
            RetainedPackCatalogAllocationOwner::Chunks => self.chunks.next_capacity_allocation_bytes(target),
            RetainedPackCatalogAllocationOwner::ObservedChunks => self.observed_chunks.next_capacity_allocation_bytes(target),
        };
        result
            .map_err(|error|RetainedPackCatalogFault::from_paged_refusal(error,"retained-pack.catalog-logical-capacity",offset))?
            .ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-allocation-state",offset))
    }

    pub fn next_allocation_bytes(&mut self) -> Result<Option<usize>, RetainedPackCatalogFault> {
        let need = match self.allocation_need() {
            Ok(need) => need,
            Err(fault) => return self.remember(Err(fault)),
        };
        let Some((owner, target, offset)) = need else { return Ok(None) };
        let requested = match self.requested_allocation_bytes(owner, target, offset) {
            Ok(requested) => requested,
            Err(fault) => return self.remember(Err(fault)),
        };
        let result = self.allocated_bytes_checked()
            .and_then(|allocated| allocated.checked_add(requested))
            .filter(|total| *total <= self.maximum_allocation_bytes)
            .ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-allocation-credits",offset));
        if let Err(fault) = result {
            return self.remember(Err(fault));
        }
        Ok(Some(requested))
    }

    pub fn reserve_allocation(&mut self, maximum_bytes: usize) -> Result<RetainedPackCatalogAllocationStep, RetainedPackCatalogAllocationError> {
        let need = match self.allocation_need() {
            Ok(need) => need,
            Err(fault) => {
                let first = *self.fault.get_or_insert(fault);
                return Err(RetainedPackCatalogAllocationError { allocated_bytes: 0, fault: first });
            }
        };
        let Some((owner, target, offset)) = need else {
            return Ok(RetainedPackCatalogAllocationStep::default());
        };
        let requested = self.requested_allocation_bytes(owner, target, offset).map_err(|fault| RetainedPackCatalogAllocationError { allocated_bytes: 0, fault })?;
        if maximum_bytes < requested {
            return Ok(RetainedPackCatalogAllocationStep::default());
        }
        let allocated = self.allocated_bytes_checked().ok_or(RetainedPackCatalogAllocationError {
            allocated_bytes: 0,
            fault: RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-allocation-overflow",offset),
        })?;
        let remaining = self.maximum_allocation_bytes.checked_sub(allocated).ok_or(RetainedPackCatalogAllocationError {
            allocated_bytes: 0,
            fault: RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-allocation-credits",offset),
        })?;
        if requested > remaining {
            let fault = RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-allocation-credits",offset);
            self.fault.get_or_insert(fault);
            return Err(RetainedPackCatalogAllocationError { allocated_bytes: 0, fault });
        }
        let result = match owner {
            RetainedPackCatalogAllocationOwner::SymbolSpans => self.symbols.symbols.reserve_capacity_one(target, maximum_bytes.min(remaining)),
            RetainedPackCatalogAllocationOwner::SymbolScalars => self.symbols.scalars.reserve_capacity_one(target, maximum_bytes.min(remaining)),
            RetainedPackCatalogAllocationOwner::Chunks => self.chunks.reserve_capacity_one(target, maximum_bytes.min(remaining)),
            RetainedPackCatalogAllocationOwner::ObservedChunks => self.observed_chunks.reserve_capacity_one(target, maximum_bytes.min(remaining)),
        };
        let step = match result {
            Ok(step) => step,
            Err(error) => {
                let fault = RetainedPackCatalogFault::from_paged_allocation(error,if error.allocated_bytes==0{"retained-pack.catalog-allocation"}else{"retained-pack.catalog-allocation-overgrant"},offset);
                let first = *self.fault.get_or_insert(fault);
                return Err(RetainedPackCatalogAllocationError { allocated_bytes: error.allocated_bytes, fault: first });
            }
        };
        if self.allocated_bytes() > self.maximum_allocation_bytes {
            let fault = RetainedPackCatalogFault::from_paged_allocation(PagedListAllocationError{kind:PagedListRefusalKind::OwnershipLimit,reason:"retained-pack.catalog-allocation-overgrant",allocated_bytes:step.allocated_bytes},"retained-pack.catalog-allocation-overgrant",offset);
            self.fault.get_or_insert(fault);
            return Err(RetainedPackCatalogAllocationError { allocated_bytes: step.allocated_bytes, fault });
        }
        Ok(RetainedPackCatalogAllocationStep { progressed: step.progressed, allocated_bytes: step.allocated_bytes })
    }

    fn remember<T>(&mut self, result: Result<T, RetainedPackCatalogFault>) -> Result<T, RetainedPackCatalogFault> {
        match result {
            Ok(value) => Ok(value),
            Err(fault) => {
                let first = *self.fault.get_or_insert(fault);
                Err(first)
            }
        }
    }

    fn process_event(&mut self, event: RetainedPackSegmentEvent) -> Result<Option<RetainedPackCatalogEvent>, RetainedPackCatalogFault> {
        match event {
            RetainedPackSegmentEvent::Begin(segment) => {
                if self.active.is_some() {
                    return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-nested-begin",segment.offset));
                }
                self.active = Some(segment);
                if segment.kind == semio_framework_replication_codec::KIND_DOCUMENT {
                    self.document_frames += 1;
                }
                if segment.kind == semio_framework_replication_codec::KIND_CHUNK {
                    self.observed_chunks.push_reserved(segment).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-observed-allocation",segment.offset))?;
                }
                Ok(None)
            }
            RetainedPackSegmentEvent::RawByte { segment, index, value } => {
                if self.active != Some(segment) {
                    return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-payload-without-segment",segment.offset));
                }
                match segment.kind {
                    semio_framework_replication_codec::KIND_MANIFEST => {
                        self.manifest.admit(value, segment.payload_offset + index)?;
                        Ok(None)
                    }
                    semio_framework_replication_codec::KIND_SYMBOLS => Ok(self
                        .symbol_parser
                        .admit(value, segment.payload_offset + index, &mut self.symbols)
                        ?
                        .map(|index| RetainedPackCatalogEvent::Item { kind: semio_framework_replication_codec::KIND_SYMBOLS, index })),
                    semio_framework_replication_codec::KIND_CHUNK_TABLE => {
                        let item = self.chunk_parser.admit(value, segment.payload_offset + index, &self.limits, &mut self.chunks)?;
                        if let Some(index) = item {
                            let entry = self.chunks.get(index as usize).expect("admitted retained chunk entry");
                            let observed = self.observed_chunks.get(index as usize).ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-chunk-unobserved",segment.payload_offset + index))?;
                            if entry.offset != observed.payload_offset || entry.stored_len != observed.stored_len || entry.raw_len != observed.raw_len {
                                return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-chunk-framing",segment.payload_offset + index));
                            }
                            return Ok(Some(RetainedPackCatalogEvent::Item { kind: semio_framework_replication_codec::KIND_CHUNK_TABLE, index }));
                        }
                        Ok(None)
                    }
                    semio_framework_replication_codec::KIND_DOCUMENT => {
                        let body_index = self.document_bytes;
                        self.document_bytes += 1;
                        self.document_hash.update(&[value]);
                        Ok(Some(RetainedPackCatalogEvent::DocumentByte { frame: self.document_frames - 1, index: body_index, value }))
                    }
                    semio_framework_replication_codec::KIND_CHUNK => Ok(None),
                    semio_framework_replication_codec::KIND_SCHEMA => {
                        let schema_index = self.schema_bytes;
                        self.schema_bytes += 1;
                        Ok(Some(RetainedPackCatalogEvent::SchemaByte { index: schema_index, value }))
                    }
                    semio_framework_replication_codec::KIND_FIELD_INDEX => {
                        let field_index = self.field_index_bytes;
                        self.field_index_bytes += 1;
                        Ok(Some(RetainedPackCatalogEvent::FieldIndexByte { index: field_index, value }))
                    }
                    _ => Ok(None),
                }
            }
            RetainedPackSegmentEvent::Complete { segment, wire_len } => {
                if self.active.take() != Some(segment) {
                    return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-segment-completion",segment.offset));
                }
                let span = ByteRange { offset: segment.offset, len: wire_len };
                match segment.kind {
                    semio_framework_replication_codec::KIND_MANIFEST if self.manifest_span.replace(span).is_some() => return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-manifest-duplicate",segment.offset)),
                    semio_framework_replication_codec::KIND_SYMBOLS if self.symbols_span.replace(span).is_some() => return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-symbols-duplicate",segment.offset)),
                    semio_framework_replication_codec::KIND_CHUNK_TABLE if self.chunks_span.replace(span).is_some() => return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-chunks-duplicate",segment.offset)),
                    semio_framework_replication_codec::KIND_DOCUMENT => {
                        self.document_span = Some(match self.document_span {
                            None => span,
                            Some(existing) if existing.offset + existing.len == span.offset => ByteRange { offset: existing.offset, len: existing.len + span.len },
                            Some(_) => return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-document-contiguous",segment.offset)),
                        });
                    }
                    _ => {}
                }
                Ok(Some(RetainedPackCatalogEvent::Item { kind: segment.kind, index: 0 }))
            }
            RetainedPackSegmentEvent::PackComplete { .. } => {
                if self.active.is_some()
                    || !matches!(self.manifest.phase, RetainedManifestPhase::Complete)
                    || (self.symbols_span.is_some() && !matches!(self.symbol_parser.phase, RetainedSymbolsPhase::Complete))
                    || (self.chunks_span.is_some() && !matches!(self.chunk_parser.phase, RetainedChunksPhase::Complete))
                {
                    return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-truncated",self.document_bytes));
                }
                self.complete = true;
                Ok(Some(RetainedPackCatalogEvent::Complete))
            }
        }
    }

    pub fn grant(&mut self) -> Result<Option<RetainedPackCatalogEvent>, RetainedPackCatalogFault> {
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if self.closed || self.closing {
            return Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvariantViolated,"retained-pack.catalog-closed",0));
        }
        match self.next_allocation_bytes() {
            Ok(Some(_)) => return Ok(None),
            Ok(None) => {}
            Err(fault) => return self.remember(Err(fault)),
        }
        let Some(event) = self.pending.take() else { return Ok(None) };
        let result = self.process_event(event);
        self.remember(result)
    }

    /// 🔤️ Borrows one already-verified symbol scalar without cloning the retained registry.
    pub fn symbol_char(&self, symbol: u64, character: usize) -> Result<Option<char>, RetainedPackCatalogFault> {
        self.symbols.symbol_char(symbol, character).map_err(|mut fault|{fault.code="retained-pack.catalog-symref";fault})
    }

    /// 📏️ Returns the scalar count of one retained symbol in constant indexed work.
    pub fn symbol_chars(&self, symbol: u64) -> Result<usize, RetainedPackCatalogFault> {
        self.symbols.symbol_chars(symbol).map_err(|mut fault|{fault.code="retained-pack.catalog-symref";fault})
    }

    pub fn symbol_span(&self, symbol: u64) -> Result<RetainedPackSymbolSpan, RetainedPackCatalogFault> {
        self.symbols.symbol_span(symbol).map_err(|mut fault|{fault.code="retained-pack.catalog-symref";fault})
    }

    pub fn chunk(&self, index: u64) -> Result<RetainedPackChunkEntry, RetainedPackCatalogFault> {
        let address=usize::try_from(index).map_err(|_|RetainedPackCatalogFault::refusal(ValueRefusalKind::OwnershipLimit,"retained-pack.catalog-chunk-index",index))?;
        self.chunks.get(address).copied().ok_or(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-chunk-index",index))
    }

    pub fn document_bytes(&self) -> u64 {
        self.document_bytes
    }

    pub fn take(&mut self, superblock: Superblock) -> Result<Option<RetainedPackCatalog>, RetainedPackCatalogFault> {
        if let Some(fault) = self.fault {
            return Err(fault);
        }
        if !self.complete || self.handed_back {
            return Ok(None);
        }
        let values = self.manifest.values;
        let raw = RawManifest {
            schema_symref: values[0],
            schema_hash: self.manifest.hash,
            doc_span: ByteRange { offset: values[1], len: values[2] },
            doc_frame_count: values[3],
            symbols_span: ByteRange { offset: values[4], len: values[5] },
            chunk_table_span: ByteRange { offset: values[6], len: values[7] },
            field_index_span: ByteRange { offset: values[8], len: values[9] },
            uncompressed_body_len: values[10],
            field_count: values[11],
            chunk_count: values[12],
            symbol_count: values[13],
        };
        if self.manifest_span != Some(ByteRange { offset: superblock.footer.manifest_offset, len: superblock.footer.manifest_len })
            || self.symbols_span.unwrap_or(ByteRange { offset: 0, len: 0 }) != raw.symbols_span
            || self.chunks_span.unwrap_or(ByteRange { offset: 0, len: 0 }) != raw.chunk_table_span
        {
            return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-manifest-span",superblock.footer.manifest_offset)));
        }
        if self.document_span.unwrap_or(ByteRange { offset: 0, len: 0 }) != raw.doc_span
            || raw.doc_frame_count != self.document_frames
            || raw.uncompressed_body_len != self.document_bytes
            || raw.symbol_count != self.symbols.len() as u64
            || raw.chunk_count != self.chunks.len() as u64
            || self.observed_chunks.len() != self.chunks.len()
        {
            return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-manifest-count",superblock.footer.manifest_offset)));
        }
        if self.document_hash.finalize().as_bytes() != &superblock.footer.content_hash.0 {
            return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-content-hash",superblock.footer.file_len)));
        }
        let schema_symbol = if self.symbols.len() == 0 && raw.schema_symref == 0 {
            None
        } else if raw.schema_symref < self.symbols.len() as u64 {
            Some(raw.schema_symref)
        } else {
            return self.remember(Err(RetainedPackCatalogFault::refusal(ValueRefusalKind::InvalidValue,"retained-pack.catalog-schema-symref",raw.schema_symref)));
        };
        let manifest = RetainedPackManifest {
            schema_symbol,
            schema_hash: raw.schema_hash,
            doc_span: raw.doc_span,
            doc_frame_count: raw.doc_frame_count,
            symbols_span: raw.symbols_span,
            chunk_table_span: raw.chunk_table_span,
            field_index_span: raw.field_index_span,
            uncompressed_body_len: raw.uncompressed_body_len,
            field_count: raw.field_count,
            chunk_count: raw.chunk_count,
            symbol_count: raw.symbol_count,
        };
        self.handed_back = true;
        Ok(Some(RetainedPackCatalog { manifest }))
    }

    pub fn progress(&self) -> RetainedPackCatalogProgress {
        RetainedPackCatalogProgress {
            symbols: self.symbols.len(),
            symbol_capacity: self.symbols.symbol_capacity(),
            symbol_utf8_bytes: self.symbol_parser.total_utf8_bytes,
            symbol_scalars: self.symbols.scalar_len(),
            symbol_scalar_capacity: self.symbols.scalar_capacity(),
            chunks: self.chunks.len(),
            chunk_capacity: self.chunks.capacity(),
            observed_chunks: self.observed_chunks.len(),
            observed_chunk_capacity: self.observed_chunks.capacity(),
            allocated_bytes: self.allocated_bytes(),
            partial_symbol_bytes: self.symbol_parser.partial_utf8_bytes(),
            pending_input: self.pending.is_some(),
            complete: self.complete,
            handed_back: self.handed_back,
        }
    }

    pub fn fault(&self) -> Option<RetainedPackCatalogFault> {
        self.fault
    }

    pub fn has_pending_input(&self) -> bool {
        self.pending.is_some()
    }

    pub fn next_release_allocation_bytes(&self) -> Result<Option<usize>, PagedListError> {
        if self.pending.is_some()
            || self.active.is_some()
            || !self.symbol_parser.closed
            || self.symbols.scalar_len() != 0
            || self.symbols.len() != 0
            || !self.chunks.is_empty()
            || !self.observed_chunks.is_empty()
        {
            return Ok(None);
        }
        if !self.symbols.terminal_is_empty() {
            return self.symbols.next_release_allocation_bytes();
        }
        if !self.chunks.terminal_is_empty() {
            return self.chunks.next_release_allocation_bytes().map(Some);
        }
        if !self.observed_chunks.terminal_is_empty() {
            return self.observed_chunks.next_release_allocation_bytes().map(Some);
        }
        Ok(None)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<RetainedPackCloseStep, PagedListError> {
        if maximum_items == 0 && maximum_bytes == 0 {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if maximum_items == 0
            && (self.pending.is_some()
                || self.active.is_some()
                || !self.symbol_parser.closed
                || self.symbols.scalar_len() != 0
                || self.symbols.len() != 0
                || !self.chunks.is_empty()
                || !self.observed_chunks.is_empty())
        {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.closing = true;
        if maximum_items != 0 {
            if self.pending.take().is_some() {
                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.active.take().is_some() {
                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if !self.symbol_parser.closed {
                self.symbol_parser.close();
                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if self.symbols.scalars.pop().is_some() || self.symbols.symbols.pop().is_some() || self.chunks.pop().is_some() || self.observed_chunks.pop().is_some() {
                return Ok(RetainedPackCloseStep::Pending { released_items: 1, released_bytes: 0 });
            }
        }
        if self.pending.is_some()
            || self.active.is_some()
            || !self.symbol_parser.closed
            || self.symbols.scalar_len() != 0
            || self.symbols.len() != 0
            || !self.chunks.is_empty()
            || !self.observed_chunks.is_empty()
        {
            return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if !self.symbols.terminal_is_empty() {
            match self.symbols.close_step(maximum_items, maximum_bytes)? {
                RetainedPackCloseStep::Pending { released_items, released_bytes } => {
                    return Ok(RetainedPackCloseStep::Pending { released_items, released_bytes });
                }
                RetainedPackCloseStep::Complete => {}
            }
        }
        for owner in [RetainedPackCatalogAllocationOwner::Chunks, RetainedPackCatalogAllocationOwner::ObservedChunks] {
            let (terminal, step) = match owner {
                RetainedPackCatalogAllocationOwner::Chunks => (self.chunks.terminal_is_empty(), self.chunks.release_empty_page(maximum_bytes)),
                RetainedPackCatalogAllocationOwner::ObservedChunks => (self.observed_chunks.terminal_is_empty(), self.observed_chunks.release_empty_page(maximum_bytes)),
                _ => unreachable!(),
            };
            if !terminal {
                let step = step?;
                return Ok(RetainedPackCloseStep::Pending { released_items: 0, released_bytes: step.released_allocation_bytes });
            }
        }
        self.closed = true;
        self.handed_back = true;
        Ok(RetainedPackCloseStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed
            && self.handed_back
            && self.pending.is_none()
            && self.active.is_none()
            && self.symbol_parser.closed
            && self.symbol_parser.partial_utf8_bytes() == 0
            && self.symbols.terminal_is_empty()
            && self.chunks.terminal_is_empty()
            && self.observed_chunks.terminal_is_empty()
    }

    #[cfg(test)]
    fn retained_symbol_scalar_ptr(&self) -> Option<*const char> {
        self.symbols.scalar_ptr()
    }
}

impl Drop for RetainedPackCatalogCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "retained pack catalog reached Drop before handback or terminal-empty close");
    }
}
//#endregion 🔖️RetainedCanonicalSource

#[cfg(test)]
#[path = "../🧪️tests/🔬️retained-pack-catalog-laws/🦀️.rs"]
mod retained_pack_catalog_laws;

#[cfg(test)]
#[path = "../🧭️producer/🗂️catalog/🧪️tests/🦀️.rs"]
mod producer_authority_tests;

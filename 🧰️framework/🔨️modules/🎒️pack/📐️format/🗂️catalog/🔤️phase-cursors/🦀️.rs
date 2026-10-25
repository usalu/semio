//! 🖱️ Byte-driven phase cursors: the manifest varint cursor, the UTF-8 scalar cursor, the symbol-table cursor and the chunk-table cursor.

use semio_framework_replication_codec::PackLimits;
use semio_framework_value::ValueRefusalKind;
use semio_framework_pack_format_retained::{RetainedPackCatalogFault, RetainedVarintCursor, RetainedVarintStep};
use super::{RetainedPackChunkEntry, RetainedPackSymbolSpan, RetainedPackSymbolTable};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RetainedManifestPhase {
    Varint(usize, RetainedVarintCursor),
    Hash(usize),
    Complete,
}

pub(super) struct RetainedManifestCursor {
    pub(super) values: [u64; 14],
    pub(super) hash: [u8; 32],
    pub(super) phase: RetainedManifestPhase,
}

impl RetainedManifestCursor {
    pub(super) fn new() -> Self {
        Self { values: [0; 14], hash: [0; 32], phase: RetainedManifestPhase::Varint(0, RetainedVarintCursor::default()) }
    }

    pub(super) fn admit(&mut self, byte: u8, offset: u64) -> Result<bool, RetainedPackCatalogFault> {
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
    pub(super) value: u32,
    minimum: u32,
    pub(super) remaining: u8,
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

    pub(super) fn preview(self,byte:u8,offset:u64)->Result<Option<char>,RetainedPackCatalogFault>{
        let mut cursor = self;
        cursor.admit(byte,offset)
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum RetainedSymbolsPhase {
    Count(RetainedVarintCursor),
    Length(RetainedVarintCursor),
    Text { scalar_start: usize, utf8_len: usize, remaining: usize },
    Complete,
}

pub(super) struct RetainedSymbolsCursor {
    pub(super) phase: RetainedSymbolsPhase,
    pub(super) expected: usize,
    maximum: usize,
    pub(super) maximum_utf8_bytes: usize,
    pub(super) maximum_scalars: usize,
    pub(super) total_utf8_bytes: usize,
    pub(super) utf8: RetainedUtf8Cursor,
    pub(super) closed: bool,
}

impl RetainedSymbolsCursor {
    pub(super) fn new(maximum: usize, maximum_utf8_bytes: usize, maximum_scalars: usize) -> Self {
        Self { phase: RetainedSymbolsPhase::Count(RetainedVarintCursor::default()), expected: 0, maximum, maximum_utf8_bytes, maximum_scalars, total_utf8_bytes: 0, utf8: RetainedUtf8Cursor::default(), closed: false }
    }

    pub(super) fn admit(&mut self,byte:u8,offset:u64,symbols:&mut RetainedPackSymbolTable)->Result<Option<u64>,RetainedPackCatalogFault>{
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

    pub(super) fn partial_utf8_bytes(&self) -> usize {
        match self.phase {
            RetainedSymbolsPhase::Text { utf8_len, remaining, .. } => utf8_len - remaining,
            _ => 0,
        }
    }

    pub(super) fn close(&mut self) {
        self.phase = RetainedSymbolsPhase::Complete;
        self.expected = 0;
        self.total_utf8_bytes = 0;
        self.utf8 = RetainedUtf8Cursor::default();
        self.closed = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RetainedChunksPhase {
    Count(RetainedVarintCursor),
    Varint(usize, RetainedVarintCursor),
    Crc(usize),
    Hash(usize),
    Complete,
}

pub(super) struct RetainedChunksCursor {
    pub(super) phase: RetainedChunksPhase,
    pub(super) expected: usize,
    maximum: usize,
    pub(super) values: [u64; 3],
    crc: [u8; 4],
    pub(super) hash: [u8; 32],
}

impl RetainedChunksCursor {
    pub(super) fn new(maximum: usize) -> Self {
        Self { phase: RetainedChunksPhase::Count(RetainedVarintCursor::default()), expected: 0, maximum, values: [0; 3], crc: [0; 4], hash: [0; 32] }
    }

    pub(super) fn admit<const N:usize>(&mut self,byte:u8,offset:u64,limits:&PackLimits,chunks:&mut semio_framework_value_persistent::list::PagedList<RetainedPackChunkEntry,N>)->Result<Option<u64>,RetainedPackCatalogFault>{
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

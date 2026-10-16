//! 📋️ The public catalog records: chunk entries, symbol spans, the manifest, allocation steps, progress and events.

use semio_framework_replication_codec::ByteRange;
use semio_framework_pack_format_retained::RetainedPackCatalogFault;

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

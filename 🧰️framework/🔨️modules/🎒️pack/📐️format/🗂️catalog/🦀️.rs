//! 🗂️ Retained canonical Pack catalog: bounded, cancellable cursors for the manifest, the symbol table, the chunk table and the inline symbols.
//! They consume the segment events admitted by `semio-framework-pack-format-retained`.

#[cfg(test)]
use semio_framework_replication_codec::{ByteRange, PackLimits};
use semio_framework_pack_format_retained::RetainedPackCatalogFault;
#[cfg(test)]
use semio_framework_pack_format_retained::{RetainedPackCatalogCause, RetainedPackCloseStep, RetainedPackSegmentEvent, RetainedPackSegmentHeader, RetainedVarintCursor, RetainedVarintStep};
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
#[path = "📋️catalog-records/🦀️.rs"]
mod catalog_records;
pub use catalog_records::{RetainedPackCatalog, RetainedPackCatalogAllocationError, RetainedPackCatalogAllocationStep, RetainedPackCatalogEvent, RetainedPackCatalogProgress, RetainedPackChunkEntry, RetainedPackManifest, RetainedPackSymbolSpan};

#[path = "🔤️phase-cursors/🦀️.rs"]
mod phase_cursors;
use phase_cursors::{RetainedChunksCursor, RetainedChunksPhase, RetainedManifestCursor, RetainedManifestPhase, RetainedSymbolsCursor, RetainedSymbolsPhase};
pub use phase_cursors::RetainedUtf8Cursor;

#[path = "🗃️symbol-table/🦀️.rs"]
mod symbol_table;
use symbol_table::{RETAINED_PACK_MAXIMUM_CHUNK_ENTRIES, RETAINED_PACK_MAXIMUM_OBSERVED_CHUNKS, RETAINED_PACK_MAXIMUM_SYMBOL_SCALARS, RETAINED_PACK_MAXIMUM_SYMBOL_SPANS, RetainedPackChunkEntries, RetainedPackObservedChunks, RetainedPackSymbolScalars, RetainedPackSymbolSpans};
pub use symbol_table::RetainedPackSymbolTable;

#[path = "🧭️catalog-cursor/🦀️.rs"]
mod catalog_cursor;
pub use catalog_cursor::RetainedPackCatalogCursor;
//#endregion 🔖️RetainedCanonicalSource

#[cfg(test)]
#[path = "../🧪️tests/🔬️retained-pack-catalog-laws/🦀️.rs"]
mod retained_pack_catalog_laws;

#[cfg(test)]
#[path = "../🧭️producer/🗂️catalog/🧪️tests/🦀️.rs"]
mod producer_authority_tests;

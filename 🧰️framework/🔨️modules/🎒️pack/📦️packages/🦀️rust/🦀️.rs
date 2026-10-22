//! 📦️ Package glue — wiring only. Domain lives at the owner `🦀️.rs` files.
//!
//! The crate's `[lib] name` is `pack`: the container format keeps the one canonical name every
//! downstream caller already spells.

// 🔓️ R7: `async fn` in a public trait is our deliberate crate-wide shape (R3 answers the lint's
// underlying `Send` concern structurally — see `AsyncPackSource`/`RangeTransport`). Never take
// rustc's suggested `-> impl Future<...> + Send` fix; it re-imposes `Send` R3 forbids.
#![allow(async_fn_in_trait)]
#![allow(ambiguous_glob_reexports, unused_imports)]

// 📡️ Container and record streams share replication codecs and the independent pack byte owner.
#[cfg(test)]
pub(crate) use semio_framework_trace_memory_testing as test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

extern crate self as pack;

pub use semio_framework_replication_codec as codec;
pub use semio_framework_replication_codec::ids;
/// 🌱️ Re-exported so a crate that already depends on `pack` (e.g. `🏗️mesh-engine`) can implement
/// `ToValue`/`FromValue` for its own types without taking a second, direct dependency on
/// `replication` — implementing a foreign trait for a local type is allowed by the orphan rule.
pub use semio_framework_value as value;

pub mod format {
    pub use semio_framework_pack_format::{ControlledPackWriter, FOOTER_MAGIC, FOOTER_SIZE, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, Footer, HEADER_SIZE, Header, MAGIC, Manifest, OPTIONAL_CANONICAL, OPTIONAL_HAS_SCHEMA, OPTIONAL_STREAMED, PackFile, PackIdentityChunk, PackIdentityChunkCursor, PackIdentitySegment, PackWriter, REQUIRED_CHUNKED, REQUIRED_COMPRESSED, REQUIRED_ENCRYPTED, REQUIRED_FOOTER_CHAIN, RecoveryReport, Superblock, VerificationLevel, WriteOptions, encode_symbols, read_footer_only, recover};
    pub use semio_framework_pack_format_retained::{RETAINED_PACK_MAXIMUM_PAGES, RETAINED_PACK_PAGE_BYTES, RetainedPackAnchorCursor, RetainedPackCatalogCause, RetainedPackCatalogFault, RetainedPackCloseStep, RetainedPackPage, RetainedPackSegmentAdmission, RetainedPackSegmentCursor, RetainedPackSegmentEvent, RetainedPackSegmentHeader, RetainedPackSourceAllocationError, RetainedPackSourceAllocationStep, RetainedPackSourceCursor, RetainedPackSourceEvent, RetainedPackSourceFault, RetainedPackSourceProgress, RetainedVarintCursor, RetainedVarintFault, RetainedVarintStep};
    pub use semio_framework_pack_format_catalog::{RetainedPackCatalog, RetainedPackCatalogAllocationError, RetainedPackCatalogAllocationStep, RetainedPackCatalogCursor, RetainedPackCatalogEvent, RetainedPackCatalogProgress, RetainedPackChunkEntry, RetainedPackManifest, RetainedPackSymbolSpan, RetainedPackSymbolTable, RetainedUtf8Cursor};
    pub use semio_framework_pack_format_catalog::retained_inline_symbols;
}

pub mod async_ {
    pub use semio_framework_pack_async::{AsyncPackSource, BoundedDemand, CancellationToken, DemandPermit, LoadPriority, ReadRequest, ReadScheduler};
}

pub mod http {
    pub use semio_framework_pack_http::{HttpPackSource, RangeRequest, RangeResponse, RangeTransport, RetryPolicy, RetryRuntime};
}

#[cfg(not(target_arch = "wasm32"))]
pub mod io {
    pub use semio_framework_pack_io::{FilePackSink, FilePackSource, StreamingPackWriter, recover_file, write_atomic};
}

#[cfg(any(test, feature = "corruption-testing"))]
#[path = "../../🧪️tests/🧨️corruption/🦀️.rs"]
pub mod corruption_testing;

#[path = "../../🦀️.rs"]
mod component;
pub use component::*;

pub use semio_framework_replication_codec::ids::*;
pub use semio_framework_replication_codec::*;

#[cfg(test)]
#[path = "../../🧪️tests/🧭️producer-authority/🦀️.rs"]
mod producer_authority_tests;

#[cfg(test)]
#[path = "../../🧪️tests/📡️codec-authority/🦀️.rs"]
mod codec_authority_tests;

pub use semio_framework_pack_error::{PackError, PackRefusal, PackCauseKind, PackRetryDisposition, PackTransportCategory, PackTransportContext, PackTransportPolicy, PackTransportProgress, PackTransportPhase, TransportAdmission, TransportContextRefusal, TransportContextRefusalCause};

pub mod record {
    pub use semio_framework_pack_record::{DecodeOptions, DecodeReport, EncodeOptions, decode_document, decode_document_controlled, decode_record_body, decode_record_body_exact, decode_record_body_exact_controlled, decode_record_body_span_exact_controlled, decode_value_record_body_exact, decode_value_record_body_exact_controlled, decode_value_record_body_span_exact_controlled, encode_document, encode_record_body, intrinsic, scalar_witness};
    pub use semio_framework_pack_record_schema::{PackSchemaField, PackSchemaGraph, PackSchemaShape, schema_hash, schema_hash_controlled};
    pub use semio_framework_pack_record_retained::{RetainedRecordBodyAllocationError, RetainedRecordBodyAllocationStep, RetainedRecordBodyCursor, RetainedRecordBodyReadLimits, RetainedRecordBodySourceDecode, RetainedRecordBodySourceStep, RetainedRecordBodyToken, RetainedRecordReceiving, RetainedValueAllocationError, RetainedValueAllocationStep, RetainedValueContainer, RetainedValueCursor, RetainedValueProgress, RetainedValueRole, RetainedValueToken};
    pub use semio_framework_pack_record_encode::{BorrowedPackSource, BorrowedProjectedPackOperation, OwnedVariantPackSource, ProjectedSymbolScratch, SourceTextKind, SourceTextLocator, encode_document_controlled, encode_projected_record_body_into, encode_record_body_controlled, encode_record_body_into, encode_value_record_body_controlled, encode_value_record_body_into};
    pub use semio_framework_pack_record_encode_cursor::{BorrowedProjectedPackCursor, BorrowedProjectedPackFailure, BorrowedProjectedPackProgress, OwnedProjectedPackWireCursor, OwnedProjectedPackWireReady, OwnedProjectedPackWireStep};
    pub use semio_framework_pack_record_preflight::{measure_document_borrowed,BorrowedSchemaHashCursor,BorrowedSchemaHashStep};
    pub use semio_framework_pack_record_encode_cursor::BorrowedPackSection;
    #[cfg(feature="deflate")]
    pub use semio_framework_pack_record_encode_cursor::{BorrowedPackDocumentCursor,BorrowedPackDocumentProgress,BorrowedPackDocumentFailure};
}

pub mod intrinsic {
    pub use semio_framework_pack_intrinsic::{IntrinsicFormat, RetainedIntrinsicBody, RetainedIntrinsicDocument, RetainedIntrinsicInput, RetainedIntrinsicOriginalCall, RetainedIntrinsicStep, decode};
}

/// 📸️ Domain-neutral representation contracts owned by framework IO.
#[path="../../../🚪️io/📸️snapshot/🦀️.rs"]
pub mod snapshot;

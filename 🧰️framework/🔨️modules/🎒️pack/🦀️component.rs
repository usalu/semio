//! 📦️ `pack` — the facade crate for the `pack` binary document format family: re-exports the
//! full public surface of `pack_core`, `pack_format`, `pack_value`, `pack_io` (native-only,
//! cfg-gated out on `wasm32`), `pack_async`, `pack_http`, and `pack_index` under one crate so
//! downstream callers (`vcs`, `dsl_derive`, apps) depend on a single `pack = { path = ... }`
//! rather than seven. Also exposes three top-level convenience functions —
//! `encode_document`/`decode_document`/`content_hash` — that are thin forwards onto
//! `pack_value`/`pack_format`.
//!
//! See the `## pack (facade)` section of the wave-0 contract at
//! `.🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md`. This crate does
//! NOT re-export `dsl_schema` itself as `crate::dsl_schema` — callers pass in
//! `&os_dsl::schema::RecordSpec`/`&os_dsl::schema::RecordValue` they already depend on directly.

//#region 🔖️Reexports

//#region 🔖️Core
pub use crate::{
    crc32c, is_minimal_varint, read_varint_i64, read_varint_u64, write_varint_i64, write_varint_u64, ByteRange, ByteReader, ByteWriter, ChunkId, CodecId, CompressionCodec, ContentHash, NoCompression, PackError, PackLimits, PackSink, PackSource,
    SegmentKind, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS,
};

/// 🆔️ Computes the canonical pack content identity for `bytes`.
pub fn content_hash(bytes: &[u8]) -> ContentHash {
    ContentHash(*blake3::hash(bytes).as_bytes())
}
//#endregion 🔖️Core

//#region 🔖️Format
#[cfg(feature = "deflate")]
pub use crate::codec::DeflateCodec;
pub use crate::format::{
    encode_symbols, read_footer_only, recover, Footer, Header, Manifest, PackFile, PackIdentitySegment, PackWriter, RecoveryReport, RetainedPackAnchorCursor, RetainedPackCatalog, RetainedPackCatalogCursor, RetainedPackCatalogEvent,
    RetainedPackChunkEntry, RetainedPackCloseStep, RetainedPackPage, RetainedPackSegmentCursor, RetainedPackSegmentEvent, RetainedPackSegmentHeader, RetainedPackSourceCursor, RetainedPackSourceEvent, RetainedPackSourceProgress, Superblock,
    VerificationLevel, WriteOptions, FOOTER_MAGIC, FOOTER_SIZE, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, HEADER_SIZE, MAGIC, OPTIONAL_CANONICAL, OPTIONAL_HAS_SCHEMA, OPTIONAL_STREAMED, REQUIRED_CHUNKED, REQUIRED_COMPRESSED, REQUIRED_ENCRYPTED,
    REQUIRED_FOOTER_CHAIN, RETAINED_PACK_PAGE_BYTES,
};
//#endregion 🔖️Format

//#region 🔖️Json
pub use crate::json::{
    array as json_array, object as json_object, parse as parse_json, parse_bytes as parse_json_bytes, to_string as json_to_string, JsonError, Lexer as JsonLexer, Number as JsonNumber, Object as JsonObject, Token as JsonToken, Value as JsonValue,
    MAX_DEPTH as JSON_MAX_DEPTH,
};
//#endregion 🔖️Json

//#region 🔖️Io
/// @emoji 🗄️ Native-only file I/O — absent from `wasm32` builds, mirroring `pack_io` itself.
#[cfg(not(target_arch = "wasm32"))]
pub use crate::io::{recover_file, write_atomic, FilePackSink, FilePackSource, StreamingPackWriter};
//#endregion 🔖️Io

//#region 🔖️Async
pub use crate::async_::{AsyncPackSource, BoundedDemand, CancellationToken, DemandPermit, LoadPriority, ReadRequest, ReadScheduler};
//#endregion 🔖️Async

//#region 🔖️Http
/// @emoji 🌐️ Native `ureq`-backed `RangeTransport` — off by default so wasm builds of this
/// facade stay lean; enable via `pack`'s own `ureq` feature (forwards to `pack_http/ureq`).
#[cfg(feature = "ureq")]
pub use crate::http::UreqRangeTransport;
pub use crate::http::{HttpPackSource, RangeRequest, RangeResponse, RangeTransport, RetryPolicy, RetryRuntime};
//#endregion 🔖️Http

//#endregion 🔖️Reexports

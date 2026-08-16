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
//! NOT re-export `dsl_schema` itself as `crate::os_pack::dsl_schema` — callers pass in
//! `&crate::os_dsl::schema::RecordSpec`/`&crate::os_dsl::schema::RecordValue` they already depend on directly.

//#region 🔖️Reexports

//#region 🔖️Core
pub use crate::os_pack::{
    crc32c, is_minimal_varint, read_varint_i64, read_varint_u64, write_varint_i64, write_varint_u64, ByteRange, ByteReader, ByteWriter, ChunkId, CodecId, CompressionCodec, ContentHash, NoCompression, PackError, PackLimits, PackSink, PackSource,
    SegmentKind, KIND_CHUNK, KIND_CHUNK_TABLE, KIND_DOCUMENT, KIND_END, KIND_FIELD_INDEX, KIND_MANIFEST, KIND_PADDING, KIND_SCHEMA, KIND_SNAPSHOT, KIND_SYMBOLS,
};
//#endregion 🔖️Core

//#region 🔖️Format
#[cfg(feature = "deflate")]
pub use crate::os_pack::format::DeflateCodec;
pub use crate::os_pack::format::{
    encode_symbols, read_footer_only, recover, Footer, Header, Manifest, PackFile, PackWriter, RecoveryReport, Superblock, VerificationLevel, WriteOptions, FOOTER_MAGIC, FOOTER_SIZE, FORMAT_VERSION_MAJOR, FORMAT_VERSION_MINOR, HEADER_SIZE, MAGIC,
    OPTIONAL_CANONICAL, OPTIONAL_HAS_SCHEMA, OPTIONAL_STREAMED, REQUIRED_CHUNKED, REQUIRED_COMPRESSED, REQUIRED_ENCRYPTED, REQUIRED_FOOTER_CHAIN,
};
//#endregion 🔖️Format

//#region 🔖️Value
pub use crate::os_pack::value::{schema_hash, DecodeOptions, DecodeReport, EncodeOptions};
//#endregion 🔖️Value

//#region 🔖️Io
/// @emoji 🗄️ Native-only file I/O — absent from `wasm32` builds, mirroring `pack_io` itself.
#[cfg(not(target_arch = "wasm32"))]
pub use crate::os_pack::io::{recover_file, write_atomic, FilePackSink, FilePackSource, StreamingPackWriter};
//#endregion 🔖️Io

//#region 🔖️Async
pub use crate::os_pack::async_::{AsyncPackSource, BoundedDemand, CancellationToken, DemandPermit, LoadPriority, ReadRequest, ReadScheduler};
//#endregion 🔖️Async

//#region 🔖️Http
/// @emoji 🌐️ Native `ureq`-backed `RangeTransport` — off by default so wasm builds of this
/// facade stay lean; enable via `pack`'s own `ureq` feature (forwards to `pack_http/ureq`).
#[cfg(feature = "ureq")]
pub use crate::os_pack::http::UreqRangeTransport;
pub use crate::os_pack::http::{HttpPackSource, RangeRequest, RangeResponse, RangeTransport, RetryPolicy};
//#endregion 🔖️Http

//#endregion 🔖️Reexports

//#region 🔖️Encode
/// @emoji 🚪️ Encodes `record` (validated against `spec`) into a complete `.spk` pack file's
/// bytes. Thin forward onto `crate::os_pack::value::encode_document` — see there for the canonical-mode
/// rules and the purity law (byte-identical output for a given `(spec, record)` regardless of
/// `HashMap` iteration order).
pub fn encode_document(spec: &crate::os_dsl::schema::RecordSpec, record: &crate::os_dsl::schema::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::os_pack::value::encode_document(spec, record, options)
}

/// @emoji 🚪️ Decodes a complete `.spk` pack file's bytes back into a `RecordValue`, plus a
/// `DecodeReport` describing anything the caller's `spec` didn't account for. Thin forward onto
/// `crate::os_pack::value::decode_document`.
pub fn decode_document(bytes: &[u8], spec: &crate::os_dsl::schema::RecordSpec, options: &DecodeOptions) -> Result<(crate::os_dsl::schema::RecordValue, DecodeReport), PackError> {
    crate::os_pack::value::decode_document(bytes, spec, options)
}

/// @emoji 🎯️ Encodes one record as a container-less binary body (symbol table + fields, no
/// header/manifest/footer, no chunking) — the payload form for operation/command records. Thin
/// forward onto `crate::os_pack::value::encode_record_body`; same determinism law as `encode_document`.
pub fn encode_record_body(spec: &crate::os_dsl::schema::RecordSpec, record: &crate::os_dsl::schema::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::os_pack::value::encode_record_body(spec, record, options)
}

/// @emoji 🎯️ Decodes an `encode_record_body` payload back into a `RecordValue` plus its
/// `DecodeReport`. Thin forward onto `crate::os_pack::value::decode_record_body`.
pub fn decode_record_body(bytes: &[u8], spec: &crate::os_dsl::schema::RecordSpec, options: &DecodeOptions) -> Result<(crate::os_dsl::schema::RecordValue, DecodeReport), PackError> {
    crate::os_pack::value::decode_record_body(bytes, spec, options)
}

/// @emoji #⃣ Reads only the trailing footer of an encoded pack file and returns its stored
/// `content_hash` — no header/manifest/document decode needed. Thin forward onto
/// `crate::os_pack::format::read_footer_only`.
pub fn content_hash(bytes: &[u8]) -> Result<ContentHash, PackError> {
    read_footer_only(&bytes).map(|footer| footer.content_hash)
}
//#endregion 🔖️Encode

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::os_dsl::schema::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};

    //#region 🔖️Fixtures
    /// @emoji 🧬️ A small 3-field record spec exercising a few different `Shape` variants
    /// (`Text`, `UInt`, `Bool`) — enough to prove the facade's wiring end to end without
    /// duplicating `pack_value`'s own exhaustive wire-tag coverage.
    fn sample_spec() -> RecordSpec {
        RecordSpec::new(Some("sample"), RecordLayout::Lines, vec![FieldSpec::new(1, "name", Shape::Text), FieldSpec::new(2, "age", Shape::UInt), FieldSpec::new(3, "active", Shape::Bool)])
    }

    fn sample_record() -> RecordValue {
        let mut fields = HashMap::new();
        fields.insert(1, FieldValue::Text("Ada Lovelace".to_string()));
        fields.insert(2, FieldValue::UInt(42));
        fields.insert(3, FieldValue::Bool(true));
        RecordValue { fields }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Document
    #[test]
    fn facade_encode_document_decode_document_round_trip() {
        let spec = sample_spec();
        let record = sample_record();

        let bytes = encode_document(&spec, &record, &EncodeOptions::default()).unwrap();
        let (decoded, report) = decode_document(&bytes, &spec, &DecodeOptions::default()).unwrap();

        assert_eq!(decoded.get(1), Some(&FieldValue::Text("Ada Lovelace".to_string())));
        assert_eq!(decoded.get(2), Some(&FieldValue::UInt(42)));
        assert_eq!(decoded.get(3), Some(&FieldValue::Bool(true)));
        assert!(report.unknown_field_ids.is_empty());
        assert!(!report.schema_drift);
    }

    #[test]
    fn facade_content_hash_is_stable_across_two_encodes() {
        let spec = sample_spec();
        let record = sample_record();
        let options = EncodeOptions::default();

        let bytes_a = encode_document(&spec, &record, &options).unwrap();
        let bytes_b = encode_document(&spec, &record, &options).unwrap();

        // 🔒️ `encode_document` is a pure function of `(spec, record)` — byte-identical output.
        assert_eq!(bytes_a, bytes_b);

        let hash_a = content_hash(&bytes_a).unwrap();
        let hash_b = content_hash(&bytes_b).unwrap();
        assert_eq!(hash_a, hash_b);
    }
    //#endregion 🔖️Document
}
//#endregion 🧪️Tests

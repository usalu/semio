//! 🎒️ os pack facade — the schema-driven half of the `.spk` family. The container itself (header,
//! footer, segments, manifest, chunk table, recovery, sources) lives in the product-neutral
//! `🧰️framework/🔨️modules/🎒️pack` crate; what stays here is everything that needs the os DSL
//! schema: the record value codec, the arbitrary/law testkit, and the `encode_document` family.

pub use pack::*;

#[allow(unused_imports)]
use crate::os_dsl;

//#region 🔖️Value
/// 🔢️ The schema-driven record value codec stays os-side because it speaks `os_dsl::schema`.
pub use crate::os_pack::value::*;

#[path = "🔎️scalar-witness/🦀️.rs"]
pub mod scalar_witness;
pub use scalar_witness::{ScalarRecordField, ScalarRecordView, ScalarRecordWireStep, ScalarRecordWireWitness};
//#endregion 🔖️Value

//#region 🔖️Encode
/// @emoji 🚪️ Encodes `record` (validated against `spec`) into a complete `.spk` pack file's
/// bytes. Thin forward onto `crate::value::encode_document` — see there for the canonical-mode
/// rules and the purity law (byte-identical output for a given `(spec, record)` regardless of
/// `HashMap` iteration order).
pub fn encode_document(spec: &crate::os_dsl::schema::RecordSpec, record: &crate::os_dsl::schema::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::value::encode_document(spec, record, options)
}

/// @emoji 🚪️ Decodes a complete `.spk` pack file's bytes back into a `RecordValue`, plus a
/// `DecodeReport` describing anything the caller's `spec` didn't account for. Thin forward onto
/// `crate::value::decode_document`.
pub fn decode_document(bytes: &[u8], spec: &crate::os_dsl::schema::RecordSpec, options: &DecodeOptions) -> Result<(crate::os_dsl::schema::RecordValue, DecodeReport), PackError> {
    crate::value::decode_document(bytes, spec, options)
}

/// @emoji 🎯️ Encodes one record as a container-less binary body (symbol table + fields, no
/// header/manifest/footer, no chunking) — the payload form for operation/command records. Thin
/// forward onto `crate::value::encode_record_body`; same determinism law as `encode_document`.
pub fn encode_record_body(spec: &crate::os_dsl::schema::RecordSpec, record: &crate::os_dsl::schema::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::value::encode_record_body(spec, record, options)
}

/// @emoji 🎯️ Decodes an `encode_record_body` payload back into a `RecordValue` plus its
/// `DecodeReport`. Thin forward onto `crate::value::decode_record_body`.
pub fn decode_record_body(bytes: &[u8], spec: &crate::os_dsl::schema::RecordSpec, options: &DecodeOptions) -> Result<(crate::os_dsl::schema::RecordValue, DecodeReport), PackError> {
    crate::value::decode_record_body(bytes, spec, options)
}

/// 🛂️ Admits one terminal schema-owned record without unknown fields or byte suffixes.
pub fn decode_record_body_exact(bytes: &[u8], spec: &crate::os_dsl::schema::RecordSpec, options: &DecodeOptions) -> Result<crate::os_dsl::schema::RecordValue, PackError> {
    crate::value::decode_record_body_exact(bytes, spec, options)
}

/// @emoji #⃣ Reads only the trailing footer of an encoded pack file and returns its stored
/// `content_hash` — no header/manifest/document decode needed. Thin forward onto
/// `crate::format::read_footer_only`.
pub fn content_hash(bytes: &[u8]) -> Result<ContentHash, PackError> {
    crate::os_io::resolve_ready(read_footer_only(&bytes)).map(|footer| footer.content_hash)
}
//#endregion 🔖️Encode

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

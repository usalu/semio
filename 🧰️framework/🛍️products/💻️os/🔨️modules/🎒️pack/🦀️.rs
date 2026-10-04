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
/// 🚪️ Encodes `record` (validated against `spec`) into a complete `.spk` pack file's
/// bytes. Thin forward onto `crate::value::encode_document` — see there for the canonical-mode
/// rules and the purity law (byte-identical output for a given `(spec, record)` regardless of
/// `HashMap` iteration order).
pub fn encode_document(spec: &semio_framework_dsl_record::RecordSpec, record: &semio_framework_dsl_record::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::value::encode_document(spec, record, options).map_err(PackError::from)
}

/// 🛫️ Emits declared document fields under cumulative caller-owned native output admission.
pub fn encode_document_controlled(spec:&semio_framework_dsl_record::RecordSpec,record:&semio_framework_dsl_record::RecordValue,options:&EncodeOptions,control:&mut protocol::value::native_encoding::NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{crate::value::encode_document_controlled(spec,record,options,control)}

/// 🚪️ Decodes a complete `.spk` pack file's bytes back into a `RecordValue`, plus a
/// `DecodeReport` describing anything the caller's `spec` didn't account for. Thin forward onto
/// `crate::value::decode_document`.
pub fn decode_document(bytes: &[u8], spec: &semio_framework_dsl_record::RecordSpec, options: &DecodeOptions) -> Result<(semio_framework_dsl_record::RecordValue, DecodeReport), PackError> {
    crate::value::decode_document(bytes, spec, options).map_err(PackError::from)
}
/// 🛬️ Reads a complete Pack file through caller-controlled physical and primitive decoding.
pub fn decode_document_controlled(bytes:&[u8],spec:&semio_framework_dsl_record::RecordSpec,options:&DecodeOptions,control:&mut protocol::value::native_decoding::NativeDecodeControl<'_>)->Result<(semio_framework_dsl_record::RecordValue,DecodeReport),PackRefusal>{
    crate::value::decode_document_controlled(bytes,spec,options,control)
}

/// 🎯️ Encodes one record as a container-less binary body (symbol table + fields, no
/// header/manifest/footer, no chunking) — the payload form for operation/command records. Thin
/// forward onto `crate::value::encode_record_body`; same determinism law as `encode_document`.
pub fn encode_record_body(spec: &semio_framework_dsl_record::RecordSpec, record: &semio_framework_dsl_record::RecordValue, options: &EncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::value::encode_record_body(spec, record, options).map_err(PackError::from)
}

/// 🛫️ Emits a terminal record through the same control as its typed field projection.
pub fn encode_record_body_controlled(spec:&semio_framework_dsl_record::RecordSpec,record:&semio_framework_dsl_record::RecordValue,options:&EncodeOptions,control:&mut protocol::value::native_encoding::NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{crate::value::encode_record_body_controlled(spec,record,options,control)}

/// 🎯️ Decodes an `encode_record_body` payload back into a `RecordValue` plus its
/// `DecodeReport`. Thin forward onto `crate::value::decode_record_body`.
pub fn decode_record_body(bytes: &[u8], spec: &semio_framework_dsl_record::RecordSpec, options: &DecodeOptions) -> Result<(semio_framework_dsl_record::RecordValue, DecodeReport), PackError> {
    crate::value::decode_record_body(bytes, spec, options).map_err(PackError::from)
}

/// 🛂️ Admits one terminal schema-owned record without unknown fields or byte suffixes.
pub fn decode_record_body_exact(bytes: &[u8], spec: &semio_framework_dsl_record::RecordSpec, options: &DecodeOptions) -> Result<semio_framework_dsl_record::RecordValue, PackError> {
    crate::value::decode_record_body_exact(bytes, spec, options).map_err(PackError::from)
}

/// 🛬️ Parses a terminal record with the same caller control used for typed construction.
pub fn decode_record_body_exact_controlled(bytes:&[u8],spec:&semio_framework_dsl_record::RecordSpec,options:&DecodeOptions,control:&mut protocol::value::native_decoding::NativeDecodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,PackRefusal>{
    crate::value::decode_record_body_exact_controlled(bytes,spec,options,control)
}

/// #⃣ Reads only the trailing footer of an encoded pack file and returns its stored
/// `content_hash` — no header/manifest/document decode needed. Thin forward onto
/// `crate::format::read_footer_only`.
pub fn content_hash(bytes: &[u8]) -> Result<ContentHash, PackError> {
    ::semio_framework_async::poll::resolve_ready(read_footer_only(&bytes)).map(|footer| footer.content_hash).map_err(PackError::from)
}
//#endregion 🔖️Encode

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

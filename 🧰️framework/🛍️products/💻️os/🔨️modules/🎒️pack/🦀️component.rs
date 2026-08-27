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

#[path = "🔎️scalar-witness/🦀️component.rs"]
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

/// @emoji #⃣ Reads only the trailing footer of an encoded pack file and returns its stored
/// `content_hash` — no header/manifest/document decode needed. Thin forward onto
/// `crate::format::read_footer_only`.
pub fn content_hash(bytes: &[u8]) -> Result<ContentHash, PackError> {
    crate::os_io::resolve_ready(read_footer_only(&bytes)).map(|footer| footer.content_hash)
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

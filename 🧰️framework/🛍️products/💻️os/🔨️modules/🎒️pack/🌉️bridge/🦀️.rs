//! 🧵️ Thin runtime bridge to `crate::{encode_document, decode_document}`, resolved as
//! `::semio_framework_os_kernel::os_pack::pack_rt::...` by `dsl_derive`'s generated `ArtifactPack` impl (app crates depend on
//! `vcs`, never on `pack` directly — same seam `::crate::os_dsl::RecordSpec`/`RecordValue` already use). Also
//! hosts the schema-less `DslValue` bridge behind `impl ArtifactPack for DslValue` below.

use super::{DecodeOptions as PackDecodeOptions, EncodeOptions as PackEncodeOptions, PackError, PackRefusal};
use semio_framework_value::DslValue;
use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::Number;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::Shape;
use std::collections::HashMap;

pub use crate::{RetainedValueContainer, RetainedValueCursor, RetainedValueRole, RetainedValueToken};
pub use pack::{
    MAGIC, RETAINED_PACK_MAXIMUM_PAGES, RETAINED_PACK_PAGE_BYTES, RetainedPackAnchorCursor, RetainedPackCatalog, RetainedPackCatalogAllocationError, RetainedPackCatalogAllocationStep, RetainedPackCatalogCursor, RetainedPackCatalogEvent,
    RetainedPackCatalogFault, RetainedPackCatalogProgress, RetainedPackCloseStep, RetainedPackPage, RetainedPackSegmentCursor, RetainedPackSegmentEvent, RetainedPackSourceAllocationError, RetainedPackSourceAllocationStep,
    RetainedPackSourceCursor, RetainedPackSourceEvent, RetainedPackSourceProgress,
};

/// 🚪️ Forwards to `crate::encode_document`.
pub fn encode_document(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::encode_document(spec, record, options)
}

/// 🛫️ Emits complete Pack documents under the caller's cumulative ownership control.
pub fn encode_document_controlled(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Vec<u8>, PackRefusal> {
    crate::encode_document_controlled(spec, record, options, control)
}

/// 🚪️ Forwards to `crate::decode_document`.
pub fn decode_document(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::DecodeReport), PackError> {
    crate::decode_document(bytes, spec, options)
}
/// 🛬️ Keeps complete physical Pack reading and typed reconstruction on one caller control.
pub fn decode_document_controlled(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(RecordValue, crate::DecodeReport), PackRefusal> {
    crate::decode_document_controlled(bytes, spec, options, control)
}

/// 🎯️ P6: container-less record body helpers for handcrafted OpBinary impls.
pub fn encode_record_body(spec: &RecordSpec, record: &RecordValue, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
    crate::encode_record_body(spec, record, options)
}
pub fn decode_record_body(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions) -> Result<(RecordValue, crate::DecodeReport), PackError> {
    crate::decode_record_body(bytes, spec, options)
}
/// 🛬️ Keeps terminal Pack parsing and typed construction under one ownership control.
pub fn decode_record_body_exact_controlled(bytes: &[u8], spec: &RecordSpec, options: &PackDecodeOptions, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<RecordValue, PackRefusal> {
    crate::decode_record_body_exact_controlled(bytes, spec, options, control)
}
pub fn write_varint_u64(out: &mut Vec<u8>, value: u64) {
    crate::write_varint_u64(out, value);
}
pub use crate::ByteReader;
/// 🎯️ Format byte every encoded operation starts with (handcrafted OpBinary convention).
pub const OP_BINARY_FORMAT: u8 = 1;

/// 🌱️ Field id the JSON bridge's synthetic single-field record wraps a whole
/// `serde_json::Value` payload in — mirrors `crate::os_dsl::DslField for serde_json::Value`'s
/// `Shape::Value` escape hatch (`dsl/rs/lib.rs`), lifted one level from "one field" to "one
/// whole document" so schema-less apps (puzzle plugins, semio_compose_rs kit) get a pack encoding too.
const VALUE_BRIDGE_FIELD_ID: u16 = 1;

fn value_bridge_spec() -> RecordSpec {
    RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, vec![semio_framework_dsl_record::FieldSpec::new(VALUE_BRIDGE_FIELD_ID, "value", semio_framework_dsl_record::Shape::Value)])
}

/// 🌱️ Encodes an arbitrary `DslValue` as a complete pack file.
pub fn encode_pack_value(value: &DslValue) -> Vec<u8> {
    let mut fields = semio_framework_dsl_record::RecordFields::default();
    fields.insert(VALUE_BRIDGE_FIELD_ID, semio_framework_dsl_record::FieldValue::Value(value.clone()));
    let record = RecordValue { fields };
    encode_document(&value_bridge_spec(), &record, &PackEncodeOptions::default()).expect("value bridge encode is infallible for a well-formed DslValue")
}

/// 🌱️ Inverse of `encode_pack_value`.
pub fn decode_pack_value(bytes: &[u8]) -> Result<DslValue, PackError> {
    let (record, _report) = decode_document(bytes, &value_bridge_spec(), &PackDecodeOptions::default())?;
    match record.get(VALUE_BRIDGE_FIELD_ID) {
        Some(semio_framework_dsl_record::FieldValue::Value(dsl_value)) => Ok(dsl_value.clone()),
        _ => Ok(DslValue::Null),
    }
}

/// 🪶️ Container-less twin of `encode_pack_value` for per-message wire payloads.
pub fn encode_wire_value(value: &DslValue) -> Vec<u8> {
    let mut fields = semio_framework_dsl_record::RecordFields::default();
    fields.insert(VALUE_BRIDGE_FIELD_ID, semio_framework_dsl_record::FieldValue::Value(value.clone()));
    let record = RecordValue { fields };
    crate::encode_record_body(&value_bridge_spec(), &record, &PackEncodeOptions::default()).expect("wire value encode is infallible for a well-formed DslValue")
}

/// 🪶️ Inverse of `encode_wire_value`.
pub fn decode_wire_value(bytes: &[u8]) -> Result<DslValue, PackRefusal> {
    decode_wire_value_with_options(bytes, &PackDecodeOptions::default())
}

/// 🛡️ Decodes one exact wire value using caller-owned limits without a second tree clone.
pub fn decode_wire_value_with_options(bytes: &[u8], options: &PackDecodeOptions) -> Result<DslValue, PackRefusal> {
    crate::decode_value_record_body_exact(bytes, VALUE_BRIDGE_FIELD_ID, &options.limits)
}

/// 🎞️ Encodes the borrowed intrinsic owner under the caller's cumulative output control.
pub fn encode_wire_value_controlled(value:&DslValue,options:&PackEncodeOptions,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{
    crate::encode_value_record_body_controlled(VALUE_BRIDGE_FIELD_ID,value,options,control)
}

/// 🎞️ Materializes the full terminal intrinsic owner under the caller's decode control.
pub fn decode_wire_value_controlled(bytes:&[u8],options:&PackDecodeOptions,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    crate::decode_value_record_body_exact_controlled(bytes,VALUE_BRIDGE_FIELD_ID,options,control)
}


/// 🎞️ Appends the exact declared intrinsic field to the caller's owned operation sink.
pub fn encode_wire_value_into(value:&DslValue,options:&PackEncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<usize,PackRefusal>{
    crate::record::encode_value_record_body_into(VALUE_BRIDGE_FIELD_ID,value,options,output,control)
}

/// 🎞️ Decodes the exact declared intrinsic field by borrowing the complete caller source.
pub fn decode_wire_value_span(bytes:crate::ByteSpan<'_>,options:&PackDecodeOptions,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    crate::record::decode_value_record_body_span_exact_controlled(bytes,VALUE_BRIDGE_FIELD_ID,options,control)
}

/// 🧩️ Compose-only bridge — external technology; converts through `DslValue` without JSON on the wire.
pub fn encode_json_value(value: &serde_json::Value) -> Vec<u8> {
    encode_pack_value(&json_value_to_dsl(value))
}

/// 🧩️ Compose-only inverse of `encode_json_value`.
pub fn decode_json_value(bytes: &[u8]) -> Result<serde_json::Value, PackError> {
    decode_pack_value(bytes).map(dsl_value_to_json)
}

/// 📦️ Prefix for base64-wrapped pack bytes in scene `*Json` string slots (TS `PACK_B64_PREFIX`).
pub const PACK_B64_PREFIX: &str = "pk:";

/// 📦️ Lossless pack snapshot as a `pk:`-prefixed base64 string. RFC 4648 standard
/// alphabet via the first-party `semio-framework-io-base64` codec — this call site is
/// guest-reachable (scene `*Json` field encoding), so it is re-pointed at the codec rather than
/// target-gated away like this crate's genuinely host-only third-party crates.
pub fn pack_value_to_base64(bytes: &[u8]) -> String {
    format!("{}{}", PACK_B64_PREFIX, semio_framework_io_base64::base64_standard_encode(bytes))
}

/// 📥️ Inverse of [`pack_value_to_base64`].
pub fn pack_value_from_base64(encoded: &str) -> Result<Vec<u8>, PackError> {
    let payload = encoded.strip_prefix(PACK_B64_PREFIX).ok_or(PackError::Refusal(PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack base64", offset: 0, detail: "missing pk: prefix".into() }))?;
    semio_framework_io_base64::base64_standard_decode(payload).map_err(|error| PackError::Refusal(PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "pack base64", offset: 0, detail: error.to_string() }))
}

/// 🎬️ Decodes a component-scene `*Json` field when it carries [`pack_value_to_base64`] bytes.
pub fn decode_scene_pack_field(encoded: &str) -> Result<DslValue, PackError> {
    if encoded.starts_with(PACK_B64_PREFIX) {
        decode_pack_value(&pack_value_from_base64(encoded)?)
    } else {
        Ok(json_value_to_dsl(&serde_json::from_str(encoded).map_err(|error| PackError::Refusal(PackRefusal::Malformed { kind: semio_framework_value::ValueRefusalKind::InvalidValue, what: "scene field", offset: 0, detail: error.to_string() }))?))
    }
}

/// 🎬️ Expands a scene `*Json` slot to JSON text for engines that still ingest stringified payloads.
pub fn scene_field_json_text(field: &str) -> Result<String, PackError> {
    if field.starts_with(PACK_B64_PREFIX) {
        let dsl = decode_pack_value(&pack_value_from_base64(field)?)?;
        Ok(serde_json::to_string(&dsl_value_to_json(dsl)).unwrap_or_else(|_| "null".into()))
    } else {
        Ok(field.to_string())
    }
}

/// 🧩️ Compose wire decode helper — renormalizes a `serde_json::Value` tree after pack decode.
// 🚫️async: E1 pure transform pipeline consumed by json_values_equal's `==` (external PartialEq) — see R9
pub fn renormalize_json_wire_value(value: serde_json::Value) -> serde_json::Value {
    dsl_value_to_json(renormalize_whole_number_floats(json_value_to_dsl(&value)))
}

// 🚫️async: E1 pure recursive transform, consumed by json_values_equal's `==` (external PartialEq) — see R9
fn json_value_to_dsl(value: &serde_json::Value) -> DslValue {
    DslValue::from(value)
}

// 🚫️async: E1 pure recursive transform, consumed by json_values_equal's `==` (external PartialEq) — see R9
pub fn dsl_value_to_json(value: DslValue) -> serde_json::Value {
    serde_json::Value::from(value)
}

/// ⚖️ Semantic JSON value equality — normalizes numeric representation (`3` vs `3.0`), a
/// distinction `DslValue`'s own `Number::UInt`/`Int`/`Float` fidelity now deliberately preserves
/// (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01) — so this compares
/// through [`renormalize_whole_number_floats`] on both sides rather than plain `DslValue` equality.
// 🚫️async: E1 pure predicate; a fleet PartialEq::eq impl calls this synchronously — see R9
pub fn json_values_equal(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    renormalize_whole_number_floats(json_value_to_dsl(a)) == renormalize_whole_number_floats(json_value_to_dsl(b))
}

/// 🔧️ Rewrites fractionless floats in a `DslValue` tree to their exact integer variant
/// (`Number::UInt`/`Number::Int`) so a `3.0` reaching this path (e.g. through a plugin/JS
/// boundary that has no integer type) compares equal to a genuine `3`.
// 🚫️async: E1 pure recursive transform, consumed by json_values_equal's `==` (external PartialEq) — see R9
pub fn renormalize_whole_number_floats(value: DslValue) -> DslValue {
    match value {
        DslValue::Number(semio_framework_value::Number::Float(n)) if n.fract() == 0.0 && n.is_finite() && n.abs() < (1u64 << 53) as f64 => {
            if n >= 0.0 {
                DslValue::Number(semio_framework_value::Number::UInt(n as u64))
            } else {
                DslValue::Number(semio_framework_value::Number::Int(n as i64))
            }
        }
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(renormalize_whole_number_floats).collect()),
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(k, v)| (k, renormalize_whole_number_floats(v))).collect()),
        other => other,
    }
}

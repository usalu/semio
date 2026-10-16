//! 📦️ `pack_value` — DSL schema-aware wire encoding/decoding of `semio_framework_dsl_record::RecordValue`
//! documents into the `pack_format` binary container. Implements every wire tag (0x00-0x16),
//! canonical-mode determinism (sorted field ids, omitted `Absent`, sorted map keys, minimal
//! varints, exact IEEE words, deterministic string interning, mandatory packed numeric
//! forms), `TableSoA` columnar encoding for `Shape::Table`, unknown-field preservation via
//! `DecodeReport`, `schema_hash`, and the top-level `encode_document`/`decode_document` entry
//! points every other `pack_*`/`vcs`/`dsl_derive` crate calls through.
//!
//! See the `## pack_value` section of the wave-0 contract at
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PACK-BINARY-DOCUMENT-LAYER-ACROSS-ALL-APPS/contract.md` for the binding
//! byte layout this module implements against.

#[cfg(test)]
use semio_framework_value::DslValue;
use semio_framework_pack_error::PackRefusal;
#[cfg(test)]
use semio_framework_dsl_record::{FieldValue, RecordSpec, RecordValue, Shape, WireEdgeLabel, WireNode, WireValue};
#[cfg(test)]
use semio_framework_pack_record_schema::schema_hash;
#[cfg(test)]
use semio_framework_pack_record_encode::{encode_record_body_controlled, encode_value_record_body_controlled};

#[path = "🧾️canonical-forms/🦀️.rs"]
mod canonical_forms;
use canonical_forms::{EncCtx, build_symbols, check_depth, encode_string, encode_string_inline, homogeneous_numeric_kind, is_tuple_shape, record_spec_of, write_symref_forced};

#[path = "📝️record-encode/🦀️.rs"]
mod record_encode;
use record_encode::{encode_record_fields, encode_value};

#[path = "📖️record-decode/🦀️.rs"]
mod record_decode;
use record_decode::{ControlledMaterialization, DecCtx, DecSource, MaterializationAdmission, ValueMaterialization, decode_dsl_value, decode_record_fields, decode_value, resolve_symref};

#[path = "🧮️table-soa/🦀️.rs"]
mod table_soa;
use table_soa::{decode_table_soa, encode_table};

#[path = "🚪️document-codec/🦀️.rs"]
mod document_codec;
pub use document_codec::{DecodeReport, decode_document, decode_document_controlled, decode_record_body, decode_record_body_exact, decode_record_body_exact_controlled, decode_record_body_span_exact_controlled, decode_value_record_body_exact, decode_value_record_body_exact_controlled, decode_value_record_body_span_exact_controlled, encode_document, encode_record_body};
/// 🎛️ First-party Pack policy is owned by the shared transport contract.
pub use semio_framework_replication_codec::{PackEncodeOptions as EncodeOptions,PackDecodeOptions as DecodeOptions};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

#[path = "🔎️scalar-witness/🦀️.rs"]
pub mod scalar_witness;

#[path="🌳️intrinsic/🦀️.rs"]
pub mod intrinsic;


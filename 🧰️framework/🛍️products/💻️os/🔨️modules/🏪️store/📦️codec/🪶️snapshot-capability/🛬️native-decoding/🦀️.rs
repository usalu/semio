//! 🛬️ Controlled physical record decoding for explicitly declared snapshot owners.
use super::{PackDecodeOptions, TextError};
use crate::os_dsl::{Limits, NativeDecodeControl, ParseOptions, RecordSpecProducer, RecordValue, SourceMode};
use crate::sqlite_snapshot::{SqliteSnapshotControl, SqliteSnapshotPhase};
use semio_framework_value::native_decoding::NativeDecodeProgress;

/// 🧮️ Preserves one cumulative ownership budget through borrowed envelope, physical parser and typed binding.
pub fn decode_sqlite_snapshot_record_native<T>(
    payload: &crate::io_schema::IoPayload,
    envelope_id: &str,
    spec: RecordSpecProducer,
    construct: impl FnOnce(&RecordValue, &mut NativeDecodeControl<'_>) -> Result<T, TextError>,
    control: &mut SqliteSnapshotControl<'_>,
) -> Result<T, String> {
    let limits = control.limits();
    control.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, 0)?;
    let length = match payload { crate::io_schema::IoPayload::Binary(bytes) => bytes.len(), crate::io_schema::IoPayload::Text(text) => text.len() };
    if length > limits.max_file_bytes { return Err("native snapshot input exceeds file byte limit".into()); }
    let mut progress = |event: NativeDecodeProgress| control.checkpoint(SqliteSnapshotPhase::DecodeNative, event.completed, event.total).is_ok();
    let mut native = NativeDecodeControl::new(limits.max_value_bytes, &mut progress);
    let spec = spec.decode(&mut native)?;
    let record = match payload {
        crate::io_schema::IoPayload::Binary(bytes) => {
            let body = super::semio_format::unwrap_binary_controlled(bytes, envelope_id, super::semio_format::Component::Pack, 1, &mut native).map_err(|error| error.to_string())?;
            super::pack_rt::decode_document_controlled(body, &spec, &PackDecodeOptions::default(), &mut native).map_err(|error| error.to_string())?.0
        }
        crate::io_schema::IoPayload::Text(text) => {
            let body = super::semio_format::split_text_preamble_controlled(text, envelope_id, super::semio_format::Component::Dsl, 1, &mut native).map_err(|error| error.to_string())?;
            crate::os_dsl::schema::parse_exact_controlled(body, &spec, &ParseOptions { limits: Limits { max_bytes: limits.max_file_bytes, ..Limits::default() }, mode: SourceMode::Document }, &mut native).map_err(|error| error.message)?
        }
    };
    construct(&record, &mut native).map_err(|error| error.message)
}

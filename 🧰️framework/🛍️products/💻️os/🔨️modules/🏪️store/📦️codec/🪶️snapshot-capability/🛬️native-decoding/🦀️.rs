//! 🛬️ Controlled physical record decoding for explicitly declared snapshot owners.
use super::{PackDecodeOptions, TextError};
use semio_framework_value::NativeDecodeControl;
use semio_framework_dsl_record::ParseOptions;
use semio_framework_dsl_record::RecordSpecProducer;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::SourceMode;
use crate::sqlite_snapshot::{SqliteSnapshotControl, SqliteSnapshotPhase};
use crate::sqlite_snapshot::{ValueError, ValueRefusalKind};
use semio_framework_diagnostic::Limits;
use semio_framework_value::native_decoding::NativeDecodeProgress;

/// 🧮️ Preserves one cumulative ownership budget through borrowed envelope, physical parser and typed binding.
pub fn decode_sqlite_snapshot_record_native<T>(
    payload: &crate::io_schema::IoPayload,
    envelope_id: &str,
    spec: RecordSpecProducer,
    construct: impl FnOnce(&RecordValue, &mut NativeDecodeControl<'_>) -> Result<T, ValueError>,
    control: &mut SqliteSnapshotControl<'_>,
) -> Result<T, ValueError> {
    let limits = control.limits();
    control.checkpoint(SqliteSnapshotPhase::DecodeNative, 0, 0)?;
    let length = match payload {
        crate::io_schema::IoPayload::Binary(bytes) => bytes.len(),
        crate::io_schema::IoPayload::Text(text) => text.len(),
    };
    if length > limits.max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native snapshot input exceeds file byte limit"));
    }
    control.allocation_stage(SqliteSnapshotPhase::DecodeNative, |remaining, checkpoint| {
        let mut progress = |event: NativeDecodeProgress| checkpoint(event.completed, event.total);
        let mut native = semio_framework_value::NativeDecodeControl::new(remaining, &mut progress);
        let result = (|| -> Result<T, ValueError> {
            let spec = spec.decode(&mut native)?;
            let record = match payload {
                crate::io_schema::IoPayload::Binary(bytes) => {
                    let body = super::semio_format::unwrap_binary_controlled(bytes, envelope_id, super::semio_format::Component::Pack, 1, &mut native).map_err(super::semio_format::SemioError::into_value_error)?;
                    pack::record::decode_document_controlled(body, &spec, &pack::record::DecodeOptions::default(), &mut native).map_err(super::PackRefusal::into_value_error)?.0
                }
                crate::io_schema::IoPayload::Text(text) => {
                    let body = super::semio_format::split_text_preamble_controlled(text, envelope_id, super::semio_format::Component::Dsl, 1, &mut native).map_err(super::semio_format::SemioError::into_value_error)?;
                    semio_framework_dsl_record::parse_exact_controlled(body, &spec, &ParseOptions { limits: Limits { max_bytes: limits.max_file_bytes, ..Limits::default() }, mode: semio_framework_dsl_record::SourceMode::Document }, &mut native)
                        .map_err(|error| ValueError::new(error.kind, error.message))?
                }
            };
            construct(&record, &mut native)
        })();
        (result, native.owned_bytes())
    })?
}

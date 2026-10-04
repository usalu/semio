//! 🛫️ Declared native record output with one caller-owned cumulative controller.
use semio_framework_dsl_record::JoinMode;
use semio_framework_value::NativeEncodeControl;
use semio_framework_dsl_record::RecordSpecProducer;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::native_encoding::EncodedRecord;
use crate::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotControl, SqliteSnapshotPhase};
use crate::sqlite_snapshot::{ValueError, ValueRefusalKind};
use semio_framework_diagnostic::TextError;
use semio_framework_value::native_encoding::NativeEncodeProgress;

/// 🧮️ Projects and emits only the owner's declared controlled record factories.
pub fn encode_sqlite_snapshot_record_native(
    encoding: SnapshotEncoding,
    envelope_id: &str,
    spec: RecordSpecProducer,
    construct: impl FnOnce(&mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError>,
    control: &mut SqliteSnapshotControl<'_>,
) -> Result<crate::io_schema::IoPayload, ValueError> {
    let limits = control.limits();
    control.checkpoint(SqliteSnapshotPhase::EncodeNative, 0, 0)?;
    let component = match encoding {
        SnapshotEncoding::Binary => super::semio_format::Component::Pack,
        SnapshotEncoding::Text => super::semio_format::Component::Dsl,
    };
    let body_limit =
        limits.max_file_bytes.checked_sub(super::semio_format::declared_envelope_prefix_len(envelope_id, component, 1)?).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "native file ceiling cannot contain its declared envelope"))?;
    control.allocation_stage(SqliteSnapshotPhase::EncodeNative, |remaining, checkpoint| {
        let mut progress = |event: NativeEncodeProgress| checkpoint(event.completed, event.total);
        let mut native = semio_framework_value::NativeEncodeControl::new(remaining, &mut progress);
        let result = (|| -> Result<crate::io_schema::IoPayload, ValueError> {
            let spec = spec.encode(&mut native)?;
            let record = semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(construct(&mut native)?);
            let output = match encoding {
                SnapshotEncoding::Binary => {
                    let mut options = pack::record::EncodeOptions::default();
                    options.limits.max_file_len = body_limit as u64;
                    let body = pack::record::encode_document_controlled(&spec, record.as_record(), &options, &mut native).map_err(super::PackRefusal::into_value_error)?;
                    crate::io_schema::IoPayload::Binary(super::semio_format::wrap_binary_controlled(envelope_id, component, 1, &body, &mut native)?)
                }
                SnapshotEncoding::Text => {
                    let body = semio_framework_dsl_record::print_controlled(record.as_record(), &spec, semio_framework_dsl_record::JoinMode::Document, body_limit, &mut native).map_err(|error| ValueError::new(error.kind, error.message))?;
                    crate::io_schema::IoPayload::Text(super::semio_format::wrap_text_controlled(envelope_id, component, 1, &body, &mut native)?)
                }
            };
            let length = match &output {
                crate::io_schema::IoPayload::Binary(bytes) => bytes.len(),
                crate::io_schema::IoPayload::Text(text) => text.len(),
            };
            if length > limits.max_file_bytes {
                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native snapshot output exceeds file byte limit"));
            }
            Ok(output)
        })();
        (result, native.owned_bytes())
    })?
}

//! 💰️ PPTX complete literal native fields settle one persistent backing stage.
use super::*;
use semio_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
use semio_os_kernel::os_store::serialization::ArtifactSqliteSnapshotHost;
use semio_framework_value::{DecodedValue, NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
fn retire(value: PptxSnapshot) {
    value.retire_sqlite_snapshot();
}
fn measured(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, limits: SqliteDatabaseLimits, native: &mut NativeEncodeControl<'_>) -> Result<usize, ValueError> {
    native.begin_stage(0)?;
    let mut writer = OpcNativeWriter { control: native, output: None, count: 0, rows: 0, limits, encoding };
    write(snapshot, &mut writer)?;
    let count = writer.count;
    native.checkpoint()?;
    Ok(count)
}
pub(super) fn preflight(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    let limits = control.limits();
    control.allocation_stage(SqliteSnapshotPhase::EncodeNative, |remaining, checkpoint, allocation| {
        let mut callback = |event: semio_framework_value::native_encoding::NativeEncodeProgress| checkpoint(event.completed, event.total);
        let mut native = NativeEncodeControl::new_forwarded(remaining, &mut callback, allocation);
        let result = (|| {
            let count = measured(snapshot, encoding, limits, &mut native)?;
            if count > remaining.saturating_sub(native.owned_bytes()) {
                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "PPTX prospective native output exceeds physical allowance"));
            }
            native.checkpoint()
        })();
        (result, native.owned_bytes())
    })?
}
pub(in crate::standards::v_ecma_376::subsets::base::io) fn encode(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, limits: SqliteDatabaseLimits, native: &mut NativeEncodeControl<'_>) -> Result<IoPayload, ValueError> {
    let total = measured(snapshot, encoding, limits, native)?;
    let mut bytes = native.allocate_vec::<u8>(total)?;
    native.begin_stage(total)?;
    let mut writer = OpcNativeWriter { control: native, output: Some(&mut bytes), count: 0, rows: 0, limits, encoding };
    write(snapshot, &mut writer)?;
    if writer.count != total || bytes.len() != total {
        return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "PPTX measured native fields differ from emitted fields"));
    }
    native.checkpoint()?;
    match encoding {
        SnapshotEncoding::Binary => Ok(IoPayload::Binary(bytes)),
        SnapshotEncoding::Text => String::from_utf8(bytes).map(IoPayload::Text).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "PPTX native output is not UTF8")),
    }
}
pub(in crate::standards::v_ecma_376::subsets::base::io) fn input(bytes: &[u8], binary: bool, limits: SqliteDatabaseLimits, native: &mut NativeDecodeControl<'_>) -> Result<PptxSnapshot, ValueError> {
    if bytes.len() > limits.max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "PPTX native input exceeds caller file ceiling"));
    }
    let body = if binary {
        store::semio_format::unwrap_binary_controlled(bytes, "stdio.pptx", store::semio_format::Component::Pack, 1, native)?
    } else {
        let text = native.borrow_text(bytes)?;
        store::semio_format::split_text_preamble_controlled(text, "stdio.pptx", store::semio_format::Component::Dsl, 1, native).map_err(store::semio_format::SemioError::into_value_error)?.as_bytes()
    };
    native.begin_stage(body.len())?;
    let snapshot = DecodedValue::new(read(&mut OpcNativeReader { bytes: body, position: 0, rows: 0, limits, binary, control: native })?, retire);
    native.checkpoint()?;
    Ok(snapshot.take())
}

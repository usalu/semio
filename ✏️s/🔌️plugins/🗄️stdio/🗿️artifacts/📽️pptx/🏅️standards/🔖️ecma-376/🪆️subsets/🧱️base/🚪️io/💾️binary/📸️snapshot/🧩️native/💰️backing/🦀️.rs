//! 💰️ PPTX complete literal native fields settle one persistent backing stage.
use super::*;
use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
use semio_framework_os_kernel::ArtifactSqliteSnapshot;
use semio_framework_value::{DecodedValue, NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};
fn retire(value: PptxSnapshot) {
    value.retire_sqlite_snapshot();
}
fn measured(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, limits: SqliteDatabaseLimits, native: &mut NativeEncodeControl<'_>) -> Result<usize, ValueError> {
    native.begin_stage(0)?;
    let mut writer = OpcNativeWriter { control: native, output: None, count: 0, rows: 0, limits, encoding };
    super::write(snapshot, &mut writer)?;
    let count = writer.count;
    native.checkpoint()?;
    Ok(count)
}
pub(super) fn preflight(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
    let limits = control.limits();
    control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative, |remaining, checkpoint| {
        let mut callback = |event: semio_framework_value::native_encoding::NativeEncodeProgress| checkpoint(event.completed, event.total);
        let mut native = NativeEncodeControl::new(remaining, &mut callback);
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
pub(crate) fn encode(snapshot: &PptxSnapshot, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>,native_control: &mut NativeEncodeControl<'_>) -> Result<IoPayload, ValueError> {
    let limits = control.limits();
    control.allocation_stage_native(SqliteSnapshotPhase::EncodeNative, |remaining, checkpoint| {
        let native_before=native_control.owned_bytes();
    let result=native_control.scoped_maximum(match native_before.checked_add(remaining) {Some(maximum) => maximum, None => return (Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native snapshot allowance overflow")), 0)}, |native| {native.scoped_observer(&mut |event:semio_framework_value::native_encoding::NativeEncodeProgress|checkpoint(event.completed,event.total),|native|{

        let result = (|| {
            let total = measured(snapshot, encoding, limits, native)?;
            let mut bytes = native.allocate_vec::<u8>(total)?;
            native.begin_stage(total)?;
            let mut writer = OpcNativeWriter { control: native, output: Some(&mut bytes), count: 0, rows: 0, limits, encoding };
            super::write(snapshot, &mut writer)?;
            if writer.count != total || bytes.len() != total {
                return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "PPTX measured native fields differ from emitted fields"));
            }
            native.checkpoint()?;
            match encoding {
                SnapshotEncoding::Binary => Ok(IoPayload::Binary(bytes)),
                SnapshotEncoding::Text => String::from_utf8(bytes).map(IoPayload::Text).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "PPTX native output is not UTF8")),
            }
        })();
        result
    })});
    (result,native_control.owned_bytes().saturating_sub(native_before))
    })?
}
pub(crate) fn input(bytes: &[u8], binary: bool, control: &mut SqliteSnapshotControl<'_>, native_control: &mut NativeDecodeControl<'_>) -> Result<PptxSnapshot, ValueError> {
    let limits = control.limits();
    if bytes.len() > limits.max_file_bytes {
        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "PPTX native input exceeds caller file ceiling"));
    }
    control.allocation_stage_native(SqliteSnapshotPhase::DecodeNative, |remaining, checkpoint| {
        let native_before = native_control.owned_bytes();
        let result = native_control.scoped_maximum(match native_before.checked_add(remaining) { Some(maximum) => maximum, None => return (Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "native snapshot allowance overflow")), 0) }, |native| {
            native.scoped_observer(&mut |event: semio_framework_value::native_decoding::NativeDecodeProgress| checkpoint(event.completed, event.total), |native| {
                (|| {
                    let body = if binary {
                        store::semio_format::unwrap_binary_controlled(bytes, "stdio.pptx", store::semio_format::Component::Pack, 1, native)?
                    } else {
                        let text = native.borrow_text(bytes)?;
                        store::semio_format::split_text_preamble_controlled(text, "stdio.pptx", store::semio_format::Component::Dsl, 1, native).map_err(store::semio_format::SemioError::into_value_error)?.as_bytes()
                    };
                    native.begin_stage(body.len())?;
                    let snapshot = DecodedValue::new(super::read(&mut OpcNativeReader { bytes: body, position: 0, rows: 0, limits, binary, control: native })?, retire);
                    native.checkpoint()?;
                    Ok(snapshot.take())
                })()
            })
        });
        (result, native_control.owned_bytes().saturating_sub(native_before))
    })?
}
